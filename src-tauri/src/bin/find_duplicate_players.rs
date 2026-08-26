use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";

    println!("Opening database: {}", db_path);
    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?;
    let pool = SqlitePool::connect_with(connect_options).await?;

    println!("\n=== Finding duplicate players in sheffield_footballers ===\n");

    // Find duplicates by first_name and surname
    let duplicates: Vec<(String, String, i64)> = sqlx::query_as(
        "SELECT first_name, surname, COUNT(*) as count
         FROM sheffield_footballers
         GROUP BY first_name, surname
         HAVING COUNT(*) > 1
         ORDER BY count DESC"
    )
    .fetch_all(&pool)
    .await?;

    if duplicates.is_empty() {
        println!("No duplicate players found!");
    } else {
        println!("Found {} duplicate name combinations:\n", duplicates.len());
        println!("{:<25} {:<25} {:<10}", "First Name", "Surname", "Count");
        println!("{}", "-".repeat(65));

        let mut total_duplicates = 0;
        for (first_name, surname, count) in &duplicates {
            println!("{:<25} {:<25} {:<10}", first_name, surname, count);
            total_duplicates += count - 1; // Count extras beyond the first
        }

        println!("\n{} duplicate name combinations", duplicates.len());
        println!("{} total duplicate player records", total_duplicates);
    }

    // Get total player count
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_footballers")
        .fetch_one(&pool)
        .await?;
    println!("\nTotal players in database: {}", total.0);

    Ok(())
}
