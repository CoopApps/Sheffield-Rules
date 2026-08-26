/// Match Simulator Orchestrator for Sheffield Rules Football
/// Coordinates possession engine, formations, and visual state tracking
/// Supports both live streaming and fast batch simulation modes

use serde::{Deserialize, Serialize};
use crate::match_engine::{
    PossessionEngine, PossessionSequence, BallState, ShotAttempt,
    SequenceOutcome, TeamSide, SimpleRng, Formation, Position2D,
    PlayerAttributes, TeamQualityCalculator, FieldZone
};

/// Match event for timeline/commentary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchEvent {
    pub minute: i32,
    pub second: i32,
    pub event_type: EventType,
    pub description: String,
    pub position: Position2D,
    pub players_involved: Vec<String>,
    pub team_side: TeamSide,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    KickOff,
    Pass,
    Shot,
    Goal,
    Rouge,     // Sheffield Rules: behind-goal placement (1862-1868)
    Save,
    Tackle,
    Turnover,
    OutOfPlay,
    ThrowIn,   // Sheffield Rules: opposite team throws in from touchline at right angles
    GoalKick,  // Sheffield Rules: defending team restarts within 6 yards
    FreeKick,  // Sheffield Rules: handling foul or offside
    HalfTime,
    FullTime,
}

/// Visual state snapshot for 2D rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualState {
    pub minute: i32,
    pub ball_position: Position2D,
    pub ball_z: f32,           // normalised height 0.0 (ground) to 1.0 (peak)
    pub player_positions: Vec<PlayerPositionState>,
    pub possession_team: TeamSide,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerPositionState {
    pub player_id: String,
    pub name: String,
    pub team: TeamSide,
    pub position: Position2D,
    pub has_ball: bool,
}

