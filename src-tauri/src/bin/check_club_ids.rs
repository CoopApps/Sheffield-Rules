use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use sqlx::Row;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";

    println!("Opening database: {}", db_path);
    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?;
    let pool = SqlitePool::connect_with(connect_options).await?;

    // Check sheffield_footballers table structure
    println!("\n=== sheffield_footballers Table Schema ===");
    let schema = sqlx::query("PRAGMA table_info(sheffield_footballers)")
        .fetch_all(&pool)
        .await?;

    for row in schema {
        let name: String = row.try_get(1)?;
        println!("Column: {}", name);
    }

    // Count total footballers
    println!("\n=== Total Footballers ===");
    let (total,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_footballers")
        .fetch_one(&pool)
        .await?;
    println!("Total: {}", total);

    // Check if club_id column exists and what values it has
    println!("\n=== Checking club_id column ===");
    let result = sqlx::query("SELECT club_id FROM sheffield_footballers LIMIT 10")
        .fetch_all(&pool)
        .await;

    match result {
        Ok(rows) => {
            println!("club_id column exists!");
            println!("Sample values:");
            for row in rows {
                let club_id: Option<String> = row.try_get("club_id").ok();
                println!("  club_id: {:?}", club_id);
            }
        }
        Err(e) => {
            println!("ERROR: club_id column might not exist: {}", e);
        }
    }

    // Check unique club_id values
    println!("\n=== Unique club_id values ===");
    let unique: Vec<(Option<String>, i64)> = sqlx::query_as(
        "SELECT club_id, COUNT(*) as count
         FROM sheffield_footballers
         GROUP BY club_id
         ORDER BY count DESC
         LIMIT 20"
    )
    .fetch_all(&pool)
    .await?;

    for (club_id, count) in unique {
        println!("  {} - {} players", club_id.unwrap_or("NULL".to_string()), count);
    }

    pool.close().await;
    Ok(())
}
