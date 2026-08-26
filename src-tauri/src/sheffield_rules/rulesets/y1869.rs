/// 1869 Sheffield Rules - Handling Restoration
/// Duration: 1869-1870
///
/// Changes from 1868:
/// - Handling allowed on attempted catch
/// - Handling allowed within 3 yards of goal
/// - Free kick retreat: 3 → 6 yards
/// - Fair catch no longer awards free kick
/// - Goal dimensions remain: 24 ft × 9 ft
///
/// Strategic impact: MEDIUM (partial restoration of handling creates new options)

use crate::sheffield_rules::ruleset::*;

#[derive(Debug, Clone)]
pub struct Ruleset1869;

impl RuleSet for Ruleset1869 {
    fn name(&self) -> &'static str {
        "1869 Sheffield Rules (Handling Restoration)"
    }

    fn year(&self) -> u32 {
        1869
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
