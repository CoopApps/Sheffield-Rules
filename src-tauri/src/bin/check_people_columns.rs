use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use sqlx::Row;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";

    println!("Opening database: {}", db_path);
    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?;
    let pool = SqlitePool::connect_with(connect_options).await?;

    println!("\n=== sheffield_people Table Columns ===");
    let schema = sqlx::query("PRAGMA table_info(sheffield_people)")
        .fetch_all(&pool)
        .await?;

    for row in schema {
        let name: String = row.try_get(1)?;
        println!("Column: {}", name);
    }

    // Check if where_born exists
    println!("\n=== Checking where_born column ===");
    let result = sqlx::query("SELECT where_born FROM sheffield_people LIMIT 5")
        .fetch_all(&pool)
        .await;

    match result {
        Ok(rows) => {
            println!("where_born column exists! Sample values:");
            for row in rows {
                let where_born: Option<String> = row.try_get("where_born").ok();
                println!("  {:?}", where_born);
            }
        }
        Err(e) => {
            println!("ERROR: where_born column might not exist: {}", e);
        }
    }

    pool.close().await;
    Ok(())
}
