/// Season Engine: Orchestrates season progression, match simulation, and event generation
///
/// This module handles:
/// - Day-by-day season advancement
/// - Match simulation with complete data storage
/// - Weather generation and cancellations
/// - Injury tracking
/// - Random events
/// - Newspaper generation
/// - Lineup selection (user-controlled)
/// - Rule negotiation for inter-code matches

pub mod match_engine;
pub mod weather;
pub mod injuries;
pub mod events;
pub mod newspaper;
pub mod lineup_selection;
pub mod rule_negotiation;
pub mod local_events;
pub mod cooperative;

pub use match_engine::{MatchEngine, CompleteMatchResult};
pub use weather::{WeatherSystem, WeatherForecast, WeatherCondition};
pub use injuries::{InjurySystem, Injury, InjurySeverity};
pub use lineup_selection::{LineupSelection, Formation, MatchPosition};
pub use rule_negotiation::{RuleNegotiation, FootballCode, NegotiationType};
pub use local_events::{LocalEventGenerator, LocalEvent, LocalEventType, EventImportance};
pub use cooperative::{CooperativeSystem, CooperativeMembership, CooperativeTransaction, TransactionType, ShopItem};
