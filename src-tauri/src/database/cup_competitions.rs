use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::game::{GameEvent, EventType, Match};
use chrono::NaiveDate;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Competition {
    pub id: String,
    pub name: String,
    pub competition_type: String,
    pub season: i64,
    pub min_division_level: Option<i64>,
    pub max_division_level: Option<i64>,
    pub current_round: i64,
    pub total_rounds: i64,
    pub is_active: bool,
    pub winner_club_id: Option<String>,
    pub runner_up_club_id: Option<String>,
    pub rules_type: String,
    pub prestige_level: String,
    pub start_week: i64,
    pub announcement_week: i64,
    pub draw_week: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CupTie {
    pub id: String,
    pub competition_id: String,
    pub round_number: i64,
    pub round_name: String,
    pub tie_number: i64,
    pub home_club_id: Option<String>,
    pub away_club_id: Option<String>,
    pub scheduled_week: i64,
    pub played: bool,
    pub home_score: Option<i64>,
    pub away_score: Option<i64>,
    pub winner_club_id: Option<String>,
    pub match_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_club_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub away_club_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EligibleClub {
    pub id: String,
    pub name: String,
    pub division_level: i64,
    pub division_name: String,
}

/// Create annual cup competitions for 1867 Fantasy Mode
/// Creates both Youdan Cup (Divisions 1-4) and Cromwell Cup (Divisions 5-10)
pub async fn create_annual_cups(pool: &SqlitePool, season: i64, start_week: i64) -> Result<Vec<Competition>, Box<dyn std::error::Error>> {
    let mut competitions = Vec::new();

    // Create Youdan Cup (prestigious, top 4 divisions)
    let youdan_id = Uuid::new_v4().to_string();
    let announcement_week = start_week - 4;
    let draw_week = start_week - 2;

    sqlx::query(
        "INSERT INTO sheffield_competitions
        (id, name, competition_type, season, min_division_level, max_division_level,
         current_round, total_rounds, is_active, rules_type, prestige_level, start_week, announcement_week, draw_week)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&youdan_id)
    .bind(format!("Youdan Cup {}", season))
    .bind("knockout_cup")
    .bind(season)
    .bind(1)
    .bind(4)
    .bind(0)
    .bind(0) // Will be calculated after draw
    .bind(true)
    .bind("sheffield_rules")
    .bind("high")
    .bind(start_week)
    .bind(announcement_week)
    .bind(draw_week)
    .execute(pool)
    .await?;

    let youdan = Competition {
        id: youdan_id.clone(),
        name: format!("Youdan Cup {}", season),
        competition_type: "knockout_cup".to_string(),
        season,
        min_division_level: Some(1),
        max_division_level: Some(4),
        current_round: 0,
        total_rounds: 0,
        is_active: true,
        winner_club_id: None,
        runner_up_club_id: None,
        rules_type: "sheffield_rules".to_string(),
        prestige_level: "high".to_string(),
        start_week,
        announcement_week,
        draw_week,
    };
    competitions.push(youdan);

    // Create Cromwell Cup (standard prestige, divisions 5-10)
    let cromwell_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO sheffield_competitions
        (id, name, competition_type, season, min_division_level, max_division_level,
         current_round, total_rounds, is_active, rules_type, prestige_level, start_week, announcement_week, draw_week)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&cromwell_id)
    .bind(format!("Cromwell Cup {}", season))
    .bind("knockout_cup")
    .bind(season)
    .bind(5)
    .bind(10)
    .bind(0)
    .bind(0)
    .bind(true)
    .bind("sheffield_rules")
    .bind("standard")
    .bind(start_week)
    .bind(announcement_week)
    .bind(draw_week)
    .execute(pool)
    .await?;

    let cromwell = Competition {
        id: cromwell_id.clone(),
        name: format!("Cromwell Cup {}", season),
        competition_type: "knockout_cup".to_string(),
        season,
        min_division_level: Some(5),
        max_division_level: Some(10),
        current_round: 0,
        total_rounds: 0,
        is_active: true,
        winner_club_id: None,
        runner_up_club_id: None,
        rules_type: "sheffield_rules".to_string(),
        prestige_level: "standard".to_string(),
        start_week,
        announcement_week,
        draw_week,
    };
    competitions.push(cromwell);

    Ok(competitions)
}

/// Create a custom cup competition with specified parameters
pub async fn create_custom_cup(
    pool: &SqlitePool,
    name: String,
    season: i64,
    min_division_level: Option<i64>,
    max_division_level: Option<i64>,
    start_week: i64,
    announcement_week: i64,
    draw_week: i64,
    prestige_level: String,
    rules_type: String,
) -> Result<Competition, Box<dyn std::error::Error>> {
    let competition_id = Uuid::new_v4().to_string();

    sqlx::query(
        "INSERT INTO sheffield_competitions
        (id, name, competition_type, season, min_division_level, max_division_level,
         current_round, total_rounds, is_active, rules_type, prestige_level, start_week, announcement_week, draw_week)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&competition_id)
    .bind(&name)
    .bind("knockout_cup")
    .bind(season)
    .bind(min_division_level)
    .bind(max_division_level)
    .bind(0)
    .bind(0) // Will be calculated after draw
    .bind(true)
    .bind(&rules_type)
    .bind(&prestige_level)
    .bind(start_week)
    .bind(announcement_week)
    .bind(draw_week)
    .execute(pool)
    .await?;

    Ok(Competition {
        id: competition_id,
        name,
        competition_type: "knockout_cup".to_string(),
        season,
        min_division_level,
        max_division_level,
        current_round: 0,
        total_rounds: 0,
        is_active: true,
        winner_club_id: None,
        runner_up_club_id: None,
        rules_type,
        prestige_level,
        start_week,
        announcement_week,
        draw_week,
    })
}

/// Update an existing cup competition
pub async fn update_cup_competition(
    pool: &SqlitePool,
    competition_id: String,
    name: String,
    min_division_level: Option<i64>,
    max_division_level: Option<i64>,
    start_week: i64,
    announcement_week: i64,
    draw_week: i64,
    prestige_level: String,
    rules_type: String,
) -> Result<Competition, Box<dyn std::error::Error>> {
    sqlx::query(
        "UPDATE sheffield_competitions
         SET name = ?, min_division_level = ?, max_division_level = ?,
             start_week = ?, announcement_week = ?, draw_week = ?,
             prestige_level = ?, rules_type = ?
         WHERE id = ?"
    )
    .bind(&name)
    .bind(min_division_level)
    .bind(max_division_level)
    .bind(start_week)
    .bind(announcement_week)
    .bind(draw_week)
    .bind(&prestige_level)
    .bind(&rules_type)
    .bind(&competition_id)
    .execute(pool)
    .await?;

    // Fetch and return the updated competition
    let comp = sqlx::query_as::<_, (
        String, String, String, i64,
        Option<i64>, Option<i64>, i64, i64, bool,
        Option<String>, Option<String>, String, String, i64,
        i64, i64
    )>(
        "SELECT id, name, competition_type, season,
                min_division_level, max_division_level, current_round, total_rounds, is_active,
                winner_club_id, runner_up_club_id, rules_type, prestige_level, start_week,
                announcement_week, draw_week
         FROM sheffield_competitions
         WHERE id = ?"
    )
    .bind(&competition_id)
    .fetch_one(pool)
    .await?;

    Ok(Competition {
        id: comp.0,
        name: comp.1,
        competition_type: comp.2,
        season: comp.3,
        min_division_level: comp.4,
        max_division_level: comp.5,
        current_round: comp.6,
        total_rounds: comp.7,
        is_active: comp.8,
        winner_club_id: comp.9,
        runner_up_club_id: comp.10,
        rules_type: comp.11,
        prestige_level: comp.12,
        start_week: comp.13,
        announcement_week: comp.14,
        draw_week: comp.15,
    })
}

/// Get all eligible clubs for a competition based on division level
pub async fn get_eligible_clubs(pool: &SqlitePool, competition_id: &str) -> Result<Vec<EligibleClub>, Box<dyn std::error::Error>> {
    let comp: (Option<i64>, Option<i64>) = sqlx::query_as(
        "SELECT min_division_level, max_division_level FROM sheffield_competitions WHERE id = ?"
    )
    .bind(competition_id)
    .fetch_one(pool)
    .await?;

    let (min_level, max_level) = comp;

    let clubs = sqlx::query_as::<_, (String, String, i64, String)>(
        "SELECT c.id, c.name, d.level, d.name as division_name
         FROM sheffield_clubs c
         JOIN sheffield_league_clubs lc ON c.id = lc.club_id
         JOIN sheffield_league_divisions d ON lc.division_id = d.id
         WHERE d.level BETWEEN ? AND ?
         ORDER BY d.level, c.name"
    )
    .bind(min_level.unwrap_or(1))
    .bind(max_level.unwrap_or(100))
    .fetch_all(pool)
    .await?;

    Ok(clubs.into_iter().map(|(id, name, division_level, division_name)| {
        EligibleClub {
            id,
            name,
            division_level,
            division_name,
        }
    }).collect())
}

/// Generate a cup draw (knockout bracket)
pub async fn generate_cup_draw(pool: &SqlitePool, competition_id: &str, seeded: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut eligible = get_eligible_clubs(pool, competition_id).await?;

    eprintln!("[CUP DRAW] Found {} eligible clubs for competition {}", eligible.len(), competition_id);

    if eligible.is_empty() {
        return Err("No eligible clubs for this competition".into());
    }

    // Randomize order unless seeded
    if !seeded {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        eligible.shuffle(&mut rng);
    } else {
        // If seeded, higher divisions go first
        eligible.sort_by(|a, b| a.division_level.cmp(&b.division_level));
    }

    let num_clubs = eligible.len();

    // Calculate total rounds needed
    let total_rounds = (num_clubs as f64).log2().ceil() as i64;

    eprintln!("[CUP DRAW] Calculated {} rounds for {} clubs", total_rounds, num_clubs);

    // Update competition with total rounds
    sqlx::query(
        "UPDATE sheffield_competitions SET total_rounds = ? WHERE id = ?"
    )
    .bind(total_rounds)
    .bind(competition_id)
    .execute(pool)
    .await?;

    // Get competition start week for scheduling
    let start_week: i64 = sqlx::query_scalar(
        "SELECT start_week FROM sheffield_competitions WHERE id = ?"
    )
    .bind(competition_id)
    .fetch_one(pool)
    .await?;

    // Create first round ties
    let mut tie_number = 1;
    let mut i = 0;

    while i < num_clubs {
        let home_club = &eligible[i];
        let away_club = if i + 1 < num_clubs {
            Some(&eligible[i + 1])
        } else {
            None // Bye
        };

        let tie_id = Uuid::new_v4().to_string();

        if let Some(away) = away_club {
            sqlx::query(
                "INSERT INTO sheffield_cup_ties
                (id, competition_id, round_number, round_name, tie_number, home_club_id, away_club_id, scheduled_week, played)
                VALUES (?, ?, 1, 'First Round', ?, ?, ?, ?, 0)"
            )
            .bind(&tie_id)
            .bind(competition_id)
            .bind(tie_number)
            .bind(&home_club.id)
            .bind(&away.id)
            .bind(start_week)
            .execute(pool)
            .await?;

            i += 2;
        } else {
            // Bye - club advances automatically
            sqlx::query(
                "INSERT INTO sheffield_cup_ties
                (id, competition_id, round_number, round_name, tie_number, home_club_id, away_club_id, scheduled_week, played, winner_club_id)
                VALUES (?, ?, 1, 'First Round', ?, ?, NULL, ?, 1, ?)"
            )
            .bind(&tie_id)
            .bind(competition_id)
            .bind(tie_number)
            .bind(&home_club.id)
            .bind(start_week)
            .bind(&home_club.id)
            .execute(pool)
            .await?;

            i += 1;
        }

        tie_number += 1;
    }

    Ok(())
}

/// Get the full bracket structure for a competition
pub async fn get_cup_bracket(pool: &SqlitePool, competition_id: &str) -> Result<Vec<CupTie>, Box<dyn std::error::Error>> {
    let ties = sqlx::query_as::<_, (
        String, String, i64, String, i64,
        Option<String>, Option<String>, i64, bool,
        Option<i64>, Option<i64>, Option<String>, Option<String>,
        Option<String>, Option<String>
    )>(
        "SELECT t.id, t.competition_id, t.round_number, t.round_name, t.tie_number,
                t.home_club_id, t.away_club_id, t.scheduled_week, t.played,
                t.home_score, t.away_score, t.winner_club_id, t.match_id,
                h.name AS home_club_name, a.name AS away_club_name
         FROM sheffield_cup_ties t
         LEFT JOIN sheffield_clubs h ON t.home_club_id = h.id
         LEFT JOIN sheffield_clubs a ON t.away_club_id = a.id
         WHERE t.competition_id = ?
         ORDER BY t.round_number, t.tie_number"
    )
    .bind(competition_id)
    .fetch_all(pool)
    .await?;

    Ok(ties.into_iter().map(|t| CupTie {
        id: t.0,
        competition_id: t.1,
        round_number: t.2,
        round_name: t.3,
        tie_number: t.4,
        home_club_id: t.5,
        away_club_id: t.6,
        scheduled_week: t.7,
        played: t.8,
        home_score: t.9,
        away_score: t.10,
        winner_club_id: t.11,
        match_id: t.12,
        home_club_name: t.13,
        away_club_name: t.14,
    }).collect())
}

/// Create Match fixtures from unplayed cup ties
/// This function converts CupTie records into Match records that can be played
pub async fn create_cup_match_fixtures(
    pool: &SqlitePool,
    season: i64,
    start_date: &str,
) -> Result<Vec<Match>, Box<dyn std::error::Error>> {
    eprintln!("[CUP FIXTURES] Creating match fixtures for season {}", season);

    // Get all unplayed cup ties for active competitions this season
    let ties = sqlx::query_as::<_, (
        String, String, i64, Option<String>, Option<String>, i64, Option<String>
    )>(
        "SELECT t.id, t.competition_id, t.round_number, t.home_club_id, t.away_club_id, t.scheduled_week, t.match_id
         FROM sheffield_cup_ties t
         INNER JOIN sheffield_competitions c ON t.competition_id = c.id
         WHERE c.season = ? AND c.is_active = 1 AND t.played = 0 AND t.match_id IS NULL
         AND t.home_club_id IS NOT NULL AND t.away_club_id IS NOT NULL
         ORDER BY t.round_number, t.tie_number"
    )
    .bind(season)
    .fetch_all(pool)
    .await?;

    eprintln!("[CUP FIXTURES] Found {} unplayed ties without match IDs", ties.len());

    if ties.is_empty() {
        return Ok(Vec::new());
    }

    // Parse the season start date
    let base_date = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
        .map_err(|e| format!("Failed to parse start date: {}", e))?;

    let mut matches = Vec::new();

    for (tie_id, comp_id, round_number, home_club_id, away_club_id, scheduled_week, _) in ties {
        // Calculate match date based on scheduled week
        // Each week is 7 days from the start date
        let match_date = base_date + chrono::Duration::weeks(scheduled_week);
        let match_date_str = match_date.format("%Y-%m-%d").to_string();

        // Create unique match ID
        let match_id = Uuid::new_v4().to_string();

        // Create Match record
        let cup_match = Match {
            id: match_id.clone(),
            gameweek: 0, // Cup matches use gameweek 0
            home_team_id: home_club_id.unwrap(),
            away_team_id: away_club_id.unwrap(),
            home_score: None,
            away_score: None,
            date: match_date_str.clone(),
            played: false,
        };

        // Update the cup tie with the match_id
        sqlx::query(
            "UPDATE sheffield_cup_ties SET match_id = ? WHERE id = ?"
        )
        .bind(&match_id)
        .bind(&tie_id)
        .execute(pool)
        .await?;

        eprintln!("[CUP FIXTURES] Created match {} for tie {} (Round {}, Week {}): {} vs {} on {}",
            match_id, tie_id, round_number, scheduled_week,
            cup_match.home_team_id, cup_match.away_team_id, match_date_str);

        matches.push(cup_match);
    }

    eprintln!("[CUP FIXTURES] Created {} cup match fixtures", matches.len());
    Ok(matches)
}

/// Process cup match results and advance winners to next round
/// This should be called after matches are played to update cup ties and create next round
pub async fn process_cup_match_results(
    pool: &SqlitePool,
    season: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("[CUP RESULTS] Processing cup match results for season {}", season);

    // Get all active cup competitions for this season
    let competitions = get_competitions_for_season(pool, season).await?;

    for competition in competitions {
        eprintln!("[CUP RESULTS] Processing competition: {}", competition.name);

        // Get all cup ties with match IDs that haven't been processed yet (played=0)
        let ties_with_matches = sqlx::query_as::<_, (
            String, String, i64, String, Option<String>, Option<String>,
            i64, String, Option<i64>, Option<i64>, bool
        )>(
            "SELECT t.id, t.competition_id, t.round_number, t.round_name,
                    t.home_club_id, t.away_club_id, t.scheduled_week,
                    t.match_id, t.home_score, t.away_score, t.played
             FROM sheffield_cup_ties t
             WHERE t.competition_id = ? AND t.match_id IS NOT NULL AND t.played = 0
             ORDER BY t.round_number, t.tie_number"
        )
        .bind(&competition.id)
        .fetch_all(pool)
        .await?;

        if ties_with_matches.is_empty() {
            continue;
        }

        eprintln!("[CUP RESULTS] Found {} unprocessed ties with matches", ties_with_matches.len());

        // Check each tie to see if its match has been played
        for (tie_id, comp_id, round_number, round_name, home_id, away_id,
             scheduled_week, match_id, _home_score, _away_score, _played) in ties_with_matches {

            // Query the actual match from the game state would be external
            // For now, we'll check if there's a result in a hypothetical matches table
            // In practice, this will be called from advance_day with game.matches data

            // Since we don't have direct access to game state here, this function will be
            // called from commands.rs with match data passed in
            // For now, just log that we found ties waiting for results
            eprintln!("[CUP RESULTS] Tie {} (Round {}) waiting for match {} result",
                tie_id, round_number, match_id);
        }

        // Check if current round is complete
        let round_complete = check_round_complete(pool, &competition.id, competition.current_round + 1).await?;

        if round_complete {
            eprintln!("[CUP RESULTS] Round {} complete for {}", competition.current_round + 1, competition.name);

            // Generate next round
            let next_round = competition.current_round + 2;
            if next_round <= competition.total_rounds {
                generate_next_round(pool, &competition.id, next_round).await?;
            } else {
                eprintln!("[CUP RESULTS] Competition {} complete!", competition.name);
                // Mark competition as complete
                sqlx::query("UPDATE sheffield_competitions SET is_active = 0 WHERE id = ?")
                    .bind(&competition.id)
                    .execute(pool)
                    .await?;
            }
        }
    }

    Ok(())
}

/// Update a cup tie with match result and determine winner
pub async fn update_cup_tie_result(
    pool: &SqlitePool,
    tie_id: &str,
    home_score: i64,
    away_score: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("[CUP TIE] Updating tie {} with result {} - {}", tie_id, home_score, away_score);

    // Determine winner
    let winner_club_id: Option<String> = if home_score > away_score {
        // Home wins
        let (home_id,): (String,) = sqlx::query_as(
            "SELECT home_club_id FROM sheffield_cup_ties WHERE id = ?"
        )
        .bind(tie_id)
        .fetch_one(pool)
        .await?;
        Some(home_id)
    } else if away_score > home_score {
        // Away wins
        let (away_id,): (String,) = sqlx::query_as(
            "SELECT away_club_id FROM sheffield_cup_ties WHERE id = ?"
        )
        .bind(tie_id)
        .fetch_one(pool)
        .await?;
        Some(away_id)
    } else {
        // Draw - for now, home team wins (TODO: implement replays)
        eprintln!("[CUP TIE] Match drawn - awarding to home team (replays not yet implemented)");
        let (home_id,): (String,) = sqlx::query_as(
            "SELECT home_club_id FROM sheffield_cup_ties WHERE id = ?"
        )
        .bind(tie_id)
        .fetch_one(pool)
        .await?;
        Some(home_id)
    };

    // Update the tie
    sqlx::query(
        "UPDATE sheffield_cup_ties
         SET played = 1, home_score = ?, away_score = ?, winner_club_id = ?
         WHERE id = ?"
    )
    .bind(home_score)
    .bind(away_score)
    .bind(&winner_club_id)
    .bind(tie_id)
    .execute(pool)
    .await?;

    eprintln!("[CUP TIE] Winner: {:?}", winner_club_id);

    Ok(())
}

/// Check if all ties in a round are complete
async fn check_round_complete(
    pool: &SqlitePool,
    competition_id: &str,
    round_number: i64,
) -> Result<bool, Box<dyn std::error::Error>> {
    let unplayed_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sheffield_cup_ties
         WHERE competition_id = ? AND round_number = ? AND played = 0"
    )
    .bind(competition_id)
    .bind(round_number)
    .fetch_one(pool)
    .await?;

    Ok(unplayed_count == 0)
}

