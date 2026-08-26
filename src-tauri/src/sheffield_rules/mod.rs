/// Sheffield Rules - Historical Football Rules (1858-1877)
///
/// This module provides a complete implementation of the Sheffield Rules
/// that governed early football from 1858 to 1877. Includes:
/// - RuleSet trait for rule abstraction
/// - 19 annual ruleset implementations covering all rule variations
/// - Game mode support (Historical, Ahistorical, From Any Year)
/// - Rouge scoring mechanic (1862-1868)
/// - Full rule progression from original 1858 to final 1877

pub mod ruleset;
pub mod rulesets;
pub mod game_mode;
pub mod clubs;
pub mod fixtures;
pub mod game;
pub mod rouge;
pub mod promotion;

// Re-export main interfaces
pub use ruleset::{
    RuleSet, TieBreakerRule, OffsideRule, HandlingRule, FreeKickTrigger,
    SetPieceRules, ThrowInRule, KickInRule, GoalKickRule, ScoringSystem,
    MatchRules, default_match_rules,
};

pub use rulesets::{
    get_ruleset_for_year, all_ruleset_years, year_changes_summary,
    Ruleset1858, Ruleset1859, Ruleset1860, Ruleset1861, Ruleset1862,
    Ruleset1863, Ruleset1864, Ruleset1865, Ruleset1866, Ruleset1867March,
    Ruleset1867October, Ruleset1868, Ruleset1869, Ruleset1870, Ruleset1871,
    Ruleset1872, Ruleset1873, Ruleset1874, Ruleset1875, Ruleset1876,
};

pub use game_mode::{
    GameMode, GameModeOption, game_mode_options, create_game_mode_from_option,
};

pub use clubs::{
    SheffieldClub, get_sheffield_clubs, get_clubs_for_game_mode,
};

pub use fixtures::{
    Fixture, generate_season_fixtures, generate_realistic_season_fixtures,
    simulate_fixture,
};

pub use rouge::{
    RougeScore, ScoreType, MatchResult, simulate_rouge_match,
};

pub use promotion::{
    Standing, PromotionEvent, RelegationEvent, SeasonEndReport,
    calculate_promotions_from_division, calculate_relegations_from_division,
    get_promotion_destination, get_relegation_destinations,
    distribute_div6_to_div5, distribute_div5_to_div6,
    process_season_end,
};
