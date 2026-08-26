/// Sheffield 1867 Commentary Library
/// Rich, historically accurate commentary for Sheffield Rules matches
/// Includes rouge scoring, touch-downs, and Victorian-era language

use serde::{Deserialize, Serialize};
use rand::Rng;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CommentaryContext {
    // Match state
    HomeWinning,
    AwayWinning,
    TiedScore,
    HomeLeadsOnRouges,
    AwayLeadsOnRouges,

    // Player state
    PlayerScored,
    PlayerScoredRouge,
    PlayerExcelling,
    PlayerStruggling,

    // Match progression
    EarlyGame,
    MidGame,
    LateGame,
    FinalMinutes,

    // Weather/Pitch
    MuddyPitch,
    WaterloggedPitch,
    HeavyRain,
    Wind,
    Fog,

    // Intensity
    HighIntensity,
    LowIntensity,
    BalancedPlay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentaryVariant {
    pub text: String,
    pub probability: f32,
    pub contexts: Vec<CommentaryContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchCommentary {
    pub minute: i32,
    pub text: String,
    pub event_type: String,
}

pub struct CommentaryLibrary {
    pub passing: Vec<CommentaryVariant>,
    pub tackling: Vec<CommentaryVariant>,
    pub shots: Vec<CommentaryVariant>,
    pub goals: Vec<CommentaryVariant>,
    pub rouges: Vec<CommentaryVariant>,  // Sheffield Rules specific
    pub touchdowns: Vec<CommentaryVariant>,  // Sheffield Rules specific
    pub throwin: Vec<CommentaryVariant>,  // Sheffield right-angle throw-ins
    pub weather: Vec<CommentaryVariant>,
    pub crowd: Vec<CommentaryVariant>,
}

impl CommentaryLibrary {
    pub fn new() -> Self {
        CommentaryLibrary {
            passing: Self::create_passing_commentary(),
            tackling: Self::create_tackle_commentary(),
            shots: Self::create_shot_commentary(),
            goals: Self::create_goal_commentary(),
            rouges: Self::create_rouge_commentary(),
            touchdowns: Self::create_touchdown_commentary(),
            throwin: Self::create_throwin_commentary(),
            weather: Self::create_weather_commentary(),
            crowd: Self::create_crowd_commentary(),
        }
    }

    fn create_passing_commentary() -> Vec<CommentaryVariant> {
        vec![
            CommentaryVariant {
                text: "{player} passes forward to {player2}".to_string(),
                probability: 0.15,
                contexts: vec![CommentaryContext::MidGame],
            },
            CommentaryVariant {
                text: "{player} finds {player2} with a neat pass".to_string(),
                probability: 0.12,
                contexts: vec![CommentaryContext::MidGame],
            },
            CommentaryVariant {
                text: "A capital pass from {player} to {player2}".to_string(),
                probability: 0.10,
                contexts: vec![CommentaryContext::PlayerExcelling],
            },
            CommentaryVariant {
                text: "{player} carefully judges the wind and finds {player2}".to_string(),
                probability: 0.08,
                contexts: vec![CommentaryContext::Wind],
            },
            CommentaryVariant {
                text: "Despite the mud, {player} manages to reach {player2}".to_string(),
                probability: 0.10,
                contexts: vec![CommentaryContext::MuddyPitch],
            },
            CommentaryVariant {
                text: "{player} struggles to find {player2} in the fog".to_string(),
                probability: 0.05,
                contexts: vec![CommentaryContext::Fog],
            },
        ]
    }

    fn create_tackle_commentary() -> Vec<CommentaryVariant> {
        vec![
            CommentaryVariant {
                text: "{player} makes a robust challenge".to_string(),
                probability: 0.15,
                contexts: vec![CommentaryContext::HighIntensity],
            },
            CommentaryVariant {
                text: "Excellent fair play from {player}".to_string(),
                probability: 0.10,
                contexts: vec![CommentaryContext::BalancedPlay],
            },
            CommentaryVariant {
                text: "{player} wins the ball cleanly".to_string(),
                probability: 0.12,
                contexts: vec![CommentaryContext::MidGame],
            },
        ]
    }

    fn create_shot_commentary() -> Vec<CommentaryVariant> {
        vec![
            CommentaryVariant {
                text: "{player} attempts a shot at goal!".to_string(),
                probability: 0.15,
                contexts: vec![CommentaryContext::MidGame],
            },
            CommentaryVariant {
                text: "A splendid effort from {player}!".to_string(),
                probability: 0.10,
                contexts: vec![CommentaryContext::PlayerExcelling],
            },
            CommentaryVariant {
                text: "{player} strikes toward the uprights!".to_string(),
                probability: 0.12,
                contexts: vec![CommentaryContext::LateGame],
            },
        ]
    }

    fn create_goal_commentary() -> Vec<CommentaryVariant> {
        vec![
            CommentaryVariant {
                text: "GOAL! {player} has scored! The ball passes between the uprights above the crossbar!".to_string(),
                probability: 0.20,
                contexts: vec![CommentaryContext::PlayerScored],
            },
            CommentaryVariant {
                text: "A magnificent goal from {player}! The crowd erupts!".to_string(),
                probability: 0.15,
                contexts: vec![CommentaryContext::PlayerScored, CommentaryContext::HighIntensity],
            },
            CommentaryVariant {
                text: "{player} scores! What a splendid effort!".to_string(),
                probability: 0.12,
                contexts: vec![CommentaryContext::PlayerScored],
            },
        ]
    }

    fn create_rouge_commentary() -> Vec<CommentaryVariant> {
        vec![
            CommentaryVariant {
                text: "Rouge! {player} has touched it down behind the goal! The ball passed below the crossbar!".to_string(),
                probability: 0.20,
                contexts: vec![CommentaryContext::PlayerScoredRouge],
            },
            CommentaryVariant {
                text: "A rouge for {team}! {player} touches down successfully!".to_string(),
                probability: 0.15,
                contexts: vec![CommentaryContext::PlayerScoredRouge],
            },
            CommentaryVariant {
                text: "{player} kicks through - it's a rouge! Touched down past the goal line!".to_string(),
                probability: 0.12,
                contexts: vec![CommentaryContext::PlayerScoredRouge],
            },
            CommentaryVariant {
                text: "The ball passes beneath the bar - {player} rushes back to touch it down for a rouge!".to_string(),
                probability: 0.10,
                contexts: vec![CommentaryContext::PlayerScoredRouge],
            },
        ]
    }

    fn create_touchdown_commentary() -> Vec<CommentaryVariant> {
        vec![
            CommentaryVariant {
                text: "{player} touches down the ball behind the goal line".to_string(),
                probability: 0.15,
                contexts: vec![CommentaryContext::MidGame],
            },
            CommentaryVariant {
                text: "Successfully touched down by {player}".to_string(),
                probability: 0.10,
                contexts: vec![CommentaryContext::MidGame],
            },
        ]
    }

    fn create_throwin_commentary() -> Vec<CommentaryVariant> {
        vec![
            CommentaryVariant {
                text: "The ball goes into touch. {team} to throw in at right angles".to_string(),
                probability: 0.20,
                contexts: vec![CommentaryContext::MidGame],
            },
            CommentaryVariant {
                text: "Touch! {player} will throw in for {team}".to_string(),
                probability: 0.15,
                contexts: vec![CommentaryContext::MidGame],
            },
            CommentaryVariant {
                text: "Out of play - {team} with the right-angle throw".to_string(),
                probability: 0.10,
                contexts: vec![CommentaryContext::MidGame],
            },
        ]
    }

    fn create_weather_commentary() -> Vec<CommentaryVariant> {
        vec![
            CommentaryVariant {
                text: "The pitch is becoming increasingly heavy with mud".to_string(),
                probability: 0.10,
                contexts: vec![CommentaryContext::MuddyPitch],
            },
            CommentaryVariant {
                text: "The heavy rain makes ball control most difficult".to_string(),
                probability: 0.10,
                contexts: vec![CommentaryContext::HeavyRain],
            },
            CommentaryVariant {
                text: "The wind is proving troublesome for both sides".to_string(),
                probability: 0.08,
                contexts: vec![CommentaryContext::Wind],
            },
            CommentaryVariant {
                text: "Visibility is poor in this fog".to_string(),
                probability: 0.05,
                contexts: vec![CommentaryContext::Fog],
            },
        ]
    }

    fn create_crowd_commentary() -> Vec<CommentaryVariant> {
        vec![
            CommentaryVariant {
                text: "The spectators are in fine voice!".to_string(),
                probability: 0.10,
                contexts: vec![CommentaryContext::HomeWinning],
            },
            CommentaryVariant {
                text: "Tremendous enthusiasm from the crowd".to_string(),
                probability: 0.08,
                contexts: vec![CommentaryContext::HighIntensity],
            },
            CommentaryVariant {
                text: "The gathered gentlemen watch intently".to_string(),
                probability: 0.05,
                contexts: vec![CommentaryContext::LowIntensity],
            },
        ]
    }

    pub fn get_commentary(&self, event_type: &str, contexts: &[CommentaryContext]) -> Option<String> {
        let library = match event_type {
            "pass" => &self.passing,
            "tackle" => &self.tackling,
            "shot" => &self.shots,
            "goal" => &self.goals,
            "rouge" => &self.rouges,
            "touchdown" => &self.touchdowns,
            "throwin" => &self.throwin,
            "weather" => &self.weather,
            "crowd" => &self.crowd,
            _ => return None,
        };

        // Filter variants that match current contexts
        let matching: Vec<_> = library
            .iter()
            .filter(|v| {
                v.contexts.is_empty() || v.contexts.iter().any(|c| contexts.contains(c))
            })
            .collect();

        if matching.is_empty() {
            return library.first().map(|v| v.text.clone());
        }

        // Weighted random selection
        let total_prob: f32 = matching.iter().map(|v| v.probability).sum();
        let mut rng = rand::thread_rng();
        let mut roll = rng.gen::<f32>() * total_prob;

        for variant in &matching {
            roll -= variant.probability;
            if roll <= 0.0 {
                return Some(variant.text.clone());
            }
        }

        matching.last().map(|v| v.text.clone())
    }
}

impl Default for CommentaryLibrary {
    fn default() -> Self {
        Self::new()
    }
}
