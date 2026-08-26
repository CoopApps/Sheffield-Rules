/// Sheffield 1867 Match Simulator
/// Authentic match simulation using Sheffield Rules (1857-1877)
/// Features rouge scoring, Victorian-era match flow, and live commentary streaming

use serde::{Deserialize, Serialize};
use rand::{Rng, SeedableRng};
use std::collections::HashMap;

use super::player_performance::PlayerPerformance;
use super::match_state::MatchStateSnapshot;
use super::commentary::{CommentaryLibrary, CommentaryContext, MatchCommentary};
use crate::match_engine::formation::Formation;

/// Real player data from database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealPlayerData {
    pub id: String,
    pub name: String,
    pub position: Option<String>,
    pub age: i32,
    // Physical attributes (0-20 scale)
    pub pace: Option<i32>,
    pub acceleration: Option<i32>,
    pub strength: Option<i32>,
    pub stamina: Option<i32>,
    pub agility: Option<i32>,
    // Technical attributes (0-20 scale)
    pub passing: Option<i32>,
    pub dribbling: Option<i32>,
    pub first_touch: Option<i32>,
    pub heading: Option<i32>,
    pub finishing: Option<i32>,
    pub tackling: Option<i32>,
    // Mental attributes (0-20 scale)
    pub composure: Option<i32>,
    pub vision: Option<i32>,
    pub decisions: Option<i32>,
    pub positioning: Option<i32>,
    pub teamwork: Option<i32>,
    pub work_rate: Option<i32>,
}

/// Main match simulator for Sheffield Rules era
pub struct SheffieldMatchSimulator {
    home_team_name: String,
    away_team_name: String,
    home_players: Vec<PlayerPerformance>,
    away_players: Vec<PlayerPerformance>,
    home_team_strength: f32,
    away_team_strength: f32,
    home_formation: Formation,  // Player-selected formation
    away_formation: Formation,  // Player-selected formation
    weather_condition: WeatherCondition,
    pitch_condition: PitchCondition,
    commentary_library: CommentaryLibrary,
    rng: rand::rngs::StdRng,
    injuries: Vec<InjuryInfo>,  // Track all injuries during match
}

/// Weather conditions affecting match play
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WeatherCondition {
    Clear,
    LightRain,
    HeavyRain,
    Wind,
    Fog,
}

/// Pitch conditions in Victorian era
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PitchCondition {
    Firm,
    Soft,
    Muddy,
    Waterlogged,
}

/// Types of match events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventType {
    KickOff,
    Pass,
    Tackle,
    Shot,
    Goal,
    Rouge,
    Touchdown,
    ThrowIn,
    Foul,
    YellowCard,
    RedCard,
    Injury,
    InjuryMinor,
    InjuryMajor,
    WeatherChange,
    CrowdReaction,
    HalfTime,
    FullTime,
}

/// Injury severity and duration (Sheffield Rules era: no substitutions!)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjuryInfo {
    pub player_name: String,
    pub severity: InjurySeverity,
    pub minute: i32,
    pub description: String,
    pub can_continue: bool,  // If false, player must leave field (no subs in 1867!)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InjurySeverity {
    Minor,      // Can continue with reduced effectiveness
    Moderate,   // Significant impact, might leave field
    Severe,     // Must leave field, team plays with 10 men
}

/// Individual match event with minute and details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchEvent {
    pub minute: i32,
    pub event_type: EventType,
    pub description: String,
    pub home_player: Option<String>,
    pub away_player: Option<String>,
    pub home_score: i32,
    pub away_score: i32,
    pub home_rouges: i32,
    pub away_rouges: i32,
}

/// Complete match result with all events and statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheffieldMatchResult {
    pub home_team: String,
    pub away_team: String,
    pub home_goals: i32,
    pub away_goals: i32,
    pub home_rouges: i32,
    pub away_rouges: i32,
    pub match_winner: MatchWinner,
    pub events: Vec<MatchEvent>,
    pub commentary: Vec<MatchCommentary>,
    pub home_player_stats: Vec<PlayerPerformance>,
    pub away_player_stats: Vec<PlayerPerformance>,
    pub total_shots_home: i32,
    pub total_shots_away: i32,
    pub possession_home: f32,
    pub possession_away: f32,
    pub injuries: Vec<InjuryInfo>,
    pub weather: WeatherCondition,
    pub pitch: PitchCondition,
    pub home_formation: Formation,
    pub away_formation: Formation,
}

/// Match result determination per Sheffield Rules
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MatchWinner {
    HomeTeam,
    AwayTeam,
    HomeTeamOnRouges,
    AwayTeamOnRouges,
    Draw,
}

impl SheffieldMatchSimulator {
    /// Create new match simulator with team details
    pub fn new(
        home_team_name: String,
        away_team_name: String,
        home_team_strength: f32,
        away_team_strength: f32,
    ) -> Self {
        Self::new_with_formations(
            home_team_name,
            away_team_name,
            home_team_strength,
            away_team_strength,
            None,
            None,
        )
    }

