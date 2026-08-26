use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";

    println!("Opening database: {}", db_path);
    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?;
    let pool = SqlitePool::connect_with(connect_options).await?;

    // Check if footballers already have postcodes via sheffield_people
    println!("\n=== Checking Postcodes in sheffield_people (linked to footballers) ===");
    let sample: Vec<(i64, String, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT 
            f.id,
            p.surname || ', ' || p.first_name as name,
            p.postcode,
            p.ecclesiastical_parish
         FROM sheffield_footballers f
         JOIN sheffield_people p ON f.person_id = p.unique_id
         LIMIT 20"
    )
    .fetch_all(&pool)
    .await?;

    println!("\nSample footballers with their postcodes:");
    for (id, name, postcode, parish) in sample {
        println!("  {} | {} | Postcode: {:?} | Parish: {:?}", 
            id, name, postcode.unwrap_or("NULL".to_string()), parish.unwrap_or("NULL".to_string()));
    }

    // Count how many have postcodes
    println!("\n=== Postcode Coverage ===");
    let (total,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_footballers"
    )
    .fetch_one(&pool)
    .await?;

    let (with_postcode,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) 
         FROM sheffield_footballers f
         JOIN sheffield_people p ON f.person_id = p.unique_id
         WHERE p.postcode IS NOT NULL AND p.postcode != ''"
    )
    .fetch_one(&pool)
    .await?;

    println!("Total footballers: {}", total);
    println!("With postcodes: {} ({:.1}%)", with_postcode, (with_postcode as f64 / total as f64) * 100.0);
    println!("Without postcodes: {}", total - with_postcode);

    pool.close().await;
    Ok(())
}
