/// Player attribute mapping system for Sheffield Rules
/// Maps 67 detailed player attributes → engine-usable quality metrics (0.0-1.0)

use serde::{Deserialize, Serialize};

/// Full player attributes from database (67 attributes on 1-20 scale)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAttributes {
    // Physical Attributes
    pub pace: i32,
    pub acceleration: i32,
    pub strength: i32,
    pub stamina: i32,
    pub balance: i32,
    pub jumping: i32,
    pub agility: i32,
    pub natural_fitness: i32,

    // Technical Attributes
    pub passing: i32,
    pub dribbling: i32,
    pub first_touch: i32,
    pub technique: i32,
    pub heading: i32,
    pub long_passing: i32,
    pub crossing: i32,
    pub long_shots: i32,
    pub tackling: i32,
    pub handling: i32,      // GK only
    pub reflexes: i32,      // GK only
    pub corners: i32,
    pub free_kicks: i32,
    pub throw_ins: i32,
    pub vision: i32,
    pub left_foot: i32,
    pub right_foot: i32,
    pub one_on_ones: i32,

    // Mental Attributes
    pub courage: i32,
    pub bravery: i32,
    pub concentration: i32,
    pub decision_making: i32,
    pub leadership: i32,
    pub aggression: i32,
    pub anticipation: i32,
    pub determination: i32,
    pub flair: i32,
    pub influence: i32,
    pub adaptability: i32,
    pub ambition: i32,
    pub loyalty: i32,
    pub pressure: i32,
    pub professionalism: i32,
    pub sportsmanship: i32,
    pub temperament: i32,

    // Positioning Attributes
    pub awareness: i32,
    pub marking: i32,
    pub positioning: i32,
    pub work_rate: i32,
    pub off_the_ball: i32,
    pub movement: i32,
    pub teamwork: i32,

    // Specialization Attributes
    pub finishing: i32,
    pub penalties: i32,
    pub set_pieces: i32,

    // Hidden Attributes (0-20)
    pub consistency: i32,
    pub important_matches: i32,
    pub injury_proneness: i32,
    pub versatility: i32,
    pub dirtiness: i32,
}

impl PlayerAttributes {
    /// Normalize attribute from 1-20 scale to 0.0-1.0
    fn normalize(value: i32) -> f32 {
        (value as f32 / 20.0).max(0.0).min(1.0)
    }

    /// Calculate overall outfield player quality (0.0-1.0)
    /// Weighted combination of key attributes for match engine
    pub fn calculate_overall_quality(&self) -> f32 {
        // Physical component (25% weight)
        let physical = (
            Self::normalize(self.pace) * 0.3 +
            Self::normalize(self.acceleration) * 0.2 +
            Self::normalize(self.strength) * 0.2 +
            Self::normalize(self.stamina) * 0.15 +
            Self::normalize(self.agility) * 0.15
        ) * 0.25;

        // Technical component (35% weight)
        let technical = (
            Self::normalize(self.passing) * 0.25 +
            Self::normalize(self.dribbling) * 0.20 +
            Self::normalize(self.first_touch) * 0.15 +
            Self::normalize(self.technique) * 0.15 +
            Self::normalize(self.tackling) * 0.15 +
            Self::normalize(self.finishing) * 0.10
        ) * 0.35;

        // Mental component (25% weight)
        let mental = (
            Self::normalize(self.decision_making) * 0.30 +
            Self::normalize(self.concentration) * 0.20 +
            Self::normalize(self.anticipation) * 0.20 +
            Self::normalize(self.determination) * 0.15 +
            Self::normalize(self.courage) * 0.15
        ) * 0.25;

        // Positioning component (15% weight)
        let positioning = (
            Self::normalize(self.awareness) * 0.30 +
            Self::normalize(self.positioning) * 0.30 +
            Self::normalize(self.work_rate) * 0.20 +
            Self::normalize(self.teamwork) * 0.20
        ) * 0.15;

        (physical + technical + mental + positioning).max(0.05).min(0.95)
    }