/// Complete match result with all statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    pub match_id: String,
    pub home_score: i32,
    pub away_score: i32,
    pub home_rouges: i32,  // Sheffield Rules (1862-1868)
    pub away_rouges: i32,
    pub statistics: MatchStatistics,
    pub events: Vec<MatchEvent>,
    pub visual_states: Vec<VisualState>,
    pub scorers: Vec<ScorerInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchStatistics {
    pub home_possession: f32,
    pub away_possession: f32,
    pub home_shots: i32,
    pub away_shots: i32,
    pub home_shots_on_target: i32,
    pub away_shots_on_target: i32,
    pub home_passes: i32,
    pub away_passes: i32,
    pub home_passes_completed: i32,
    pub away_passes_completed: i32,
    pub home_turnovers: i32,
    pub away_turnovers: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScorerInfo {
    pub player_name: String,
    pub minute: i32,
    pub is_rouge: bool,  // Sheffield Rules: rouge vs goal
}

/// Main match simulator
pub struct MatchSimulator {
    match_id: String,
    possession_engine: PossessionEngine,
    home_formation: Formation,
    away_formation: Formation,
    home_players: Vec<PlayerAttributes>,
    away_players: Vec<PlayerAttributes>,
    rng: SimpleRng,

    // Match state tracking
    current_minute: i32,
    home_score: i32,
    away_score: i32,
    home_rouges: i32,
    away_rouges: i32,

    // Event/state accumulation
    events: Vec<MatchEvent>,
    visual_states: Vec<VisualState>,
    scorers: Vec<ScorerInfo>,

    // Statistics tracking
    stats: MatchStatistics,
    home_possession_time: i32,
    away_possession_time: i32,

    // Sheffield Rules configuration
    rouge_active: bool,
    match_duration: i32,  // Usually 90, but can vary

    // Ball z-height arc: 0.0 = ground, advances 0..1 during a shot arc then resets
    ball_arc_t: f32,

    // Continuous ball position (moves smoothly each sub-tick toward target zone)
    ball_continuous_pos: Position2D,
    // Target ball position (set when zone changes)
    ball_target_pos: Position2D,

    // AI player movement: persistent positions updated each capture_visual_state()
    home_player_pos: Vec<Position2D>,
    away_player_pos: Vec<Position2D>,
}

impl MatchSimulator {
    /// Create new match simulator
    pub fn new(
        match_id: String,
        home_players: Vec<PlayerAttributes>,
        away_players: Vec<PlayerAttributes>,
        rule_year: i32,
        seed: u64,
    ) -> Self {
        // Calculate team qualities from player attributes
        let home_quality = TeamQualityCalculator::calculate_team_quality(&home_players, "MID");
        let away_quality = TeamQualityCalculator::calculate_team_quality(&away_players, "MID");

        // Calculate captain cohesion (assume first player is captain for now)
        let home_captain_cohesion = TeamQualityCalculator::calculate_captain_cohesion(
            home_players.first()
        );
        let away_captain_cohesion = TeamQualityCalculator::calculate_captain_cohesion(
            away_players.first()
        );

        // Determine if rouge scoring is active (1862-1867; abolished October 1868)
        let rouge_active = rule_year >= 1862 && rule_year <= 1867;

        // Create formations based on year
        let home_formation = Formation::for_year(rule_year);
        let away_formation = home_formation.mirror();

        // Weather multiplier (default 1.0, can be configured)
        let weather_multiplier = 1.0;

        // Create possession engine
        let possession_engine = PossessionEngine::new(
            home_quality,
            away_quality,
            home_captain_cohesion,
            away_captain_cohesion,
            weather_multiplier,
            rouge_active,
        );

        // Initialise persistent player positions from formation defaults
        let home_player_pos: Vec<Position2D> = home_formation.positions.iter()
            .map(|fp| fp.position.clone())
            .collect();
        let away_player_pos: Vec<Position2D> = away_formation.positions.iter()
            .map(|fp| fp.position.clone())
            .collect();

        Self {
            match_id,
            possession_engine,
            home_formation,
            away_formation,
            home_players,
            away_players,
            rng: SimpleRng::new(seed),
            current_minute: 0,
            home_score: 0,
            away_score: 0,
            home_rouges: 0,
            away_rouges: 0,
            events: Vec::new(),
            visual_states: Vec::new(),
            scorers: Vec::new(),
            stats: MatchStatistics {
                home_possession: 0.0,
                away_possession: 0.0,
                home_shots: 0,
                away_shots: 0,
                home_shots_on_target: 0,
                away_shots_on_target: 0,
                home_passes: 0,
                away_passes: 0,
                home_passes_completed: 0,
                away_passes_completed: 0,
                home_turnovers: 0,
                away_turnovers: 0,
            },
            home_possession_time: 0,
            away_possession_time: 0,
            rouge_active,
            match_duration: 90,
            ball_arc_t: 0.0,
            ball_continuous_pos: Position2D::new(0.5, 0.5),
            ball_target_pos: Position2D::new(0.5, 0.5),
            home_player_pos,
            away_player_pos,
        }
    }

    /// Set weather conditions (affects possession engine)
    pub fn set_weather(&mut self, weather: &str, pitch_condition: &str) {
        let weather_mult = Self::calculate_weather_multiplier(weather, pitch_condition);
        // Update possession engine weather multiplier
        self.possession_engine = PossessionEngine::new(
            self.possession_engine.ball_state.possession_team as u8 as f32, // placeholder
            0.0,  // Will be overridden - need to refactor
            0.0,
            0.0,
            weather_mult,
            self.rouge_active,
        );
    }

    fn calculate_weather_multiplier(weather: &str, pitch_condition: &str) -> f32 {
        let weather_factor = match weather.to_lowercase().as_str() {
            "clear" => 1.0,
            "rainy" | "rain" => 0.85,
            "windy" | "wind" => 0.90,
            "foggy" | "fog" => 0.80,
            _ => 0.9,
        };

        let pitch_factor = match pitch_condition.to_lowercase().as_str() {
            "perfect" => 1.0,
            "good" => 0.95,
            "poor" => 0.85,
            "very poor" => 0.75,
            _ => 0.9,
        };

        let multiplier: f32 = weather_factor * pitch_factor;
        multiplier.max(0.7).min(1.0)
    }

    /// Live streaming simulation (emits events via callback for real-time viewing).
    /// Each game-minute = 1 real second, split into 20 sub-ticks of 50ms each.
    /// Visual states are emitted at every sub-tick so the frontend gets smooth 20fps updates.
    pub fn simulate_with_streaming<F>(mut self, mut callback: F) -> MatchResult
    where
        F: FnMut(&MatchEvent, &VisualState, &MatchStatistics),
    {
        const SUBTICKS_PER_MINUTE: u32 = 20;   // 20 × 50ms = 1000ms per game-minute
        const SUBTICK_MS: u64 = 50;

        // Kickoff event
        let kickoff_event = MatchEvent {
            minute: 0,
            second: 0,
            event_type: EventType::KickOff,
            description: "Match kicks off!".to_string(),
            position: Position2D::new(0.5, 0.5),
            players_involved: Vec::new(),
            team_side: TeamSide::Home,
        };

        self.ball_target_pos = Position2D::new(0.5, 0.5);
        self.ball_continuous_pos = Position2D::new(0.5, 0.5);

        self.add_event(kickoff_event.clone());
        self.capture_visual_state_smooth();
        callback(&kickoff_event, &self.visual_states.last().unwrap(), &self.stats);

        // Simulate each minute
        while self.current_minute < self.match_duration {
            // Run game logic for this minute (decides events, sets ball_target_pos)
            self.simulate_minute_streaming(&mut callback);
            self.current_minute += 1;

            // Stream sub-tick visual states for this minute's ball movement
            for _tick in 0..SUBTICKS_PER_MINUTE {
                std::thread::sleep(std::time::Duration::from_millis(SUBTICK_MS));
                self.capture_visual_state_smooth();
                // Emit a synthetic Pass event as the visual state carrier
                let tick_event = MatchEvent {
                    minute: self.current_minute,
                    second: 0,
                    event_type: EventType::Pass,
                    description: String::new(),
                    position: self.ball_continuous_pos.clone(),
                    players_involved: Vec::new(),
                    team_side: self.possession_engine.ball_state.possession_team,
                };
                callback(&tick_event, &self.visual_states.last().unwrap(), &self.stats);
            }

            // Half-time at 45 minutes
            if self.current_minute == 45 {
                let halftime_event = MatchEvent {
                    minute: 45,
                    second: 0,
                    event_type: EventType::HalfTime,
                    description: "Half-time".to_string(),
                    position: Position2D::new(0.5, 0.5),
                    players_involved: Vec::new(),
                    team_side: TeamSide::Home,
                };

                self.ball_target_pos = Position2D::new(0.5, 0.5);
                self.add_event(halftime_event.clone());
                self.capture_visual_state_smooth();
                callback(&halftime_event, &self.visual_states.last().unwrap(), &self.stats);

                // Pause at half-time (3 seconds = 60 sub-ticks)
                for _ in 0..60_u32 {
                    std::thread::sleep(std::time::Duration::from_millis(SUBTICK_MS));
                    self.capture_visual_state_smooth();
                    let tick_event = MatchEvent {
                        minute: 45,
                        second: 0,
                        event_type: EventType::Pass,
                        description: String::new(),
                        position: self.ball_continuous_pos.clone(),
                        players_involved: Vec::new(),
                        team_side: self.possession_engine.ball_state.possession_team,
                    };
                    callback(&tick_event, &self.visual_states.last().unwrap(), &self.stats);
                }
            }
        }

        // Full-time event
        let fulltime_event = MatchEvent {
            minute: self.match_duration,
            second: 0,
            event_type: EventType::FullTime,
            description: format!(
                "Full-time! Final score: {} - {} {}",
                self.home_score,
                self.away_score,
                if self.rouge_active {
                    format!("(Rouges: {} - {})", self.home_rouges, self.away_rouges)
                } else {
                    String::new()
                }
            ),
            position: Position2D::new(0.5, 0.5),
            players_involved: Vec::new(),
            team_side: TeamSide::Home,
        };

        self.add_event(fulltime_event.clone());
        self.capture_visual_state_smooth();
        callback(&fulltime_event, &self.visual_states.last().unwrap(), &self.stats);

        // Calculate final possession percentages
        let total_time = self.home_possession_time + self.away_possession_time;
        if total_time > 0 {
            self.stats.home_possession = (self.home_possession_time as f32 / total_time as f32) * 100.0;
            self.stats.away_possession = (self.away_possession_time as f32 / total_time as f32) * 100.0;
        }

        MatchResult {
            match_id: self.match_id.clone(),
            home_score: self.home_score,
            away_score: self.away_score,
            home_rouges: self.home_rouges,
            away_rouges: self.away_rouges,
            statistics: self.stats.clone(),
            events: self.events.clone(),
            visual_states: self.visual_states.clone(),
            scorers: self.scorers.clone(),
        }
    }

    /// Capture visual state using smooth continuous ball position (ball_continuous_pos).
    /// Moves ball_continuous_pos toward ball_target_pos each call (lerp step).
    fn capture_visual_state_smooth(&mut self) {
        const BALL_SPEED: f32 = 0.04; // normalised units per sub-tick (fast enough to cross pitch in ~1 minute)

        // Move continuous ball toward target
        let dx = self.ball_target_pos.x - self.ball_continuous_pos.x;
        let dy = self.ball_target_pos.y - self.ball_continuous_pos.y;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist > BALL_SPEED {
            self.ball_continuous_pos.x += dx / dist * BALL_SPEED;
            self.ball_continuous_pos.y += dy / dist * BALL_SPEED;
        } else {
            self.ball_continuous_pos.x = self.ball_target_pos.x;
            self.ball_continuous_pos.y = self.ball_target_pos.y;
        }

        // Add small sinusoidal wobble for organic feel
        let t = self.visual_states.len() as f32 * 0.15;
        let wobble_x = (t * 1.1).sin() * 0.008;
        let wobble_y = (t * 0.9 + 0.5).cos() * 0.008;
        let ball_pos = Position2D::new(
            (self.ball_continuous_pos.x + wobble_x).max(0.02).min(0.98),
            (self.ball_continuous_pos.y + wobble_y).max(0.05).min(0.95),
        );

        // Ball z-height arc
        self.ball_arc_t = (self.ball_arc_t + 0.05).min(1.0);
        let ball_z = (std::f32::consts::PI * self.ball_arc_t).sin() * 0.5;

        let possession = self.possession_engine.ball_state.possession_team;
        const PLAYER_SPEED: f32 = 0.004; // slower per sub-tick (20 sub-ticks = same as before)

        let mut player_positions: Vec<PlayerPositionState> = Vec::new();

        // Home team
        for (i, formation_pos) in self.home_formation.positions.iter().enumerate() {
            if self.home_players.get(i).is_some() {
                let target = Self::player_target(
                    &formation_pos.role,
                    &formation_pos.position,
                    &ball_pos,
                    possession,
                    TeamSide::Home,
                );
                let cur = self.home_player_pos.get_mut(i).unwrap();
                let dx = target.x - cur.x;
                let dy = target.y - cur.y;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist > PLAYER_SPEED {
                    cur.x += dx / dist * PLAYER_SPEED;
                    cur.y += dy / dist * PLAYER_SPEED;
                } else {
                    cur.x = target.x;
                    cur.y = target.y;
                }
                cur.x = cur.x.max(0.02).min(0.98);
                cur.y = cur.y.max(0.05).min(0.95);
                player_positions.push(PlayerPositionState {
                    player_id: format!("home_{}", i),
                    name: format!("Player {}", i + 1),
                    team: TeamSide::Home,
                    position: Position2D::new(cur.x, cur.y),
                    has_ball: false,
                });
            }
        }

        // Away team
        let away_offset = player_positions.len();
        for (i, formation_pos) in self.away_formation.positions.iter().enumerate() {
            if self.away_players.get(i).is_some() {
                let target = Self::player_target(
                    &formation_pos.role,
                    &formation_pos.position,
                    &ball_pos,
                    possession,
                    TeamSide::Away,
                );
                let cur = self.away_player_pos.get_mut(i).unwrap();
                let dx = target.x - cur.x;
                let dy = target.y - cur.y;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist > PLAYER_SPEED {
                    cur.x += dx / dist * PLAYER_SPEED;
                    cur.y += dy / dist * PLAYER_SPEED;
                } else {
                    cur.x = target.x;
                    cur.y = target.y;
                }
                cur.x = cur.x.max(0.02).min(0.98);
                cur.y = cur.y.max(0.05).min(0.95);
                player_positions.push(PlayerPositionState {
                    player_id: format!("away_{}", i),
                    name: format!("Player {}", i + 1),
                    team: TeamSide::Away,
                    position: Position2D::new(cur.x, cur.y),
                    has_ball: false,
                });
            }
        }

        // Assign has_ball to closest possessing-team player
        let possessing_range = match possession {
            TeamSide::Home => 0..away_offset,
            TeamSide::Away => away_offset..player_positions.len(),
        };
        let mut closest_idx: Option<usize> = None;
        let mut closest_dist = f32::MAX;
        for idx in possessing_range {
            let d = player_positions[idx].position.distance_to(&ball_pos);
            if d < closest_dist {
                closest_dist = d;
                closest_idx = Some(idx);
            }
        }
        if let Some(idx) = closest_idx {
            player_positions[idx].has_ball = true;
        }

        self.visual_states.push(VisualState {
            minute: self.current_minute,
            ball_position: ball_pos,
            ball_z,
            player_positions,
            possession_team: possession,
        });
    }

    /// Fast batch simulation (no streaming, optimized for AI matches)
    pub fn simulate_fast(mut self) -> MatchResult {
        // Kickoff event
        self.add_event(MatchEvent {
            minute: 0,
            second: 0,
            event_type: EventType::KickOff,
            description: "Match kicks off!".to_string(),
            position: Position2D::new(0.5, 0.5),
            players_involved: Vec::new(),
            team_side: TeamSide::Home,
        });

        // Simulate each minute
        while self.current_minute < self.match_duration {
            self.simulate_minute();
            self.current_minute += 1;

            // Half-time at 45 minutes
            if self.current_minute == 45 {
                self.add_event(MatchEvent {
                    minute: 45,
                    second: 0,
                    event_type: EventType::HalfTime,
                    description: "Half-time".to_string(),
                    position: Position2D::new(0.5, 0.5),
                    players_involved: Vec::new(),
                    team_side: TeamSide::Home,
                });
            }
        }

        // Full-time event
        self.add_event(MatchEvent {
            minute: self.match_duration,
            second: 0,
            event_type: EventType::FullTime,
            description: format!(
                "Full-time! Final score: {} - {} {}",
                self.home_score,
                self.away_score,
                if self.rouge_active {
                    format!("(Rouges: {} - {})", self.home_rouges, self.away_rouges)
                } else {
                    String::new()
                }
            ),
            position: Position2D::new(0.5, 0.5),
            players_involved: Vec::new(),
            team_side: TeamSide::Home,
        });

        // Calculate final possession percentages
        let total_time = self.home_possession_time + self.away_possession_time;
        if total_time > 0 {
            self.stats.home_possession = (self.home_possession_time as f32 / total_time as f32) * 100.0;
            self.stats.away_possession = (self.away_possession_time as f32 / total_time as f32) * 100.0;
        }

        MatchResult {
            match_id: self.match_id,
            home_score: self.home_score,
            away_score: self.away_score,
            home_rouges: self.home_rouges,
            away_rouges: self.away_rouges,
            statistics: self.stats,
            events: self.events,
            visual_states: self.visual_states,
            scorers: self.scorers,
        }
    }

    /// Simulate one minute of match time (non-streaming)
    fn simulate_minute(&mut self) {
        self.possession_engine.update_time(self.current_minute);

        // Average 2-3 possession sequences per minute
        let sequences_this_minute = self.rng.random_range(2, 3);

        for _ in 0..sequences_this_minute {
            let sequence = self.possession_engine.attempt_possession_sequence(&mut self.rng);
            self.process_sequence(sequence);

            // Capture visual state every minute
            if self.current_minute % 1 == 0 {
                self.capture_visual_state();
            }
        }
    }

    /// Simulate one minute of match time with streaming callbacks
    fn simulate_minute_streaming<F>(&mut self, callback: &mut F)
    where
        F: FnMut(&MatchEvent, &VisualState, &MatchStatistics),
    {
        self.possession_engine.update_time(self.current_minute);

        // Average 2-3 possession sequences per minute
        let sequences_this_minute = self.rng.random_range(2, 3);

        for _ in 0..sequences_this_minute {
            let sequence = self.possession_engine.attempt_possession_sequence(&mut self.rng);
            self.process_sequence_streaming(sequence, callback);
        }

        // Note: Visual state updates are already emitted in process_sequence_streaming
        // No need to re-emit the last event here as it would cause duplicates
    }

    /// Process a completed possession sequence (streaming version with callbacks)
    fn process_sequence_streaming<F>(&mut self, sequence: PossessionSequence, callback: &mut F)
    where
        F: FnMut(&MatchEvent, &VisualState, &MatchStatistics),
    {
        // Update ball target position based on end zone + possession team direction
        let new_target = self.zone_to_position_for_team(
            sequence.end_zone,
            self.possession_engine.ball_state.possession_team,
        );
        self.ball_target_pos = new_target;

        // Update possession time tracking
        let possession_duration = (sequence.passes_completed * 5).min(20);
        match self.possession_engine.ball_state.possession_team {
            TeamSide::Home => self.home_possession_time += possession_duration,
            TeamSide::Away => self.away_possession_time += possession_duration,
        }

        // Update pass statistics
        match self.possession_engine.ball_state.possession_team {
            TeamSide::Home => {
                self.stats.home_passes += sequence.passes_attempted;
                self.stats.home_passes_completed += sequence.passes_completed;
            }
            TeamSide::Away => {
                self.stats.away_passes += sequence.passes_attempted;
                self.stats.away_passes_completed += sequence.passes_completed;
            }
        }

        // Process outcome
        match sequence.outcome {
            SequenceOutcome::ShotAttempted => {
                if let Some(shot) = sequence.final_shot {
                    self.process_shot_streaming(shot, sequence.minute, callback);
                }
            }
            SequenceOutcome::TurnoverInDefense |
            SequenceOutcome::TurnoverInMidfield |
            SequenceOutcome::TurnoverInAttack => {
                // Update turnover stats
                let losing_team = self.possession_engine.ball_state.possession_team;
                let tackling_team = match losing_team {
                    TeamSide::Home => TeamSide::Away,
                    TeamSide::Away => TeamSide::Home,
                };
                match losing_team {
                    TeamSide::Home => self.stats.away_turnovers += 1,
                    TeamSide::Away => self.stats.home_turnovers += 1,
                }

                let turnover_pos = self.zone_to_position(sequence.end_zone);
                let tackler_id = self.closest_player_id(tackling_team, &turnover_pos);

                // Emit a Tackle event (drives slide animation on frontend)
                let tackle_event = MatchEvent {
                    minute: sequence.minute,
                    second: 0,
                    event_type: EventType::Tackle,
                    description: "Tackle!".to_string(),
                    position: turnover_pos.clone(),
                    players_involved: vec![tackler_id],
                    team_side: tackling_team,
                };

                // Also emit the Turnover event
                let turnover_event = MatchEvent {
                    minute: sequence.minute,
                    second: 0,
                    event_type: EventType::Turnover,
                    description: "Possession lost".to_string(),
                    position: turnover_pos,
                    players_involved: Vec::new(),
                    team_side: losing_team,
                };

                self.add_event(tackle_event.clone());
                self.add_event(turnover_event.clone());
                self.capture_visual_state_smooth();
                callback(&tackle_event, &self.visual_states.last().unwrap(), &self.stats);
                callback(&turnover_event, &self.visual_states.last().unwrap(), &self.stats);
            }
            SequenceOutcome::OutOfPlay => {
                let event = MatchEvent {
                    minute: sequence.minute,
                    second: 0,
                    event_type: EventType::OutOfPlay,
                    description: "Ball out of play".to_string(),
                    position: self.zone_to_position(sequence.end_zone),
                    players_involved: Vec::new(),
                    team_side: self.possession_engine.ball_state.possession_team,
                };

                self.add_event(event.clone());
                self.capture_visual_state_smooth();
                callback(&event, &self.visual_states.last().unwrap(), &self.stats);
            }
            SequenceOutcome::ThrowIn => {
                // A3: Opposite team throws in from touchline at right angles
                let throwing_team = self.possession_engine.ball_state.possession_team;
                let event = MatchEvent {
                    minute: sequence.minute,
                    second: 0,
                    event_type: EventType::ThrowIn,
                    description: "Throw-in! Ball thrown in from touchline at right angles".to_string(),
                    position: self.zone_to_position(sequence.end_zone),
                    players_involved: Vec::new(),
                    team_side: throwing_team,
                };
                self.add_event(event.clone());
                self.capture_visual_state_smooth();
                callback(&event, &self.visual_states.last().unwrap(), &self.stats);
            }
            SequenceOutcome::GoalKick => {
                // A3: Defending team restarts within 6 yards of goal
                let defending_team = self.possession_engine.ball_state.possession_team;
                let event = MatchEvent {
                    minute: sequence.minute,
                    second: 0,
                    event_type: EventType::GoalKick,
                    description: "Goal kick! Defending team restarts within 6 yards".to_string(),
                    position: self.zone_to_position(sequence.end_zone),
                    players_involved: Vec::new(),
                    team_side: defending_team,
                };
                self.add_event(event.clone());
                self.capture_visual_state_smooth();
                callback(&event, &self.visual_states.last().unwrap(), &self.stats);
            }
            SequenceOutcome::HandlingFoul => {
                // A4: Free kick awarded to the team that now has possession
                let fouled_team = self.possession_engine.ball_state.possession_team;
                let event = MatchEvent {
                    minute: sequence.minute,
                    second: 0,
                    event_type: EventType::FreeKick,
                    description: "Free kick! Handling foul — no knocking or pushing the ball with the hand".to_string(),
                    position: self.zone_to_position(sequence.end_zone),
                    players_involved: Vec::new(),
                    team_side: fouled_team,
                };
                self.add_event(event.clone());
                self.capture_visual_state_smooth();
                callback(&event, &self.visual_states.last().unwrap(), &self.stats);
            }
            SequenceOutcome::Offside => {
                // A5: Free kick to defending team for offside
                let defending_team = self.possession_engine.ball_state.possession_team;
                let event = MatchEvent {
                    minute: sequence.minute,
                    second: 0,
                    event_type: EventType::FreeKick,
                    description: "Offside! Player between opponent's goal and goalkeeper — free kick".to_string(),
                    position: self.zone_to_position(sequence.end_zone),
                    players_involved: Vec::new(),
                    team_side: defending_team,
                };
                self.add_event(event.clone());
                self.capture_visual_state_smooth();
                callback(&event, &self.visual_states.last().unwrap(), &self.stats);
            }
            SequenceOutcome::HalfEnded => {}
        }
    }

    /// Process a completed possession sequence (non-streaming)
    fn process_sequence(&mut self, sequence: PossessionSequence) {
        // Update possession time tracking
        let possession_duration = (sequence.passes_completed * 5).min(20); // ~5 seconds per pass
        match self.possession_engine.ball_state.possession_team {
            TeamSide::Home => self.home_possession_time += possession_duration,
            TeamSide::Away => self.away_possession_time += possession_duration,
        }

        // Update pass statistics
        match self.possession_engine.ball_state.possession_team {
            TeamSide::Home => {
                self.stats.home_passes += sequence.passes_attempted;
                self.stats.home_passes_completed += sequence.passes_completed;
            }
            TeamSide::Away => {
                self.stats.away_passes += sequence.passes_attempted;
                self.stats.away_passes_completed += sequence.passes_completed;
            }
        }

        // Process outcome
        match sequence.outcome {
            SequenceOutcome::ShotAttempted => {
                if let Some(shot) = sequence.final_shot {
                    self.process_shot(shot, sequence.minute);
                }
            }
            SequenceOutcome::TurnoverInDefense |
            SequenceOutcome::TurnoverInMidfield |
            SequenceOutcome::TurnoverInAttack => {
                // Update turnover stats
                match self.possession_engine.ball_state.possession_team {
                    TeamSide::Home => self.stats.away_turnovers += 1,  // Away lost possession
                    TeamSide::Away => self.stats.home_turnovers += 1,
                }

                self.add_event(MatchEvent {
                    minute: sequence.minute,
                    second: 0,
                    event_type: EventType::Turnover,
                    description: "Possession lost".to_string(),
                    position: self.zone_to_position(sequence.end_zone),
                    players_involved: Vec::new(),
                    team_side: self.possession_engine.ball_state.possession_team,
                });
            }
            SequenceOutcome::OutOfPlay => {
                self.add_event(MatchEvent {
                    minute: sequence.minute,
                    second: 0,
                    event_type: EventType::OutOfPlay,
                    description: "Ball out of play".to_string(),
                    position: self.zone_to_position(sequence.end_zone),
                    players_involved: Vec::new(),
                    team_side: self.possession_engine.ball_state.possession_team,
                });
            }
            SequenceOutcome::ThrowIn => {
                self.add_event(MatchEvent {
                    minute: sequence.minute,
                    second: 0,
                    event_type: EventType::ThrowIn,
                    description: "Throw-in! Ball thrown in from touchline at right angles".to_string(),
                    position: self.zone_to_position(sequence.end_zone),
                    players_involved: Vec::new(),
                    team_side: self.possession_engine.ball_state.possession_team,
                });
            }
            SequenceOutcome::GoalKick => {
                self.add_event(MatchEvent {
                    minute: sequence.minute,
                    second: 0,
                    event_type: EventType::GoalKick,
                    description: "Goal kick! Defending team restarts within 6 yards".to_string(),
                    position: self.zone_to_position(sequence.end_zone),
                    players_involved: Vec::new(),
                    team_side: self.possession_engine.ball_state.possession_team,
                });
            }
            SequenceOutcome::HandlingFoul | SequenceOutcome::Offside => {
                let desc = if matches!(sequence.outcome, SequenceOutcome::Offside) {
                    "Offside! Free kick to defending team".to_string()
                } else {
                    "Free kick! Handling foul".to_string()
                };
                self.add_event(MatchEvent {
                    minute: sequence.minute,
                    second: 0,
                    event_type: EventType::FreeKick,
                    description: desc,
                    position: self.zone_to_position(sequence.end_zone),
                    players_involved: Vec::new(),
                    team_side: self.possession_engine.ball_state.possession_team,
                });
            }
            SequenceOutcome::HalfEnded => {}
        }
    }

    /// Find the ID of the closest player on a given team to a position
    fn closest_player_id(&self, team: TeamSide, target: &Position2D) -> String {
        let (players, prefix, formation) = match team {
            TeamSide::Home => (&self.home_players, "home", &self.home_formation),
            TeamSide::Away => (&self.away_players, "away", &self.away_formation),
        };
        let mut best_id = format!("{}_0", prefix);
        let mut best_dist = f32::MAX;
        for (i, _) in players.iter().enumerate() {
            if let Some(fp) = formation.positions.get(i) {
                let d = fp.position.distance_to(target);
                if d < best_dist {
                    best_dist = d;
                    best_id = format!("{}_{}", prefix, i);
                }
            }
        }
        best_id
    }

    /// Process a shot attempt (streaming version)
    fn process_shot_streaming<F>(&mut self, mut shot: ShotAttempt, minute: i32, callback: &mut F)
    where
        F: FnMut(&MatchEvent, &VisualState, &MatchStatistics),
    {
        let shooting_team = self.possession_engine.ball_state.possession_team;

        // Ball moves to shooting position (attacking third, near goal)
        let shot_x = if shooting_team == TeamSide::Home { 0.82 } else { 0.18 };
        let shot_y = self.ball_continuous_pos.y; // keep current y
        self.ball_target_pos = Position2D::new(shot_x, shot_y);

        // Reset ball arc so next capture starts a fresh parabola
        self.ball_arc_t = 0.0;

        // Emit a Shot event before processing outcome
        let shot_pos = Position2D::new(0.80, 0.5);
        let shooter_id = self.closest_player_id(shooting_team, &shot_pos);
        let shot_event = MatchEvent {
            minute,
            second: 0,
            event_type: EventType::Shot,
            description: "Shot!".to_string(),
            position: shot_pos.clone(),
            players_involved: vec![shooter_id],
            team_side: shooting_team,
        };
        self.add_event(shot_event.clone());
        self.capture_visual_state_smooth();
        callback(&shot_event, &self.visual_states.last().unwrap(), &self.stats);

        // Update shot statistics
        match shooting_team {
            TeamSide::Home => {
                self.stats.home_shots += 1;
                if shot.on_target {
                    self.stats.home_shots_on_target += 1;
                }
            }
            TeamSide::Away => {
                self.stats.away_shots += 1;
                if shot.on_target {
                    self.stats.away_shots_on_target += 1;
                }
            }
        }

        // Check if shot results in goal
        if shot.on_target {
            let goal_prob = self.possession_engine.calculate_goal_probability(&shot);
            if self.rng.random_float() < goal_prob {
                shot.goal = true;

                // Update score
                match shooting_team {
                    TeamSide::Home => self.home_score += 1,
                    TeamSide::Away => self.away_score += 1,
                }

                // Add scorer
                self.scorers.push(ScorerInfo {
                    player_name: "Player".to_string(),
                    minute,
                    is_rouge: false,
                });

                // Ball goes into the net — move target to actual goal position
                let goal_x = if shooting_team == TeamSide::Home { 0.95 } else { 0.05 };
                let goal_y = 0.48 + (minute as f32 * 0.07).sin() * 0.05; // slight y variance
                self.ball_target_pos = Position2D::new(goal_x, goal_y);
                self.ball_arc_t = 0.0; // fresh arc for shot going in

                let goal_pos = Position2D::new(goal_x, goal_y);
                let goal_scorer_id = self.closest_player_id(shooting_team, &goal_pos);
                let event = MatchEvent {
                    minute,
                    second: 0,
                    event_type: EventType::Goal,
                    description: "GOAL!".to_string(),
                    position: goal_pos,
                    players_involved: vec![goal_scorer_id],
                    team_side: shooting_team,
                };

                self.add_event(event.clone());
                self.capture_visual_state_smooth();
                callback(&event, &self.visual_states.last().unwrap(), &self.stats);
            } else {
                // Save
                let event = MatchEvent {
                    minute,
                    second: 0,
                    event_type: EventType::Save,
                    description: "Saved by the goalkeeper!".to_string(),
                    position: Position2D::new(0.85, 0.5),
                    players_involved: Vec::new(),
                    team_side: shooting_team,
                };

                self.add_event(event.clone());
                self.capture_visual_state_smooth();
                callback(&event, &self.visual_states.last().unwrap(), &self.stats);
            }
        } else if self.rouge_active {
            // A1: Positional rouge detection (Oct 1867 rules)
            // Goal is centred at y=0.5, normalised half-width ≈ 0.03 each side.
            // Rouge flags are a further 0.04 each side beyond the posts.
            // Ball y in (0.47..0.50) or (0.50..0.53) → within goal → handled above (on_target)
            // Ball y in (0.43..0.47) or (0.53..0.57) → between post and rouge flag → rouge
            // Ball y outside rouge zone → goal kick only
            let ball_y = self.possession_engine.ball_state.current_zone as i32 as f32 * 0.25 + 0.5;
            let ball_y_variance = ((minute as f64 * 0.7 + 1.3).sin() * 0.12) as f32;
            let effective_y = (ball_y + ball_y_variance).max(0.0).min(1.0);
            // Distance from centre (y=0.5)
            let dist_from_centre = (effective_y - 0.5).abs();
            // Rouge zone: post is at 0.03 from centre, rouge flag at 0.07 from centre
            let in_rouge_zone = dist_from_centre > 0.03 && dist_from_centre < 0.07;

            if in_rouge_zone {
                shot.is_rouge = true;

                match shooting_team {
                    TeamSide::Home => self.home_rouges += 1,
                    TeamSide::Away => self.away_rouges += 1,
                }

                self.scorers.push(ScorerInfo {
                    player_name: "Player".to_string(),
                    minute,
                    is_rouge: true,
                });

                let event = MatchEvent {
                    minute,
                    second: 0,
                    event_type: EventType::Rouge,
                    description: "Rouge! Ball passes between the rouge flag and the goalpost".to_string(),
                    position: Position2D::new(0.95, effective_y),
                    players_involved: Vec::new(),
                    team_side: shooting_team,
                };

                self.add_event(event.clone());
                self.capture_visual_state_smooth();
                callback(&event, &self.visual_states.last().unwrap(), &self.stats);
            }
            // else: ball wide/over → no rouge, just a goal kick (possession already switched by engine)
        }
    }

    /// Process a shot attempt (non-streaming)
    fn process_shot(&mut self, mut shot: ShotAttempt, minute: i32) {
        let shooting_team = self.possession_engine.ball_state.possession_team;

        // Update shot statistics
        match shooting_team {
            TeamSide::Home => {
                self.stats.home_shots += 1;
                if shot.on_target {
                    self.stats.home_shots_on_target += 1;
                }
            }
            TeamSide::Away => {
                self.stats.away_shots += 1;
                if shot.on_target {
                    self.stats.away_shots_on_target += 1;
                }
            }
        }

        // Check if shot results in goal
        if shot.on_target {
            let goal_prob = self.possession_engine.calculate_goal_probability(&shot);
            if self.rng.random_float() < goal_prob {
                shot.goal = true;

                // Update score
                match shooting_team {
                    TeamSide::Home => self.home_score += 1,
                    TeamSide::Away => self.away_score += 1,
                }

                // Add scorer
                self.scorers.push(ScorerInfo {
                    player_name: "Player".to_string(),  // TODO: Track actual scorer
                    minute,
                    is_rouge: false,
                });

                self.add_event(MatchEvent {
                    minute,
                    second: 0,
                    event_type: EventType::Goal,
                    description: "GOAL!".to_string(),
                    position: Position2D::new(0.85, 0.5),
                    players_involved: Vec::new(),
                    team_side: shooting_team,
                });
            } else {
                // Save
                self.add_event(MatchEvent {
                    minute,
                    second: 0,
                    event_type: EventType::Save,
                    description: "Saved by the goalkeeper!".to_string(),
                    position: Position2D::new(0.85, 0.5),
                    players_involved: Vec::new(),
                    team_side: shooting_team,
                });
            }
        } else if self.rouge_active {
            // A1: Positional rouge detection (Oct 1867 rules)
            let ball_y_variance = ((minute as f64 * 0.7 + 1.3).sin() * 0.12) as f32;
            let effective_y = (0.5_f32 + ball_y_variance).max(0.0).min(1.0);
            let dist_from_centre = (effective_y - 0.5).abs();
            let in_rouge_zone = dist_from_centre > 0.03 && dist_from_centre < 0.07;

            if in_rouge_zone {
                shot.is_rouge = true;

                match shooting_team {
                    TeamSide::Home => self.home_rouges += 1,
                    TeamSide::Away => self.away_rouges += 1,
                }

                self.scorers.push(ScorerInfo {
                    player_name: "Player".to_string(),
                    minute,
                    is_rouge: true,
                });

                self.add_event(MatchEvent {
                    minute,
                    second: 0,
                    event_type: EventType::Rouge,
                    description: "Rouge! Ball passes between the rouge flag and the goalpost".to_string(),
                    position: Position2D::new(0.95, effective_y),
                    players_involved: Vec::new(),
                    team_side: shooting_team,
                });
            }
        }
    }

    /// Add event to timeline
    fn add_event(&mut self, event: MatchEvent) {
        self.events.push(event);
    }

    /// Role-based target position for AI movement.
    /// `formation_pos` is the player's default formation position (home: attacks x→1, away: attacks x→0).
    /// Returns the desired position this frame; caller moves player toward it.
    fn player_target(
        role: &str,
        formation_pos: &Position2D,
        ball_pos: &Position2D,
        possession: TeamSide,
        this_team: TeamSide,
    ) -> Position2D {
        let we_have_ball = possession == this_team;
        // For away team, invert x so logic is always "our goal at low-x, their goal at high-x"
        // then re-invert at the end.
        let (fp_x, ball_x) = if this_team == TeamSide::Away {
            (1.0 - formation_pos.x, 1.0 - ball_pos.x)
        } else {
            (formation_pos.x, ball_pos.x)
        };
        let fp_y = formation_pos.y;
        let ball_y = ball_pos.y;

        let (tx, ty) = match role {
            "GK" => {
                // Stay on goal line (x≈0.05 home), track ball y within ±0.12 of centre
                let gy = (ball_y - 0.5).max(-0.12).min(0.12) + 0.5;
                (0.05_f32, gy)
            }
            "FB" | "CB" => {
                if we_have_ball {
                    // Push up to x=0.35, track ball y ±0.20
                    let ty = (ball_y - 0.5).max(-0.20).min(0.20) + 0.5;
                    (0.35_f32.max(fp_x), ty)
                } else {
                    // Drop back to x=0.20, compress toward centre
                    let ty = fp_y * 0.5 + 0.5 * 0.5;
                    (0.20_f32.min(fp_x), ty)
                }
            }
            "MID" => {
                if we_have_ball {
                    // Track ball x 60%/formation 40%, track ball y ±0.15
                    let tx = ball_x * 0.6 + fp_x * 0.4;
                    let ty = (ball_y - 0.5).max(-0.15).min(0.15) + 0.5;
                    (tx, ty)
                } else {
                    // Drop to own half, hold formation y
                    let tx = ball_x * 0.4 + fp_x * 0.6;
                    (tx.min(0.55), fp_y)
                }
            }
            "FWD" | "WG" => {
                if we_have_ball {
                    // Push to x=0.75; wingers spread wide
                    let tx = 0.75_f32.max(fp_x);
                    let ty = fp_y; // maintain width
                    (tx, ty)
                } else {
                    // Drop to x=0.5, maintain width
                    (0.50_f32, fp_y)
                }
            }
            _ => (fp_x, fp_y),
        };

        // Re-invert for away team
        let out_x = if this_team == TeamSide::Away { 1.0 - tx } else { tx };
        Position2D::new(out_x.max(0.02).min(0.98), ty.max(0.05).min(0.95))
    }

    /// Capture current visual state for 2D replay
    fn capture_visual_state(&mut self) {
        // Get ball position from possession engine zone, with Y-axis variance per minute
        let minute = self.current_minute;
        let possession = self.possession_engine.ball_state.possession_team;
        let mut ball_position = self.zone_to_position(self.possession_engine.ball_state.current_zone);
        let ball_y_variance = ((minute as f64 * 0.7 + 1.3).sin() * 0.12) as f32;
        ball_position.y = (ball_position.y + ball_y_variance).max(0.1).min(0.9);

        // Ball z-height arc: advance t → compute z = sin(π·t) → reset at 1
        self.ball_arc_t = (self.ball_arc_t + 0.2).min(1.0);
        let ball_z = (std::f32::consts::PI * self.ball_arc_t).sin();

        const PLAYER_SPEED: f32 = 0.018; // normalised units per capture

        let mut player_positions: Vec<PlayerPositionState> = Vec::new();

        // Home team players — move toward role-based targets
        for (i, formation_pos) in self.home_formation.positions.iter().enumerate() {
            if self.home_players.get(i).is_some() {
                let target = Self::player_target(
                    &formation_pos.role,
                    &formation_pos.position,
                    &ball_position,
                    possession,
                    TeamSide::Home,
                );
                // Incrementally move current position toward target
                let cur = self.home_player_pos.get_mut(i).unwrap();
                let dx = target.x - cur.x;
                let dy = target.y - cur.y;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist > PLAYER_SPEED {
                    cur.x += dx / dist * PLAYER_SPEED;
                    cur.y += dy / dist * PLAYER_SPEED;
                } else {
                    cur.x = target.x;
                    cur.y = target.y;
                }
                cur.x = cur.x.max(0.02).min(0.98);
                cur.y = cur.y.max(0.05).min(0.95);

                let pos = Position2D::new(cur.x, cur.y);
                player_positions.push(PlayerPositionState {
                    player_id: format!("home_{}", i),
                    name: format!("Player {}", i + 1),
                    team: TeamSide::Home,
                    position: pos,
                    has_ball: false,
                });
            }
        }

        // Away team players — move toward role-based targets
        let away_offset = player_positions.len();
        for (i, formation_pos) in self.away_formation.positions.iter().enumerate() {
            if self.away_players.get(i).is_some() {
                let target = Self::player_target(
                    &formation_pos.role,
                    &formation_pos.position,
                    &ball_position,
                    possession,
                    TeamSide::Away,
                );
                let cur = self.away_player_pos.get_mut(i).unwrap();
                let dx = target.x - cur.x;
                let dy = target.y - cur.y;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist > PLAYER_SPEED {
                    cur.x += dx / dist * PLAYER_SPEED;
                    cur.y += dy / dist * PLAYER_SPEED;
                } else {
                    cur.x = target.x;
                    cur.y = target.y;
                }
                cur.x = cur.x.max(0.02).min(0.98);
                cur.y = cur.y.max(0.05).min(0.95);

                let pos = Position2D::new(cur.x, cur.y);
                player_positions.push(PlayerPositionState {
                    player_id: format!("away_{}", i),
                    name: format!("Player {}", i + 1),
                    team: TeamSide::Away,
                    position: pos,
                    has_ball: false,
                });
            }
        }

        // Assign has_ball = true to the single closest player on the possessing team
        let possessing_range = match possession {
            TeamSide::Home => 0..away_offset,
            TeamSide::Away => away_offset..player_positions.len(),
        };
        let mut closest_idx: Option<usize> = None;
        let mut closest_dist = f32::MAX;
        for idx in possessing_range {
            let d = player_positions[idx].position.distance_to(&ball_position);
            if d < closest_dist {
                closest_dist = d;
                closest_idx = Some(idx);
            }
        }
        if let Some(idx) = closest_idx {
            player_positions[idx].has_ball = true;
        }

        self.visual_states.push(VisualState {
            minute,
            ball_position,
            ball_z,
            player_positions,
            possession_team: possession,
        });
    }

    /// Convert field zone to approximate position
    fn zone_to_position(&self, zone: FieldZone) -> Position2D {
        match zone {
            FieldZone::DefensiveThird => Position2D::new(0.25, 0.5),
            FieldZone::MiddleThird => Position2D::new(0.50, 0.5),
            FieldZone::AttackingThird => Position2D::new(0.75, 0.5),
        }
    }

    /// Convert field zone to position accounting for team direction.
    /// Home attacks toward x=1.0, away attacks toward x=0.0.
    fn zone_to_position_for_team(&self, zone: FieldZone, team: TeamSide) -> Position2D {
        // Add small y variance per-minute to prevent ball always being on centre line
        let y_offset = ((self.current_minute as f32 * 0.37 + 0.5).sin() * 0.18).abs() - 0.09;
        let y = (0.5 + y_offset).max(0.15).min(0.85);
        let x = match (zone, team) {
            (FieldZone::DefensiveThird, TeamSide::Home) => 0.20,
            (FieldZone::MiddleThird,    TeamSide::Home) => 0.50,
            (FieldZone::AttackingThird, TeamSide::Home) => 0.78,
            (FieldZone::DefensiveThird, TeamSide::Away) => 0.80,
            (FieldZone::MiddleThird,    TeamSide::Away) => 0.50,
            (FieldZone::AttackingThird, TeamSide::Away) => 0.22,
        };
        Position2D::new(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_player() -> PlayerAttributes {
        PlayerAttributes {
            pace: 15, acceleration: 14, strength: 12, stamina: 16,
            balance: 13, jumping: 11, agility: 14, natural_fitness: 15,
            passing: 16, dribbling: 14, first_touch: 15, technique: 14,
            heading: 12, long_passing: 13, crossing: 11, long_shots: 10,
            tackling: 13, handling: 5, reflexes: 5, corners: 10,
            free_kicks: 11, throw_ins: 10, vision: 14, left_foot: 12,
            right_foot: 16, one_on_ones: 12, courage: 15, bravery: 14,
            concentration: 15, decision_making: 16, leadership: 12,
            aggression: 11, anticipation: 14, determination: 15,
            flair: 13, influence: 11, adaptability: 12, ambition: 14,
            loyalty: 13, pressure: 8, professionalism: 15,
            sportsmanship: 16, temperament: 14, awareness: 15,
            marking: 12, positioning: 14, work_rate: 16,
            off_the_ball: 13, movement: 14, teamwork: 15,
            finishing: 14, penalties: 12, set_pieces: 11,
            consistency: 13, important_matches: 14, injury_proneness: 6,
            versatility: 11, dirtiness: 5,
        }
    }

    #[test]
    fn test_match_simulator_creation() {
        let home_players = vec![create_test_player(); 11];
        let away_players = vec![create_test_player(); 11];

        let simulator = MatchSimulator::new(
            "test_match_1".to_string(),
            home_players,
            away_players,
            1867,  // Sheffield FA founded, rouge active
            12345,
        );

        assert_eq!(simulator.match_id, "test_match_1");
        assert_eq!(simulator.rouge_active, true);  // 1867 is within rouge era
        assert_eq!(simulator.current_minute, 0);
    }

    #[test]
    fn test_weather_multiplier() {
        let clear = MatchSimulator::calculate_weather_multiplier("clear", "perfect");
        let rainy = MatchSimulator::calculate_weather_multiplier("rainy", "poor");

        assert_eq!(clear, 1.0);
        assert!(rainy < clear);
        assert!(rainy >= 0.7);
    }
}