/// Generate the next round of a cup competition
async fn generate_next_round(
    pool: &SqlitePool,
    competition_id: &str,
    next_round_number: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    eprintln!("[CUP NEXT ROUND] Generating round {} for competition {}", next_round_number, competition_id);

    // Get winners from previous round
    let winners = sqlx::query_as::<_, (String, String)>(
        "SELECT winner_club_id, tie_number
         FROM sheffield_cup_ties
         WHERE competition_id = ? AND round_number = ? AND winner_club_id IS NOT NULL
         ORDER BY tie_number"
    )
    .bind(competition_id)
    .bind(next_round_number - 1)
    .fetch_all(pool)
    .await?;

    eprintln!("[CUP NEXT ROUND] Found {} winners from previous round", winners.len());

    if winners.is_empty() {
        return Err("No winners found from previous round".into());
    }

    // Get competition details for scheduling
    let (start_week, current_round): (i64, i64) = sqlx::query_as(
        "SELECT start_week, current_round FROM sheffield_competitions WHERE id = ?"
    )
    .bind(competition_id)
    .fetch_one(pool)
    .await?;

    // Schedule next round 2 weeks after start (1 week gap between rounds)
    let next_round_week = start_week + ((next_round_number - 1) * 2);

    // Determine round name
    let round_name = match winners.len() {
        32 => "Round of 32",
        16 => "Round of 16",
        8 => "Quarter-finals",
        4 => "Semi-finals",
        2 => "Final",
        _ => "Next Round",
    };

    eprintln!("[CUP NEXT ROUND] Creating {} with {} teams on week {}", round_name, winners.len(), next_round_week);

    // Pair up winners
    let mut tie_number = 1;
    for i in (0..winners.len()).step_by(2) {
        if i + 1 < winners.len() {
            let home_club_id = &winners[i].0;
            let away_club_id = &winners[i + 1].0;

            let tie_id = Uuid::new_v4().to_string();

            sqlx::query(
                "INSERT INTO sheffield_cup_ties
                (id, competition_id, round_number, round_name, tie_number, home_club_id, away_club_id, scheduled_week, played)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, 0)"
            )
            .bind(&tie_id)
            .bind(competition_id)
            .bind(next_round_number)
            .bind(round_name)
            .bind(tie_number)
            .bind(home_club_id)
            .bind(away_club_id)
            .bind(next_round_week)
            .execute(pool)
            .await?;

            eprintln!("[CUP NEXT ROUND] Created tie {}: {} vs {}", tie_number, home_club_id, away_club_id);
            tie_number += 1;
        }
    }

    // Update competition current_round
    sqlx::query(
        "UPDATE sheffield_competitions SET current_round = ? WHERE id = ?"
    )
    .bind(next_round_number)
    .bind(competition_id)
    .execute(pool)
    .await?;

    eprintln!("[CUP NEXT ROUND] Round {} created successfully with {} ties", next_round_number, tie_number - 1);

    Ok(())
}

