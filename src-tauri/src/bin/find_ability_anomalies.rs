use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";

    println!("Opening database: {}", db_path);
    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?;
    let pool = SqlitePool::connect_with(connect_options).await?;

    println!("\n=== Finding players where current_ability > potential_ability ===\n");

    let anomalies: Vec<(i64, String, String, String, Option<i32>, Option<i32>)> = sqlx::query_as(
        "SELECT person_id, first_name, surname, club_id, current_ability, potential_ability
         FROM sheffield_footballers
         WHERE current_ability IS NOT NULL
           AND potential_ability IS NOT NULL
           AND current_ability > potential_ability
         ORDER BY (current_ability - potential_ability) DESC
         LIMIT 20"
    )
    .fetch_all(&pool)
    .await?;

    if anomalies.is_empty() {
        println!("No anomalies found! All players have current_ability <= potential_ability");
    } else {
        println!("Found {} players with current_ability > potential_ability:\n", anomalies.len());
        println!("{:<12} {:<30} {:<30} {:<15} {:<15} {:<10}",
            "person_id", "name", "club", "current", "potential", "diff");
        println!("{}", "-".repeat(120));

        for (person_id, first_name, surname, club_id, current, potential) in anomalies {
            let name = format!("{} {}", first_name, surname);
            let diff = current.unwrap() - potential.unwrap();
            println!("{:<12} {:<30} {:<30} {:<15} {:<15} {:<10}",
                person_id,
                name,
                club_id,
                current.unwrap(),
                potential.unwrap(),
                diff
            );
        }
    }

    // Also check total stats
    println!("\n=== Overall stats ===");
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_footballers WHERE current_ability IS NOT NULL")
        .fetch_one(&pool)
        .await?;
    println!("Total players with current_ability: {}", total.0);

    let with_pace: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_footballers WHERE pace IS NOT NULL")
        .fetch_one(&pool)
        .await?;
    println!("Total players with pace: {}", with_pace.0);

    Ok(())
}