    /// Calculate goalkeeper-specific quality (0.0-1.0)
    pub fn calculate_goalkeeper_quality(&self) -> f32 {
        // Goalkeeper-specific attributes
        let shot_stopping = (
            Self::normalize(self.reflexes) * 0.40 +
            Self::normalize(self.handling) * 0.30 +
            Self::normalize(self.positioning) * 0.30
        ) * 0.60;

        // Physical attributes (20% for GK)
        let physical = (
            Self::normalize(self.agility) * 0.40 +
            Self::normalize(self.jumping) * 0.30 +
            Self::normalize(self.strength) * 0.30
        ) * 0.20;

        // Mental attributes (20% for GK)
        let mental = (
            Self::normalize(self.concentration) * 0.40 +
            Self::normalize(self.anticipation) * 0.30 +
            Self::normalize(self.decision_making) * 0.30
        ) * 0.20;

        (shot_stopping + physical + mental).max(0.25).min(0.95)
    }

    /// Calculate passing ability (for possession engine)
    pub fn calculate_passing_quality(&self) -> f32 {
        (
            Self::normalize(self.passing) * 0.40 +
            Self::normalize(self.long_passing) * 0.20 +
            Self::normalize(self.vision) * 0.20 +
            Self::normalize(self.decision_making) * 0.10 +
            Self::normalize(self.technique) * 0.10
        ).max(0.0).min(1.0)
    }

    /// Calculate dribbling/ball control ability
    pub fn calculate_dribbling_quality(&self) -> f32 {
        (
            Self::normalize(self.dribbling) * 0.40 +
            Self::normalize(self.first_touch) * 0.25 +
            Self::normalize(self.agility) * 0.20 +
            Self::normalize(self.balance) * 0.15
        ).max(0.0).min(1.0)
    }

    /// Calculate defensive ability
    pub fn calculate_defensive_quality(&self) -> f32 {
        (
            Self::normalize(self.tackling) * 0.35 +
            Self::normalize(self.marking) * 0.25 +
            Self::normalize(self.positioning) * 0.20 +
            Self::normalize(self.anticipation) * 0.20
        ).max(0.0).min(1.0)
    }

    /// Calculate attacking/finishing ability
    pub fn calculate_attacking_quality(&self) -> f32 {
        (
            Self::normalize(self.finishing) * 0.40 +
            Self::normalize(self.off_the_ball) * 0.20 +
            Self::normalize(self.positioning) * 0.15 +
            Self::normalize(self.concentration) * 0.15 +
            Self::normalize(self.anticipation) * 0.10
        ).max(0.0).min(1.0)
    }

    /// Calculate physical/pace attributes
    pub fn calculate_pace_quality(&self) -> f32 {
        (
            Self::normalize(self.pace) * 0.60 +
            Self::normalize(self.acceleration) * 0.40
        ).max(0.0).min(1.0)
    }

    /// Get composure (normalized pressure resistance)
    pub fn composure(&self) -> i32 {
        // Inverse of pressure (high pressure = low composure)
        20 - self.pressure.min(20).max(0)
    }
}

/// Team quality calculator - aggregates 11 player qualities
pub struct TeamQualityCalculator;

impl TeamQualityCalculator {
    /// Calculate team overall quality from player attributes
    /// Returns normalized 0.0-1.0 quality rating
    pub fn calculate_team_quality(players: &[PlayerAttributes], position: &str) -> f32 {
        if players.is_empty() {
            return 0.5; // Default average
        }

        let qualities: Vec<f32> = players
            .iter()
            .map(|p| {
                if position == "GK" {
                    p.calculate_goalkeeper_quality()
                } else {
                    p.calculate_overall_quality()
                }
            })
            .collect();

        let sum: f32 = qualities.iter().sum();
        let average = sum / qualities.len() as f32;

        average.max(0.05).min(0.95)
    }

