use sqlx::{SqlitePool, FromRow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LeagueMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    pub season_year: i64,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl Default for LeagueMetadata {
    fn default() -> Self {
        LeagueMetadata {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Fantasy 1867".to_string(),
            description: "All 186 historical clubs in unified pyramid".to_string(),
            season_year: 1867,
            is_active: true,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Create the league metadata table
pub async fn create_league_metadata_table(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS league_metadata (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            description TEXT NOT NULL,
            season_year INTEGER NOT NULL,
            is_active BOOLEAN NOT NULL DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Get all leagues
pub async fn get_all_leagues(pool: &SqlitePool) -> Result<Vec<LeagueMetadata>, Box<dyn std::error::Error>> {
    let leagues = sqlx::query_as::<_, LeagueMetadata>(
        r#"
        SELECT id, name, description, season_year, is_active, created_at, updated_at
        FROM league_metadata
        ORDER BY season_year DESC, name ASC
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(leagues)
}

/// Get active league
pub async fn get_active_league(pool: &SqlitePool) -> Result<Option<LeagueMetadata>, Box<dyn std::error::Error>> {
    let league = sqlx::query_as::<_, LeagueMetadata>(
        r#"
        SELECT id, name, description, season_year, is_active, created_at, updated_at
        FROM league_metadata
        WHERE is_active = 1
        LIMIT 1
        "#
    )
    .fetch_optional(pool)
    .await?;

    Ok(league)
}

/// Create a new league
pub async fn create_league(
    pool: &SqlitePool,
    name: String,
    description: String,
    season_year: i64,
) -> Result<LeagueMetadata, Box<dyn std::error::Error>> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        r#"
        INSERT INTO league_metadata (id, name, description, season_year, is_active, created_at, updated_at)
        VALUES (?, ?, ?, ?, 0, ?, ?)
        "#
    )
    .bind(&id)
    .bind(&name)
    .bind(&description)
    .bind(season_year)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    Ok(LeagueMetadata {
        id,
        name,
        description,
        season_year,
        is_active: false,
        created_at: now.clone(),
        updated_at: now,
    })
}

/// Set active league (deactivates all others)
pub async fn set_active_league(
    pool: &SqlitePool,
    league_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // First deactivate all leagues
    sqlx::query("UPDATE league_metadata SET is_active = 0")
        .execute(pool)
        .await?;

    // Then activate the selected league
    sqlx::query("UPDATE league_metadata SET is_active = 1 WHERE id = ?")
        .bind(&league_id)
        .execute(pool)
        .await?;

    Ok(())
}

/// Delete a league
pub async fn delete_league(
    pool: &SqlitePool,
    league_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::query("DELETE FROM league_metadata WHERE id = ?")
        .bind(&league_id)
        .execute(pool)
        .await?;

    Ok(())
}

/// Initialize the Fantasy 1867 league if it doesn't exist
pub async fn initialize_default_league(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    // Check if any leagues exist
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM league_metadata")
        .fetch_one(pool)
        .await?;

    if count.0 == 0 {
        // Create the default Fantasy 1867 league
        let default_league = LeagueMetadata::default();
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO league_metadata (id, name, description, season_year, is_active, created_at, updated_at)
            VALUES (?, ?, ?, ?, 1, ?, ?)
            "#
        )
        .bind(&default_league.id)
        .bind(&default_league.name)
        .bind(&default_league.description)
        .bind(default_league.season_year)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;

        // Also create default league configuration
        let default_config = crate::database::league_config::LeagueConfig::default();
        crate::database::league_config::save_league_config(pool, &default_config).await?;
    }

    Ok(())
}
