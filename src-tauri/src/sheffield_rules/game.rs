/// Sheffield Rules Game State
///
/// Manages the state of a Sheffield Rules game including:
/// - Current season and date
/// - User's selected club
/// - Game fixtures and results
/// - League standings
/// - Game mode and rules progression

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::fixtures::Fixture;
use super::ruleset::RuleSet;
use super::game_mode::GameMode;
use super::clubs::SheffieldClub;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheffieldGameState {
    pub id: String,
    pub game_mode: GameMode,
    pub user_club_id: String,
    pub user_club_name: String,
    pub season_start_year: u32,
    pub current_date: String, // ISO 8601 format
    pub season_end_date: String,
    pub fixtures: Vec<Fixture>,
    pub standings: Vec<LeagueStanding>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeagueStanding {
    pub club_id: String,
    pub club_name: String,
    pub position: u8,
    pub played: u8,
    pub won: u8,
    pub drawn: u8,
    pub lost: u8,
    pub goals_for: u16,
    pub goals_against: u16,
    pub goal_difference: i16,
    pub points: u16,
    pub rouges: u8, // For 1862-1868 era with rouge scoring
}

impl SheffieldGameState {
    /// Create a new Sheffield Rules game
    pub fn new(
        game_mode: GameMode,
        user_club_id: String,
        user_club_name: String,
        season_start_year: u32,
        clubs: Vec<SheffieldClub>,
    ) -> Self {
        // Determine season start date based on historical context
        let season_start_date = format!("{}-10-01", season_start_year); // Most seasons started in October
        let season_end_date = format!("{}-08-31", season_start_year + 1);

        // Generate fixtures
        let fixtures = super::fixtures::generate_realistic_season_fixtures(
            &clubs,
            season_start_year,
            &season_start_date,
        );

        // Initialize standings
        let standings: Vec<LeagueStanding> = clubs
            .iter()
            .enumerate()
            .map(|(idx, club)| LeagueStanding {
                club_id: club.id.clone(),
                club_name: club.name.clone(),
                position: (idx + 1) as u8,
                played: 0,
                won: 0,
                drawn: 0,
                lost: 0,
                goals_for: 0,
                goals_against: 0,
                goal_difference: 0,
                points: 0,
                rouges: 0,
            })
            .collect();

        let now = chrono::Utc::now().to_rfc3339();

        SheffieldGameState {
            id: Uuid::new_v4().to_string(),
            game_mode,
            user_club_id,
            user_club_name,
            season_start_year,
            current_date: season_start_date,
            season_end_date,
            fixtures,
            standings,
            created_at: now.clone(),
            updated_at: now,
        }
    }

    /// Get the current ruleset for this game based on the season year
    pub fn get_current_ruleset(&self) -> std::sync::Arc<dyn RuleSet> {
        super::rulesets::get_ruleset_for_year(self.season_start_year, None)
    }

    /// Advance the game by one day
    pub fn advance_day(&mut self) {
        if let Ok(current) = chrono::NaiveDate::parse_from_str(&self.current_date, "%Y-%m-%d") {
            if let Some(next_date) = current.succ_opt() {
                self.current_date = next_date.to_string();
                self.updated_at = chrono::Utc::now().to_rfc3339();
            }
        }
    }

    /// Get fixtures for the next N days
    pub fn get_upcoming_fixtures(&self, days: u8) -> Vec<Fixture> {
        if let Ok(current_date) = chrono::NaiveDate::parse_from_str(&self.current_date, "%Y-%m-%d") {
            let end_date = current_date
                .checked_add_signed(chrono::Duration::days(days as i64))
                .unwrap_or(current_date);

            self.fixtures
                .iter()
                .filter(|f| {
                    !f.played
                        && f.date >= self.current_date
                        && f.date
                            <= end_date
                                .format("%Y-%m-%d")
                                .to_string()
                })
                .cloned()
                .collect()
        } else {
            vec![]
        }
    }

    /// Simulate a match and update standings
    pub fn simulate_match(&mut self, fixture_id: &str) -> bool {
        // Find and simulate the fixture
        let fixture_found = self.fixtures.iter_mut().find(|f| f.id == fixture_id).is_some();

        if fixture_found {
            if let Some(fixture) = self.fixtures.iter_mut().find(|f| f.id == fixture_id) {
                super::fixtures::simulate_fixture(fixture);
            }

            // Get a clone of the fixture for standings update
            if let Some(fixture) = self.fixtures.iter().find(|f| f.id == fixture_id) {
                let fixture_copy = fixture.clone();
                self.update_standings_from_fixture(&fixture_copy);
            }

            self.updated_at = chrono::Utc::now().to_rfc3339();
            true
        } else {
            false
        }
    }

    /// Update standings after a fixture is played
    fn update_standings_from_fixture(&mut self, fixture: &Fixture) {
        let home_score = fixture.home_score.unwrap_or(0);
        let away_score = fixture.away_score.unwrap_or(0);

        // Update home team
        if let Some(home_standing) = self
            .standings
            .iter_mut()
            .find(|s| s.club_id == fixture.home_team_id)
        {
            home_standing.played += 1;
            home_standing.goals_for += home_score as u16;
            home_standing.goals_against += away_score as u16;

            if home_score > away_score {
                home_standing.won += 1;
                home_standing.points += 2; // 2 points for win in 1888
            } else if home_score == away_score {
                home_standing.drawn += 1;
                home_standing.points += 1;
            } else {
                home_standing.lost += 1;
            }
        }

        // Update away team
        if let Some(away_standing) = self
            .standings
            .iter_mut()
            .find(|s| s.club_id == fixture.away_team_id)
        {
            away_standing.played += 1;
            away_standing.goals_for += away_score as u16;
            away_standing.goals_against += home_score as u16;

            if away_score > home_score {
                away_standing.won += 1;
                away_standing.points += 2;
            } else if away_score == home_score {
                away_standing.drawn += 1;
                away_standing.points += 1;
            } else {
                away_standing.lost += 1;
            }
        }

        // Recalculate goal differences and sort
        for standing in &mut self.standings {
            standing.goal_difference =
                standing.goals_for as i16 - standing.goals_against as i16;
        }

        self.standings.sort_by(|a, b| {
            b.points
                .cmp(&a.points)
                .then_with(|| b.goal_difference.cmp(&a.goal_difference))
                .then_with(|| b.goals_for.cmp(&a.goals_for))
        });

        // Update positions
        for (idx, standing) in self.standings.iter_mut().enumerate() {
            standing.position = (idx + 1) as u8;
        }
    }

    /// Check if season is complete
    pub fn is_season_complete(&self) -> bool {
        self.fixtures.iter().all(|f| f.played)
    }

    /// Get user's team standing
    pub fn get_user_standing(&self) -> Option<LeagueStanding> {
        self.standings
            .iter()
            .find(|s| s.club_id == self.user_club_id)
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_creation() {
        let clubs = vec![SheffieldClub {
            id: "sheffield-fc".to_string(),
            name: "Sheffield FC".to_string(),
            founded_year: 1857,
            ground: "East Bank".to_string(),
            origin: "Other".to_string(),
            city: None,
            region: None,
        }];

        let game = SheffieldGameState::new(
            GameMode::HistoricalTimeline,
            "sheffield-fc".to_string(),
            "Sheffield FC".to_string(),
            1858,
            clubs,
        );

        assert_eq!(game.season_start_year, 1858);
        assert_eq!(game.user_club_id, "sheffield-fc");
        assert!(!game.standings.is_empty());
    }
}
