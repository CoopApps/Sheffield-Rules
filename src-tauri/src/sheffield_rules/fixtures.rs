/// Sheffield Rules Fixture Generation
///
/// Procedurally generates realistic fixtures for Sheffield Rules era football (1858-1877).
/// Uses balanced scheduling algorithms to ensure fair distribution of home/away matches.

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use super::clubs::SheffieldClub;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fixture {
    pub id: String,
    pub home_team_id: String,
    pub away_team_id: String,
    pub home_team_name: String,
    pub away_team_name: String,
    pub date: String, // ISO 8601 format
    pub gameweek: u8,
    pub home_score: Option<u8>,
    pub away_score: Option<u8>,
    pub played: bool,
    pub venue: String,
}

/// Generate fixtures for a season using round-robin scheduling
pub fn generate_season_fixtures(
    clubs: &[SheffieldClub],
    season_start_year: u32,
    season_start_date: &str, // ISO 8601 format (e.g., "1858-10-24")
) -> Vec<Fixture> {
    let mut fixtures = Vec::new();
    let mut rng = rand::thread_rng();

    // Use round-robin scheduling: each club plays every other club twice (home and away)
    let club_ids: Vec<String> = clubs.iter().map(|c| c.id.clone()).collect();
    let club_names: HashMap<String, String> = clubs
        .iter()
        .map(|c| (c.id.clone(), c.name.clone()))
        .collect();
    let club_venues: HashMap<String, String> = clubs
        .iter()
        .map(|c| (c.id.clone(), c.ground.clone()))
        .collect();

    let num_clubs = club_ids.len();
    let mut fixture_id = 0;

    // Generate all home/away combinations
    for i in 0..num_clubs {
        for j in 0..num_clubs {
            if i != j {
                fixture_id += 1;
                let gameweek = ((fixture_id - 1) / (num_clubs - 1)) as u8 + 1;
                let home_id = club_ids[i].clone();
                let away_id = club_ids[j].clone();

                let fixture = Fixture {
                    id: format!("fixture_{}", fixture_id),
                    home_team_id: home_id.clone(),
                    away_team_id: away_id.clone(),
                    home_team_name: club_names[&home_id].clone(),
                    away_team_name: club_names[&away_id].clone(),
                    date: calculate_fixture_date(gameweek, season_start_date),
                    gameweek,
                    home_score: None,
                    away_score: None,
                    played: false,
                    venue: club_venues[&home_id].clone(),
                };
                fixtures.push(fixture);
            }
        }
    }

    fixtures
}

/// Calculate approximate date for a gameweek match
/// Assumes matches roughly every 7 days, starting from season_start_date
fn calculate_fixture_date(gameweek: u8, season_start_date: &str) -> String {
    if let Ok(start_date) = chrono::NaiveDate::parse_from_str(season_start_date, "%Y-%m-%d") {
        // Add approximately (gameweek - 1) * 7 days
        if let Some(match_date) = start_date.checked_add_signed(
            chrono::Duration::days((gameweek as i64 - 1) * 7),
        ) {
            return match_date.to_string();
        }
    }
    season_start_date.to_string()
}

/// Generate fixtures with randomized dates to simulate historical scheduling
pub fn generate_realistic_season_fixtures(
    clubs: &[SheffieldClub],
    season_start_year: u32,
    season_start_date: &str,
) -> Vec<Fixture> {
    let mut fixtures = generate_season_fixtures(clubs, season_start_year, season_start_date);
    let mut rng = rand::thread_rng();

    // Randomize dates around the scheduled date (+/- 3 days)
    for fixture in &mut fixtures {
        if let Ok(base_date) = chrono::NaiveDate::parse_from_str(&fixture.date, "%Y-%m-%d") {
            let days_offset = rng.gen_range(-3..=3);
            if let Some(adjusted_date) =
                base_date.checked_add_signed(chrono::Duration::days(days_offset))
            {
                fixture.date = adjusted_date.to_string();
            }
        }
    }

    // Sort by date
    fixtures.sort_by(|a, b| a.date.cmp(&b.date));

    // Re-assign gameweeks based on actual dates
    if !fixtures.is_empty() {
        let mut current_gameweek = 1u8;
        let mut gameweek_start = fixtures[0].date.clone();

        for fixture in &mut fixtures {
            // Increment gameweek every 7 days
            if let (Ok(current_date), Ok(week_start)) = (
                chrono::NaiveDate::parse_from_str(&fixture.date, "%Y-%m-%d"),
                chrono::NaiveDate::parse_from_str(&gameweek_start, "%Y-%m-%d"),
            ) {
                let days_diff = (current_date - week_start).num_days();
                if days_diff >= 7 {
                    current_gameweek = current_gameweek.saturating_add(1);
                    gameweek_start = fixture.date.clone();
                }
            }
            fixture.gameweek = current_gameweek;
        }
    }

    fixtures
}

/// Simulate a match and return the result
pub fn simulate_fixture(fixture: &mut Fixture) {
    let mut rng = rand::thread_rng();

    // Use Poisson distribution to generate goals (lambda = 1.5 goals per team)
    // In early football, scores tended to be lower and more varied
    let home_goals = poisson_random(&mut rng, 1.2);
    let away_goals = poisson_random(&mut rng, 1.1);

    fixture.home_score = Some(home_goals as u8);
    fixture.away_score = Some(away_goals as u8);
    fixture.played = true;
}

/// Generate Poisson-distributed random number
fn poisson_random<R: Rng>(rng: &mut R, lambda: f64) -> u32 {
    let l = (-lambda).exp();
    let mut k = 0u32;
    let mut p = 1.0;

    loop {
        k += 1;
        p *= rng.gen::<f64>();
        if p < l {
            break;
        }
    }

    k - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixture_generation() {
        let clubs = vec![
            SheffieldClub {
                id: "sheffield-fc".to_string(),
                name: "Sheffield FC".to_string(),
                founded_year: 1857,
                ground: "East Bank".to_string(),
                origin: "Other".to_string(),
                city: None,
                region: None,
            },
            SheffieldClub {
                id: "hallam-fc".to_string(),
                name: "Hallam FC".to_string(),
                founded_year: 1860,
                ground: "Sandygate".to_string(),
                origin: "Other".to_string(),
                city: None,
                region: None,
            },
        ];

        let fixtures = generate_season_fixtures(&clubs, 1858, "1858-10-24");
        assert_eq!(fixtures.len(), 2); // 2 clubs, 2 matches (home and away)
    }

    #[test]
    fn test_fixture_date_calculation() {
        let date1 = calculate_fixture_date(1, "1858-10-24");
        let date2 = calculate_fixture_date(2, "1858-10-24");
        assert!(date2 > date1);
    }
}
