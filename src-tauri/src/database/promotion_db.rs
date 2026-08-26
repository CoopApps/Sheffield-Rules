/// Database operations for promotion and relegation system
/// Handles reading standings, updating club divisions, and recording movement history

use sqlx::SqlitePool;
use uuid::Uuid;
use crate::sheffield_rules::promotion::Standing;

/// Get final standings for a division in a given season
pub async fn get_division_standings(
    pool: &SqlitePool,
    division_id: &str,
    season: u16,
) -> Result<Vec<Standing>, sqlx::Error> {
    let results = sqlx::query_as::<_, (String, String, i32, i32, i32, i32, i32, i32, i32, i32)>(
        "SELECT
            sc.id,
            sc.name,
            ss.position,
            COALESCE(ss.played, 0),
            COALESCE(ss.won, 0),
            COALESCE(ss.drawn, 0),
            COALESCE(ss.lost, 0),
            COALESCE(ss.goals_for, 0),
            COALESCE(ss.goals_against, 0),
            COALESCE(ss.points, 0)
        FROM sheffield_league_clubs slc
        JOIN sheffield_clubs sc ON slc.club_id = sc.id
        LEFT JOIN sheffield_standings ss ON sc.id = ss.club_id AND ss.season = ?
        WHERE slc.division_id = ?
        ORDER BY ss.position ASC, sc.name ASC"
    )
    .bind(season)
    .bind(division_id)
    .fetch_all(pool)
    .await?;

    Ok(results
        .into_iter()
        .enumerate()
        .map(|(idx, (club_id, club_name, position, played, won, drawn, lost, gf, ga, points))| {
            Standing {
                club_id,
                club_name,
                position: position.max(1), // Default to position if not set
                played,
                won,
                drawn,
                lost,
                goals_for: gf,
                goals_against: ga,
                points,
            }
        })
        .collect())
}

/// Simpler version that returns Vec directly
pub async fn get_division_standings_simple(
    pool: &SqlitePool,
    division_id: &str,
    season: u16,
) -> Result<Vec<Standing>, sqlx::Error> {
    use sqlx::Row;

    let results = sqlx::query(
        "SELECT
            sc.id,
            sc.name,
            slc.position_in_division,
            COALESCE(ss.played, 0) as played,
            COALESCE(ss.won, 0) as won,
            COALESCE(ss.drawn, 0) as drawn,
            COALESCE(ss.lost, 0) as lost,
            COALESCE(ss.goals_for, 0) as goals_for,
            COALESCE(ss.goals_against, 0) as goals_against,
            COALESCE(ss.points, 0) as points
        FROM sheffield_league_clubs slc
        JOIN sheffield_clubs sc ON slc.club_id = sc.id
        LEFT JOIN sheffield_standings ss ON sc.id = ss.club_id AND ss.season = ?
        WHERE slc.division_id = ?
        ORDER BY slc.position_in_division ASC"
    )
    .bind(season)
    .bind(division_id)
    .fetch_all(pool)
    .await?;

    Ok(results
        .into_iter()
        .enumerate()
        .map(|(idx, row)| Standing {
            club_id: row.get::<String, _>(0),
            club_name: row.get::<String, _>(1),
            position: (idx + 1) as i32,
            played: row.get::<i32, _>(3),
            won: row.get::<i32, _>(4),
            drawn: row.get::<i32, _>(5),
            lost: row.get::<i32, _>(6),
            goals_for: row.get::<i32, _>(7),
            goals_against: row.get::<i32, _>(8),
            points: row.get::<i32, _>(9),
        })
        .collect())
}

