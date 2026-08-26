/// 1873 Sheffield Rules - Same as 1871
/// Duration: 1873
///
/// No rule changes from 1871
/// Same as 1871

use crate::sheffield_rules::ruleset::*;

#[derive(Debug, Clone)]
pub struct Ruleset1873;

impl RuleSet for Ruleset1873 {
    fn name(&self) -> &'static str {
        "1873 Sheffield Rules"
    }

    fn year(&self) -> u32 {
        1873
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
        HandlingRule::ThreeYardZoneAndNoExtension
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
