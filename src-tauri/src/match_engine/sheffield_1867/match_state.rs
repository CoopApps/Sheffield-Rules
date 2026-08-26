/// Sheffield 1867 Match State Display System
/// Provides real-time match information with rouge scoring support

use serde::{Deserialize, Serialize};
use super::player_performance::PlayerPerformance;

/// Real-time snapshot of match conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchStateSnapshot {
    pub minute: i32,
    pub home_team: String,
    pub away_team: String,
    pub home_goals: i32,
    pub away_goals: i32,
    pub home_rouges: i32,  // Sheffield Rules 1862-1868
    pub away_rouges: i32,
    pub possession_home: f32,
    pub possession_away: f32,

    // Team condition
    pub home_fatigue: f32,
    pub away_fatigue: f32,
    pub home_morale: f32,
    pub away_morale: f32,
    pub home_intensity: f32,
    pub away_intensity: f32,

    // Recent events
    pub last_score_minute: i32,
    pub scores_this_interval: i32,
    pub shots_this_interval: i32,

    // Player performances
    pub home_top_performers: Vec<PlayerSnapshot>,
    pub away_top_performers: Vec<PlayerSnapshot>,
    pub home_struggling_players: Vec<PlayerSnapshot>,
    pub away_struggling_players: Vec<PlayerSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSnapshot {
    pub player_id: i32,
    pub player_name: String,
    pub position: String,
    pub form_rating: f32,
    pub fitness: f32,
    pub morale: f32,
}

impl MatchStateSnapshot {
    pub fn new() -> Self {
        MatchStateSnapshot {
            minute: 0,
            home_team: String::new(),
            away_team: String::new(),
            home_goals: 0,
            away_goals: 0,
            home_rouges: 0,
            away_rouges: 0,
            possession_home: 0.5,
            possession_away: 0.5,
            home_fatigue: 0.0,
            away_fatigue: 0.0,
            home_morale: 0.7,
            away_morale: 0.7,
            home_intensity: 0.5,
            away_intensity: 0.5,
            last_score_minute: -1,
            scores_this_interval: 0,
            shots_this_interval: 0,
            home_top_performers: vec![],
            away_top_performers: vec![],
            home_struggling_players: vec![],
            away_struggling_players: vec![],
        }
    }

    pub fn from_game_state(
        minute: i32,
        home_team_name: String,
        away_team_name: String,
        home_goals: i32,
        away_goals: i32,
        home_rouges: i32,
        away_rouges: i32,
        home_players: &[PlayerPerformance],
        away_players: &[PlayerPerformance],
        home_fatigue: f32,
        away_fatigue: f32,
        home_morale: f32,
        away_morale: f32,
        home_intensity: f32,
        away_intensity: f32,
        last_score_minute: i32,
    ) -> Self {
        // Get top 3 performers for each team
        let mut home_perf: Vec<_> = home_players.to_vec();
        home_perf.sort_by(|a, b| b.current_form.partial_cmp(&a.current_form).unwrap());
        let home_top = home_perf
            .iter()
            .take(3)
            .map(|p| PlayerSnapshot {
                player_id: p.player_id,
                player_name: p.player_name.clone(),
                position: p.position.clone(),
                form_rating: p.current_form,
                fitness: p.fitness_level,
                morale: p.morale,
            })
            .collect();

        let home_struggling = home_perf
            .iter()
            .rev()
            .take(2)
            .map(|p| PlayerSnapshot {
                player_id: p.player_id,
                player_name: p.player_name.clone(),
                position: p.position.clone(),
                form_rating: p.current_form,
                fitness: p.fitness_level,
                morale: p.morale,
            })
            .collect();

        let mut away_perf: Vec<_> = away_players.to_vec();
        away_perf.sort_by(|a, b| b.current_form.partial_cmp(&a.current_form).unwrap());
        let away_top = away_perf
            .iter()
            .take(3)
            .map(|p| PlayerSnapshot {
                player_id: p.player_id,
                player_name: p.player_name.clone(),
                position: p.position.clone(),
                form_rating: p.current_form,
                fitness: p.fitness_level,
                morale: p.morale,
            })
            .collect();

        let away_struggling = away_perf
            .iter()
            .rev()
            .take(2)
            .map(|p| PlayerSnapshot {
                player_id: p.player_id,
                player_name: p.player_name.clone(),
                position: p.position.clone(),
                form_rating: p.current_form,
                fitness: p.fitness_level,
                morale: p.morale,
            })
            .collect();

        MatchStateSnapshot {
            minute,
            home_team: home_team_name,
            away_team: away_team_name,
            home_goals,
            away_goals,
            home_rouges,
            away_rouges,
            possession_home: 0.5,
            possession_away: 0.5,
            home_fatigue,
            away_fatigue,
            home_morale,
            away_morale,
            home_intensity,
            away_intensity,
            last_score_minute,
            scores_this_interval: 0,
            shots_this_interval: 0,
            home_top_performers: home_top,
            away_top_performers: away_top,
            home_struggling_players: home_struggling,
            away_struggling_players: away_struggling,
        }
    }

    /// Get Sheffield Rules score display (goals, rouges)
    pub fn get_score_display(&self) -> String {
        if self.home_rouges > 0 || self.away_rouges > 0 {
            format!(
                "{} goals {} rouges - {} goals {} rouges",
                self.home_goals, self.home_rouges,
                self.away_goals, self.away_rouges
            )
        } else {
            format!("{} - {}", self.home_goals, self.away_goals)
        }
    }

    /// Determine match result per Sheffield Rules
    pub fn get_match_result(&self) -> String {
        if self.home_goals != self.away_goals {
            if self.home_goals > self.away_goals {
                format!("{} leads", self.home_team)
            } else {
                format!("{} leads", self.away_team)
            }
        } else if self.home_rouges != self.away_rouges {
            if self.home_rouges > self.away_rouges {
                format!("{} leads on rouges", self.home_team)
            } else {
                format!("{} leads on rouges", self.away_team)
            }
        } else {
            "Match level".to_string()
        }
    }
}

impl Default for MatchStateSnapshot {
    fn default() -> Self {
        Self::new()
    }
}
