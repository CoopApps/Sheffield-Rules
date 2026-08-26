use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let sql_path = "D:/projects/Saturday at Three/assign_pyramid.sql";

    println!("Opening database: {}", db_path);
    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?;
    let pool = SqlitePool::connect_with(connect_options).await?;

    println!("Reading SQL file: {}", sql_path);
    let sql = std::fs::read_to_string(sql_path)?;

    println!("Assigning clubs to pyramid...");
    sqlx::raw_sql(&sql).execute(&pool).await?;

    // Count assignments
    let (main_count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_league_clubs WHERE is_reserve_team = 0")
        .fetch_one(&pool)
        .await?;

    let (reserve_count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_league_clubs WHERE is_reserve_team = 1")
        .fetch_one(&pool)
        .await?;

    println!("✓ Success!");
    println!("  {} main clubs assigned to divisions", main_count);
    println!("  {} reserve clubs assigned to reserve divisions", reserve_count);
    println!("  {} total assignments", main_count + reserve_count);

    pool.close().await;
    Ok(())
}
