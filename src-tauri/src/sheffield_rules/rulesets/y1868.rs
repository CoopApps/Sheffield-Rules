/// 1868 Sheffield Rules - MAJOR REWRITE #2 (POST-ROUGE, BIGGEST TRANSFORMATION)
/// Duration: 1868-1869
///
/// MASSIVE changes:
/// - ROUGE ABOLISHED (ends 6-year experiment)
/// - Goal width DOUBLED: 24 ft × 9 ft (2x wider!)
/// - CORNER KICKS INTRODUCED (first in football history)
/// - Throw-ins REPLACED with kick-ins (can go any direction)
/// - Fair catch reintroduced
/// - Free kicks extended to pushing/tripping
/// - Only scoring method: goals
///
/// Strategic impact: EXTREME (fundamentally different game - widest goals in history)
/// The 24-ft goal with corner kicks creates a completely different tactical approach.
/// Wider goals favor side play and crossing. Kick-ins allow ball control that throws don't.

use crate::sheffield_rules::ruleset::*;

#[derive(Debug, Clone)]
pub struct Ruleset1868;

impl RuleSet for Ruleset1868 {
    fn name(&self) -> &'static str {
        "1868 Sheffield Rules (Post-Rouge, Corner Kicks)"
    }

    fn year(&self) -> u32 {
        1868
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
        HandlingRule::FairCatchOnly
    }

    fn free_kick_triggers(&self) -> Vec<FreeKickTrigger> {
        vec![
            FreeKickTrigger::FairCatch,
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
