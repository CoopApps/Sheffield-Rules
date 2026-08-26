use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";

    println!("Opening database: {}", db_path);
    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?;
    let pool = SqlitePool::connect_with(connect_options).await?;

    // Check a few of the player IDs that were shown in the debug output
    let test_ids = vec![76920, 80950, 109857, 47546, 90643];

    for player_id in test_ids {
        // Check if this person_id exists
        let exists: Option<(String, String)> = sqlx::query_as(
            "SELECT first_name, surname
             FROM sheffield_footballers
             WHERE person_id = ?"
        )
        .bind(player_id)
        .fetch_optional(&pool)
        .await?;

        if let Some((first_name, surname)) = exists {
            println!("✓ person_id {} EXISTS: {} {}", player_id, first_name, surname);
        } else {
            println!("✗ person_id {} DOES NOT EXIST", player_id);
        }
    }

    Ok(())
}
