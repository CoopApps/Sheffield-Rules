/// 1862 Sheffield Rules - MAJOR REWRITE #1 (ROUGE ERA BEGINS)
/// Duration: 1862-1867
///
/// Major changes:
/// - ROUGE INTRODUCED as tiebreaker (touch-down required)
/// - Goal dimensions: 12 ft × 9 ft
/// - Changed ends at half-time
/// - Fair catch still allowed
///
/// Strategic impact: VERY HIGH (unique mechanic - rouge as 2-tier scoring system)
/// The rouge system: If no goal scored, kicking the ball over the goal line awards 1 rouge point.
/// A rouge only counts if the attacking team touches down behind goal line.

use crate::sheffield_rules::ruleset::*;

#[derive(Debug, Clone)]
pub struct Ruleset1862;

impl RuleSet for Ruleset1862 {
    fn name(&self) -> &'static str {
        "1862 Sheffield Rules (Rouge Era)"
    }

    fn year(&self) -> u32 {
        1862
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
