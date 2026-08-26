/// 1860 Sheffield Rules - First Amendment (minor)
/// Duration: 1860-1861
///
/// Changes from 1858:
/// - Handling rules tightened (no more pushing/hitting)
/// - Fair catch only
///
/// Strategic impact: LOW (subtle change to handling)

use crate::sheffield_rules::ruleset::*;

#[derive(Debug, Clone)]
pub struct Ruleset1860;

impl RuleSet for Ruleset1860 {
    fn name(&self) -> &'static str {
        "1860 Sheffield Rules"
    }

    fn year(&self) -> u32 {
        1860
    }

    fn goal_width(&self) -> Option<u16> {
        None
    }

    fn goal_height(&self) -> Option<u16> {
        None
    }

    fn tiebreaker(&self) -> TieBreakerRule {
        TieBreakerRule::None
    }

    fn offside_rule(&self) -> OffsideRule {
        OffsideRule::None
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
            goal_kick: GoalKickRule::WheneverNoGoal,
            corner_kicks: false,
        }
    }

    fn scoring_system(&self) -> ScoringSystem {
        ScoringSystem::GoalsOnly {
            points_per_goal: 1,
            points_per_draw: 0,
        }
    }
}
