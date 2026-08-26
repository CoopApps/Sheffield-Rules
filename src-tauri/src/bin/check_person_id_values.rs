use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";

    println!("Opening database: {}", db_path);
    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?;
    let pool = SqlitePool::connect_with(connect_options).await?;

    // Check the data types and values in sheffield_footballers
    println!("\n=== Checking sheffield_footballers schema and sample data ===");

    let samples: Vec<(i64, i64, String, String, String)> = sqlx::query_as(
        "SELECT id, person_id, first_name, surname, club_id
         FROM sheffield_footballers
         WHERE club_id = '105th-regiment'
         LIMIT 5"
    )
    .fetch_all(&pool)
    .await?;

    println!("\nSample rows from sheffield_footballers:");
    println!("{:<10} {:<15} {:<20} {:<10}", "id", "person_id", "name", "club_id");
    println!("{}", "-".repeat(70));
    for (id, person_id, first_name, surname, club_id) in samples {
        println!("{:<10} {:<15} {:<20} {:<10}", id, person_id, format!("{} {}", first_name, surname), club_id);
    }

    // Check if person_id matches unique_id in sheffield_people
    println!("\n=== Checking person_id -> unique_id relationship ===");

    let check: Vec<(i64, i64, String, String)> = sqlx::query_as(
        "SELECT f.person_id, p.unique_id, f.first_name, f.surname
         FROM sheffield_footballers f
         JOIN sheffield_people p ON f.person_id = p.unique_id
         WHERE f.club_id = '105th-regiment'
         LIMIT 5"
    )
    .fetch_all(&pool)
    .await?;

    println!("\nVerifying foreign key relationship:");
    println!("{:<15} {:<15} {:<30}", "f.person_id", "p.unique_id", "name");
    println!("{}", "-".repeat(70));
    for (person_id, unique_id, first_name, surname) in check {
        let match_status = if person_id == unique_id { "✓" } else { "✗" };
        println!("{:<15} {:<15} {:<30} {}", person_id, unique_id, format!("{} {}", first_name, surname), match_status);
    }

    Ok(())
}
