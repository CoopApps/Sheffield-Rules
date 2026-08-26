/// 1867 October Sheffield Rules - Minor Amendment
/// Duration: October 1867 - December 1867
///
/// Changes from March 1867:
/// - Kick-out rule refinement (only if over crossbar)
/// - Throw-ins now awarded from 10 yards
/// - All other rules unchanged
///
/// Strategic impact: LOW-MEDIUM (minor refinement to kick-out rules)

use crate::sheffield_rules::ruleset::*;

#[derive(Debug, Clone)]
pub struct Ruleset1867October;

impl RuleSet for Ruleset1867October {
    fn name(&self) -> &'static str {
        "1867 October Sheffield Rules"
    }

    fn year(&self) -> u32 {
        1867
    }

    fn goal_width(&self) -> Option<u16> {
        Some(12)
    }

    fn goal_height(&self) -> Option<u16> {
        Some(9)
    }

    fn tiebreaker(&self) -> TieBreakerRule {
        TieBreakerRule::RougeWithoutTouchdown
    }

    fn offside_rule(&self) -> OffsideRule {
        OffsideRule::OneOpponentLevel
    }

    fn handling_rules(&self) -> HandlingRule {
        HandlingRule::NoHandling
    }

    fn free_kick_triggers(&self) -> Vec<FreeKickTrigger> {
        vec![
            FreeKickTrigger::Handball,
            FreeKickTrigger::Tripping,
            FreeKickTrigger::Hacking,
        ]
    }

    fn set_piece_rules(&self) -> SetPieceRules {
        SetPieceRules {
            throw_in: ThrowInRule::AwayTeamIfInTouch,
            kick_in: None,
            goal_kick: GoalKickRule::OnlyIfOverCrossbar,
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
