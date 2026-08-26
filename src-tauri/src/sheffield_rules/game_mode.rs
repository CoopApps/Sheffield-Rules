/// Sheffield Rules Game Mode System
///
/// Supports three distinct ways to play Sheffield Rules:
/// 1. Historical Timeline (1858-1877) - Rules progress chronologically
/// 2. Ahistorical Single Ruleset - Play indefinitely with one rule version
/// 3. Historical From Any Year - Start any year, rules progress to 1877

use serde::{Deserialize, Serialize};
use crate::sheffield_rules::rulesets;

/// The game modes available for Sheffield Rules
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GameMode {
    /// Historical Timeline (1858-1877) with automatic rule progression
    HistoricalTimeline,
    /// Play indefinitely with a single ruleset
    AhistoricalSingleRuleset { year: u32 },
    /// Start from any year, rules progress chronologically to 1877
    HistoricalFromYear { start_year: u32 },
    /// Sheffield and Hallamshire League (1867 Fantasy) - All 186 historical clubs in unified pyramid
    SheffieldAndHallamshireLeague { use_rouges: bool },
}

impl GameMode {
    pub fn name(&self) -> &'static str {
        match self {
            GameMode::HistoricalTimeline => "Historical Timeline (1858-1877)",
            GameMode::AhistoricalSingleRuleset { .. } => "Ahistorical Single Ruleset",
            GameMode::HistoricalFromYear { .. } => "Historical From Any Year",
            GameMode::SheffieldAndHallamshireLeague { .. } => "Sheffield and Hallamshire League",
        }
    }

    pub fn description(&self) -> String {
        match self {
            GameMode::HistoricalTimeline => {
                "Play through all 19 years (1858-1877) with rules evolving historically.".to_string()
            }
            GameMode::AhistoricalSingleRuleset { year } => {
                format!(
                    "Play indefinitely with {} ruleset. Rules stay constant.",
                    rulesets::get_ruleset_for_year(*year, None).name()
                )
            }
            GameMode::HistoricalFromYear { start_year } => {
                let end_year = 1877;
                let years = end_year - start_year + 1;
                format!(
                    "Start {}, rules progress to 1877 ({} years total).",
                    start_year, years
                )
            }
            GameMode::SheffieldAndHallamshireLeague { use_rouges } => {
                if *use_rouges {
                    "Fantasy 1867: All 186 historical clubs in unified league pyramid with rouge scoring.".to_string()
                } else {
                    "Fantasy 1867: All 186 historical clubs in unified league pyramid without rouge scoring.".to_string()
                }
            }
        }
    }

    /// Get the current ruleset for a given in-game year
    pub fn get_ruleset_for_season(&self, season_year: u32) -> std::sync::Arc<dyn crate::sheffield_rules::RuleSet> {
        match self {
            GameMode::HistoricalTimeline => {
                // Progress from 1858 through 1877
                if season_year < 1858 || season_year > 1877 {
                    panic!("Historical Timeline: season year {} out of range", season_year);
                }
                rulesets::get_ruleset_for_year(season_year, None)
            }
            GameMode::AhistoricalSingleRuleset { year } => {
                // Always use the selected year's rules
                rulesets::get_ruleset_for_year(*year, None)
            }
            GameMode::HistoricalFromYear { start_year } => {
                // Progress from start_year through 1877
                if season_year < *start_year || season_year > 1877 {
                    panic!(
                        "Historical From {}: season year {} out of range",
                        start_year, season_year
                    );
                }
                rulesets::get_ruleset_for_year(season_year, None)
            }
            GameMode::SheffieldAndHallamshireLeague { use_rouges } => {
                // Fantasy 1867: Use 1867 or 1862 rules depending on rouge preference
                if *use_rouges {
                    rulesets::get_ruleset_for_year(1862, None) // Rouge era ruleset
                } else {
                    rulesets::get_ruleset_for_year(1867, None) // Pre-rouge ruleset
                }
            }
        }
    }

    /// Check if the game should end naturally (for timeline modes)
    pub fn is_final_season(&self, season_year: u32) -> bool {
        match self {
            GameMode::HistoricalTimeline => season_year >= 1877,
            GameMode::AhistoricalSingleRuleset { .. } => false,
            GameMode::HistoricalFromYear { .. } => season_year >= 1877,
            GameMode::SheffieldAndHallamshireLeague { .. } => false, // Indefinite fantasy league
        }
    }

    /// Get the total expected seasons (None = indefinite)
    pub fn total_seasons(&self) -> Option<u32> {
        match self {
            GameMode::HistoricalTimeline => Some(20), // 1858-1877 inclusive
            GameMode::AhistoricalSingleRuleset { .. } => None,
            GameMode::HistoricalFromYear { start_year } => Some(1877 - start_year + 1),
            GameMode::SheffieldAndHallamshireLeague { .. } => None, // Indefinite
        }
    }
}

