/// Outcome-Aware Commentary Mapper
/// Selects commentary variants based on event OUTCOMES from the Play Outcome Engine
/// This ensures commentary reflects what actually happened, not just the event type
/// Integrates with both human and computer AI play generation

use crate::simulation::{
    PlayOutcomeEngine, PassOutcome, TackleOutcome, ShotOutcome, InjuryResult,
    EnhancedCommentaryLibrary, MatchNarrativeContext, CommentaryContext,
};
use crate::models::Player;

/// Maps event outcomes to appropriate commentary variants
pub struct OutcomeCommentaryMapper;

impl OutcomeCommentaryMapper {
    /// Generate commentary for a pass outcome
    /// Called AFTER pass outcome is calculated, not before
    pub fn commentary_for_pass(
        passer_name: &str,
        receiver_name: &str,
        outcome: PassOutcome,
        completion_prob: f32,
        context: &MatchNarrativeContext,
    ) -> String {
        match outcome {
            PassOutcome::Completed => {
                if completion_prob > 0.85 {
                    // Difficult pass completed
                    format!("<p1> threads a precise pass to <p2> despite the pressure")
                } else if completion_prob > 0.65 {
                    // Standard pass
                    format!("<p1> passes to <p2>")
                } else {
                    // Risky pass that came off
                    format!("<p1> takes a chance and finds <p2> - what a ball!")
                }
            }
            PassOutcome::Intercepted => {
                format!("The opposition intercepts the pass from <p1> intended for <p2>")
            }
            PassOutcome::OutOfPlay => {
                format!("<p1>'s pass to <p2> sails out of play")
            }
        }
    }

    /// Generate commentary for a tackle outcome
    pub fn commentary_for_tackle(
        defender_name: &str,
        attacker_name: &str,
        outcome: TackleOutcome,
        success_prob: f32,
        context: &MatchNarrativeContext,
    ) -> String {
        match outcome {
            TackleOutcome::Success => {
                if success_prob > 0.75 {
                    // Dominant defender
                    format!("<p1> reads the play perfectly and slides in to dispossess <p2>")
                } else {
                    format!("<p1> wins the ball from <p2>")
                }
            }
            TackleOutcome::Dodged => {
                format!("<p2> nimbly avoids <p1>'s challenge and skips past")
            }
            TackleOutcome::YellowCard => {
                format!("<p1> brings down <p2> recklessly - the referee shows a yellow card!")
            }
            TackleOutcome::Foul => {
                format!("<p1> leaves his foot in dangerously on <p2> - RED CARD! The crowd erupts!")
            }
        }
    }

    /// Generate commentary for a shot outcome
    pub fn commentary_for_shot(
        shooter_name: &str,
        goalkeeper_name: &str,
        outcome: ShotOutcome,
        goal_prob: f32,
        context: &MatchNarrativeContext,
    ) -> String {
        match outcome {
            ShotOutcome::Goal => {
                format!("GOAL! <p1> SCORES! The ball is in the net!")
            }
            ShotOutcome::SavedByGoalkeeper => {
                format!("What a save! <p2> palms away <p1>'s effort with a brilliant reflexes!")
            }
            ShotOutcome::OnTarget => {
                format!("<p1>'s shot is on target but deflects away from <p2>")
            }
            ShotOutcome::WideOfPost => {
                format!("<p1>'s shot flies just wide of the post - so close!")
            }
            ShotOutcome::Blocked => {
                format!("The defense blocks <p1>'s shot before it reaches <p2>")
            }
        }
    }

    /// Generate commentary for an injury outcome
    pub fn commentary_for_injury(
        injured_player_name: &str,
        outcome: InjuryResult,
        context: &MatchNarrativeContext,
    ) -> String {
        match outcome {
            InjuryResult::None => String::new(),  // No injury, no commentary
            InjuryResult::Minor => {
                format!("<p1> takes a knock but gets straight back on his feet")
            }
            InjuryResult::Serious => {
                format!("<p1> goes down injured - the trainer is rushing on to the pitch")
            }
            InjuryResult::CarriedOff => {
                format!("It's a serious injury for <p1> - he cannot continue. <t> are now down to ten men!")
            }
        }
    }

    /// Calculate active commentary contexts based on event outcomes
    /// This enriches the context builder's generic contexts with outcome-specific ones
    pub fn contexts_from_outcome(
        outcome_type: &str,
        success: bool,
        context: &mut MatchNarrativeContext,
    ) {
        // Add outcome-specific contexts to the existing context list
        match outcome_type {
            "pass" => {
                if success {
                    // Successful pass might trigger "possession_maintained"
                    // This would need to be added to CommentaryContext enum
                } else {
                    // Loss of possession could trigger "turnover" context
                }
            }
            "tackle" => {
                if success {
                    // Successful tackle: defensive strength shown
                } else {
                    // Failed tackle: attacker getting dangerous chances
                }
            }
            "shot" => {
                if success {
                    // Goal: team momentum, excitement
                    context.unique_events_used.insert("goal_scored".to_string());
                } else {
                    // Near miss: frustration or relief
                }
            }
            "injury" => {
                // Injury: disruption to team
                context.unique_events_used.insert("player_injured".to_string());
            }
            _ => {}
        }
    }
}

