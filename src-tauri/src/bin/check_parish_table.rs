use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";

    println!("Opening database: {}", db_path);
    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?;
    let pool = SqlitePool::connect_with(connect_options).await?;

    // Check sheffield_parishes table structure
    println!("\n=== sheffield_parishes Table Schema ===");
    let schema = sqlx::query("PRAGMA table_info(sheffield_parishes)")
        .fetch_all(&pool)
        .await?;

    use sqlx::Row;
    for row in schema {
        let name: String = row.try_get(1)?;
        let type_: String = row.try_get(2)?;
        println!("  {} ({})", name, type_);
    }

    // Get sample parishes with postcodes
    println!("\n=== Sample Parishes ===");
    let parishes: Vec<(String, String, Option<String>)> = sqlx::query_as(
        "SELECT id, name, postcode FROM sheffield_parishes LIMIT 20"
    )
    .fetch_all(&pool)
    .await?;

    for (id, name, postcode) in parishes {
        println!("  {} | {} -> {:?}", id, name, postcode.unwrap_or("NULL".to_string()));
    }

    // Count total
    let (total,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_parishes")
        .fetch_one(&pool)
        .await?;
    
    println!("\nTotal parishes: {}", total);

    pool.close().await;
    Ok(())
}