/// Get announcement text based on competition name
fn get_announcement_text(competition_name: &str) -> (String, String) {
    if competition_name.contains("Youdan") {
        get_youdan_cup_announcement_text(1867)
    } else if competition_name.contains("Cromwell") {
        get_cromwell_cup_announcement_text(1867)
    } else {
        let title = competition_name.to_string();
        let description = format!("A new cup competition, the {}, has been announced. All eligible clubs are invited to participate in this exciting knockout tournament.", competition_name);
        (title, description)
    }
}

/// Create a news item for a cup announcement (persists to database)
pub async fn create_cup_announcement_news(
    pool: &SqlitePool,
    competition: &Competition,
    publish_date: &str,
) -> Result<String, sqlx::Error> {
    let news_id = Uuid::new_v4().to_string();

    let (headline, body_text) = get_announcement_text(&competition.name);

    eprintln!("[CUP NEWS] Creating news item: {}", headline);
    eprintln!("[CUP NEWS] Publish date: {}", publish_date);

    sqlx::query(
        r#"
        INSERT INTO sheffield_news_items (
            id, headline, article_type, publish_date, body_text,
            is_read, is_important, requires_action,
            has_action_button, action_button_text, action_type, action_data
        ) VALUES (?, ?, 'cup_announcement', ?, ?, 0, 1, 0, 0, NULL, NULL, NULL)
        "#
    )
    .bind(&news_id)
    .bind(&headline)
    .bind(publish_date)
    .bind(&body_text)
    .execute(pool)
    .await?;

    eprintln!("[CUP NEWS] Created news item with ID: {}", news_id);

    Ok(news_id)
}

