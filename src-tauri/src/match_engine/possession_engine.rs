/// Possession-Based Football Physics Engine for Sheffield Rules (1858-1877)
/// Adapted from Football Man's possession physics
///
/// Key principles:
/// - Possession chains have exponential failure rates
/// - Each pass compounds the risk of turnover
/// - Quality differential amplifies at every level
/// - Shots only occur after successful chain build-up
/// - Adapted for Sheffield Rules (rouge scoring, variable team sizes)

use serde::{Deserialize, Serialize};

/// Simple RNG for deterministic simulation
pub struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    pub fn random_float(&mut self) -> f32 {
        // Linear congruential generator
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        ((self.state / 65536) % 32768) as f32 / 32768.0
    }

    pub fn random_range(&mut self, min: i32, max: i32) -> i32 {
        min + (self.random_float() * (max - min + 1) as f32) as i32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldZone {
    DefensiveThird,
    MiddleThird,
    AttackingThird,
}

impl FieldZone {
    pub fn from_x_position(x: f32) -> Self {
        if x < 0.33 {
            FieldZone::DefensiveThird
        } else if x < 0.67 {
            FieldZone::MiddleThird
        } else {
            FieldZone::AttackingThird
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TeamSide {
    Home,
    Away,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SequenceOutcome {
    ShotAttempted,
    TurnoverInDefense,
    TurnoverInMidfield,
    TurnoverInAttack,
    OutOfPlay,        // Generic, used internally before zone discrimination
    ThrowIn,          // Ball went into touch: opposite team throws in at right angles
    GoalKick,         // Ball went behind byline: defending team restarts within 6 yards
    HandlingFoul,     // Sheffield Rules: knock-on / carrying → free kick to opposition
    Offside,          // Sheffield Rules: attacker between opponent's goal and their GK
    HalfEnded,
}

/// Tracks the ball state during a possession sequence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BallState {
    pub possession_team: TeamSide,
    pub current_zone: FieldZone,
    pub pass_chain_length: i32,
    pub time_in_possession: i32,  // Seconds
    pub under_pressure: bool,
}

impl BallState {
    pub fn new() -> Self {
        Self {
            possession_team: TeamSide::Home,
            current_zone: FieldZone::MiddleThird,
            pass_chain_length: 0,
            time_in_possession: 0,
            under_pressure: false,
        }
    }

    pub fn reset_sequence(&mut self) {
        self.pass_chain_length = 0;
        self.time_in_possession = 0;
        self.under_pressure = false;
    }

    pub fn switch_possession(&mut self) {
        self.possession_team = match self.possession_team {
            TeamSide::Home => TeamSide::Away,
            TeamSide::Away => TeamSide::Home,
        };
        self.reset_sequence();
    }
}

/// Records one full possession sequence (3-15 passes typically)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PossessionSequence {
    pub minute: i32,
    pub start_zone: FieldZone,
    pub end_zone: FieldZone,
    pub passes_attempted: i32,
    pub passes_completed: i32,
    pub outcome: SequenceOutcome,
    pub final_shot: Option<ShotAttempt>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShotAttempt {
    pub zone: FieldZone,
    pub distance: f32,  // 0.0 (far) to 1.0 (penalty spot)
    pub on_target: bool,
    pub goal: bool,
    pub is_rouge: bool,  // Sheffield Rules: behind-goal placement (1862-1868)
}

/// Possession engine - replaces SWOS reactive AI with realistic sequences
pub struct PossessionEngine {
    pub ball_state: BallState,

    // Team quality (0.0-1.0) from PlayerAttributes aggregation
    home_team_quality: f32,
    away_team_quality: f32,

    // Captain cohesion (leadership + decision_making) / 2
    home_captain_cohesion: f32,
    away_captain_cohesion: f32,

    // Goalkeeper attributes (separated for shot-stopping)
    home_goalkeeper_quality: f32,  // reflexes + handling, normalized 0.0-1.0
    away_goalkeeper_quality: f32,

    // Match conditions
    minute: i32,
    fatigue: f32,  // 0.0 to 1.0 (minute / 90)
    weather_multiplier: f32,  // 0.7 to 1.0

    // Sheffield Rules: Rouge scoring (1862-1868)
    rouge_active: bool,  // Enable rouge scoring for this match
}

impl PossessionEngine {
    pub fn new(
        home_quality: f32,
        away_quality: f32,
        home_captain_cohesion: f32,
        away_captain_cohesion: f32,
        weather_multiplier: f32,
        rouge_active: bool,
    ) -> Self {
        Self {
            ball_state: BallState::new(),
            home_team_quality: home_quality,
            away_team_quality: away_quality,
            home_captain_cohesion,
            away_captain_cohesion,
            home_goalkeeper_quality: away_quality * 0.8,  // Default: 80% of team quality
            away_goalkeeper_quality: home_quality * 0.8,
            minute: 0,
            fatigue: 0.0,
            weather_multiplier,
            rouge_active,
        }
    }

    /// Set goalkeeper quality separately (reflexes + handling normalized)
    pub fn set_home_goalkeeper_quality(&mut self, gk_quality: f32) {
        self.home_goalkeeper_quality = gk_quality.max(0.0).min(1.0);
    }

    pub fn set_away_goalkeeper_quality(&mut self, gk_quality: f32) {
        self.away_goalkeeper_quality = gk_quality.max(0.0).min(1.0);
    }

    /// Update match time and fatigue
    pub fn update_time(&mut self, minute: i32) {
        self.minute = minute;
        self.fatigue = (minute as f32 / 90.0).min(1.0);
    }

    /// Attempt one possession sequence (3-15 passes typically)
    pub fn attempt_possession_sequence(
        &mut self,
        rng: &mut SimpleRng,
    ) -> PossessionSequence {
        let minute = self.minute;
        let start_zone = self.ball_state.current_zone;
        self.ball_state.reset_sequence();

        loop {
            self.ball_state.pass_chain_length += 1;

            // Try to complete one pass
            let pass_success = self.attempt_pass(rng);

            // A4: Sheffield Rules handling foul (~8% per pass in any zone)
            if rng.random_float() < 0.08 {
                self.ball_state.switch_possession();
                return PossessionSequence {
                    minute,
                    start_zone,
                    end_zone: self.ball_state.current_zone,
                    passes_attempted: self.ball_state.pass_chain_length,
                    passes_completed: self.ball_state.pass_chain_length - 1,
                    outcome: SequenceOutcome::HandlingFoul,
                    final_shot: None,
                };
            }

            if !pass_success {
                // Turnover - possession lost
                let outcome = match self.ball_state.current_zone {
                    FieldZone::DefensiveThird => SequenceOutcome::TurnoverInDefense,
                    FieldZone::MiddleThird => SequenceOutcome::TurnoverInMidfield,
                    FieldZone::AttackingThird => SequenceOutcome::TurnoverInAttack,
                };

                self.ball_state.switch_possession();

                return PossessionSequence {
                    minute,
                    start_zone,
                    end_zone: self.ball_state.current_zone,
                    passes_attempted: self.ball_state.pass_chain_length,
                    passes_completed: self.ball_state.pass_chain_length - 1,
                    outcome,
                    final_shot: None,
                };
            }

            // Advance possession zone (defensive → mid → attacking)
            self.advance_possession_zone(rng);

            // A5: Sheffield Rules offside (~10% when FWD in attacking third)
            // Any player between opponent's goal and GK is offside unless they followed ball
            if self.ball_state.current_zone == FieldZone::AttackingThird
                && rng.random_float() < 0.10
            {
                // Offside → free kick to defending team
                self.ball_state.switch_possession();
                return PossessionSequence {
                    minute,
                    start_zone,
                    end_zone: FieldZone::AttackingThird,
                    passes_attempted: self.ball_state.pass_chain_length,
                    passes_completed: self.ball_state.pass_chain_length,
                    outcome: SequenceOutcome::Offside,
                    final_shot: None,
                };
            }

            // Can we take a shot from here?
            if self.ball_state.current_zone == FieldZone::AttackingThird
                && self.ball_state.pass_chain_length >= 2  // Allow early shots
            {
                let shot_prob = self.calculate_shot_probability();
                if rng.random_float() < shot_prob {
                    // Attempt shot
                    let shot = self.attempt_shot(rng);
                    return PossessionSequence {
                        minute,
                        start_zone,
                        end_zone: FieldZone::AttackingThird,
                        passes_attempted: self.ball_state.pass_chain_length,
                        passes_completed: self.ball_state.pass_chain_length,
                        outcome: SequenceOutcome::ShotAttempted,
                        final_shot: Some(shot),
                    };
                }
            }

            // Max 15 passes before forced outcome (realistic buildups)
            // A3: Distinguish touchline (ThrowIn) vs byline (GoalKick) based on zone
            if self.ball_state.pass_chain_length >= 15 {
                let out_outcome = match self.ball_state.current_zone {
                    // Ball in middle third going out of play = touchline throw-in
                    FieldZone::MiddleThird => {
                        self.ball_state.switch_possession();
                        SequenceOutcome::ThrowIn
                    }
                    // Ball in attacking third going out = goal kick (defending team)
                    FieldZone::AttackingThird => {
                        self.ball_state.switch_possession();
                        SequenceOutcome::GoalKick
                    }
                    // Ball in defensive third going out = throw-in for attackers
                    FieldZone::DefensiveThird => {
                        self.ball_state.switch_possession();
                        SequenceOutcome::ThrowIn
                    }
                };
                return PossessionSequence {
                    minute,
                    start_zone,
                    end_zone: self.ball_state.current_zone,
                    passes_attempted: self.ball_state.pass_chain_length,
                    passes_completed: self.ball_state.pass_chain_length,
                    outcome: out_outcome,
                    final_shot: None,
                };
            }
        }
    }

    /// Attempt one pass in the sequence
    fn attempt_pass(&self, rng: &mut SimpleRng) -> bool {
        let base_success = 0.75;  // Base pass success probability

        // Get attacking and defending team quality
        let (attacking_quality, defending_quality) = match self.ball_state.possession_team {
            TeamSide::Home => (self.home_team_quality, self.away_team_quality),
            TeamSide::Away => (self.away_team_quality, self.home_team_quality),
        };

        // Get captain cohesion for attacking team
        let captain_cohesion = match self.ball_state.possession_team {
            TeamSide::Home => self.home_captain_cohesion,
            TeamSide::Away => self.away_captain_cohesion,
        };

        // Quality multiplier: better attackers pass better, weaker defenders press worse
        let quality_multiplier = (0.5 + (attacking_quality * 1.0))
            * (0.5 + ((1.0 - defending_quality) * 0.8));

        // Zone difficulty
        let zone_multiplier = match self.ball_state.current_zone {
            FieldZone::DefensiveThird => 1.08,  // Safer
            FieldZone::MiddleThird => 1.00,     // Standard
            FieldZone::AttackingThird => 0.92,  // Harder (defenders compact)
        };

        // Fatigue penalty (more tired = worse passing)
        let fatigue_multiplier = 1.0 - (self.fatigue * 0.17);

        // Pressure increases with chain length
        let pressure_multiplier = 1.0 - ((self.ball_state.pass_chain_length as f32 - 1.0) * 0.05)
            .min(0.35);

        // Captain cohesion affects passing coordination
        let cohesion_multiplier = 0.5 + (captain_cohesion * 0.5);  // 0.5 to 1.0

        // Weather effects (rain/fog reduce passing)
        let weather_mult = self.weather_multiplier;

        // Calculate final pass success
        let pass_success = (base_success
            * quality_multiplier
            * zone_multiplier
            * fatigue_multiplier
            * pressure_multiplier
            * cohesion_multiplier
            * weather_mult)
            .max(0.25)
            .min(0.95);

        rng.random_float() < pass_success
    }

    /// Advance ball one zone forward (defensive → mid → attacking)
    fn advance_possession_zone(&mut self, rng: &mut SimpleRng) {
        let base_advance_prob = match self.ball_state.current_zone {
            FieldZone::DefensiveThird => 0.80,
            FieldZone::MiddleThird => 0.60,
            FieldZone::AttackingThird => 0.0,  // Already at attacking zone
        };

        if rng.random_float() < base_advance_prob {
            self.ball_state.current_zone = match self.ball_state.current_zone {
                FieldZone::DefensiveThird => FieldZone::MiddleThird,
                FieldZone::MiddleThird => FieldZone::AttackingThird,
                FieldZone::AttackingThird => FieldZone::AttackingThird,
            };
        }
    }

    /// Calculate probability of taking a shot
    fn calculate_shot_probability(&self) -> f32 {
        let (attacking_quality, defending_quality) = match self.ball_state.possession_team {
            TeamSide::Home => (self.home_team_quality, self.away_team_quality),
            TeamSide::Away => (self.away_team_quality, self.home_team_quality),
        };

        // Base shot probability
        let mut shot_prob = 0.35;

        // Longer chains = better chances (buildup quality)
        shot_prob += (self.ball_state.pass_chain_length as f32 * 0.05).min(0.25);

        // Quality differential
        shot_prob += (attacking_quality - defending_quality) * 0.20;

        // Zone bonus (attacking third)
        if self.ball_state.current_zone == FieldZone::AttackingThird {
            shot_prob += 0.15;
        }

        shot_prob.max(0.15).min(0.85)
    }

    /// Attempt a shot
    fn attempt_shot(&self, rng: &mut SimpleRng) -> ShotAttempt {
        // Shot distance: longer chains = closer range
        let distance = (0.6 - (self.ball_state.pass_chain_length as f32 * 0.05))
            .max(0.3)
            .min(0.9);

        // On target probability (80% base)
        let on_target_prob = 0.80;
        let on_target = rng.random_float() < on_target_prob;

        ShotAttempt {
            zone: FieldZone::AttackingThird,
            distance,
            on_target,
            goal: false,  // Will be determined by goal conversion
            is_rouge: false,  // Will be determined if shot misses goal but behind
        }
    }

    /// Calculate goal conversion probability from shot
    pub fn calculate_goal_probability(&self, shot: &ShotAttempt) -> f32 {
        if !shot.on_target {
            return 0.0;
        }

        // Get striker and goalkeeper quality
        let (striker_quality, gk_quality) = match self.ball_state.possession_team {
            TeamSide::Home => (self.home_team_quality, self.away_goalkeeper_quality),
            TeamSide::Away => (self.away_team_quality, self.home_goalkeeper_quality),
        };

        // Base conversion (50%)
        let mut conversion = 0.50;

        // Striker quality adds (0.0-1.0 adds up to 30%)
        conversion += striker_quality * 0.30;

        // Goalkeeper quality subtracts (0.0-1.0 subtracts up to 25%)
        conversion -= gk_quality * 0.25;

        // Distance penalty (closer = better)
        conversion += (1.0 - shot.distance) * 0.15;

        // Clamp to realistic range
        conversion.max(0.05).min(0.95)
    }

    /// Sheffield Rules: Calculate rouge probability (1862-1868)
    /// If shot misses goal, can it go behind for a rouge (1 point)?
    pub fn calculate_rouge_probability(&self, shot: &ShotAttempt) -> f32 {
        if !self.rouge_active || shot.on_target {
            return 0.0;  // No rouge if on target or rouge not active
        }

        // Off-target shots have 30% chance of going behind for rouge
        // (vs 70% going wide/over for no score)
        0.30
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possession_engine_creation() {
        let engine = PossessionEngine::new(0.80, 0.60, 0.85, 0.70, 0.95, false);
        assert_eq!(engine.ball_state.possession_team, TeamSide::Home);
        assert_eq!(engine.ball_state.current_zone, FieldZone::MiddleThird);
    }

    #[test]
    fn test_ball_state_possession_switch() {
        let mut ball = BallState::new();
        assert_eq!(ball.possession_team, TeamSide::Home);

        ball.switch_possession();
        assert_eq!(ball.possession_team, TeamSide::Away);

        ball.switch_possession();
        assert_eq!(ball.possession_team, TeamSide::Home);
    }

    #[test]
    fn test_simple_rng() {
        let mut rng = SimpleRng::new(12345);
        let val1 = rng.random_float();
        let val2 = rng.random_float();

        // Should generate different values
        assert_ne!(val1, val2);

        // Should be in range [0.0, 1.0)
        assert!(val1 >= 0.0 && val1 < 1.0);
        assert!(val2 >= 0.0 && val2 < 1.0);
    }

    #[test]
    fn test_rouge_scoring() {
        let engine = PossessionEngine::new(0.75, 0.75, 0.70, 0.70, 1.0, true);

        let off_target_shot = ShotAttempt {
            zone: FieldZone::AttackingThird,
            distance: 0.5,
            on_target: false,
            goal: false,
            is_rouge: false,
        };

        let rouge_prob = engine.calculate_rouge_probability(&off_target_shot);
        assert_eq!(rouge_prob, 0.30);  // 30% chance of rouge for off-target shots

        // Test with rouge inactive
        let engine_no_rouge = PossessionEngine::new(0.75, 0.75, 0.70, 0.70, 1.0, false);
        let rouge_prob_inactive = engine_no_rouge.calculate_rouge_probability(&off_target_shot);
        assert_eq!(rouge_prob_inactive, 0.0);  // No rouge when inactive
    }
}
