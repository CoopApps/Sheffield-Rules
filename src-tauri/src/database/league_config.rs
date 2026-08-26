use sqlx::{SqlitePool, FromRow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LeagueConfig {
    pub id: String,
    pub season_year: i64,
    pub season_start_date: String,  // e.g., "1867-09-01"
    pub season_end_date: String,    // e.g., "1868-05-31"

    // Points system
    pub points_for_win: i64,
    pub points_for_draw: i64,
    pub points_for_loss: i64,

    // Match scheduling
    pub default_match_day: String,        // "Saturday", "Wednesday", etc.
    pub default_kickoff_time: String,     // "15:00"
    pub allow_midweek_fixtures: bool,
    pub midweek_day: Option<String>,      // "Wednesday"
    pub midweek_kickoff_time: Option<String>, // "19:30"

    // Cancellation and rearrangement rules
    pub enable_weather_cancellations: bool,
    pub cancellation_threshold: i64,      // Weather severity 1-10
    pub rearrangement_window_weeks: i64,  // How many weeks to reschedule within
    pub priority_rearrangement: bool,     // Reschedule ASAP vs end of season

    // Playoff configuration
    pub enable_promotion_playoffs: bool,
    pub playoff_teams_per_division: i64,  // How many teams in playoff (2, 4, 6, etc.)
    pub playoff_format: String,           // "single_leg", "two_leg_home_away", "neutral_venue"

    pub enable_relegation_playoffs: bool,
    pub relegation_playoff_teams: i64,

    // Sheffield Rules specific
    pub enable_rouge_scoring: bool,       // Use rouge as tiebreaker (1862-1868)
    pub rouge_prevents_draws: bool,       // Rouge prevents match draws

    pub created_at: String,
    pub updated_at: String,
}

impl Default for LeagueConfig {
    fn default() -> Self {
        LeagueConfig {
            id: uuid::Uuid::new_v4().to_string(),
            season_year: 1867,
            season_start_date: "1867-09-01".to_string(),
            season_end_date: "1868-05-31".to_string(),

            // Standard points: 3 for win, 1 for draw, 0 for loss
            points_for_win: 3,
            points_for_draw: 1,
            points_for_loss: 0,

            // Traditional Saturday 3pm kickoffs
            default_match_day: "Saturday".to_string(),
            default_kickoff_time: "15:00".to_string(),
            allow_midweek_fixtures: false,
            midweek_day: Some("Wednesday".to_string()),
            midweek_kickoff_time: Some("19:30".to_string()),

            // Weather and rearrangement
            enable_weather_cancellations: true,
            cancellation_threshold: 7,
            rearrangement_window_weeks: 4,
            priority_rearrangement: false,

            // Playoffs disabled by default
            enable_promotion_playoffs: false,
            playoff_teams_per_division: 4,
            playoff_format: "two_leg_home_away".to_string(),

            enable_relegation_playoffs: false,
            relegation_playoff_teams: 2,

            // Sheffield Rules: Rouge enabled by default for historical accuracy
            enable_rouge_scoring: false,
            rouge_prevents_draws: true,

            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Create the league configuration table
pub async fn create_league_config_table(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS sheffield_league_config (
            id TEXT PRIMARY KEY,
            season_year INTEGER NOT NULL,
            season_start_date TEXT NOT NULL,
            season_end_date TEXT NOT NULL,

            points_for_win INTEGER NOT NULL DEFAULT 3,
            points_for_draw INTEGER NOT NULL DEFAULT 1,
            points_for_loss INTEGER NOT NULL DEFAULT 0,

            default_match_day TEXT NOT NULL DEFAULT 'Saturday',
            default_kickoff_time TEXT NOT NULL DEFAULT '15:00',
            allow_midweek_fixtures BOOLEAN DEFAULT 0,
            midweek_day TEXT,
            midweek_kickoff_time TEXT,

            enable_weather_cancellations BOOLEAN DEFAULT 1,
            cancellation_threshold INTEGER DEFAULT 7,
            rearrangement_window_weeks INTEGER DEFAULT 4,
            priority_rearrangement BOOLEAN DEFAULT 0,

            enable_promotion_playoffs BOOLEAN DEFAULT 0,
            playoff_teams_per_division INTEGER DEFAULT 4,
            playoff_format TEXT DEFAULT 'two_leg_home_away',

            enable_relegation_playoffs BOOLEAN DEFAULT 0,
            relegation_playoff_teams INTEGER DEFAULT 2,

            enable_rouge_scoring BOOLEAN DEFAULT 0,
            rouge_prevents_draws BOOLEAN DEFAULT 1,

            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get or create league configuration
pub async fn get_league_config(pool: &SqlitePool) -> Result<LeagueConfig, Box<dyn std::error::Error>> {
    // Try to get existing config
    let result = sqlx::query_as::<_, LeagueConfig>(
        r#"
        SELECT
            id, season_year, season_start_date, season_end_date,
            points_for_win, points_for_draw, points_for_loss,
            default_match_day, default_kickoff_time,
            allow_midweek_fixtures, midweek_day, midweek_kickoff_time,
            enable_weather_cancellations, cancellation_threshold,
            rearrangement_window_weeks, priority_rearrangement,
            enable_promotion_playoffs, playoff_teams_per_division, playoff_format,
            enable_relegation_playoffs, relegation_playoff_teams,
            enable_rouge_scoring, rouge_prevents_draws,
            created_at, updated_at
        FROM sheffield_league_config
        LIMIT 1
        "#
    )
    .fetch_optional(pool)
    .await?;

    if let Some(config) = result {
        Ok(config)
    } else {
        // Create default config
        let default_config = LeagueConfig::default();
        save_league_config(pool, &default_config).await?;
        Ok(default_config)
    }
}

/// Save or update league configuration
pub async fn save_league_config(
    pool: &SqlitePool,
    config: &LeagueConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let updated_at = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO sheffield_league_config (
            id, season_year, season_start_date, season_end_date,
            points_for_win, points_for_draw, points_for_loss,
            default_match_day, default_kickoff_time,
            allow_midweek_fixtures, midweek_day, midweek_kickoff_time,
            enable_weather_cancellations, cancellation_threshold,
            rearrangement_window_weeks, priority_rearrangement,
            enable_promotion_playoffs, playoff_teams_per_division, playoff_format,
            enable_relegation_playoffs, relegation_playoff_teams,
            enable_rouge_scoring, rouge_prevents_draws,
            created_at, updated_at
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        ON CONFLICT(id) DO UPDATE SET
            season_year = excluded.season_year,
            season_start_date = excluded.season_start_date,
            season_end_date = excluded.season_end_date,
            points_for_win = excluded.points_for_win,
            points_for_draw = excluded.points_for_draw,
            points_for_loss = excluded.points_for_loss,
            default_match_day = excluded.default_match_day,
            default_kickoff_time = excluded.default_kickoff_time,
            allow_midweek_fixtures = excluded.allow_midweek_fixtures,
            midweek_day = excluded.midweek_day,
            midweek_kickoff_time = excluded.midweek_kickoff_time,
            enable_weather_cancellations = excluded.enable_weather_cancellations,
            cancellation_threshold = excluded.cancellation_threshold,
            rearrangement_window_weeks = excluded.rearrangement_window_weeks,
            priority_rearrangement = excluded.priority_rearrangement,
            enable_promotion_playoffs = excluded.enable_promotion_playoffs,
            playoff_teams_per_division = excluded.playoff_teams_per_division,
            playoff_format = excluded.playoff_format,
            enable_relegation_playoffs = excluded.enable_relegation_playoffs,
            relegation_playoff_teams = excluded.relegation_playoff_teams,
            enable_rouge_scoring = excluded.enable_rouge_scoring,
            rouge_prevents_draws = excluded.rouge_prevents_draws,
            updated_at = ?
        "#
    )
    .bind(&config.id)
    .bind(config.season_year)
    .bind(&config.season_start_date)
    .bind(&config.season_end_date)
    .bind(config.points_for_win)
    .bind(config.points_for_draw)
    .bind(config.points_for_loss)
    .bind(&config.default_match_day)
    .bind(&config.default_kickoff_time)
    .bind(config.allow_midweek_fixtures)
    .bind(&config.midweek_day)
    .bind(&config.midweek_kickoff_time)
    .bind(config.enable_weather_cancellations)
    .bind(config.cancellation_threshold)
    .bind(config.rearrangement_window_weeks)
    .bind(config.priority_rearrangement)
    .bind(config.enable_promotion_playoffs)
    .bind(config.playoff_teams_per_division)
    .bind(&config.playoff_format)
    .bind(config.enable_relegation_playoffs)
    .bind(config.relegation_playoff_teams)
    .bind(config.enable_rouge_scoring)
    .bind(config.rouge_prevents_draws)
    .bind(&config.created_at)
    .bind(&updated_at)
    .bind(&updated_at)
    .execute(pool)
    .await?;

    Ok(())
}
