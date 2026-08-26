use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";

    println!("Opening database: {}", db_path);
    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?;
    let pool = SqlitePool::connect_with(connect_options).await?;

    println!("\nClearing all footballer stats...");

    // Just clear the main stats - pace, position, and current_ability
    let result = sqlx::query(
        "UPDATE sheffield_footballers SET
            position = NULL,
            pace = NULL,
            current_ability = NULL"
    )
    .execute(&pool)
    .await?;

    println!("Cleared stats for {} players", result.rows_affected());

    // Verify
    let with_stats: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_footballers WHERE pace IS NOT NULL")
        .fetch_one(&pool)
        .await?;

    println!("Players with stats remaining: {}", with_stats.0);
    println!("\nDone! All stats cleared.");

    Ok(())
}
