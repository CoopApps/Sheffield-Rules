use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";

    println!("Opening database: {}", db_path);
    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?;
    let pool = SqlitePool::connect_with(connect_options).await?;

    println!("\n========================================");
    println!("SHEFFIELD FOOTBALLERS STATS CHECK");
    println!("========================================");

    // Get total count
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_footballers")
        .fetch_one(&pool)
        .await?;
    println!("Total footballers: {}", total.0);

    // Count with pace
    let with_pace: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_footballers WHERE pace IS NOT NULL")
        .fetch_one(&pool)
        .await?;
    println!("Players with pace stat: {}", with_pace.0);

    // Count with position
    let with_position: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_footballers WHERE position IS NOT NULL")
        .fetch_one(&pool)
        .await?;
    println!("Players with position: {}", with_position.0);

    // Count with current_ability
    let with_ca: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_footballers WHERE current_ability IS NOT NULL")
        .fetch_one(&pool)
        .await?;
    println!("Players with current_ability: {}", with_ca.0);

    println!("========================================");

    // Show sample player
    let sample: Option<(String, String, Option<String>, Option<i32>, Option<i32>, Option<i32>, i64)> = sqlx::query_as(
        "SELECT first_name, surname, position, pace, acceleration, current_ability, person_id FROM sheffield_footballers WHERE pace IS NOT NULL LIMIT 1"
    )
    .fetch_optional(&pool)
    .await?;

    if let Some((first_name, surname, position, pace, acceleration, ca, person_id)) = sample {
        println!("\nSample player with stats:");
        println!("{} {} (person_id: {})", first_name, surname, person_id);
        println!("Position: {}", position.unwrap_or("None".to_string()));
        println!("Pace: {}, Acceleration: {}", pace.unwrap_or(0), acceleration.unwrap_or(0));
        println!("Current Ability: {}", ca.unwrap_or(0));
    } else {
        println!("\nNo players found with stats.");
    }

    Ok(())
}