    /// Create new match simulator with optional formation selection
    pub fn new_with_formations(
        home_team_name: String,
        away_team_name: String,
        home_team_strength: f32,
        away_team_strength: f32,
        home_formation_code: Option<String>,
        away_formation_code: Option<String>,
    ) -> Self {
        let home_players = Self::generate_team_players(&home_team_name, home_team_strength);
        let away_players = Self::generate_team_players(&away_team_name, away_team_strength);

        // Get formations - default to era-appropriate formation if not specified
        let home_formation = Self::get_formation_from_code(home_formation_code, 1867);
        let away_formation = Self::get_formation_from_code(away_formation_code, 1867);

        SheffieldMatchSimulator {
            home_team_name,
            away_team_name,
            home_players,
            away_players,
            home_team_strength,
            away_team_strength,
            home_formation,
            away_formation,
            weather_condition: WeatherCondition::Clear,
            pitch_condition: PitchCondition::Firm,
            commentary_library: CommentaryLibrary::new(),
            rng: rand::rngs::StdRng::from_entropy(),
            injuries: Vec::new(),
        }
    }

    /// Create new match simulator with real player data from database
    /// This constructor uses actual player attributes instead of generic team strength
    pub fn new_with_squads(
        home_team_name: String,
        away_team_name: String,
        home_squad: Vec<RealPlayerData>,
        away_squad: Vec<RealPlayerData>,
    ) -> Self {
        Self::new_with_squads_and_formations(
            home_team_name,
            away_team_name,
            home_squad,
            away_squad,
            None,
            None,
        )
    }

    /// Create new match simulator with real player data and optional formation selection
    pub fn new_with_squads_and_formations(
        home_team_name: String,
        away_team_name: String,
        home_squad: Vec<RealPlayerData>,
        away_squad: Vec<RealPlayerData>,
        home_formation_code: Option<String>,
        away_formation_code: Option<String>,
    ) -> Self {
        let home_players = Self::convert_squad_to_performance(home_squad);
        let away_players = Self::convert_squad_to_performance(away_squad);

        // Calculate average team strengths from player attributes
        let home_team_strength = Self::calculate_team_strength(&home_players);
        let away_team_strength = Self::calculate_team_strength(&away_players);

        // Get formations - default to era-appropriate formation if not specified
        let home_formation = Self::get_formation_from_code(home_formation_code, 1867);
        let away_formation = Self::get_formation_from_code(away_formation_code, 1867);

        SheffieldMatchSimulator {
            home_team_name,
            away_team_name,
            home_players,
            away_players,
            home_team_strength,
            away_team_strength,
            home_formation,
            away_formation,
            weather_condition: WeatherCondition::Clear,
            pitch_condition: PitchCondition::Firm,
            commentary_library: CommentaryLibrary::new(),
            rng: rand::rngs::StdRng::from_entropy(),
            injuries: Vec::new(),
        }
    }

    /// Generate players for a team with Victorian-era positions
    fn generate_team_players(team_name: &str, team_strength: f32) -> Vec<PlayerPerformance> {
        let positions = vec![
            "Goalkeeper",
            "Back",
            "Back",
            "Half-Back",
            "Half-Back",
            "Half-Back",
            "Forward",
            "Forward",
            "Forward",
            "Forward",
            "Forward",
        ];

        positions
            .iter()
            .enumerate()
            .map(|(idx, pos)| {
                let mut player = PlayerPerformance::new(
                    idx as i32 + 1,
                    format!("{} Player {}", team_name, idx + 1),
                    pos.to_string(),
                );
                player.current_form = team_strength;
                player.attacking_rating = team_strength;
                player.defensive_rating = team_strength;
                player.passing_accuracy = team_strength;
                player
            })
            .collect()
    }

    /// Convert real player data from database to PlayerPerformance
    fn convert_squad_to_performance(squad: Vec<RealPlayerData>) -> Vec<PlayerPerformance> {
        squad.into_iter().enumerate().map(|(idx, player)| {
            let mut perf = PlayerPerformance::new(
                idx as i32 + 1,
                player.name.clone(),
                player.position.unwrap_or_else(|| "Forward".to_string()),
            );

            // Convert 0-20 scale to 0.0-1.0 scale
            let normalize = |val: Option<i32>| -> f32 {
                val.unwrap_or(10) as f32 / 20.0
            };

            // Calculate attacking rating from finishing, dribbling, pace
            let finishing = normalize(player.finishing);
            let dribbling = normalize(player.dribbling);
            let pace = normalize(player.pace);
            perf.attacking_rating = (finishing + dribbling + pace) / 3.0;

            // Calculate defensive rating from tackling, positioning, strength
            let tackling = normalize(player.tackling);
            let positioning = normalize(player.positioning);
            let strength = normalize(player.strength);
            perf.defensive_rating = (tackling + positioning + strength) / 3.0;

            // Calculate passing accuracy from passing, vision, first_touch
            let passing = normalize(player.passing);
            let vision = normalize(player.vision);
            let first_touch = normalize(player.first_touch);
            perf.passing_accuracy = (passing + vision + first_touch) / 3.0;

            // Set initial form based on composure and work_rate
            let composure = normalize(player.composure);
            let work_rate = normalize(player.work_rate);
            perf.current_form = (composure + work_rate) / 2.0;

            // Set morale and fitness
            perf.morale = 0.65; // Start with decent morale
            perf.fitness_level = normalize(player.stamina);

            perf
        }).collect()
    }