/// Update a club's division and position
pub async fn update_club_division(
    pool: &SqlitePool,
    club_id: &str,
    new_division_id: &str,
    new_position: i32,
) -> Result<(), sqlx::Error> {
    // Find existing club assignment
    let old_assignment = sqlx::query(
        "SELECT id FROM sheffield_league_clubs WHERE club_id = ?"
    )
    .bind(club_id)
    .fetch_optional(pool)
    .await?;

    if let Some(_old) = old_assignment {
        // Delete old assignment
        sqlx::query("DELETE FROM sheffield_league_clubs WHERE club_id = ?")
            .bind(club_id)
            .execute(pool)
            .await?;
    }

    // Insert new assignment
    let new_id = format!("{}-{}", club_id, new_division_id);
    sqlx::query(
        "INSERT INTO sheffield_league_clubs
         (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
         VALUES (?, ?, ?, ?, 0, NULL)"
    )
    .bind(&new_id)
    .bind(new_division_id)
    .bind(club_id)
    .bind(new_position)
    .execute(pool)
    .await?;

    Ok(())
}

/// Batch update multiple clubs' divisions
pub async fn batch_update_club_divisions(
    pool: &SqlitePool,
    updates: Vec<(String, String, i32)>, // (club_id, new_division, new_position)
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    for (club_id, new_division_id, new_position) in updates {
        // Delete old assignment
        sqlx::query("DELETE FROM sheffield_league_clubs WHERE club_id = ?")
            .bind(&club_id)
            .execute(&mut *tx)
            .await?;

        // Insert new assignment
        let new_id = format!("{}-{}", club_id, new_division_id);
        sqlx::query(
            "INSERT INTO sheffield_league_clubs
             (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
             VALUES (?, ?, ?, ?, 0, NULL)"
        )
        .bind(&new_id)
        .bind(&new_division_id)
        .bind(&club_id)
        .bind(new_position)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}

/// Record a promotion or relegation event in history
pub async fn record_movement_event(
    pool: &SqlitePool,
    event_type: &str, // "promotion" or "relegation"
    season: u16,
    club_id: &str,
    from_division_id: &str,
    to_division_id: &str,
    final_position: i32,
) -> Result<(), sqlx::Error> {
    let id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO sheffield_league_movement_history
         (id, season, event_type, club_id, from_division_id, to_division_id, final_position, is_reserve_team)
         VALUES (?, ?, ?, ?, ?, ?, ?, 0)"
    )
    .bind(&id)
    .bind(season)
    .bind(event_type)
    .bind(club_id)
    .bind(from_division_id)
    .bind(to_division_id)
    .bind(final_position)
    .execute(pool)
    .await?;

    Ok(())
}

/// Recalculate division positions based on current standings
pub async fn recalculate_division_positions(
    pool: &SqlitePool,
    division_id: &str,
    season: u16,
) -> Result<(), sqlx::Error> {
    // Get all clubs in division with their current standings
    let standings = get_division_standings_simple(pool, division_id, season).await?;

    // Update positions
    let mut tx = pool.begin().await?;

    for (idx, standing) in standings.iter().enumerate() {
        let new_position = (idx + 1) as i32;

        sqlx::query(
            "UPDATE sheffield_league_clubs
             SET position_in_division = ?
             WHERE club_id = ? AND division_id = ?"
        )
        .bind(new_position)
        .bind(&standing.club_id)
        .bind(division_id)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}

/// Get all divisions with their clubs
pub async fn get_all_division_clubs(
    pool: &SqlitePool,
) -> Result<Vec<(String, Vec<String>)>, sqlx::Error> {
    use sqlx::Row;

    let divisions = sqlx::query("SELECT DISTINCT division_id FROM sheffield_league_clubs")
        .fetch_all(pool)
        .await?;

    let mut result = vec![];

    for div_row in divisions {
        let division_id = div_row.get::<String, _>(0);

        let clubs = sqlx::query_scalar::<_, String>(
            "SELECT club_id FROM sheffield_league_clubs WHERE division_id = ? ORDER BY position_in_division"
        )
        .bind(&division_id)
        .fetch_all(pool)
        .await?;

        result.push((division_id, clubs));
    }

    Ok(result)
}

/// Get the current division of a club
pub async fn get_club_current_division(
    pool: &SqlitePool,
    club_id: &str,
) -> Result<Option<String>, sqlx::Error> {
    sqlx::query_scalar::<_, String>(
        "SELECT division_id FROM sheffield_league_clubs WHERE club_id = ?"
    )
    .bind(club_id)
    .fetch_optional(pool)
    .await
}

/// Get club's position in current division
pub async fn get_club_position_in_division(
    pool: &SqlitePool,
    club_id: &str,
) -> Result<Option<i32>, sqlx::Error> {
    sqlx::query_scalar::<_, i32>(
        "SELECT position_in_division FROM sheffield_league_clubs WHERE club_id = ?"
    )
    .bind(club_id)
    .fetch_optional(pool)
    .await
}

/// Get movement history for a club
pub async fn get_club_movement_history(
    pool: &SqlitePool,
    club_id: &str,
) -> Result<Vec<(u16, String, String, String)>, sqlx::Error> {
    sqlx::query_as::<_, (u16, String, String, String)>(
        "SELECT season, event_type, from_division_id, to_division_id
         FROM sheffield_league_movement_history
         WHERE club_id = ?
         ORDER BY season DESC"
    )
    .bind(club_id)
    .fetch_all(pool)
    .await
}

/// Check if season-end processing has already been done
pub async fn has_season_been_processed(
    pool: &SqlitePool,
    season: u16,
) -> Result<bool, sqlx::Error> {
    let count = sqlx::query_scalar::<_, i32>(
        "SELECT COUNT(*) FROM sheffield_league_movement_history WHERE season = ?"
    )
    .bind(season)
    .fetch_one(pool)
    .await?;

    Ok(count > 0)
}