/// Menu options for game mode selection
pub fn game_mode_options() -> Vec<GameModeOption> {
    vec![
        GameModeOption {
            id: "sheffield-hallamshire-league",
            name: "Sheffield & Hallamshire League (1867 Fantasy)",
            description: "All 186 historical Sheffield clubs in unified league pyramid from 1867 onwards.",
            category: "Fantasy",
        },
        GameModeOption {
            id: "historical-timeline",
            name: "Historical Timeline (1858-1877)",
            description: "Play through 20 seasons with rules evolving historically.",
            category: "Historical",
        },
        GameModeOption {
            id: "ahistorical-1862",
            name: "Ahistorical: 1862 Rouge Era",
            description: "Play indefinitely with 1862 rules (rouge with touchdown required).",
            category: "Ahistorical",
        },
        GameModeOption {
            id: "ahistorical-1868",
            name: "Ahistorical: 1868 Post-Rouge",
            description: "Play indefinitely with 1868 rules (corner kicks, 24×9 ft goals).",
            category: "Ahistorical",
        },
        GameModeOption {
            id: "ahistorical-1875",
            name: "Ahistorical: 1875 Stable Era",
            description: "Play indefinitely with 1875 rules (most balanced version).",
            category: "Ahistorical",
        },
        GameModeOption {
            id: "historical-from-1858",
            name: "Historical From 1858",
            description: "Start 1858, rules progress through all 20 seasons to 1877.",
            category: "HistoricalFrom",
        },
        GameModeOption {
            id: "historical-from-1862",
            name: "Historical From 1862",
            description: "Start 1862 (Rouge Era), rules progress through 1877 (16 seasons).",
            category: "HistoricalFrom",
        },
        GameModeOption {
            id: "historical-from-1868",
            name: "Historical From 1868",
            description: "Start 1868 (Post-Rouge), rules progress through 1877 (10 seasons).",
            category: "HistoricalFrom",
        },
        GameModeOption {
            id: "historical-from-1875",
            name: "Historical From 1875",
            description: "Start 1875 (Stable Era), rules progress through 1877 (3 seasons).",
            category: "HistoricalFrom",
        },
    ]
}

/// Represents a selectable game mode option in the UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameModeOption {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub category: &'static str, // "Historical", "Ahistorical", "HistoricalFrom"
}

/// Parse a game mode option ID and return the corresponding GameMode
/// Note: For Sheffield & Hallamshire League, this returns a default with use_rouges = false
/// The UI should set the correct use_rouges value before creating the game
pub fn create_game_mode_from_option(option_id: &str) -> Result<GameMode, String> {
    match option_id {
        "sheffield-hallamshire-league" => Ok(GameMode::SheffieldAndHallamshireLeague { use_rouges: false }),
        "historical-timeline" => Ok(GameMode::HistoricalTimeline),
        "ahistorical-1862" => Ok(GameMode::AhistoricalSingleRuleset { year: 1862 }),
        "ahistorical-1868" => Ok(GameMode::AhistoricalSingleRuleset { year: 1868 }),
        "ahistorical-1875" => Ok(GameMode::AhistoricalSingleRuleset { year: 1875 }),
        "historical-from-1858" => Ok(GameMode::HistoricalFromYear { start_year: 1858 }),
        "historical-from-1862" => Ok(GameMode::HistoricalFromYear { start_year: 1862 }),
        "historical-from-1868" => Ok(GameMode::HistoricalFromYear { start_year: 1868 }),
        "historical-from-1875" => Ok(GameMode::HistoricalFromYear { start_year: 1875 }),
        _ => Err(format!("Unknown game mode option: {}", option_id)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_mode_names() {
        assert_eq!(GameMode::HistoricalTimeline.name(), "Historical Timeline (1858-1877)");
        assert_eq!(
            GameMode::AhistoricalSingleRuleset { year: 1862 }.name(),
            "Ahistorical Single Ruleset"
        );
    }

    #[test]
    fn test_total_seasons() {
        assert_eq!(GameMode::HistoricalTimeline.total_seasons(), Some(20));
        assert_eq!(GameMode::AhistoricalSingleRuleset { year: 1875 }.total_seasons(), None);
        assert_eq!(
            GameMode::HistoricalFromYear { start_year: 1862 }.total_seasons(),
            Some(16)
        );
        assert_eq!(
            GameMode::HistoricalFromYear { start_year: 1875 }.total_seasons(),
            Some(3)
        );
    }

    #[test]
    fn test_is_final_season() {
        assert!(!GameMode::HistoricalTimeline.is_final_season(1876));
        assert!(GameMode::HistoricalTimeline.is_final_season(1877));

        assert!(!GameMode::AhistoricalSingleRuleset { year: 1875 }.is_final_season(1877));

        assert!(GameMode::HistoricalFromYear { start_year: 1858 }.is_final_season(1877));
    }

    #[test]
    fn test_create_game_mode_from_option() {
        let mode = create_game_mode_from_option("ahistorical-1862").unwrap();
        assert_eq!(mode, GameMode::AhistoricalSingleRuleset { year: 1862 });

        let mode = create_game_mode_from_option("historical-from-1875").unwrap();
        assert_eq!(
            mode,
            GameMode::HistoricalFromYear { start_year: 1875 }
        );

        assert!(create_game_mode_from_option("invalid-mode").is_err());
    }

    #[test]
    fn test_game_mode_options_count() {
        let options = game_mode_options();
        assert_eq!(options.len(), 9); // Now includes Sheffield & Hallamshire League
    }

    #[test]
    fn test_sheffield_hallamshire_league_mode() {
        let mode = GameMode::SheffieldAndHallamshireLeague { use_rouges: true };
        assert_eq!(mode.name(), "Sheffield and Hallamshire League");
        assert!(mode.description().contains("rouge scoring"));
        assert_eq!(mode.total_seasons(), None); // Indefinite
        assert!(!mode.is_final_season(1900));
    }

    #[test]
    fn test_create_sheffield_league_from_option() {
        let mode = create_game_mode_from_option("sheffield-hallamshire-league").unwrap();
        match mode {
            GameMode::SheffieldAndHallamshireLeague { use_rouges } => {
                assert_eq!(use_rouges, false);
            }
            _ => panic!("Wrong game mode type"),
        }
    }
}
