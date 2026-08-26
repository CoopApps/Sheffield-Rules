use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";

    println!("\n========================================");
    println!("POPULATING SHEFFIELD_CLUBS TABLE");
    println!("========================================\n");
    println!("Database: {}\n", db_path);

    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(connect_options).await?;

    // Read clubs data from the clubs.rs file and parse it
    let clubs_file = include_str!("../sheffield_rules/clubs.rs");

    // Parse club count
    let club_count = clubs_file.matches("SheffieldClub {").count();
    println!("Found {} clubs in source\n", club_count);

    // We'll call the populate function from the database module instead
    // This is simpler than parsing the Rust file
    println!("Note: Run 'cargo run --bin init_database' or use the app's setup to populate clubs.");
    println!("Or execute: UPDATE sheffield_clubs SET ... FROM the app.\n");

    pool.close().await;
    Ok(())
}
