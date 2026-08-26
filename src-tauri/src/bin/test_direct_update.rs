use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";

    println!("Opening database: {}", db_path);
    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?;
    let pool = SqlitePool::connect_with(connect_options).await?;

    // First, let's see what person_ids exist
    println!("\nFinding a sample person_id...");
    let sample: Option<(i64, String, String)> = sqlx::query_as(
        "SELECT person_id, first_name, surname
         FROM sheffield_footballers
         WHERE club_id = '105th-regiment'
         LIMIT 1"
    )
    .fetch_optional(&pool)
    .await?;

    if let Some((person_id, first_name, surname)) = sample {
        println!("Found player: {} {} (person_id: {})", first_name, surname, person_id);

        // Try to update this player
        println!("\nAttempting UPDATE with person_id...");
        let result = sqlx::query(
            "UPDATE sheffield_footballers
             SET position = ?, pace = ?, current_ability = ?
             WHERE person_id = ?"
        )
        .bind("CB")
        .bind(50)
        .bind(75)
        .bind(person_id)
        .execute(&pool)
        .await?;

        println!("Rows affected: {}", result.rows_affected());

        // Verify the update
        let check: Option<(Option<String>, Option<i32>, Option<i32>)> = sqlx::query_as(
            "SELECT position, pace, current_ability
             FROM sheffield_footballers
             WHERE person_id = ?"
        )
        .bind(person_id)
        .fetch_optional(&pool)
        .await?;

        if let Some((pos, pace, ca)) = check {
            println!("\nVerification:");
            println!("Position: {:?}", pos);
            println!("Pace: {:?}", pace);
            println!("Current Ability: {:?}", ca);
        }
    } else {
        println!("No players found!");
    }

    Ok(())
}
