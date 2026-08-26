/// Lineup Selection System: User-controlled team selection
///
/// Historical accuracy:
/// - No substitutions in 1860s-1870s (allowed from 1965!)
/// - If player injured during match, team plays with fewer men
/// - Formations were fluid (often 1-2-7 or 1-1-8 in Sheffield)
/// - Jersey numbers not used until 1928

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineupSelection {
    pub match_id: String,
    pub club_id: String,
    pub selected_players: Vec<SelectedPlayer>,
    pub formation: Formation,
    pub captain_player_id: String,
    pub is_user_controlled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectedPlayer {
    pub player_id: String,
    pub player_name: String,
    pub position: MatchPosition,
    pub overall_rating: i32,
    pub fitness: i32,
    pub form: i32,
    pub is_injured: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Formation {
    Sheffield1858,     // 1-2-7 (GK, 2 backs, 7 forwards)
    Sheffield1867,     // 1-1-3-5 (GK, 1 back, 1 half-back, 3 midfield, 5 forwards)
    Sheffield1872,     // 1-2-2-5 (GK, 2 backs, 2 half-backs, 5 forwards)
    Nottingham1862,    // 1-1-2-6 (GK, 1 back, 1 half, 2 mid, 6 forwards)
    Cambridge1863,     // 1-2-2-5 (GK, 2 backs, 2 half-backs, 5 forwards)
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchPosition {
    Goalkeeper,
    Back,        // Full-back
    HalfBack,    // Half-back (defensive midfielder)
    Midfielder,  // Midfielder
    Forward,     // Forward/Attacker
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerWithFitness {
    pub player_id: String,
    pub player_name: String,
    pub position: String,
    pub overall_rating: i32,
    pub fitness: i32,
    pub form: i32,
    pub is_injured: bool,
    pub morale: i32,
}

impl LineupSelection {
    /// Create suggested lineup for user approval
    pub async fn create_suggested_lineup(
        pool: &SqlitePool,
        match_id: String,
        club_id: String,
    ) -> Result<Self, String> {
        // Get all available players
        let available_players = Self::get_available_squad(pool, &club_id).await?;

        if available_players.is_empty() {
            return Err("No players available for selection".to_string());
        }

        // Default formation for Sheffield clubs in 1867
        let formation = Formation::Sheffield1867;

        // Sort by effective rating (rating * fitness * form)
        let mut sorted_players = available_players;
        sorted_players.sort_by(|a, b| {
            let a_score = Self::calculate_effective_rating(a);
            let b_score = Self::calculate_effective_rating(b);
            b_score.partial_cmp(&a_score).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Select best 11 non-injured players
        let selected: Vec<SelectedPlayer> = sorted_players
            .iter()
            .filter(|p| !p.is_injured)
            .take(11)
            .enumerate()
            .map(|(idx, p)| SelectedPlayer {
                player_id: p.player_id.clone(),
                player_name: p.player_name.clone(),
                position: Self::assign_position(&formation, idx),
                overall_rating: p.overall_rating,
                fitness: p.fitness,
                form: p.form,
                is_injured: p.is_injured,
            })
            .collect();

        if selected.len() < 11 {
            return Err(format!("Only {} players available, need 11", selected.len()));
        }

        // First player (goalkeeper) is often captain
        let captain = selected[0].player_id.clone();

        Ok(Self {
            match_id,
            club_id,
            selected_players: selected,
            formation,
            captain_player_id: captain,
            is_user_controlled: true,
        })
    }

    /// AI auto-selects lineup for non-user teams
    pub async fn create_ai_lineup(
        pool: &SqlitePool,
        match_id: String,
        club_id: String,
        formation: Formation,
    ) -> Result<Self, String> {
        let available_players = Self::get_available_squad(pool, &club_id).await?;

        let mut sorted_players = available_players;
        sorted_players.sort_by(|a, b| {
            let a_score = Self::calculate_effective_rating(a);
            let b_score = Self::calculate_effective_rating(b);
            b_score.partial_cmp(&a_score).unwrap_or(std::cmp::Ordering::Equal)
        });

        let selected: Vec<SelectedPlayer> = sorted_players
            .iter()
            .filter(|p| !p.is_injured)
            .take(11)
            .enumerate()
            .map(|(idx, p)| SelectedPlayer {
                player_id: p.player_id.clone(),
                player_name: p.player_name.clone(),
                position: Self::assign_position(&formation, idx),
                overall_rating: p.overall_rating,
                fitness: p.fitness,
                form: p.form,
                is_injured: p.is_injured,
            })
            .collect();

        let captain = selected.first().map(|p| p.player_id.clone()).unwrap_or_default();

        Ok(Self {
            match_id,
            club_id,
            selected_players: selected,
            formation,
            captain_player_id: captain,
            is_user_controlled: false,
        })
    }

    async fn get_available_squad(
        pool: &SqlitePool,
        club_id: &str,
    ) -> Result<Vec<PlayerWithFitness>, String> {
        let players = sqlx::query_as::<_, (String, String, String, i32, i32, i32, bool, i32)>(
            r#"
            SELECT
                id, name, position, overall_rating,
                fitness, form, is_injured, morale
            FROM sheffield_footballers
            WHERE club_id = ?
            ORDER BY overall_rating DESC
            "#
        )
        .bind(club_id)
        .fetch_all(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(players.into_iter().map(|p| PlayerWithFitness {
            player_id: p.0,
            player_name: p.1,
            position: p.2,
            overall_rating: p.3,
            fitness: p.4,
            form: p.5,
            is_injured: p.6,
            morale: p.7,
        }).collect())
    }

    fn calculate_effective_rating(player: &PlayerWithFitness) -> f32 {
        let fitness_factor = player.fitness as f32 / 100.0;
        let form_factor = (player.form as f32 + 5.0) / 15.0; // Form is -5 to +5
        let morale_factor = player.morale as f32 / 10.0;

        player.overall_rating as f32 * fitness_factor * form_factor * morale_factor
    }

    fn assign_position(formation: &Formation, index: usize) -> MatchPosition {
        match formation {
            Formation::Sheffield1858 => {
                // 1-2-7: GK, 2 backs, 7 forwards
                match index {
                    0 => MatchPosition::Goalkeeper,
                    1 | 2 => MatchPosition::Back,
                    _ => MatchPosition::Forward,
                }
            }
            Formation::Sheffield1867 => {
                // 1-1-3-5: GK, 1 back, 1 half-back, 3 midfielders, 5 forwards
                match index {
                    0 => MatchPosition::Goalkeeper,
                    1 => MatchPosition::Back,
                    2 => MatchPosition::HalfBack,
                    3..=5 => MatchPosition::Midfielder,
                    _ => MatchPosition::Forward,
                }
            }
            Formation::Sheffield1872 => {
                // 1-2-2-5: GK, 2 backs, 2 half-backs, 5 forwards
                match index {
                    0 => MatchPosition::Goalkeeper,
                    1 | 2 => MatchPosition::Back,
                    3 | 4 => MatchPosition::HalfBack,
                    _ => MatchPosition::Forward,
                }
            }
            Formation::Nottingham1862 => {
                // 1-1-2-6: GK, 1 back, 1 half, 2 mid, 6 forwards
                match index {
                    0 => MatchPosition::Goalkeeper,
                    1 => MatchPosition::Back,
                    2 => MatchPosition::HalfBack,
                    3 | 4 => MatchPosition::Midfielder,
                    _ => MatchPosition::Forward,
                }
            }
            Formation::Cambridge1863 => {
                // 1-2-2-5: GK, 2 backs, 2 half-backs, 5 forwards
                match index {
                    0 => MatchPosition::Goalkeeper,
                    1 | 2 => MatchPosition::Back,
                    3 | 4 => MatchPosition::HalfBack,
                    _ => MatchPosition::Forward,
                }
            }
            Formation::Custom(_) => MatchPosition::Forward,
        }
    }

    /// Save lineup to database
    pub async fn save(&self, pool: &SqlitePool) -> Result<(), String> {
        sqlx::query(
            r#"
            INSERT INTO sheffield_match_lineup_selections
            (match_id, club_id, formation, captain_player_id)
            VALUES (?, ?, ?, ?)
            ON CONFLICT(match_id, club_id) DO UPDATE SET
                formation = excluded.formation,
                captain_player_id = excluded.captain_player_id
            "#
        )
        .bind(&self.match_id)
        .bind(&self.club_id)
        .bind(self.formation_str())
        .bind(&self.captain_player_id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn formation_str(&self) -> String {
        match &self.formation {
            Formation::Sheffield1858 => "1-2-7".to_string(),
            Formation::Sheffield1867 => "1-1-3-5".to_string(),
            Formation::Sheffield1872 => "1-2-2-5".to_string(),
            Formation::Nottingham1862 => "1-1-2-6".to_string(),
            Formation::Cambridge1863 => "1-2-2-5".to_string(),
            Formation::Custom(s) => s.clone(),
        }
    }
}

impl MatchPosition {
    pub fn to_string(&self) -> String {
        match self {
            MatchPosition::Goalkeeper => "GK".to_string(),
            MatchPosition::Back => "FB".to_string(),
            MatchPosition::HalfBack => "HB".to_string(),
            MatchPosition::Midfielder => "MID".to_string(),
            MatchPosition::Forward => "FWD".to_string(),
        }
    }
}
