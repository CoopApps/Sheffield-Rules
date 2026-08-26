/// 1861 Sheffield Rules - Same as 1860
/// Duration: 1861
///
/// No rule changes from 1860
/// Same as 1860 (fair catch only, unspecified goals)

use crate::sheffield_rules::ruleset::*;

#[derive(Debug, Clone)]
pub struct Ruleset1861;

impl RuleSet for Ruleset1861 {
    fn name(&self) -> &'static str {
        "1861 Sheffield Rules"
    }

    fn year(&self) -> u32 {
        1861
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