/// Generate a cup announcement event (DEPRECATED - use create_cup_announcement_news instead)
pub fn create_cup_announcement_event(competition: &Competition, announcement_date: &str) -> GameEvent {
    let (title, description) = get_announcement_text(&competition.name);

    GameEvent {
        id: Uuid::new_v4().to_string(),
        event_type: EventType::CupAnnouncement {
            cup_name: competition.name.clone(),
            competition_id: competition.id.clone(),
            title,
            description,
            eligible_divisions: vec![], // Will be populated from database if needed
        },
        date: announcement_date.to_string(),
        requires_user_action: false,
        processed: false,
        result: None,
    }
}

/// Generate a cup draw event
pub async fn create_cup_draw_event(
    pool: &SqlitePool,
    competition: &Competition,
    draw_date: &str,
) -> Result<GameEvent, Box<dyn std::error::Error>> {
    let ties = get_cup_bracket(pool, &competition.id).await?;

    // Build draw summary text
    let mut draw_text = format!(
        "The draw has been made for the {}. The following ties will be played in Round 1:\n\n",
        competition.name
    );

    // Build draw bracket for JSON
    let mut draw_bracket = Vec::new();

    for tie in &ties {
        if tie.round_number == 1 {
            if let Some(home_id) = &tie.home_club_id {
                let home_name: String = sqlx::query_scalar(
                    "SELECT name FROM sheffield_clubs WHERE id = ?"
                )
                .bind(home_id)
                .fetch_one(pool)
                .await?;

                if let Some(away_id) = &tie.away_club_id {
                    let away_name: String = sqlx::query_scalar(
                        "SELECT name FROM sheffield_clubs WHERE id = ?"
                    )
                    .bind(away_id)
                    .fetch_one(pool)
                    .await?;

                    draw_text.push_str(&format!("• {} vs {}\n", home_name, away_name));

                    draw_bracket.push(serde_json::json!({
                        "home": home_name,
                        "away": away_name,
                    }));
                } else {
                    draw_text.push_str(&format!("• {} (bye)\n", home_name));
                }
            }
        }
    }

    Ok(GameEvent {
        id: Uuid::new_v4().to_string(),
        event_type: EventType::CupDraw {
            cup_name: competition.name.clone(),
            competition_id: competition.id.clone(),
            title: format!("{} - Draw Made", competition.name),
            description: draw_text,
            draw_bracket,
        },
        date: draw_date.to_string(),
        requires_user_action: false,
        processed: false,
        result: None,
    })
}