    /// Calculate average team strength from player performance data
    fn calculate_team_strength(players: &[PlayerPerformance]) -> f32 {
        if players.is_empty() {
            return 0.5;
        }

        let total_strength: f32 = players.iter().map(|p| {
            (p.attacking_rating + p.defensive_rating + p.passing_accuracy) / 3.0
        }).sum();

        total_strength / players.len() as f32
    }

    /// Get formation from code or default to year-appropriate formation
    fn get_formation_from_code(formation_code: Option<String>, year: i32) -> Formation {
        match formation_code {
            Some(code) => {
                match code.as_str() {
                    "1-1-8" => Formation::ultra_attacking_1_1_8(),
                    "1-2-7" => Formation::early_sheffield_1_2_7(),
                    "1-3-6" => Formation::transitional_1_3_6(),
                    "2-3-5" => Formation::pyramid_2_3_5(),
                    "2-2-6" => Formation::attacking_2_2_6(),
                    _ => Formation::for_year(year),  // Invalid code, use year default
                }
            }
            None => Formation::for_year(year),  // No code provided, use year default
        }
    }

    /// Set weather conditions
    pub fn set_weather(&mut self, weather: WeatherCondition) {
        self.weather_condition = weather;
    }

    /// Set pitch conditions
    pub fn set_pitch(&mut self, pitch: PitchCondition) {
        self.pitch_condition = pitch;
    }

    /// Simulate complete match with minute-by-minute events (fast mode)
    pub fn simulate_match_fast(&mut self) -> SheffieldMatchResult {
        let mut events = Vec::new();
        let mut commentary = Vec::new();
        let mut home_goals = 0;
        let mut away_goals = 0;
        let mut home_rouges = 0;
        let mut away_rouges = 0;
        let mut possession_home = 0.0;
        let mut possession_away = 0.0;

        // Kick-off
        events.push(MatchEvent {
            minute: 0,
            event_type: EventType::KickOff,
            description: format!("{} kicks off against {}", self.home_team_name, self.away_team_name),
            home_player: None,
            away_player: None,
            home_score: 0,
            away_score: 0,
            home_rouges: 0,
            away_rouges: 0,
        });

        // Simulate 90 minutes
        for minute in 1..=90 {
            // Update player fitness
            for player in &mut self.home_players {
                player.update_fitness(minute);
            }
            for player in &mut self.away_players {
                player.update_fitness(minute);
            }

            // Determine possession for this minute
            let possession_roll = self.rng.gen::<f32>();
            let home_has_possession = possession_roll < self.calculate_possession_probability();

            if home_has_possession {
                possession_home += 1.0;
            } else {
                possession_away += 1.0;
            }

            // Generate events for this minute
            let minute_events = self.simulate_minute(
                minute,
                home_has_possession,
                &mut home_goals,
                &mut away_goals,
                &mut home_rouges,
                &mut away_rouges,
            );

            for event in minute_events {
                // Generate commentary for event
                if let Some(comment) = self.generate_commentary_for_event(&event) {
                    commentary.push(comment);
                }
                events.push(event);
            }

            // Half-time
            if minute == 45 {
                events.push(MatchEvent {
                    minute: 45,
                    event_type: EventType::HalfTime,
                    description: format!(
                        "Half-time: {} {} goals {} rouges - {} {} goals {} rouges",
                        self.home_team_name, home_goals, home_rouges,
                        self.away_team_name, away_goals, away_rouges
                    ),
                    home_player: None,
                    away_player: None,
                    home_score: home_goals,
                    away_score: away_goals,
                    home_rouges,
                    away_rouges,
                });
            }

            // Occasional weather commentary
            if minute % 15 == 0 && self.rng.gen::<f32>() < 0.3 {
                if let Some(weather_comment) = self.generate_weather_commentary(minute) {
                    commentary.push(weather_comment);
                }
            }
        }

        // Full-time
        events.push(MatchEvent {
            minute: 90,
            event_type: EventType::FullTime,
            description: format!(
                "Full-time: {} {} goals {} rouges - {} {} goals {} rouges",
                self.home_team_name, home_goals, home_rouges,
                self.away_team_name, away_goals, away_rouges
            ),
            home_player: None,
            away_player: None,
            home_score: home_goals,
            away_score: away_goals,
            home_rouges,
            away_rouges,
        });

        // Calculate total shots
        let total_shots_home: i32 = self.home_players.iter().map(|p| p.shots_attempted).sum();
        let total_shots_away: i32 = self.away_players.iter().map(|p| p.shots_attempted).sum();

        // Normalize possession
        let total_possession = possession_home + possession_away;
        possession_home = possession_home / total_possession;
        possession_away = possession_away / total_possession;

        // Determine winner per Sheffield Rules
        let match_winner = Self::determine_winner(home_goals, away_goals, home_rouges, away_rouges);

        SheffieldMatchResult {
            home_team: self.home_team_name.clone(),
            away_team: self.away_team_name.clone(),
            home_goals,
            away_goals,
            home_rouges,
            away_rouges,
            match_winner,
            events,
            commentary,
            home_player_stats: self.home_players.clone(),
            away_player_stats: self.away_players.clone(),
            total_shots_home,
            total_shots_away,
            possession_home,
            possession_away,
            injuries: self.injuries.clone(),
            weather: self.weather_condition,
            pitch: self.pitch_condition,
            home_formation: self.home_formation.clone(),
            away_formation: self.away_formation.clone(),
        }
    }

