/// 1875 Sheffield Rules - Final Refinements (STABLE ERA)
/// Duration: 1875-1877
///
/// Changes from 1871:
/// - Crossbar height: 9 ft → 8 ft (matches FA)
/// - Goal: 24 ft × 8 ft (final standardization)
/// - Change ends standardized to half-time only
/// - Goalkeeper handling clarified
/// - Umpires supplied with flags
/// - All other rules from 1871 unchanged
///
/// Strategic impact: LOW (most polished and balanced version)
/// This is essentially the stable ruleset that Sheffield used through 1877.

use crate::sheffield_rules::ruleset::*;

#[derive(Debug, Clone)]
pub struct Ruleset1875;

impl RuleSet for Ruleset1875 {
    fn name(&self) -> &'static str {
        "1875 Sheffield Rules (Stable Era)"
    }

    fn year(&self) -> u32 {
        1875
    }

    fn goal_width(&self) -> Option<u16> {
        Some(24)
    }

    fn goal_height(&self) -> Option<u16> {
        Some(8)
    }

    fn tiebreaker(&self) -> TieBreakerRule {
        TieBreakerRule::None
    }

    fn offside_rule(&self) -> OffsideRule {
        OffsideRule::OneOpponentLevel
    }

    fn handling_rules(&self) -> HandlingRule {
        HandlingRule::GoalkeeperInDefense
    }

    fn free_kick_triggers(&self) -> Vec<FreeKickTrigger> {
        vec![
            FreeKickTrigger::Handball,
            FreeKickTrigger::Tripping,
            FreeKickTrigger::Hacking,
            FreeKickTrigger::Pushing,
            FreeKickTrigger::ChargingFromBehind,
            FreeKickTrigger::FoulPlay,
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
