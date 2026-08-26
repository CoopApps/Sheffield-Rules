/// 1865 Sheffield Rules - Strict Offside Experiment
/// Duration: 1865 only
///
/// Changes from 1863:
/// - Stricter offside attempted (any player ahead is offside)
/// - This experiment failed and was reverted in 1866
/// - Goal dimensions remain: 12 ft × 9 ft
///
/// Strategic impact: HIGH (very restrictive offside heavily constrains play)

use crate::sheffield_rules::ruleset::*;

#[derive(Debug, Clone)]
pub struct Ruleset1865;

impl RuleSet for Ruleset1865 {
    fn name(&self) -> &'static str {
        "1865 Sheffield Rules (Strict Offside)"
    }

    fn year(&self) -> u32 {
        1865
    }

    fn goal_width(&self) -> Option<u16> {
        Some(12)
    }

    fn goal_height(&self) -> Option<u16> {
        Some(9)
    }

    fn tiebreaker(&self) -> TieBreakerRule {
        TieBreakerRule::RougeWithTouchdown
    }

    fn offside_rule(&self) -> OffsideRule {
        OffsideRule::AnyPlayerAhead
    }

    fn handling_rules(&self) -> HandlingRule {
        HandlingRule::FairCatchOnly
    }

    fn free_kick_triggers(&self) -> Vec<FreeKickTrigger> {
        vec![FreeKickTrigger::FairCatch]
    }

    fn set_piece_rules(&self) -> SetPieceRules {
        SetPieceRules {
            throw_in: ThrowInRule::FirstTeamToTouch,
            kick_in: None,
            goal_kick: GoalKickRule::WheneverNoGoalOrRouge,
            corner_kicks: false,
        }
    }

    fn scoring_system(&self) -> ScoringSystem {
        ScoringSystem::GoalsAndRouges {
            goal_points: 1,
            rouge_points: 1,
            draw_points: 0,
        }
    }
}