/// Get all competitions for a season
pub async fn get_competitions_for_season(pool: &SqlitePool, season: i64) -> Result<Vec<Competition>, sqlx::Error> {
    let comps = sqlx::query_as::<_, (
        String, String, String, i64,
        Option<i64>, Option<i64>, i64, i64, bool,
        Option<String>, Option<String>, String, String, i64,
        i64, i64
    )>(
        "SELECT id, name, competition_type, season,
                min_division_level, max_division_level, current_round, total_rounds, is_active,
                winner_club_id, runner_up_club_id, rules_type, prestige_level, start_week,
                announcement_week, draw_week
         FROM sheffield_competitions
         WHERE season = ?
         ORDER BY prestige_level DESC, name"
    )
    .bind(season)
    .fetch_all(pool)
    .await?;

    Ok(comps.into_iter().map(|c| Competition {
        id: c.0,
        name: c.1,
        competition_type: c.2,
        season: c.3,
        min_division_level: c.4,
        max_division_level: c.5,
        current_round: c.6,
        total_rounds: c.7,
        is_active: c.8,
        winner_club_id: c.9,
        runner_up_club_id: c.10,
        rules_type: c.11,
        prestige_level: c.12,
        start_week: c.13,
        announcement_week: c.14,
        draw_week: c.15,
    }).collect())
}

