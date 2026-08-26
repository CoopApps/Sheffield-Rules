/// Sheffield 1867 Match Engine Module
/// Comprehensive match simulation system for Sheffield Rules era (1857-1877)

pub mod player_performance;
pub mod match_state;
pub mod commentary;
pub mod match_simulator;
#[cfg(test)]
mod tests;

pub use player_performance::PlayerPerformance;
pub use match_state::{MatchStateSnapshot, PlayerSnapshot};
pub use commentary::{CommentaryLibrary, CommentaryContext, MatchCommentary};
pub use match_simulator::{
    SheffieldMatchSimulator, SheffieldMatchResult, MatchEvent, RealPlayerData,
    InjuryInfo, InjurySeverity, WeatherCondition, PitchCondition
};
