use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";

    println!("Opening database: {}", db_path);
    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?
        .create_if_missing(false);
    let pool = SqlitePool::connect_with(connect_options).await?;

    println!("\n=== Finding and fixing players where current_ability > potential_ability ===\n");

    // First, find the anomalies
    let anomalies: Vec<(i64, String, String, i32, i32)> = sqlx::query_as(
        "SELECT person_id, first_name, surname, current_ability, potential_ability
         FROM sheffield_footballers
         WHERE current_ability IS NOT NULL
           AND potential_ability IS NOT NULL
           AND current_ability > potential_ability"
    )
    .fetch_all(&pool)
    .await?;

    if anomalies.is_empty() {
        println!("No anomalies found!");
    } else {
        println!("Found {} players with current_ability > potential_ability\n", anomalies.len());

        for (person_id, first_name, surname, current, potential) in &anomalies {
            let new_potential = potential + 30;
            println!("Fixing {} {} (person_id: {}): potential {} -> {}",
                first_name, surname, person_id, potential, new_potential);

            // Update the potential_ability
            sqlx::query(
                "UPDATE sheffield_footballers
                 SET potential_ability = ?
                 WHERE person_id = ?"
            )
            .bind(new_potential)
            .bind(person_id)
            .execute(&pool)
            .await?;
        }

        println!("\n✓ Fixed {} players", anomalies.len());

        // Verify no anomalies remain
        let remaining: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM sheffield_footballers
             WHERE current_ability IS NOT NULL
               AND potential_ability IS NOT NULL
               AND current_ability > potential_ability"
        )
        .fetch_one(&pool)
        .await?;

        println!("Remaining anomalies: {}", remaining.0);
    }

    Ok(())
}