    /// Simulate match with streaming events (for live display)
    pub fn simulate_match_live(&mut self) -> Vec<MatchEvent> {
        // For now, return same as fast mode
        // In future, this could yield events one-by-one via async streams
        self.simulate_match_fast().events
    }

    /// Simulate a single minute of play
    fn simulate_minute(
        &mut self,
        minute: i32,
        home_has_possession: bool,
        home_goals: &mut i32,
        away_goals: &mut i32,
        home_rouges: &mut i32,
        away_rouges: &mut i32,
    ) -> Vec<MatchEvent> {
        let mut events = Vec::new();

        // Random injury check (very rare, ~1% chance per minute)
        if self.rng.gen::<f32>() < 0.01 {
            if let Some(injury_event) = self.simulate_injury(minute, home_has_possession, *home_goals, *away_goals, *home_rouges, *away_rouges) {
                events.push(injury_event);
            }
        }

        // Event probability varies by minute
        let event_probability = self.calculate_event_probability(minute);

        if self.rng.gen::<f32>() > event_probability {
            return events; // Quiet minute
        }

        // Determine what type of event occurs
        let event_roll = self.rng.gen::<f32>();

        if event_roll < 0.60 {
            // Pass attempt (60%)
            if let Some(event) = self.simulate_pass(minute, home_has_possession, *home_goals, *away_goals, *home_rouges, *away_rouges) {
                events.push(event);
            }
        } else if event_roll < 0.75 {
            // Tackle (15%)
            if let Some(event) = self.simulate_tackle(minute, home_has_possession, *home_goals, *away_goals, *home_rouges, *away_rouges) {
                events.push(event);
            }
        } else if event_roll < 0.90 {
            // Shot attempt (15%)
            if let Some(event) = self.simulate_shot(
                minute,
                home_has_possession,
                home_goals,
                away_goals,
                home_rouges,
                away_rouges,
            ) {
                events.push(event);
            }
        } else {
            // Throw-in or other (10%)
            if let Some(event) = self.simulate_throwin(minute, home_has_possession, *home_goals, *away_goals, *home_rouges, *away_rouges) {
                events.push(event);
            }
        }

        events
    }

    /// Simulate a pass
    fn simulate_pass(
        &mut self,
        minute: i32,
        home_has_possession: bool,
        home_goals: i32,
        away_goals: i32,
        home_rouges: i32,
        away_rouges: i32,
    ) -> Option<MatchEvent> {
        // Get all data before creating mutable borrows
        let (passer_idx, passer_name, receiver_name) = if home_has_possession {
            let passer_idx = self.rng.gen_range(0..self.home_players.len());
            let receiver_idx = self.rng.gen_range(0..self.home_players.len());
            let receiver_name = self.home_players[receiver_idx].player_name.clone();
            let passer_name = self.home_players[passer_idx].player_name.clone();
            (passer_idx, passer_name, receiver_name)
        } else {
            let passer_idx = self.rng.gen_range(0..self.away_players.len());
            let receiver_idx = self.rng.gen_range(0..self.away_players.len());
            let receiver_name = self.away_players[receiver_idx].player_name.clone();
            let passer_name = self.away_players[passer_idx].player_name.clone();
            (passer_idx, passer_name, receiver_name)
        };

        let weather_mod = self.get_weather_modifier();
        let pass_roll = self.rng.gen::<f32>();

        let passer = if home_has_possession {
            &mut self.home_players[passer_idx]
        } else {
            &mut self.away_players[passer_idx]
        };

        let pass_success = pass_roll < passer.passing_accuracy * weather_mod;
        passer.record_pass(pass_success);

        if pass_success {
            Some(MatchEvent {
                minute,
                event_type: EventType::Pass,
                description: format!("{} passes to {}", passer_name, receiver_name),
                home_player: if home_has_possession { Some(passer_name.clone()) } else { None },
                away_player: if !home_has_possession { Some(passer_name.clone()) } else { None },
                home_score: home_goals,
                away_score: away_goals,
                home_rouges,
                away_rouges,
            })
        } else {
            Some(MatchEvent {
                minute,
                event_type: EventType::Pass,
                description: format!("{} misplaces a pass", passer_name),
                home_player: if home_has_possession { Some(passer_name.clone()) } else { None },
                away_player: if !home_has_possession { Some(passer_name.clone()) } else { None },
                home_score: home_goals,
                away_score: away_goals,
                home_rouges,
                away_rouges,
            })
        }
    }