/// Delete all competitions for a season (cleanup)
pub async fn delete_competitions_for_season(pool: &SqlitePool, season: i64) -> Result<(), Box<dyn std::error::Error>> {
    // First delete all cup ties for competitions in this season
    sqlx::query(
        "DELETE FROM sheffield_cup_ties WHERE competition_id IN (SELECT id FROM sheffield_competitions WHERE season = ?)"
    )
    .bind(season)
    .execute(pool)
    .await?;

    // Then delete the competitions
    sqlx::query(
        "DELETE FROM sheffield_competitions WHERE season = ?"
    )
    .bind(season)
    .execute(pool)
    .await?;

    Ok(())
}

/// Generate the historical announcement text for the Youdan Cup
pub fn get_youdan_cup_announcement_text(season: i64) -> (String, String) {
    let title = format!("The Youdan Football Cup {}", season);
    let description = format!(
        "Representatives of football clubs from Sheffield and the neighbourhood assembled at the Adelphi Hotel \
         to hear an extraordinary announcement.\n\n\
         \
         Mr. Thomas Youdan, the esteemed proprietor of the Theatre Royal and several other theatrical \
         establishments, has generously offered to sponsor a grand Football Challenge Cup competition. \
         The trophy, to be manufactured to a prize-winning design, shall be competed for by the various \
         football clubs of Sheffield under the Sheffield Rules.\n\n\
         \
         A Prize Committee has been formed to oversee the arrangements. The competition shall be open to \
         ALL clubs of the Sheffield & Hallamshire League, from the highest division to the lowest. Matches will be played \
         with twelve players on each side, lasting ninety minutes, with extra time if necessary.\n\n\
         \
         This represents the most prestigious football competition yet organized in Sheffield, and perhaps \
         in all of England. The winning club shall receive not only the magnificent silver trophy but also \
         the eternal glory of being the first champions of cup football.\n\n\
         \
         All clubs are invited to enter. The draw for the first round will be held shortly at the \
         Adelphi Hotel."
    );
    (title, description)
}

