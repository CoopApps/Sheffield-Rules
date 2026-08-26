/// 1866 Sheffield Rules - Offside Reversion
/// Duration: 1866
///
/// Changes from 1865:
/// - Offside reverted to weak rule (one opponent level) after 1865 experiment failed
/// - Goal dimensions remain: 12 ft × 9 ft
/// - All other rules unchanged from 1863
///
/// Strategic impact: MEDIUM (reverts to less restrictive offside)

use crate::sheffield_rules::ruleset::*;

#[derive(Debug, Clone)]
pub struct Ruleset1866;

impl RuleSet for Ruleset1866 {
    fn name(&self) -> &'static str {
        "1866 Sheffield Rules"
    }

    fn year(&self) -> u32 {
        1866
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
        OffsideRule::OneOpponentLevel
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