    /// Simulate a tackle
    fn simulate_tackle(
        &mut self,
        minute: i32,
        home_has_possession: bool,
        home_goals: i32,
        away_goals: i32,
        home_rouges: i32,
        away_rouges: i32,
    ) -> Option<MatchEvent> {
        let (idx, team_name) = if !home_has_possession {
            let idx = self.rng.gen_range(0..self.home_players.len());
            (idx, &self.home_team_name)
        } else {
            let idx = self.rng.gen_range(0..self.away_players.len());
            (idx, &self.away_team_name)
        };

        // Generate random number before borrowing
        let tackle_roll = self.rng.gen::<f32>();

        let tackler = if !home_has_possession {
            &mut self.home_players[idx]
        } else {
            &mut self.away_players[idx]
        };

        let tackle_success = tackle_roll < tackler.defensive_rating;
        tackler.record_tackle(tackle_success);

        Some(MatchEvent {
            minute,
            event_type: EventType::Tackle,
            description: if tackle_success {
                format!("{} wins the ball with a robust challenge", tackler.player_name)
            } else {
                format!("{} attempts a challenge", tackler.player_name)
            },
            home_player: if !home_has_possession { Some(tackler.player_name.clone()) } else { None },
            away_player: if home_has_possession { Some(tackler.player_name.clone()) } else { None },
            home_score: home_goals,
            away_score: away_goals,
            home_rouges,
            away_rouges,
        })
    }

    /// Simulate a shot (can result in goal or rouge per Sheffield Rules)
    fn simulate_shot(
        &mut self,
        minute: i32,
        home_has_possession: bool,
        home_goals: &mut i32,
        away_goals: &mut i32,
        home_rouges: &mut i32,
        away_rouges: &mut i32,
    ) -> Option<MatchEvent> {
        let shooter_idx = if home_has_possession {
            self.rng.gen_range(0..self.home_players.len())
        } else {
            self.rng.gen_range(0..self.away_players.len())
        };

        // Get weather modifier before borrowing shooter
        let weather_mod = self.get_weather_modifier();
        let shot_roll = self.rng.gen::<f32>();

        let shooter = if home_has_possession {
            &mut self.home_players[shooter_idx]
        } else {
            &mut self.away_players[shooter_idx]
        };

        let shooter_name = shooter.player_name.clone();

        // Shot success probability
        let shot_quality = shooter.attacking_rating * weather_mod;

        if shot_roll < shot_quality * 0.15 {
            // GOAL! (above the crossbar per Sheffield Rules)
            shooter.record_shot(true);
            if home_has_possession {
                *home_goals += 1;
            } else {
                *away_goals += 1;
            }

            Some(MatchEvent {
                minute,
                event_type: EventType::Goal,
                description: format!(
                    "GOAL! {} scores! The ball passes between the uprights above the crossbar!",
                    shooter_name
                ),
                home_player: if home_has_possession { Some(shooter_name.clone()) } else { None },
                away_player: if !home_has_possession { Some(shooter_name.clone()) } else { None },
                home_score: *home_goals,
                away_score: *away_goals,
                home_rouges: *home_rouges,
                away_rouges: *away_rouges,
            })
        } else if shot_roll < shot_quality * 0.35 {
            // ROUGE! (below the crossbar, touched down per Sheffield Rules 1862-1868)
            shooter.record_rouge();
            if home_has_possession {
                *home_rouges += 1;
            } else {
                *away_rouges += 1;
            }

            Some(MatchEvent {
                minute,
                event_type: EventType::Rouge,
                description: format!(
                    "Rouge! {} kicks through - the ball passes below the crossbar and is touched down behind the goal!",
                    shooter_name
                ),
                home_player: if home_has_possession { Some(shooter_name.clone()) } else { None },
                away_player: if !home_has_possession { Some(shooter_name.clone()) } else { None },
                home_score: *home_goals,
                away_score: *away_goals,
                home_rouges: *home_rouges,
                away_rouges: *away_rouges,
            })
        } else {
            // Shot missed
            shooter.record_shot(false);
            Some(MatchEvent {
                minute,
                event_type: EventType::Shot,
                description: format!("{} attempts a shot but it goes wide", shooter_name),
                home_player: if home_has_possession { Some(shooter_name.clone()) } else { None },
                away_player: if !home_has_possession { Some(shooter_name.clone()) } else { None },
                home_score: *home_goals,
                away_score: *away_goals,
                home_rouges: *home_rouges,
                away_rouges: *away_rouges,
            })
        }
    }

    /// Simulate a throw-in (Sheffield Rules: right-angle throws)
    fn simulate_throwin(
        &mut self,
        minute: i32,
        home_has_possession: bool,
        home_goals: i32,
        away_goals: i32,
        home_rouges: i32,
        away_rouges: i32,
    ) -> Option<MatchEvent> {
        let team_name = if home_has_possession {
            &self.home_team_name
        } else {
            &self.away_team_name
        };

        Some(MatchEvent {
            minute,
            event_type: EventType::ThrowIn,
            description: format!("The ball goes into touch. {} to throw in at right angles", team_name),
            home_player: None,
            away_player: None,
            home_score: home_goals,
            away_score: away_goals,
            home_rouges,
            away_rouges,
        })
    }