    /// Calculate captain cohesion (leadership + decision making)
    pub fn calculate_captain_cohesion(captain_attrs: Option<&PlayerAttributes>) -> f32 {
        match captain_attrs {
            Some(attrs) => {
                let leadership = PlayerAttributes::normalize(attrs.leadership);
                let decision_making = PlayerAttributes::normalize(attrs.decision_making);
                (leadership + decision_making) / 2.0
            }
            None => 0.5, // Default neutral cohesion
        }
    }

    /// Calculate team passing ability (for possession chains)
    pub fn calculate_team_passing(players: &[PlayerAttributes]) -> f32 {
        if players.is_empty() {
            return 0.5;
        }

        let sum: f32 = players
            .iter()
            .map(|p| p.calculate_passing_quality())
            .sum();

        (sum / players.len() as f32).max(0.05).min(0.95)
    }

    /// Calculate team defensive ability
    pub fn calculate_team_defending(players: &[PlayerAttributes]) -> f32 {
        if players.is_empty() {
            return 0.5;
        }

        let sum: f32 = players
            .iter()
            .map(|p| p.calculate_defensive_quality())
            .sum();

        (sum / players.len() as f32).max(0.05).min(0.95)
    }

    /// Calculate team attacking ability
    pub fn calculate_team_attacking(players: &[PlayerAttributes]) -> f32 {
        if players.is_empty() {
            return 0.5;
        }

        let sum: f32 = players
            .iter()
            .map(|p| p.calculate_attacking_quality())
            .sum();

        (sum / players.len() as f32).max(0.05).min(0.95)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_player() -> PlayerAttributes {
        PlayerAttributes {
            pace: 15, acceleration: 14, strength: 12, stamina: 16,
            balance: 13, jumping: 11, agility: 14, natural_fitness: 15,
            passing: 16, dribbling: 14, first_touch: 15, technique: 14,
            heading: 12, long_passing: 13, crossing: 11, long_shots: 10,
            tackling: 13, handling: 5, reflexes: 5, corners: 10,
            free_kicks: 11, throw_ins: 10, vision: 14, left_foot: 12,
            right_foot: 16, one_on_ones: 12, courage: 15, bravery: 14,
            concentration: 15, decision_making: 16, leadership: 12,
            aggression: 11, anticipation: 14, determination: 15,
            flair: 13, influence: 11, adaptability: 12, ambition: 14,
            loyalty: 13, pressure: 8, professionalism: 15,
            sportsmanship: 16, temperament: 14, awareness: 15,
            marking: 12, positioning: 14, work_rate: 16,
            off_the_ball: 13, movement: 14, teamwork: 15,
            finishing: 14, penalties: 12, set_pieces: 11,
            consistency: 13, important_matches: 14, injury_proneness: 6,
            versatility: 11, dirtiness: 5,
        }
    }

    #[test]
    fn test_normalize_attribute() {
        assert_eq!(PlayerAttributes::normalize(0), 0.0);
        assert_eq!(PlayerAttributes::normalize(10), 0.5);
        assert_eq!(PlayerAttributes::normalize(20), 1.0);
    }

    #[test]
    fn test_overall_quality() {
        let player = create_test_player();
        let quality = player.calculate_overall_quality();

        // Should be above average (0.5) for good attributes
        assert!(quality > 0.6 && quality < 0.8);
    }

    #[test]
    fn test_team_quality() {
        let players = vec![create_test_player(); 11];
        let quality = TeamQualityCalculator::calculate_team_quality(&players, "MID");

        assert!(quality > 0.5 && quality < 0.9);
    }

    #[test]
    fn test_goalkeeper_quality() {
        let mut gk = create_test_player();
        gk.reflexes = 18;
        gk.handling = 17;
        gk.positioning = 16;

        let quality = gk.calculate_goalkeeper_quality();
        assert!(quality > 0.7); // Should be high for good GK stats
    }

    #[test]
    fn test_captain_cohesion() {
        let player = create_test_player();
        let cohesion = TeamQualityCalculator::calculate_captain_cohesion(Some(&player));

        // leadership=12, decision_making=16 → avg 14/20 = 0.7
        assert!((cohesion - 0.7).abs() < 0.01);
    }
}
