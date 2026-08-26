use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use sqlx::Row;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";

    println!("Opening database: {}", db_path);
    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?;
    let pool = SqlitePool::connect_with(connect_options).await?;

    // Test the query that db_get_all_players uses
    println!("\n=== Testing Player Query ===");
    let result = sqlx::query(
        "SELECT
            f.id,
            p.ecclesiastical_parish
         FROM sheffield_footballers f
         JOIN sheffield_people p ON f.person_id = p.unique_id
         LIMIT 10"
    )
    .fetch_all(&pool)
    .await;

    match result {
        Ok(rows) => {
            println!("Success! Found {} rows", rows.len());
            for row in rows {
                let id: i64 = row.get(0);
                let parish: Option<String> = row.get(1);
                println!("  ID: {} | Parish: {}", id, parish.unwrap_or("NULL".to_string()));
            }
        }
        Err(e) => {
            println!("ERROR: {}", e);
        }
    }

    // Count unique parishes
    println!("\n=== Counting Unique Parishes ===");
    let parishes: Vec<(Option<String>, i64)> = sqlx::query_as(
        "SELECT p.ecclesiastical_parish, COUNT(*) as count
         FROM sheffield_footballers f
         JOIN sheffield_people p ON f.person_id = p.unique_id
         WHERE p.ecclesiastical_parish IS NOT NULL AND p.ecclesiastical_parish != ''
         GROUP BY p.ecclesiastical_parish
         ORDER BY count DESC
         LIMIT 20"
    )
    .fetch_all(&pool)
    .await?;

    for (parish, count) in parishes {
        println!("  {} - {} players", parish.unwrap_or("NULL".to_string()), count);
    }

    pool.close().await;
    Ok(())
}
