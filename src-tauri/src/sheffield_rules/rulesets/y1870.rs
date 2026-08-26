/// 1870 Sheffield Rules - Same as 1869
/// Duration: 1870
///
/// No rule changes from 1869
/// Same as 1869 (handling restoration, wide goals, corner kicks)

use crate::sheffield_rules::ruleset::*;

#[derive(Debug, Clone)]
pub struct Ruleset1870;

impl RuleSet for Ruleset1870 {
    fn name(&self) -> &'static str {
        "1870 Sheffield Rules"
    }

    fn year(&self) -> u32 {
        1870
    }

    fn goal_width(&self) -> Option<u16> {
        Some(24)
    }

    fn goal_height(&self) -> Option<u16> {
        Some(9)
    }

    fn tiebreaker(&self) -> TieBreakerRule {
        TieBreakerRule::None
    }

    fn offside_rule(&self) -> OffsideRule {
        OffsideRule::OneOpponentLevel
    }

    fn handling_rules(&self) -> HandlingRule {
        HandlingRule::FairCatchAndAttemptedCatch
    }

    fn free_kick_triggers(&self) -> Vec<FreeKickTrigger> {
        vec![
            FreeKickTrigger::Handball,
            FreeKickTrigger::Tripping,
            FreeKickTrigger::Pushing,
        ]
    }

    fn set_piece_rules(&self) -> SetPieceRules {
        SetPieceRules {
            throw_in: ThrowInRule::NoThrowIns,
            kick_in: Some(KickInRule::InAnyDirection),
            goal_kick: GoalKickRule::VariableByLocation,
            corner_kicks: true,
        }
    }

    fn scoring_system(&self) -> ScoringSystem {
        ScoringSystem::GoalsOnly {
            points_per_goal: 1,
            points_per_draw: 0,
        }
    }
}
