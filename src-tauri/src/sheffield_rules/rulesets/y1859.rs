/// 1859 Sheffield Rules - Same as 1858
/// Duration: 1859
///
/// No rule changes from 1858
/// Same as 1858 (original rules, fair catch with pushing/hitting)

use crate::sheffield_rules::ruleset::*;

#[derive(Debug, Clone)]
pub struct Ruleset1859;

impl RuleSet for Ruleset1859 {
    fn name(&self) -> &'static str {
        "1859 Sheffield Rules"
    }

    fn year(&self) -> u32 {
        1859
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
        HandlingRule::FairCatchAndPushing
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
