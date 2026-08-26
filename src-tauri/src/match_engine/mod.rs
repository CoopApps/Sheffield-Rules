/// Match Engine Module for Saturday at Three
/// Implements Football Man's possession physics + narrative commentary
/// Adapted for Sheffield Rules (1858-1877)

pub mod enhanced_commentary_library;
pub mod formation;
// pub mod outcome_commentary_mapper;  // Temporarily disabled due to broken imports
pub mod player_attributes;
pub mod possession_engine;
pub mod match_simulator;
pub mod live_engine;
pub mod sheffield_1867;

// Re-export key types
pub use formation::{Formation, FormationPosition, Position2D};
pub use player_attributes::{PlayerAttributes, TeamQualityCalculator};
pub use possession_engine::{
    PossessionEngine, PossessionSequence, BallState, ShotAttempt,
    SequenceOutcome, TeamSide, SimpleRng, FieldZone
};
pub use match_simulator::{
    MatchSimulator, MatchResult, MatchEvent, MatchStatistics,
    VisualState, PlayerPositionState, ScorerInfo, EventType
};
