/// 1876 Sheffield Rules - Final Amendment
/// Duration: 1876-1877
///
/// Changes from 1875:
/// - Goalkeeper handling rule matches FA approach
/// - All other rules remain unchanged
/// - Goal: 24 ft × 8 ft
///
/// Strategic impact: VERY LOW (minor clarification, essentially same as 1875)

use crate::sheffield_rules::ruleset::*;

#[derive(Debug, Clone)]
pub struct Ruleset1876;

impl RuleSet for Ruleset1876 {
    fn name(&self) -> &'static str {
        "1876 Sheffield Rules"
    }

    fn year(&self) -> u32 {
        1876
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
