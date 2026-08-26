use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";

    println!("Opening database: {}", db_path);
    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?;
    let pool = SqlitePool::connect_with(connect_options).await?;

    println!("\n=== Checking sheffield_clubs table ===");

    // Check total clubs
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_clubs")
        .fetch_one(&pool)
        .await?;
    println!("Total clubs: {}", total.0);

    // Get sample clubs
    let clubs: Vec<(String, String)> = sqlx::query_as(
        "SELECT id, name FROM sheffield_clubs LIMIT 10"
    )
    .fetch_all(&pool)
    .await?;

    println!("\nSample clubs:");
    println!("{:<30} {:<30}", "id", "name");
    println!("{}", "-".repeat(60));
    for (id, name) in clubs {
        println!("{:<30} {:<30}", id, name);
    }

    Ok(())
}
