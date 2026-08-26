use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";

    println!("Opening database: {}", db_path);
    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?;
    let pool = SqlitePool::connect_with(connect_options).await?;

    // List all tables
    println!("\n=== All Tables ===");
    let tables: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"
    )
    .fetch_all(&pool)
    .await?;

    for (table,) in &tables {
        println!("  - {}", table);
    }

    // Check sheffield_parishes
    println!("\n=== sheffield_parishes ===");
    let parish_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_parishes")
        .fetch_one(&pool)
        .await?;
    println!("Count: {}", parish_count.0);

    if parish_count.0 > 0 {
        let parishes: Vec<(String, String, Option<String>)> = sqlx::query_as(
            "SELECT id, name, postcode FROM sheffield_parishes LIMIT 10"
        )
        .fetch_all(&pool)
        .await?;

        for (id, name, postcode) in parishes {
            println!("  {} | {} | {}", id, name, postcode.unwrap_or("NULL".to_string()));
        }
    }

    // Check if sheffield_footballers exists
    println!("\n=== Checking sheffield_footballers ===");
    let result: Result<(i64,), sqlx::Error> = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_footballers"
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok((count,)) => {
            println!("sheffield_footballers exists with {} rows", count);

            if count > 0 {
                let sample: Vec<(String, Option<String>, Option<String>)> = sqlx::query_as(
                    "SELECT name, ecclesiastical_parish, where_born FROM sheffield_footballers LIMIT 5"
                )
                .fetch_all(&pool)
                .await?;

                println!("Sample players:");
                for (name, parish, born) in sample {
                    println!("  {} | Parish: {} | Born: {}",
                        name,
                        parish.unwrap_or("NULL".to_string()),
                        born.unwrap_or("NULL".to_string())
                    );
                }
            }
        }
        Err(_) => {
            println!("sheffield_footballers table does NOT exist");
        }
    }

    // Check sheffield_clubs
    println!("\n=== sheffield_clubs ===");
    let club_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_clubs")
        .fetch_one(&pool)
        .await?;
    println!("Count: {}", club_count.0);

    pool.close().await;
    Ok(())
}
