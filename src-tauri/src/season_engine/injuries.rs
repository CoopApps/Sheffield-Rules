/// Injury System: Tracks player injuries with era-appropriate rates
///
/// Features:
/// - Era-specific injury rates (higher in 1860s-1870s)
/// - Different injury types (muscle, joint, bone, cuts, etc.)
/// - Injury severity determines recovery time
/// - Historical context (no shin pads until 1874)

use rand::Rng;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjurySystem {
    pub base_injury_rate: f32, // per player per match
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Injury {
    pub player_id: String,
    pub injury_type: InjuryType,
    pub occurred_date: NaiveDate,
    pub expected_return_date: NaiveDate,
    pub severity: InjurySeverity,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InjuryType {
    Muscle,
    Joint,
    Bone,
    Concussion,
    Cut,
    Bruising,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InjurySeverity {
    Minor,    // 1-2 weeks
    Moderate, // 3-6 weeks
    Serious,  // 7-12 weeks
    Career,   // Season-ending or career-ending
}

impl Default for InjurySystem {
    fn default() -> Self {
        Self {
            base_injury_rate: 0.025, // 2.5% chance per player per match
        }
    }
}

impl InjurySystem {
    pub fn new(base_injury_rate: f32) -> Self {
        Self { base_injury_rate }
    }

    /// Roll for injuries during a match
    pub fn check_match_injuries(
        &self,
        home_team: &[String], // player_ids
        away_team: &[String],
        ruleset_year: i64,
        current_date: NaiveDate,
    ) -> Vec<Injury> {
        let mut injuries = vec![];
        let mut rng = rand::thread_rng();

        // Historical note: Football was more dangerous in 1860s-1870s
        // No shin pads until 1874, heavier balls, rougher play
        let era_modifier = match ruleset_year {
            1858..=1870 => 1.5, // Higher injury rate
            1871..=1877 => 1.2, // Moderate
            _ => 1.0,            // Modern
        };

        let effective_rate = self.base_injury_rate * era_modifier;

        for player_id in home_team.iter().chain(away_team.iter()) {
            let injury_roll: f32 = rng.gen();

            if injury_roll < effective_rate {
                let severity = self.roll_injury_severity(&mut rng);
                let injury_type = self.roll_injury_type(&mut rng, ruleset_year);
                let weeks_out = self.calculate_weeks_out(&severity, &mut rng);

                let expected_return = current_date
                    .checked_add_signed(chrono::Duration::weeks(weeks_out))
                    .unwrap_or(current_date);

                injuries.push(Injury {
                    player_id: player_id.clone(),
                    injury_type: injury_type.clone(),
                    occurred_date: current_date,
                    expected_return_date: expected_return,
                    severity: severity.clone(),
                    description: self.generate_injury_description(&injury_type, &severity),
                });
            }
        }

        injuries
    }

    fn roll_injury_severity(&self, rng: &mut impl Rng) -> InjurySeverity {
        let roll: f32 = rng.gen();
        match roll {
            x if x < 0.60 => InjurySeverity::Minor,
            x if x < 0.85 => InjurySeverity::Moderate,
            x if x < 0.98 => InjurySeverity::Serious,
            _ => InjurySeverity::Career,
        }
    }

    fn roll_injury_type(&self, rng: &mut impl Rng, year: i64) -> InjuryType {
        // Earlier years: more cuts/bruising (no shin pads)
        if year < 1874 {
            let roll: f32 = rng.gen();
            match roll {
                x if x < 0.30 => InjuryType::Cut,
                x if x < 0.50 => InjuryType::Bruising,
                x if x < 0.70 => InjuryType::Muscle,
                x if x < 0.85 => InjuryType::Joint,
                x if x < 0.95 => InjuryType::Bone,
                _ => InjuryType::Concussion,
            }
        } else {
            let roll: f32 = rng.gen();
            match roll {
                x if x < 0.40 => InjuryType::Muscle,
                x if x < 0.65 => InjuryType::Joint,
                x if x < 0.80 => InjuryType::Bruising,
                x if x < 0.90 => InjuryType::Bone,
                x if x < 0.95 => InjuryType::Cut,
                _ => InjuryType::Concussion,
            }
        }
    }

    fn calculate_weeks_out(&self, severity: &InjurySeverity, rng: &mut impl Rng) -> i64 {
        match severity {
            InjurySeverity::Minor => 1 + (rng.gen::<i64>() % 2).abs(),
            InjurySeverity::Moderate => 3 + (rng.gen::<i64>() % 4).abs(),
            InjurySeverity::Serious => 7 + (rng.gen::<i64>() % 6).abs(),
            InjurySeverity::Career => 13 + (rng.gen::<i64>() % 26).abs(), // 13-38 weeks
        }
    }

    fn generate_injury_description(
        &self,
        injury_type: &InjuryType,
        severity: &InjurySeverity,
    ) -> String {
        let type_desc = match injury_type {
            InjuryType::Muscle => "muscle strain",
            InjuryType::Joint => "joint injury",
            InjuryType::Bone => "bone injury",
            InjuryType::Concussion => "concussion",
            InjuryType::Cut => "cut",
            InjuryType::Bruising => "heavy bruising",
        };

        let severity_desc = match severity {
            InjurySeverity::Minor => "minor",
            InjurySeverity::Moderate => "moderate",
            InjurySeverity::Serious => "serious",
            InjurySeverity::Career => "severe",
        };

        format!("{} {} to", severity_desc, type_desc)
    }

    /// Historical injury descriptions with Victorian flavor
    pub fn generate_historical_description(
        &self,
        injury_type: &InjuryType,
        severity: &InjurySeverity,
    ) -> String {
        match (injury_type, severity) {
            (InjuryType::Cut, InjurySeverity::Minor) => {
                "sustained a cut to the shin and left the field temporarily".to_string()
            }
            (InjuryType::Cut, _) => {
                "received a severe gash and was conveyed from the field".to_string()
            }
            (InjuryType::Bruising, InjurySeverity::Minor) => {
                "was badly bruised in a collision but continued gamely".to_string()
            }
            (InjuryType::Bruising, _) => {
                "sustained heavy bruising and was unable to continue".to_string()
            }
            (InjuryType::Muscle, _) => {
                "pulled up lame with a muscle complaint".to_string()
            }
            (InjuryType::Joint, _) => {
                "twisted his ankle in a tackle and was carried from the ground".to_string()
            }
            (InjuryType::Bone, _) => {
                "sustained what is feared to be a broken bone".to_string()
            }
            (InjuryType::Concussion, _) => {
                "was rendered insensible after a heavy blow and required medical attention".to_string()
            }
        }
    }

    /// Apply daily healing (call this every day)
    pub fn heal_injuries(&self, injuries: &mut Vec<Injury>, current_date: NaiveDate) {
        injuries.retain(|injury| current_date < injury.expected_return_date);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_injury_generation() {
        let injury_system = InjurySystem::default();
        let players: Vec<String> = (0..22).map(|i| format!("player_{}", i)).collect();
        let current_date = NaiveDate::from_ymd_opt(1867, 10, 15).unwrap();

        let injuries = injury_system.check_match_injuries(
            &players[0..11],
            &players[11..22],
            1867,
            current_date,
        );

        // With 2.5% rate and 1.5x era modifier = ~3.75% per player
        // Expected: ~0.82 injuries per match (22 players)
        // Most matches will have 0-2 injuries
        assert!(injuries.len() <= 5);
    }

    #[test]
    fn test_injury_healing() {
        let injury_system = InjurySystem::default();
        let current_date = NaiveDate::from_ymd_opt(1867, 10, 15).unwrap();

        let mut injuries = vec![
            Injury {
                player_id: "player_1".to_string(),
                injury_type: InjuryType::Cut,
                occurred_date: current_date,
                expected_return_date: current_date
                    .checked_add_signed(chrono::Duration::weeks(1))
                    .unwrap(),
                severity: InjurySeverity::Minor,
                description: "Cut to shin".to_string(),
            },
            Injury {
                player_id: "player_2".to_string(),
                injury_type: InjuryType::Muscle,
                occurred_date: current_date,
                expected_return_date: current_date
                    .checked_add_signed(chrono::Duration::weeks(4))
                    .unwrap(),
                severity: InjurySeverity::Moderate,
                description: "Muscle strain".to_string(),
            },
        ];

        // Advance 2 weeks
        let future_date = current_date
            .checked_add_signed(chrono::Duration::weeks(2))
            .unwrap();
        injury_system.heal_injuries(&mut injuries, future_date);

        // First injury should be healed (1 week), second still injured (4 weeks)
        assert_eq!(injuries.len(), 1);
        assert_eq!(injuries[0].player_id, "player_2");
    }
}
