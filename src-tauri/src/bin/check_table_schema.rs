use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";

    println!("Opening database: {}", db_path);
    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?;
    let pool = SqlitePool::connect_with(connect_options).await?;

    println!("\n=== Checking sheffield_footballers table schema ===\n");

    let columns: Vec<(i32, String, String, i32, Option<String>, i32)> = sqlx::query_as(
        "PRAGMA table_info(sheffield_footballers)"
    )
    .fetch_all(&pool)
    .await?;

    println!("{:<5} {:<35} {:<15} {:<10} {:<15} {:<5}", "cid", "name", "type", "notnull", "dflt_value", "pk");
    println!("{}", "-".repeat(90));

    for (cid, name, col_type, notnull, dflt_value, pk) in &columns {
        println!("{:<5} {:<35} {:<15} {:<10} {:<15} {:<5}",
            cid,
            name,
            col_type,
            notnull,
            dflt_value.as_deref().unwrap_or("NULL"),
            pk
        );
    }

    // Specifically check for pace and current_ability
    println!("\n=== Checking for specific columns ===");
    let has_pace = columns.iter().any(|(_, name, _, _, _, _)| name == "pace");
    let has_current_ability = columns.iter().any(|(_, name, _, _, _, _)| name == "current_ability");

    println!("Has 'pace' column: {}", has_pace);
    println!("Has 'current_ability' column: {}", has_current_ability);

    Ok(())
}