/// Bridge between Play Outcome Engine and match simulation
/// This is what gets called when an event occurs in the match
pub struct OutcomeAwareEventGenerator {
    pub outcome_engine: PlayOutcomeEngine,
    pub commentary_mapper: OutcomeCommentaryMapper,
}

impl OutcomeAwareEventGenerator {
    /// Process a passing action with intelligent outcome calculation
    /// Returns (outcome, commentary, should_swap_possession)
    pub fn process_pass_event(
        passer: &Player,
        receiver: &Player,
        distance: f32,
        defensive_pressure: f32,
        match_state: &crate::simulation::MatchState,
        context: &MatchNarrativeContext,
        rng: &mut crate::simulation::SimpleRng,
    ) -> (PassOutcome, String, bool) {
        let (outcome, prob) = PlayOutcomeEngine::calculate_pass_outcome(
            passer,
            distance,
            defensive_pressure,
            match_state,
            rng,
        );

        let commentary = OutcomeCommentaryMapper::commentary_for_pass(
            &passer.name,
            &receiver.name,
            outcome,
            prob,
            context,
        );

        // Only completed passes maintain possession
        let maintains_possession = matches!(outcome, PassOutcome::Completed);

        (outcome, commentary, maintains_possession)
    }

    /// Process a tackling action with injury risk calculation
    /// Returns (tackle_outcome, injury_result, commentary)
    pub fn process_tackle_event(
        defender: &Player,
        attacker: &Player,
        match_intensity: f32,
        match_state: &crate::simulation::MatchState,
        context: &MatchNarrativeContext,
        rng: &mut crate::simulation::SimpleRng,
    ) -> (TackleOutcome, InjuryResult, String) {
        let (tackle_outcome, success_prob) = PlayOutcomeEngine::calculate_tackle_outcome(
            defender,
            attacker,
            match_intensity,
            match_state,
            rng,
        );

        // Calculate injury risk for tackled player
        let injury = PlayOutcomeEngine::calculate_injury_risk(
            attacker,
            match_intensity * 0.8,
            match_state.weather_severity,
            match_state.pitch_condition_degradation,
            1.0 - match_state.fatigue,
            45,  // Approxim ate minute
            rng,
        );

        let commentary = if !matches!(injury, InjuryResult::None) {
            OutcomeCommentaryMapper::commentary_for_injury(&attacker.name, injury, context)
        } else {
            OutcomeCommentaryMapper::commentary_for_tackle(
                &defender.name,
                &attacker.name,
                tackle_outcome,
                success_prob,
                context,
            )
        };

        (tackle_outcome, injury, commentary)
    }

    /// Process a shooting action with goalkeeper interaction
    /// Returns (shot_outcome, commentary, is_goal)
    pub fn process_shot_event(
        shooter: &Player,
        goalkeeper: &Player,
        distance: f32,
        match_intensity: f32,
        match_state: &crate::simulation::MatchState,
        context: &MatchNarrativeContext,
        rng: &mut crate::simulation::SimpleRng,
    ) -> (ShotOutcome, String, bool) {
        let (outcome, goal_prob) = PlayOutcomeEngine::calculate_shot_outcome(
            shooter,
            goalkeeper,
            distance,
            match_intensity,
            match_state,
            rng,
        );

        let is_goal = matches!(outcome, ShotOutcome::Goal);

        let commentary = OutcomeCommentaryMapper::commentary_for_shot(
            &shooter.name,
            &goalkeeper.name,
            outcome,
            goal_prob,
            context,
        );

        (outcome, commentary, is_goal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass_commentary_completed() {
        let context = MatchNarrativeContext::new(1, 1, 2, "Home".to_string(), "Away".to_string(), 1);
        let text = OutcomeCommentaryMapper::commentary_for_pass(
            "Player1",
            "Player2",
            PassOutcome::Completed,
            0.8,
            &context,
        );
        assert!(!text.is_empty());
        assert!(text.contains("pass"));
    }

    #[test]
    fn test_injury_commentary_carried_off() {
        let context = MatchNarrativeContext::new(1, 1, 2, "Home".to_string(), "Away".to_string(), 1);
        let text = OutcomeCommentaryMapper::commentary_for_injury(
            "Player1",
            InjuryResult::CarriedOff,
            &context,
        );
        assert!(text.contains("ten men"));
    }

    #[test]
    fn test_shot_commentary_goal() {
        let context = MatchNarrativeContext::new(1, 1, 2, "Home".to_string(), "Away".to_string(), 1);
        let text =
            OutcomeCommentaryMapper::commentary_for_shot("Striker", "Keeper", ShotOutcome::Goal, 0.3, &context);
        assert!(text.contains("GOAL"));
    }
}