    /// Simulate an injury (Sheffield Rules era: no substitutions!)
    fn simulate_injury(
        &mut self,
        minute: i32,
        _home_has_possession: bool,
        home_goals: i32,
        away_goals: i32,
        home_rouges: i32,
        away_rouges: i32,
    ) -> Option<MatchEvent> {
        // Randomly select a team
        let is_home_team = self.rng.gen::<bool>();

        let players = if is_home_team {
            &mut self.home_players
        } else {
            &mut self.away_players
        };

        // Select random player who isn't already injured
        let available_players: Vec<usize> = players.iter()
            .enumerate()
            .filter(|(_, p)| !p.is_substituted_off)
            .map(|(i, _)| i)
            .collect();

        if available_players.is_empty() {
            return None;
        }

        let player_idx = available_players[self.rng.gen_range(0..available_players.len())];
        let player = &mut players[player_idx];

        // Determine injury severity based on player fitness and pitch conditions
        let pitch_danger = match self.pitch_condition {
            PitchCondition::Firm => 0.3,
            PitchCondition::Soft => 0.5,
            PitchCondition::Muddy => 0.7,
            PitchCondition::Waterlogged => 0.9,
        };

        let severity_roll = self.rng.gen::<f32>() * pitch_danger;
        let (severity, can_continue, description_suffix) = if severity_roll < 0.15 {
            // Minor injury - can continue with reduced effectiveness
            player.fitness_level *= 0.85;
            player.current_form *= 0.90;
            (InjurySeverity::Minor, true, "receives treatment on the field and bravely continues")
        } else if severity_roll < 0.30 {
            // Moderate injury - significant impact
            player.fitness_level *= 0.65;
            player.current_form *= 0.75;
            (InjurySeverity::Moderate, true, "is struggling but refuses to leave the field")
        } else {
            // Severe injury - must leave field (no subs in 1867!)
            player.is_substituted_off = true;
            player.fitness_level = 0.0;
            (InjurySeverity::Severe, false, "cannot continue! The team must play on with ten men")
        };

        let player_name = player.player_name.clone();
        let injury_descriptions = vec![
            "collision",
            "awkward fall",
            "twisted ankle",
            "pulled muscle",
            "heavy tackle",
            "clash of heads",
        ];

        let cause = injury_descriptions[self.rng.gen_range(0..injury_descriptions.len())];
        let description = format!("{} suffers a {} and {}", player_name, cause, description_suffix);

        // Record injury
        self.injuries.push(InjuryInfo {
            player_name: player_name.clone(),
            severity: severity.clone(),
            minute,
            description: description.clone(),
            can_continue,
        });

        Some(MatchEvent {
            minute,
            event_type: match severity {
                InjurySeverity::Minor => EventType::InjuryMinor,
                InjurySeverity::Moderate => EventType::Injury,
                InjurySeverity::Severe => EventType::InjuryMajor,
            },
            description,
            home_player: if is_home_team { Some(player_name.clone()) } else { None },
            away_player: if !is_home_team { Some(player_name) } else { None },
            home_score: home_goals,
            away_score: away_goals,
            home_rouges,
            away_rouges,
        })
    }

    /// Calculate possession probability for home team
    fn calculate_possession_probability(&self) -> f32 {
        let strength_diff = self.home_team_strength - self.away_team_strength;
        (0.5 + strength_diff * 0.3).clamp(0.2, 0.8)
    }

    /// Calculate event probability based on match minute
    fn calculate_event_probability(&self, minute: i32) -> f32 {
        match minute {
            1..=15 => 0.25,      // Opening period
            16..=30 => 0.30,     // Building play
            31..=45 => 0.35,     // End of first half
            46..=60 => 0.30,     // Second half start
            61..=75 => 0.35,     // Mid second half
            76..=90 => 0.40,     // Final push
            _ => 0.25,
        }
    }

    /// Get weather modifier for skill checks
    fn get_weather_modifier(&self) -> f32 {
        match self.weather_condition {
            WeatherCondition::Clear => 1.0,
            WeatherCondition::LightRain => 0.95,
            WeatherCondition::HeavyRain => 0.85,
            WeatherCondition::Wind => 0.90,
            WeatherCondition::Fog => 0.80,
        }
    }

    /// Get pitch modifier for skill checks
    fn get_pitch_modifier(&self) -> f32 {
        match self.pitch_condition {
            PitchCondition::Firm => 1.0,
            PitchCondition::Soft => 0.95,
            PitchCondition::Muddy => 0.85,
            PitchCondition::Waterlogged => 0.75,
        }
    }

    /// Generate commentary for an event
    fn generate_commentary_for_event(&mut self, event: &MatchEvent) -> Option<MatchCommentary> {
        let contexts = self.get_current_contexts(event);

        let event_type_str = match event.event_type {
            EventType::Pass => "pass",
            EventType::Tackle => "tackle",
            EventType::Shot => "shot",
            EventType::Goal => "goal",
            EventType::Rouge => "rouge",
            EventType::ThrowIn => "throwin",
            EventType::Touchdown => "touchdown",
            _ => return None,
        };

        let mut commentary_text = self.commentary_library
            .get_commentary(event_type_str, &contexts)?;

        // Replace placeholders
        if let Some(player_name) = &event.home_player {
            commentary_text = commentary_text.replace("{player}", player_name);
            commentary_text = commentary_text.replace("{team}", &self.home_team_name);
        } else if let Some(player_name) = &event.away_player {
            commentary_text = commentary_text.replace("{player}", player_name);
            commentary_text = commentary_text.replace("{team}", &self.away_team_name);
        }

        Some(MatchCommentary {
            minute: event.minute,
            text: commentary_text,
            event_type: event_type_str.to_string(),
        })
    }

