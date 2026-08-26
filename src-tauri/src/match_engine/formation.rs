/// Formation system for Sheffield Rules football (1858-1877)
/// Supports variable team sizes and historical formations

use serde::{Deserialize, Serialize};
use super::possession_engine::FieldZone;

/// Position on the pitch (normalized coordinates 0.0-1.0)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position2D {
    pub x: f32,  // 0.0 (own goal) to 1.0 (opponent goal)
    pub y: f32,  // 0.0 (left touchline) to 1.0 (right touchline)
}

impl Position2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x: x.max(0.0).min(1.0),
            y: y.max(0.0).min(1.0),
        }
    }

    /// Calculate distance to another position
    pub fn distance_to(&self, other: &Position2D) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Player position template (role + default coordinates)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormationPosition {
    pub role: String,           // "GK", "CB", "FB", "MID", "FWD", "WG"
    pub position: Position2D,   // Default position on pitch
    pub zone: FieldZone,        // Defensive/Middle/Attacking third
}

// FieldZone is now imported from possession_engine.rs to avoid duplication

/// Football formation template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Formation {
    pub name: String,
    pub code: String,               // "2-3-5", "2-2-6", etc.
    pub era_start_year: Option<i32>,
    pub era_end_year: Option<i32>,
    pub positions: Vec<FormationPosition>,
    pub description: String,
}

