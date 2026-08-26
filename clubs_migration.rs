/// Migrate sheffield_clubs table to add any missing columns
/// Add this function to sheffield_db.rs after migrate_sheffield_footballers
pub async fn migrate_sheffield_clubs(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Get existing columns
    let rows = sqlx::query("PRAGMA table_info(sheffield_clubs)")
        .fetch_all(pool)
        .await?;

    use sqlx::Row;
    let existing_cols: std::collections::HashSet<String> = rows.iter()
        .map(|r| r.get::<String, _>("name"))
        .collect();

    // Columns that may be missing from older DB instances
    let required_columns: &[(&str, &str)] = &[
        ("ground_name", "TEXT"),
        ("city", "TEXT DEFAULT ''"),
        ("region", "TEXT DEFAULT ''"),
    ];

    for (col, def) in required_columns {
        if !existing_cols.contains(*col) {
            let sql = format!("ALTER TABLE sheffield_clubs ADD COLUMN {} {}", col, def);
            sqlx::query(&sql).execute(pool).await?;
        }
    }

    // If ground_name was just added and is NULL, copy origin to it
    if !existing_cols.contains("ground_name") {
        sqlx::query("UPDATE sheffield_clubs SET ground_name = origin WHERE ground_name IS NULL")
            .execute(pool)
            .await?;
    }

    Ok(())
}