    /// Generate weather commentary
    fn generate_weather_commentary(&mut self, minute: i32) -> Option<MatchCommentary> {
        let contexts = vec![self.get_weather_context()];
        let commentary_text = self.commentary_library.get_commentary("weather", &contexts)?;

        Some(MatchCommentary {
            minute,
            text: commentary_text,
            event_type: "weather".to_string(),
        })
    }

    /// Get current commentary contexts based on match state
    fn get_current_contexts(&self, event: &MatchEvent) -> Vec<CommentaryContext> {
        let mut contexts = Vec::new();

        // Score contexts
        if event.home_score > event.away_score {
            contexts.push(CommentaryContext::HomeWinning);
        } else if event.away_score > event.home_score {
            contexts.push(CommentaryContext::AwayWinning);
        } else if event.home_rouges > event.away_rouges {
            contexts.push(CommentaryContext::HomeLeadsOnRouges);
        } else if event.away_rouges > event.home_rouges {
            contexts.push(CommentaryContext::AwayLeadsOnRouges);
        } else {
            contexts.push(CommentaryContext::TiedScore);
        }

        // Event contexts
        match event.event_type {
            EventType::Goal => contexts.push(CommentaryContext::PlayerScored),
            EventType::Rouge => contexts.push(CommentaryContext::PlayerScoredRouge),
            _ => {}
        }

        // Time contexts
        match event.minute {
            1..=15 => contexts.push(CommentaryContext::EarlyGame),
            16..=75 => contexts.push(CommentaryContext::MidGame),
            76..=85 => contexts.push(CommentaryContext::LateGame),
            86..=90 => contexts.push(CommentaryContext::FinalMinutes),
            _ => {}
        }

        // Weather/pitch contexts
        contexts.push(self.get_weather_context());

        contexts
    }

    /// Get weather context
    fn get_weather_context(&self) -> CommentaryContext {
        match self.weather_condition {
            WeatherCondition::HeavyRain => CommentaryContext::HeavyRain,
            WeatherCondition::Wind => CommentaryContext::Wind,
            WeatherCondition::Fog => CommentaryContext::Fog,
            _ => match self.pitch_condition {
                PitchCondition::Muddy => CommentaryContext::MuddyPitch,
                PitchCondition::Waterlogged => CommentaryContext::WaterloggedPitch,
                _ => CommentaryContext::BalancedPlay,
            },
        }
    }

    /// Determine winner per Sheffield Rules (goals first, then rouges, then draw)
    fn determine_winner(
        home_goals: i32,
        away_goals: i32,
        home_rouges: i32,
        away_rouges: i32,
    ) -> MatchWinner {
        if home_goals > away_goals {
            MatchWinner::HomeTeam
        } else if away_goals > home_goals {
            MatchWinner::AwayTeam
        } else if home_rouges > away_rouges {
            MatchWinner::HomeTeamOnRouges
        } else if away_rouges > home_rouges {
            MatchWinner::AwayTeamOnRouges
        } else {
            MatchWinner::Draw
        }
    }

    /// Get current match state snapshot
    pub fn get_match_state(
        &self,
        minute: i32,
        home_goals: i32,
        away_goals: i32,
        home_rouges: i32,
        away_rouges: i32,
    ) -> MatchStateSnapshot {
        let home_fatigue = self.home_players.iter()
            .map(|p| 1.0 - p.fitness_level)
            .sum::<f32>() / self.home_players.len() as f32;

        let away_fatigue = self.away_players.iter()
            .map(|p| 1.0 - p.fitness_level)
            .sum::<f32>() / self.away_players.len() as f32;

        let home_morale = self.home_players.iter()
            .map(|p| p.morale)
            .sum::<f32>() / self.home_players.len() as f32;

        let away_morale = self.away_players.iter()
            .map(|p| p.morale)
            .sum::<f32>() / self.away_players.len() as f32;

        MatchStateSnapshot::from_game_state(
            minute,
            self.home_team_name.clone(),
            self.away_team_name.clone(),
            home_goals,
            away_goals,
            home_rouges,
            away_rouges,
            &self.home_players,
            &self.away_players,
            home_fatigue,
            away_fatigue,
            home_morale,
            away_morale,
            0.5, // home_intensity
            0.5, // away_intensity
            -1,  // last_score_minute
        )
    }
}

