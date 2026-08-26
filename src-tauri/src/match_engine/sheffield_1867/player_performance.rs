/// Sheffield 1867 Player Performance Tracking System
/// Adapted from Football Man for Sheffield Rules era
/// Tracks individual player stats during match execution

use serde::{Deserialize, Serialize};

/// Individual player performance metrics tracked during a match
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerPerformance {
    pub player_id: i32,
    pub player_name: String,
    pub position: String,

    // Performance metrics (0.0-1.0)
    pub current_form: f32,
    pub fitness_level: f32,
    pub morale: f32,
    pub passing_accuracy: f32,
    pub defensive_rating: f32,
    pub attacking_rating: f32,

    // Match action statistics
    pub passes_completed: i32,
    pub passes_attempted: i32,
    pub tackles_attempted: i32,
    pub tackles_won: i32,
    pub shots_attempted: i32,
    pub goals_scored: i32,
    pub rouges_scored: i32,  // Sheffield Rules 1862-1868
    pub assists: i32,
    pub yellow_cards: u8,
    pub red_cards: u8,

    // Tactical awareness
    pub minutes_played: i32,
    pub is_substituted_off: bool,
    pub impact_on_team_morale: f32,
}

impl PlayerPerformance {
    pub fn new(player_id: i32, player_name: String, position: String) -> Self {
        PlayerPerformance {
            player_id,
            player_name,
            position,
            current_form: 0.5,
            fitness_level: 1.0,
            morale: 0.5,
            passing_accuracy: 0.7,
            defensive_rating: 0.6,
            attacking_rating: 0.6,
            passes_completed: 0,
            passes_attempted: 0,
            tackles_attempted: 0,
            tackles_won: 0,
            shots_attempted: 0,
            goals_scored: 0,
            rouges_scored: 0,
            assists: 0,
            yellow_cards: 0,
            red_cards: 0,
            minutes_played: 0,
            is_substituted_off: false,
            impact_on_team_morale: 0.0,
        }
    }

    pub fn record_pass(&mut self, successful: bool) {
        self.passes_attempted += 1;
        if successful {
            self.passes_completed += 1;
            self.current_form = (self.current_form + 0.02).min(1.0);
            self.morale = (self.morale + 0.01).min(1.0);
        } else {
            self.current_form = (self.current_form - 0.01).max(0.0);
        }
    }

    pub fn record_tackle(&mut self, successful: bool) {
        self.tackles_attempted += 1;
        if successful {
            self.tackles_won += 1;
            self.current_form = (self.current_form + 0.03).min(1.0);
            self.morale = (self.morale + 0.02).min(1.0);
            self.defensive_rating = (self.defensive_rating + 0.01).min(1.0);
        } else {
            self.current_form = (self.current_form - 0.02).max(0.0);
        }
    }

    pub fn record_shot(&mut self, successful: bool) {
        self.shots_attempted += 1;
        if successful {
            self.goals_scored += 1;
            self.current_form = 1.0;
            self.morale = 1.0;
            self.attacking_rating = (self.attacking_rating + 0.05).min(1.0);
            self.impact_on_team_morale = (self.impact_on_team_morale + 0.15).min(0.5);
        } else {
            self.current_form = (self.current_form - 0.01).max(0.0);
            self.attacking_rating = (self.attacking_rating - 0.01).max(0.0);
        }
    }

    /// Sheffield Rules: Record a rouge (behind-goal placement, 1862-1868)
    pub fn record_rouge(&mut self) {
        self.rouges_scored += 1;
        self.current_form = (self.current_form + 0.04).min(1.0);
        self.morale = (self.morale + 0.03).min(1.0);
        self.attacking_rating = (self.attacking_rating + 0.02).min(1.0);
        self.impact_on_team_morale = (self.impact_on_team_morale + 0.08).min(0.5);
    }

    pub fn record_assist(&mut self) {
        self.assists += 1;
        self.current_form = (self.current_form + 0.04).min(1.0);
        self.morale = (self.morale + 0.03).min(1.0);
        self.attacking_rating = (self.attacking_rating + 0.02).min(1.0);
        self.impact_on_team_morale = (self.impact_on_team_morale + 0.10).min(0.5);
    }

    pub fn record_yellow_card(&mut self) {
        self.yellow_cards += 1;
        self.current_form = (self.current_form - 0.05).max(0.0);
        self.morale = (self.morale - 0.10).max(0.0);
    }

    pub fn record_red_card(&mut self) {
        self.red_cards += 1;
        self.current_form = 0.0;
        self.morale = 0.0;
        self.is_substituted_off = true;
    }

    pub fn update_fitness(&mut self, minutes_elapsed: i32) {
        self.minutes_played = minutes_elapsed;
        let fatigue_factor = (minutes_elapsed as f32) / 90.0;
        self.fitness_level = (1.0 - fatigue_factor).max(0.1);
    }

    pub fn get_current_rating(&self) -> f32 {
        let base_rating = match self.position.to_lowercase().as_str() {
            p if p.contains("back") => self.defensive_rating,
            p if p.contains("mid") => (self.defensive_rating + self.attacking_rating) / 2.0,
            p if p.contains("forward") || p.contains("wing") => self.attacking_rating,
            _ => 0.6,
        };

        (self.current_form * 0.4 + self.fitness_level * 0.4 + base_rating * 0.2).min(1.0)
    }

    pub fn get_passing_accuracy_percentage(&self) -> f32 {
        if self.passes_attempted == 0 {
            0.7
        } else {
            (self.passes_completed as f32 / self.passes_attempted as f32).min(1.0)
        }
    }

    pub fn is_performing_well(&self) -> bool {
        self.get_current_rating() > 0.65
    }

    pub fn is_struggling(&self) -> bool {
        self.get_current_rating() < 0.45
    }

    pub fn get_performance_summary(&self) -> String {
        if self.red_cards > 0 {
            format!("{} has been sent off", self.player_name)
        } else if self.is_substituted_off {
            format!("{} has been substituted off", self.player_name)
        } else if self.goals_scored > 0 {
            format!("{} has scored {} goal(s)", self.player_name, self.goals_scored)
        } else if self.rouges_scored > 0 {
            format!("{} has scored {} rouge(s)", self.player_name, self.rouges_scored)
        } else if self.is_performing_well() {
            format!("{} is playing well", self.player_name)
        } else if self.is_struggling() {
            format!("{} is struggling", self.player_name)
        } else {
            format!("{} is playing adequately", self.player_name)
        }
    }
}
