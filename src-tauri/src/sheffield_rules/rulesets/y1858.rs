/// 1858 Sheffield Rules - Original Rules
/// Duration: 1858-1860 (with minor handling changes in 1860)
///
/// Key features:
/// - Fair catch allowed
/// - No offside rule
/// - Unspecified goal dimensions
/// - No formal tiebreaker
/// - First team to touch ball after going out gets throw-in
///
/// Strategic impact: LOW (least formalized rules, more informal play)

use crate::sheffield_rules::ruleset::*;

#[derive(Debug, Clone)]
pub struct Ruleset1858;

impl RuleSet for Ruleset1858 {
    fn name(&self) -> &'static str {
        "1858 Sheffield Rules (Original)"
    }

    fn year(&self) -> u32 {
        1858
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