/// Generate the historical announcement text for the Cromwell Cup
pub fn get_cromwell_cup_announcement_text(season: i64) -> (String, String) {
    let title = format!("The Cromwell Football Cup {}", season);
    let description = format!(
        "Following the great success and interest generated by Mr. Youdan's Football Challenge Cup, \
         another theatre proprietor has stepped forward to sponsor a second competition.\n\n\
         \
         This new cup, to be known as the Cromwell Cup, shall be open to clubs from the fifth through \
         tenth divisions of the Sheffield & Hallamshire League. Like the Youdan Cup, matches will be \
         played under Sheffield Rules with twelve players per side.\n\n\
         \
         This presents a magnificent opportunity for clubs in the lower divisions to compete for silverware \
         and prove their worth against their peers. The competition promises to be keenly contested, with \
         many ambitious clubs seeking to make their mark.\n\n\
         \
         All eligible clubs are encouraged to enter. The draw will be announced in due course."
    );
    (title, description)
}

/// Generate the announcement text for the cup draw
pub fn get_draw_announcement_text(cup_name: &str, season: i64, bracket: &[CupTie]) -> (String, String) {
    let title = format!("{} - Draw Announced", cup_name);

    // Group ties by round for formatted display
    let round_1_ties: Vec<&CupTie> = bracket.iter().filter(|t| t.round_number == 1).collect();

    let mut description = format!(
        "The draw for the {} has been made at the Adelphi Hotel. \
         Great interest was shown by representatives of the various clubs, and the matches promise \
         to provide excellent sport for the Sheffield footballing public.\n\n\
         \
         The following ties have been drawn for the first round:\n\n",
        cup_name
    );

    for (i, tie) in round_1_ties.iter().enumerate() {
        description.push_str(&format!(
            "{}. {} vs {}\n",
            i + 1,
            tie.home_club_name.as_deref().unwrap_or("TBD"),
            tie.away_club_name.as_deref().unwrap_or("TBD")
        ));
    }

    description.push_str(&format!(
        "\nMatches will be played under Sheffield Rules with twelve players per side. \
         The first round ties are scheduled to commence shortly. All clubs and their supporters \
         eagerly await these historic encounters."
    ));

    (title, description)
}

