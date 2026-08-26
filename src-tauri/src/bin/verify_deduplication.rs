use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("VERIFYING DEDUPLICATION");
    println!("========================================\n");

    // Check total records
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unmatched_genealogy")
        .fetch_one(&pool)
        .await?;
    println!("Total records in unmatched_genealogy: {}\n", total.0);

    // Check for remaining duplicates
    let duplicates: Vec<(String, String, Option<i64>, i64)> = sqlx::query_as(
        "SELECT name, address, birth_year, COUNT(*) as count
         FROM unmatched_genealogy
         GROUP BY name, address, birth_year
         HAVING COUNT(*) > 1
         ORDER BY count DESC"
    )
    .fetch_all(&pool)
    .await?;

    if duplicates.is_empty() {
        println!("✓ No duplicates found! Deduplication successful.");
    } else {
        println!("⚠ Found {} duplicate groups remaining:", duplicates.len());
        for (i, (name, address, birth_year, count)) in duplicates.iter().take(10).enumerate() {
            println!("  {}. {} at {} (birth: {}) - {} duplicates",
                i + 1,
                name,
                address,
                birth_year.map(|y| y.to_string()).unwrap_or("N/A".to_string()),
                count
            );
        }
    }

    println!("\n========================================\n");

    Ok(())
}