impl SheffieldMatchResult {
    /// Get match result description per Sheffield Rules
    pub fn get_result_description(&self) -> String {
        match &self.match_winner {
            MatchWinner::HomeTeam => {
                format!(
                    "{} defeats {} by {} goals to {}",
                    self.home_team,
                    self.away_team,
                    self.home_goals,
                    self.away_goals
                )
            }
            MatchWinner::AwayTeam => {
                format!(
                    "{} defeats {} by {} goals to {}",
                    self.away_team,
                    self.home_team,
                    self.away_goals,
                    self.home_goals
                )
            }
            MatchWinner::HomeTeamOnRouges => {
                format!(
                    "{} defeats {} on rouges ({} goals {} rouges to {} goals {} rouges)",
                    self.home_team,
                    self.away_team,
                    self.home_goals,
                    self.home_rouges,
                    self.away_goals,
                    self.away_rouges
                )
            }
            MatchWinner::AwayTeamOnRouges => {
                format!(
                    "{} defeats {} on rouges ({} goals {} rouges to {} goals {} rouges)",
                    self.away_team,
                    self.home_team,
                    self.away_goals,
                    self.away_rouges,
                    self.home_goals,
                    self.home_rouges
                )
            }
            MatchWinner::Draw => {
                format!(
                    "{} and {} draw {} goals {} rouges each",
                    self.home_team,
                    self.away_team,
                    self.home_goals,
                    self.home_rouges
                )
            }
        }
    }

    /// Get summary statistics
    pub fn get_match_summary(&self) -> String {
        format!(
            "Final Score: {} {} goals {} rouges - {} {} goals {} rouges\n\
             Shots: {} - {}\n\
             Possession: {:.1}% - {:.1}%\n\
             Result: {}",
            self.home_team,
            self.home_goals,
            self.home_rouges,
            self.away_team,
            self.away_goals,
            self.away_rouges,
            self.total_shots_home,
            self.total_shots_away,
            self.possession_home * 100.0,
            self.possession_away * 100.0,
            self.get_result_description()
        )
    }

    /// Get top performers from the match
    pub fn get_top_performers(&self, count: usize) -> Vec<&PlayerPerformance> {
        let mut all_players: Vec<_> = self.home_player_stats.iter()
            .chain(self.away_player_stats.iter())
            .collect();

        all_players.sort_by(|a, b| {
            b.get_current_rating().partial_cmp(&a.get_current_rating()).unwrap()
        });

        all_players.into_iter().take(count).collect()
    }

    /// Get goal scorers
    pub fn get_goal_scorers(&self) -> Vec<String> {
        let mut scorers = Vec::new();

        for player in &self.home_player_stats {
            if player.goals_scored > 0 {
                scorers.push(format!(
                    "{} ({}) - {} goal(s)",
                    player.player_name,
                    self.home_team,
                    player.goals_scored
                ));
            }
        }

        for player in &self.away_player_stats {
            if player.goals_scored > 0 {
                scorers.push(format!(
                    "{} ({}) - {} goal(s)",
                    player.player_name,
                    self.away_team,
                    player.goals_scored
                ));
            }
        }

        scorers
    }

    /// Get rouge scorers (Sheffield Rules)
    pub fn get_rouge_scorers(&self) -> Vec<String> {
        let mut scorers = Vec::new();

        for player in &self.home_player_stats {
            if player.rouges_scored > 0 {
                scorers.push(format!(
                    "{} ({}) - {} rouge(s)",
                    player.player_name,
                    self.home_team,
                    player.rouges_scored
                ));
            }
        }

        for player in &self.away_player_stats {
            if player.rouges_scored > 0 {
                scorers.push(format!(
                    "{} ({}) - {} rouge(s)",
                    player.player_name,
                    self.away_team,
                    player.rouges_scored
                ));
            }
        }

        scorers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_simulation() {
        let mut simulator = SheffieldMatchSimulator::new(
            "Sheffield FC".to_string(),
            "Hallam FC".to_string(),
            0.6,
            0.5,
        );

        let result = simulator.simulate_match_fast();

        assert_eq!(result.home_team, "Sheffield FC");
        assert_eq!(result.away_team, "Hallam FC");
        assert!(result.events.len() > 0);
        assert!(result.home_goals >= 0);
        assert!(result.away_goals >= 0);
        assert!(result.home_rouges >= 0);
        assert!(result.away_rouges >= 0);
    }

    #[test]
    fn test_sheffield_rules_scoring() {
        // Goals beat rouges
        assert_eq!(
            SheffieldMatchSimulator::determine_winner(2, 1, 0, 5),
            MatchWinner::HomeTeam
        );

        // Rouges decide if goals tied
        assert_eq!(
            SheffieldMatchSimulator::determine_winner(1, 1, 3, 1),
            MatchWinner::HomeTeamOnRouges
        );

        // Draw if both tied
        assert_eq!(
            SheffieldMatchSimulator::determine_winner(1, 1, 2, 2),
            MatchWinner::Draw
        );
    }

    #[test]
    fn test_weather_conditions() {
        let mut simulator = SheffieldMatchSimulator::new(
            "Sheffield FC".to_string(),
            "Hallam FC".to_string(),
            0.6,
            0.5,
        );

        simulator.set_weather(WeatherCondition::HeavyRain);
        assert_eq!(simulator.get_weather_modifier(), 0.85);

        simulator.set_weather(WeatherCondition::Clear);
        assert_eq!(simulator.get_weather_modifier(), 1.0);
    }
}
