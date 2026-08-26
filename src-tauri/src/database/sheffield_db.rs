use sqlx::sqlite::{SqlitePool, SqlitePoolOptions, SqliteConnectOptions};
use std::str::FromStr;

/// Create or connect to the Sheffield Rules database
pub async fn create_sheffield_database(db_path: &str) -> Result<SqlitePool, sqlx::Error> {
    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?
        .create_if_missing(true)
        .foreign_keys(true)
        .busy_timeout(std::time::Duration::from_secs(30)); // Wait up to 30 seconds if database is locked

    let pool = SqlitePoolOptions::new()
        .max_connections(1)  // Use single connection to avoid locking issues
        .connect_with(connect_options)
        .await?;

    Ok(pool)
}

/// Initialize the database schema
pub async fn initialize_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let schema = include_str!("./sheffield_schema.sql");

    // Execute each statement separately since SQLite doesn't support multiple statements in one execute
    for statement in schema.split(';') {
        let trimmed = statement.trim();
        if !trimmed.is_empty() {
            sqlx::raw_sql(trimmed).execute(pool).await?;
        }
    }

    Ok(())
}

/// Migrate sheffield_footballers table to add any missing columns
/// SQLite doesn't support IF NOT EXISTS on ALTER TABLE, so we check PRAGMA table_info first
pub async fn migrate_sheffield_footballers(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Get existing columns
    let rows = sqlx::query("PRAGMA table_info(sheffield_footballers)")
        .fetch_all(pool)
        .await?;

    use sqlx::Row;
    let existing_cols: std::collections::HashSet<String> = rows.iter()
        .map(|r| r.get::<String, _>("name"))
        .collect();

    // Columns that may be missing from older DB instances
    let required_columns: &[(&str, &str)] = &[
        ("acceleration",       "INTEGER"),
        ("natural_fitness",    "INTEGER"),
        ("technique",          "INTEGER"),
        ("corners",            "INTEGER"),
        ("free_kicks",         "INTEGER"),
        ("throw_ins",          "INTEGER"),
        ("vision",             "INTEGER"),
        ("left_foot",          "INTEGER"),
        ("right_foot",         "INTEGER"),
        ("one_on_ones",        "INTEGER"),
        ("bravery",            "INTEGER"),
        ("adaptability",       "INTEGER"),
        ("ambition",           "INTEGER"),
        ("loyalty",            "INTEGER"),
        ("pressure",           "INTEGER"),
        ("professionalism",    "INTEGER"),
        ("sportsmanship",      "INTEGER"),
        ("temperament",        "INTEGER"),
        ("movement",           "INTEGER"),
        ("injury_proneness",   "INTEGER"),
        ("important_matches",  "INTEGER"),
        ("current_ability",    "INTEGER"),
        ("potential_ability",  "INTEGER"),
        ("current_reputation", "INTEGER"),
        ("has_stats",          "BOOLEAN DEFAULT 0"),
        ("where_born",         "TEXT"),
        ("birth_town",         "TEXT"),
        ("birth_county",       "TEXT"),
        ("birth_country",      "TEXT"),
        ("civil_parish",       "TEXT"),
        ("ecclesiastical_parish", "TEXT"),
        ("registration_district", "TEXT"),
        ("first_name",         "TEXT"),
        ("middle_name",        "TEXT"),
        ("surname",            "TEXT"),
        ("census_age",         "INTEGER"),
        ("census_relation",    "TEXT"),
        ("census_gender",      "TEXT"),
        ("census_ed",          "TEXT"),
        ("census_household_schedule", "TEXT"),
        ("census_piece",       "TEXT"),
        ("census_folio",       "TEXT"),
        ("census_page",        "TEXT"),
    ];

    for (col, def) in required_columns {
        if !existing_cols.contains(*col) {
            let sql = format!("ALTER TABLE sheffield_footballers ADD COLUMN {} {}", col, def);
            sqlx::query(&sql).execute(pool).await?;
        }
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_database() {
        let pool = create_sheffield_database("sqlite::memory:")
            .await
            .expect("Failed to create database");

        initialize_schema(&pool)
            .await
            .expect("Failed to initialize schema");

        // Verify a table exists
        let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sqlite_master WHERE type='table'")
            .fetch_one(&pool)
            .await
            .expect("Failed to query tables");

        assert!(result.0 > 0, "No tables created");
    }
}