impl Formation {
    /// Create the classic 2-3-5 Pyramid (Sheffield's iconic formation 1860s)
    pub fn pyramid_2_3_5() -> Self {
        let positions = vec![
            // Goalkeeper
            FormationPosition {
                role: "GK".to_string(),
                position: Position2D::new(0.05, 0.5),
                zone: FieldZone::DefensiveThird,
            },
            // 2 Full Backs (wide defensive positions)
            FormationPosition {
                role: "FB".to_string(),
                position: Position2D::new(0.20, 0.25),
                zone: FieldZone::DefensiveThird,
            },
            FormationPosition {
                role: "FB".to_string(),
                position: Position2D::new(0.20, 0.75),
                zone: FieldZone::DefensiveThird,
            },
            // 3 Half Backs (midfield)
            FormationPosition {
                role: "MID".to_string(),
                position: Position2D::new(0.40, 0.5),
                zone: FieldZone::MiddleThird,
            },
            FormationPosition {
                role: "MID".to_string(),
                position: Position2D::new(0.45, 0.25),
                zone: FieldZone::MiddleThird,
            },
            FormationPosition {
                role: "MID".to_string(),
                position: Position2D::new(0.45, 0.75),
                zone: FieldZone::MiddleThird,
            },
            // 5 Forwards (attacking pyramid)
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.70, 0.5),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.75, 0.3),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.75, 0.7),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.85, 0.2),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.85, 0.8),
                zone: FieldZone::AttackingThird,
            },
        ];

        Formation {
            name: "2-3-5 Pyramid".to_string(),
            code: "2-3-5".to_string(),
            era_start_year: Some(1858),
            era_end_year: Some(1890),
            positions,
            description: "The classic Sheffield formation - 2 full backs, 3 half backs, 5 forwards in pyramid shape".to_string(),
        }
    }

    /// Create ultra-attacking 1-1-8 formation (earliest Sheffield era ~1860)
    /// One back, one half-back, eight forwards - pure attacking football
    pub fn ultra_attacking_1_1_8() -> Self {
        let positions = vec![
            // Goalkeeper
            FormationPosition {
                role: "GK".to_string(),
                position: Position2D::new(0.05, 0.5),
                zone: FieldZone::DefensiveThird,
            },
            // 1 Back (lone defender)
            FormationPosition {
                role: "FB".to_string(),
                position: Position2D::new(0.20, 0.5),
                zone: FieldZone::DefensiveThird,
            },
            // 1 Half Back (central midfielder)
            FormationPosition {
                role: "MID".to_string(),
                position: Position2D::new(0.40, 0.5),
                zone: FieldZone::MiddleThird,
            },
            // 8 Forwards (complete attacking dominance)
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.65, 0.15),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.65, 0.35),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.65, 0.65),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.65, 0.85),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.80, 0.20),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.80, 0.45),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.80, 0.70),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.90, 0.50),
                zone: FieldZone::AttackingThird,
            },
        ];

        Formation {
            name: "1-1-8 Ultra Attack".to_string(),
            code: "1-1-8".to_string(),
            era_start_year: Some(1858),
            era_end_year: Some(1863),
            positions,
            description: "Early Sheffield formation - 1 back, 1 half back, 8 forwards. Maximum attacking football with minimal defense.".to_string(),
        }
    }

    /// Create 1-2-7 formation (early Sheffield ~1862-1865)
    /// Slightly more defensive than 1-1-8
    pub fn early_sheffield_1_2_7() -> Self {
        let positions = vec![
            // Goalkeeper
            FormationPosition {
                role: "GK".to_string(),
                position: Position2D::new(0.05, 0.5),
                zone: FieldZone::DefensiveThird,
            },
            // 1 Back (central defender)
            FormationPosition {
                role: "FB".to_string(),
                position: Position2D::new(0.20, 0.5),
                zone: FieldZone::DefensiveThird,
            },
            // 2 Half Backs (wide midfield)
            FormationPosition {
                role: "MID".to_string(),
                position: Position2D::new(0.40, 0.30),
                zone: FieldZone::MiddleThird,
            },
            FormationPosition {
                role: "MID".to_string(),
                position: Position2D::new(0.40, 0.70),
                zone: FieldZone::MiddleThird,
            },
            // 7 Forwards (attacking wave)
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.65, 0.20),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.65, 0.40),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.65, 0.60),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.65, 0.80),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.85, 0.25),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.85, 0.50),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.85, 0.75),
                zone: FieldZone::AttackingThird,
            },
        ];

        Formation {
            name: "1-2-7 Early Sheffield".to_string(),
            code: "1-2-7".to_string(),
            era_start_year: Some(1862),
            era_end_year: Some(1866),
            positions,
            description: "Transitional Sheffield formation - 1 back, 2 half backs, 7 forwards. More balanced than 1-1-8.".to_string(),
        }
    }

    /// Create 1-3-6 formation (mid Sheffield era ~1865-1870)
    /// Transition towards the pyramid formation
    pub fn transitional_1_3_6() -> Self {
        let positions = vec![
            // Goalkeeper
            FormationPosition {
                role: "GK".to_string(),
                position: Position2D::new(0.05, 0.5),
                zone: FieldZone::DefensiveThird,
            },
            // 1 Back (central defender)
            FormationPosition {
                role: "FB".to_string(),
                position: Position2D::new(0.20, 0.5),
                zone: FieldZone::DefensiveThird,
            },
            // 3 Half Backs (developing midfield line)
            FormationPosition {
                role: "MID".to_string(),
                position: Position2D::new(0.40, 0.25),
                zone: FieldZone::MiddleThird,
            },
            FormationPosition {
                role: "MID".to_string(),
                position: Position2D::new(0.40, 0.50),
                zone: FieldZone::MiddleThird,
            },
            FormationPosition {
                role: "MID".to_string(),
                position: Position2D::new(0.40, 0.75),
                zone: FieldZone::MiddleThird,
            },
            // 6 Forwards (still very attacking)
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.70, 0.20),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.70, 0.40),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.70, 0.60),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.70, 0.80),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.85, 0.30),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.85, 0.70),
                zone: FieldZone::AttackingThird,
            },
        ];

        Formation {
            name: "1-3-6 Transitional".to_string(),
            code: "1-3-6".to_string(),
            era_start_year: Some(1865),
            era_end_year: Some(1872),
            positions,
            description: "Transitional Sheffield formation - 1 back, 3 half backs, 6 forwards. Evolution toward the 2-3-5 pyramid.".to_string(),
        }
    }

    /// Create attacking 2-2-6 formation (very aggressive)
    pub fn attacking_2_2_6() -> Self {
        let positions = vec![
            // Goalkeeper
            FormationPosition {
                role: "GK".to_string(),
                position: Position2D::new(0.05, 0.5),
                zone: FieldZone::DefensiveThird,
            },
            // 2 Full Backs
            FormationPosition {
                role: "FB".to_string(),
                position: Position2D::new(0.20, 0.30),
                zone: FieldZone::DefensiveThird,
            },
            FormationPosition {
                role: "FB".to_string(),
                position: Position2D::new(0.20, 0.70),
                zone: FieldZone::DefensiveThird,
            },
            // 2 Half Backs
            FormationPosition {
                role: "MID".to_string(),
                position: Position2D::new(0.45, 0.35),
                zone: FieldZone::MiddleThird,
            },
            FormationPosition {
                role: "MID".to_string(),
                position: Position2D::new(0.45, 0.65),
                zone: FieldZone::MiddleThird,
            },
            // 6 Forwards (ultra-attacking)
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.70, 0.20),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.70, 0.45),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.70, 0.70),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.85, 0.15),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.85, 0.50),
                zone: FieldZone::AttackingThird,
            },
            FormationPosition {
                role: "FWD".to_string(),
                position: Position2D::new(0.85, 0.85),
                zone: FieldZone::AttackingThird,
            },
        ];

        Formation {
            name: "2-2-6 Attacking".to_string(),
            code: "2-2-6".to_string(),
            era_start_year: Some(1858),
            era_end_year: Some(1875),
            positions,
            description: "Ultra-attacking formation with 6 forwards - high risk, high reward".to_string(),
        }
    }

    /// Get formation appropriate for a given year (historically accurate)
    pub fn for_year(year: i32) -> Self {
        match year {
            // Early Sheffield era: ultra-attacking 1-1-8
            1858..=1861 => Self::ultra_attacking_1_1_8(),

            // Early-mid Sheffield: transitioning to 1-2-7
            1862..=1864 => Self::early_sheffield_1_2_7(),

            // Mid Sheffield: evolving to 1-3-6
            1865..=1869 => Self::transitional_1_3_6(),

            // Late Sheffield era: classic 2-3-5 pyramid emerges
            1870..=1890 => Self::pyramid_2_3_5(),

            // Post-1890: modern 2-3-5 WM formation
            1891.. => Self::pyramid_2_3_5(),

            // Default fallback
            _ => Self::pyramid_2_3_5(),
        }
    }

    /// Get all available formations for a given era
    pub fn all_for_era(year: i32) -> Vec<Self> {
        match year {
            1858..=1861 => vec![
                Self::ultra_attacking_1_1_8(),
                Self::early_sheffield_1_2_7(),
            ],
            1862..=1864 => vec![
                Self::ultra_attacking_1_1_8(),
                Self::early_sheffield_1_2_7(),
                Self::transitional_1_3_6(),
            ],
            1865..=1869 => vec![
                Self::early_sheffield_1_2_7(),
                Self::transitional_1_3_6(),
                Self::attacking_2_2_6(),
            ],
            1870..=1890 => vec![
                Self::transitional_1_3_6(),
                Self::pyramid_2_3_5(),
                Self::attacking_2_2_6(),
            ],
            _ => vec![Self::pyramid_2_3_5()],
        }
    }

    /// Mirror formation for away team (flip X coordinates)
    pub fn mirror(&self) -> Self {
        let mut mirrored = self.clone();
        mirrored.positions = mirrored
            .positions
            .into_iter()
            .map(|mut pos| {
                pos.position.x = 1.0 - pos.position.x;
                pos.zone = FieldZone::from_x_position(pos.position.x);
                pos
            })
            .collect();
        mirrored.name = format!("{} (Away)", self.name);
        mirrored
    }

    /// Get number of players in formation
    pub fn player_count(&self) -> usize {
        self.positions.len()
    }

    /// Validate formation has required positions
    pub fn is_valid(&self) -> bool {
        // Must have at least 1 goalkeeper
        let has_gk = self.positions.iter().any(|p| p.role == "GK");

        // Must have at least 8 players (minimum for Sheffield Rules)
        let min_players = self.positions.len() >= 8;

        has_gk && min_players
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pyramid_formation() {
        let formation = Formation::pyramid_2_3_5();
        assert_eq!(formation.player_count(), 11);
        assert_eq!(formation.code, "2-3-5");
        assert!(formation.is_valid());
    }

    #[test]
    fn test_formation_mirror() {
        let home = Formation::pyramid_2_3_5();
        let away = home.mirror();

        // GK should be near opposite ends
        assert!(home.positions[0].position.x < 0.1);
        assert!(away.positions[0].position.x > 0.9);
    }

    #[test]
    fn test_field_zones() {
        assert_eq!(FieldZone::from_x_position(0.1), FieldZone::DefensiveThird);
        assert_eq!(FieldZone::from_x_position(0.5), FieldZone::MiddleThird);
        assert_eq!(FieldZone::from_x_position(0.8), FieldZone::AttackingThird);
    }

    #[test]
    fn test_position_distance() {
        let pos1 = Position2D::new(0.0, 0.0);
        let pos2 = Position2D::new(1.0, 1.0);
        let distance = pos1.distance_to(&pos2);

        // Should be sqrt(2) ≈ 1.414
        assert!((distance - 1.414).abs() < 0.01);
    }

    #[test]
    fn test_ultra_attacking_1_1_8() {
        let formation = Formation::ultra_attacking_1_1_8();
        assert_eq!(formation.player_count(), 11);
        assert_eq!(formation.code, "1-1-8");
        assert!(formation.is_valid());

        // Count positions by role
        let backs = formation.positions.iter().filter(|p| p.role == "FB").count();
        let mids = formation.positions.iter().filter(|p| p.role == "MID").count();
        let fwds = formation.positions.iter().filter(|p| p.role == "FWD").count();

        assert_eq!(backs, 1);
        assert_eq!(mids, 1);
        assert_eq!(fwds, 8);
    }

    #[test]
    fn test_early_sheffield_1_2_7() {
        let formation = Formation::early_sheffield_1_2_7();
        assert_eq!(formation.player_count(), 11);
        assert_eq!(formation.code, "1-2-7");
        assert!(formation.is_valid());

        let backs = formation.positions.iter().filter(|p| p.role == "FB").count();
        let mids = formation.positions.iter().filter(|p| p.role == "MID").count();
        let fwds = formation.positions.iter().filter(|p| p.role == "FWD").count();

        assert_eq!(backs, 1);
        assert_eq!(mids, 2);
        assert_eq!(fwds, 7);
    }

    #[test]
    fn test_transitional_1_3_6() {
        let formation = Formation::transitional_1_3_6();
        assert_eq!(formation.player_count(), 11);
        assert_eq!(formation.code, "1-3-6");
        assert!(formation.is_valid());

        let backs = formation.positions.iter().filter(|p| p.role == "FB").count();
        let mids = formation.positions.iter().filter(|p| p.role == "MID").count();
        let fwds = formation.positions.iter().filter(|p| p.role == "FWD").count();

        assert_eq!(backs, 1);
        assert_eq!(mids, 3);
        assert_eq!(fwds, 6);
    }

    #[test]
    fn test_formation_for_year() {
        // Early era should use 1-1-8
        let formation_1860 = Formation::for_year(1860);
        assert_eq!(formation_1860.code, "1-1-8");

        // Mid era should use 1-2-7
        let formation_1863 = Formation::for_year(1863);
        assert_eq!(formation_1863.code, "1-2-7");

        // Transition era should use 1-3-6
        let formation_1867 = Formation::for_year(1867);
        assert_eq!(formation_1867.code, "1-3-6");

        // Late Sheffield era should use 2-3-5
        let formation_1875 = Formation::for_year(1875);
        assert_eq!(formation_1875.code, "2-3-5");
    }

    #[test]
    fn test_all_for_era() {
        let formations_1860 = Formation::all_for_era(1860);
        assert!(formations_1860.len() >= 2);

        let formations_1867 = Formation::all_for_era(1867);
        assert!(formations_1867.len() >= 3);
    }
}