/// Initialize the Youdan Cup competition for the 1867 season and generate game events
/// This creates the competition with all teams eligible and returns events for announcement and draw
/// Returns empty vec if competition already exists for this year (prevents duplicates)
pub async fn initialize_season_cup_competitions(
    pool: &SqlitePool,
    year: i32,
    start_date: &str,
) -> Result<Vec<GameEvent>, Box<dyn std::error::Error>> {
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("🏆 Initializing Youdan Cup {}", year);
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Check if Youdan Cup already exists for this year
    eprintln!("   Checking if Youdan Cup {} already exists...", year);
    let existing_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM sheffield_competitions WHERE season = ? AND name LIKE 'Youdan Cup%'"
    )
    .bind(year as i64)
    .fetch_one(pool)
    .await?;

    if existing_count > 0 {
        eprintln!("   ⚠️  Youdan Cup {} already exists in database", year);
        eprintln!("   Skipping creation to prevent duplicates\n");
        eprintln!("╔══════════════════════════════════════════════════════════════════════════╗");
        eprintln!("║  ⏭️  YOUDAN CUP ALREADY EXISTS - SKIPPED                                 ║");
        eprintln!("╚══════════════════════════════════════════════════════════════════════════╝\n");
        return Ok(Vec::new());
    }

    eprintln!("   ✅ No existing Youdan Cup found for {}, proceeding with creation\n", year);

    let mut events = Vec::new();

    // Parse start date to calculate event dates
    let base_date = chrono::NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
        .map_err(|e| format!("Failed to parse start date: {}", e))?;

    // Youdan Cup announced on February 21, 1867
    // Game starts January 7, so Feb 21 is 45 days later (Day 46)
    let announcement_date = base_date + chrono::Duration::days(45);
    let announcement_date_str = announcement_date.format("%Y-%m-%d").to_string();

    // Draw is 1 week after announcement (Day 53)
    let draw_date = announcement_date + chrono::Duration::days(7);
    let draw_date_str = draw_date.format("%Y-%m-%d").to_string();

    // First round matches 1 week after draw (Day 60)
    let first_round_date = draw_date + chrono::Duration::days(7);

    // Calculate week numbers for database
    let start_week = 9; // Week 9 of the season (approximately early March)
    let announcement_week = 7; // Week 7 (late February)
    let draw_week = 8; // Week 8 (early March)

    // Create the Youdan Cup competition with ALL teams (no division restrictions)
    let youdan_id = Uuid::new_v4().to_string();

    eprintln!("   Creating Youdan Cup competition in database...");
    eprintln!("   - ID: {}", youdan_id);
    eprintln!("   - All teams eligible (no division restrictions)");
    eprintln!("   - Announcement: {} (Day 46)", announcement_date.format("%B %d, %Y"));
    eprintln!("   - Draw: {} (Day 53)", draw_date.format("%B %d, %Y"));
    eprintln!("   - First Round: {} (Day 60)\n", first_round_date.format("%B %d, %Y"));

    sqlx::query(
        "INSERT INTO sheffield_competitions
        (id, name, competition_type, season, min_division_level, max_division_level,
         current_round, total_rounds, is_active, rules_type, prestige_level, start_week, announcement_week, draw_week)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&youdan_id)
    .bind(format!("Youdan Cup {}", year))
    .bind("knockout_cup")
    .bind(year as i64)
    .bind(1)   // min_division_level: 1 (includes all teams)
    .bind(10)  // max_division_level: 10 (all divisions)
    .bind(0)
    .bind(0) // Total rounds will be calculated after draw
    .bind(true)
    .bind("sheffield_rules")
    .bind("high")
    .bind(start_week)
    .bind(announcement_week)
    .bind(draw_week)
    .execute(pool)
    .await?;

    let youdan = Competition {
        id: youdan_id.clone(),
        name: format!("Youdan Cup {}", year),
        competition_type: "knockout_cup".to_string(),
        season: year as i64,
        min_division_level: Some(1),
        max_division_level: Some(10),
        current_round: 0,
        total_rounds: 0,
        is_active: true,
        winner_club_id: None,
        runner_up_club_id: None,
        rules_type: "sheffield_rules".to_string(),
        prestige_level: "high".to_string(),
        start_week,
        announcement_week,
        draw_week,
    };

    eprintln!("   ✅ Youdan Cup created successfully\n");

    // Create announcement event
    eprintln!("   Creating announcement event...");
    let announcement_event = create_cup_announcement_event(&youdan, &announcement_date_str);
    eprintln!("   ✅ Announcement event scheduled for {}\n", announcement_date.format("%B %d, %Y"));
    events.push(announcement_event);

    // Create draw event (will be triggered on draw date to generate bracket)
    eprintln!("   Creating draw event...");
    let draw_event = GameEvent {
        id: Uuid::new_v4().to_string(),
        event_type: EventType::CupDrawPending {
            cup_name: youdan.name.clone(),
            competition_id: youdan.id.clone(),
            title: format!("{} - Draw to be Made", youdan.name),
            description: format!(
                "The draw for the {} will be held at the Adelphi Hotel. \
                 All participating clubs eagerly await to discover their opponents in this historic tournament.",
                youdan.name
            ),
        },
        date: draw_date_str.clone(),
        requires_user_action: false,
        processed: false,
        result: None,
    };
    eprintln!("   ✅ Draw event scheduled for {}\n", draw_date.format("%B %d, %Y"));
    events.push(draw_event);

    eprintln!("╔══════════════════════════════════════════════════════════════════════════╗");
    eprintln!("║  ✅ YOUDAN CUP INITIALIZATION COMPLETE                                   ║");
    eprintln!("╚══════════════════════════════════════════════════════════════════════════╝\n");
    eprintln!("   Competition ID: {}", youdan_id);
    eprintln!("   {} events created", events.len());
    eprintln!("   Next steps:");
    eprintln!("   1. Announcement on {}", announcement_date.format("%B %d, %Y"));
    eprintln!("   2. Draw on {}", draw_date.format("%B %d, %Y"));
    eprintln!("   3. First round matches from {}\n", first_round_date.format("%B %d, %Y"));

    Ok(events)
}
