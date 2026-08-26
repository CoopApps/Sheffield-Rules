use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let sql_path = "D:/projects/Saturday at Three/populate_sheffield_parishes.sql";

    println!("Opening database: {}", db_path);
    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?;
    let pool = SqlitePool::connect_with(connect_options).await?;

    println!("Reading SQL file: {}", sql_path);
    let sql = std::fs::read_to_string(sql_path)?;

    println!("Executing SQL...");
    sqlx::raw_sql(&sql).execute(&pool).await?;

    // Count clubs
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_clubs")
        .fetch_one(&pool)
        .await?;

    println!("✓ Success! {} clubs loaded into Sheffield1867.db", count.0);

    pool.close().await;
    Ok(())
}
