use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";

    println!("Opening database: {}", db_path);
    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?;
    let pool = SqlitePool::connect_with(connect_options).await?;

    println!("\n=== Adding club_id column to sheffield_footballers ===");

    // Check if column already exists
    let check_result = sqlx::query("SELECT club_id FROM sheffield_footballers LIMIT 1")
        .fetch_optional(&pool)
        .await;

    match check_result {
        Ok(_) => {
            println!("Column 'club_id' already exists in sheffield_footballers table.");
        }
        Err(_) => {
            println!("Column 'club_id' does not exist. Adding it now...");

            // Add the column with default value 'UNASSIGNED'
            sqlx::query("ALTER TABLE sheffield_footballers ADD COLUMN club_id TEXT DEFAULT 'UNASSIGNED'")
                .execute(&pool)
                .await?;

            println!("✓ Successfully added club_id column to sheffield_footballers");

            // Verify the column was added
            let count: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM sheffield_footballers WHERE club_id = 'UNASSIGNED'"
            )
            .fetch_one(&pool)
            .await?;

            println!("✓ Verified: {} players now have club_id = 'UNASSIGNED'", count.0);
        }
    }

    pool.close().await;
    Ok(())
}
