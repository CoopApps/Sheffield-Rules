/// 1867 March Sheffield Rules - MAJOR REWRITE (Sheffield FA Rules)
/// Duration: March 1867 - September 1867
///
/// Major changes:
/// - Handling completely banned (first time)
/// - Rouge changed to NOT require touch-down (just kicking over line)
/// - Weak offside rule (one opponent level)
/// - Throw-ins awarded against kicking team
/// - Goal kick from 6 yards
///
/// Strategic impact: VERY HIGH (handling ban is revolutionary, changes play dramatically)

use crate::sheffield_rules::ruleset::*;

#[derive(Debug, Clone)]
pub struct Ruleset1867March;

impl RuleSet for Ruleset1867March {
    fn name(&self) -> &'static str {
        "1867 March Sheffield Rules (FA Rules)"
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
            goal_kick: GoalKickRule::From6Yards,
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
