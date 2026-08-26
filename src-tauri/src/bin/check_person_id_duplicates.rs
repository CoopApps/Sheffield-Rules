use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";

    println!("Opening database: {}", db_path);
    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?;
    let pool = SqlitePool::connect_with(connect_options).await?;

    println!("\n=== Checking for duplicate person_id values in sheffield_footballers ===\n");

    // Find duplicate person_id values
    let duplicates: Vec<(i64, i64)> = sqlx::query_as(
        "SELECT person_id, COUNT(*) as count
         FROM sheffield_footballers
         GROUP BY person_id
         HAVING COUNT(*) > 1
         ORDER BY count DESC"
    )
    .fetch_all(&pool)
    .await?;

    if duplicates.is_empty() {
        println!("✓ No duplicate person_id values found!");
        println!("✓ All person_id values in sheffield_footballers are unique.");
        println!("\nThis means the 'duplicate names' are actually different people");
        println!("who happen to share the same first_name and surname combination.");
    } else {
        println!("⚠ Found {} person_id values that appear multiple times:\n", duplicates.len());
        println!("{:<15} {:<10}", "person_id", "Count");
        println!("{}", "-".repeat(30));

        let mut total_duplicate_records = 0;
        for (person_id, count) in &duplicates {
            println!("{:<15} {:<10}", person_id, count);
            total_duplicate_records += count - 1;
        }

        println!("\n{} person_id values appear multiple times", duplicates.len());
        println!("{} total duplicate records (beyond first occurrence)", total_duplicate_records);

        // Show details for a few examples
        if !duplicates.is_empty() {
            println!("\n=== Example duplicate records (first 3 person_ids) ===\n");
            for (person_id, _) in duplicates.iter().take(3) {
                let records: Vec<(i64, String, String, String)> = sqlx::query_as(
                    "SELECT person_id, first_name, surname, club_id
                     FROM sheffield_footballers
                     WHERE person_id = ?"
                )
                .bind(person_id)
                .fetch_all(&pool)
                .await?;

                println!("person_id {}: {} records", person_id, records.len());
                for (pid, fname, sname, club) in records {
                    println!("  {} {} - club: {}", fname, sname, club);
                }
                println!();
            }
        }
    }

    // Get total counts
    let total_players: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_footballers")
        .fetch_one(&pool)
        .await?;

    let unique_person_ids: (i64,) = sqlx::query_as("SELECT COUNT(DISTINCT person_id) FROM sheffield_footballers")
        .fetch_one(&pool)
        .await?;

    println!("\n=== Summary ===");
    println!("Total player records: {}", total_players.0);
    println!("Unique person_id values: {}", unique_person_ids.0);
    println!("Difference: {}", total_players.0 - unique_person_ids.0);

    Ok(())
}
