/// Sheffield Rules - RuleSet trait and enums
/// Defines the complete ruleset abstraction for all 19 annual versions

use serde::{Deserialize, Serialize};

// ============================================================================
// CORE TRAIT
// ============================================================================

pub trait RuleSet: Send + Sync {
    fn name(&self) -> &'static str;
    fn year(&self) -> u32;
    fn goal_width(&self) -> Option<u16>;
    fn goal_height(&self) -> Option<u16>;
    fn tiebreaker(&self) -> TieBreakerRule;
    fn offside_rule(&self) -> OffsideRule;
    fn handling_rules(&self) -> HandlingRule;
    fn free_kick_triggers(&self) -> Vec<FreeKickTrigger>;
    fn set_piece_rules(&self) -> SetPieceRules;
    fn scoring_system(&self) -> ScoringSystem;
}

// ============================================================================
// TIEBREAKER RULES
// ============================================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TieBreakerRule {
    None,
    RougeWithTouchdown,      // 1862-1867: Required touching down behind goal
    RougeWithoutTouchdown,   // 1867-1868: Just kicking between rouge flags
}

// ============================================================================
// OFFSIDE RULES
// ============================================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OffsideRule {
    None,                       // 1858-1862: No offside
    OneOpponentLevel,           // 1863+: Weak offside, one opponent level
    StrictThreeOpponents,       // FA rules: Three opponents
    AnyPlayerAhead,             // 1865 experimental: Any player ahead is offside
}

// ============================================================================
// HANDLING RULES
// ============================================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum HandlingRule {
    FairCatchOnly,                              // 1858-1866
    FairCatchAndPushing,                        // 1858 original (fair catch, pushing, hitting)
    NoHandling,                                 // 1867 March onwards (mostly)
    FairCatchAndAttemptedCatch,                 // 1869: Fair catch + attempted catch
    AttemptedCatchOnly,                         // Attempted catch, no fair catch
    ThreeYardZoneAndNoExtension,                // 1871+: Only within 3 yards, hand not extended
    GoalkeeperInDefense,                        // 1875+: Goalkeeper only
}

// ============================================================================
// FREE KICK TRIGGERS
// ============================================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FreeKickTrigger {
    FairCatch,
    Handball,
    Tripping,
    Hacking,
    Pushing,
    ChargingFromBehind,
    FoulPlay,
    Offside,
}

// ============================================================================
// SET PIECE RULES
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetPieceRules {
    pub throw_in: ThrowInRule,
    pub kick_in: Option<KickInRule>,
    pub goal_kick: GoalKickRule,
    pub corner_kicks: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ThrowInRule {
    FirstTeamToTouch,           // Original: awarded to first team to touch after going out
    AwayTeamIfInTouch,          // 1867+: awarded against team who kicked it out
    NoThrowIns,                 // 1868+: replaced with kick-ins
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum KickInRule {
    InAnyDirection,             // 1868+: Can kick any direction
    RestrictedDirection,        // Not used in Sheffield, but for abstraction
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GoalKickRule {
    From25Yards,                // Early Sheffield
    From10Yards,                // 1861+
    From6Yards,                 // 1867+
    VariableByLocation,         // 1868+: Different rules based on where ball left
    WheneverNoGoal,             // Trigger: whenever no goal scored
    WheneverNoGoalOrRouge,      // Trigger: whenever no goal or rouge scored
    OnlyIfOverCrossbar,         // 1867 Oct: Only after going over crossbar
}

// ============================================================================
// SCORING SYSTEM
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScoringSystem {
    GoalsOnly {
        points_per_goal: u16,
        points_per_draw: u16,
    },
    GoalsAndRouges {
        goal_points: u16,
        rouge_points: u16,
        draw_points: u16,
    },
}

impl ScoringSystem {
    pub fn has_rouge(&self) -> bool {
        matches!(self, ScoringSystem::GoalsAndRouges { .. })
    }

    pub fn has_only_goals(&self) -> bool {
        matches!(self, ScoringSystem::GoalsOnly { .. })
    }
}

// ============================================================================
// MATCH RULES (Derived from RuleSet)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchRules {
    pub match_duration_minutes: u16,
    pub minimum_players_per_side: Option<u16>,
    pub goal_width: Option<u16>,
    pub goal_height: Option<u16>,
}

// ============================================================================
// HELPER FUNCTION
// ============================================================================

pub fn default_match_rules() -> MatchRules {
    MatchRules {
        match_duration_minutes: 90,
        minimum_players_per_side: None,
        goal_width: None,
        goal_height: None,
    }
}
