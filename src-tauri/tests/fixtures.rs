//! The season's card: a proper double round-robin, every club playing exactly
//! once each matchday, on consecutive Saturdays from the season's opening.

use saturday_at_three::fsim_bridge;
use sqlx::sqlite::SqlitePoolOptions;

const REAL_DB: &str = "D:/projects/saturday at three/Sheffield1867.db";

#[tokio::test]
async fn a_season_is_drawn_as_a_proper_round_robin() {
    let dst = std::env::temp_dir().join("fsim_fixtures_test.db");
    std::fs::copy(REAL_DB, &dst).expect("copy");
    let pool = SqlitePoolOptions::new().connect(&format!("sqlite:{}", dst.display())).await.unwrap();

    // Clear any pre-existing card for the season, then draw it.
    sqlx::query("DELETE FROM sheffield_matches WHERE season = 1867").execute(&pool).await.unwrap();
    let created = fsim_bridge::generate_league_fixtures(&pool, 1867, "1867-09-07").await.expect("draw");
    assert!(created > 0, "a season's fixtures are drawn ({created})");
    println!("drew {created} league fixtures from 1867-09-07");

    // Idempotent — drawing again changes nothing.
    let again = fsim_bridge::generate_league_fixtures(&pool, 1867, "1867-09-07").await.unwrap();
    assert_eq!(again, 0, "a season already drawn is left alone");

    // Take one division and check the shape of its card.
    let div: (String,) = sqlx::query_as(
        "SELECT lc.division_id FROM sheffield_league_clubs lc JOIN sheffield_clubs c ON c.id = lc.club_id \
         WHERE c.is_reserve_team = 0 GROUP BY lc.division_id HAVING COUNT(*) >= 4 LIMIT 1")
        .fetch_one(&pool).await.unwrap();
    let clubs: Vec<(String,)> = sqlx::query_as(
        "SELECT lc.club_id FROM sheffield_league_clubs lc JOIN sheffield_clubs c ON c.id = lc.club_id \
         WHERE lc.division_id = ? AND c.is_reserve_team = 0").bind(&div.0).fetch_all(&pool).await.unwrap();
    let n = clubs.len();
    let first = &clubs[0].0;

    // Each club plays every other home and away: 2(n-1) matches.
    let played_by: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_matches WHERE season = 1867 AND (home_club_id = ? OR away_club_id = ?)")
        .bind(first).bind(first).fetch_one(&pool).await.unwrap();
    assert_eq!(played_by.0 as usize, 2 * (n - 1), "each club plays every other twice");

    // Home and away are balanced.
    let home: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_matches WHERE season = 1867 AND home_club_id = ?")
        .bind(first).fetch_one(&pool).await.unwrap();
    assert_eq!(home.0 as usize, n - 1, "half its matches are at home");

    // THE key property: a club never plays twice on the same day.
    let clashes: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM (SELECT match_date, club, COUNT(*) c FROM ( \
           SELECT match_date, home_club_id AS club FROM sheffield_matches WHERE season = 1867 \
           UNION ALL SELECT match_date, away_club_id FROM sheffield_matches WHERE season = 1867) \
         GROUP BY match_date, club HAVING c > 1)")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(clashes.0, 0, "no club is down for two matches on one day");

    // The card runs on Saturdays from the opening, week by week.
    let dates: Vec<(String,)> = sqlx::query_as(
        "SELECT DISTINCT match_date FROM sheffield_matches WHERE season = 1867 ORDER BY match_date LIMIT 3")
        .fetch_all(&pool).await.unwrap();
    assert_eq!(dates[0].0, "1867-09-07", "the season opens on the appointed Saturday");
    assert_eq!(dates[1].0, "1867-09-14", "and runs weekly");
}
