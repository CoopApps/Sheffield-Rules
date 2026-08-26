use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite:D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(database_url).await?;

    println!("========================================");
    println!("CLEARING BUSINESS COLUMNS");
    println!("========================================\n");

    // Count records before clearing
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy
         WHERE business_surname IS NOT NULL
         OR business_forename IS NOT NULL
         OR business_title IS NOT NULL
         OR business_occupation IS NOT NULL
         OR business_address IS NOT NULL
         OR business_year IS NOT NULL
         OR business_source IS NOT NULL"
    )
    .fetch_one(&pool)
    .await?;

    println!("Records with business data: {}\n", count.0);

    // Clear all business columns
    println!("Clearing business columns...");
    let result = sqlx::query(
        "UPDATE unmatched_genealogy
         SET business_surname = NULL,
             business_forename = NULL,
             business_title = NULL,
             business_occupation = NULL,
             business_address = NULL,
             business_year = NULL,
             business_source = NULL"
    )
    .execute(&pool)
    .await?;

    println!("Updated {} records\n", result.rows_affected());

    // Verify clearing
    let verify: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy
         WHERE business_surname IS NOT NULL
         OR business_forename IS NOT NULL
         OR business_title IS NOT NULL
         OR business_occupation IS NOT NULL
         OR business_address IS NOT NULL
         OR business_year IS NOT NULL
         OR business_source IS NOT NULL"
    )
    .fetch_one(&pool)
    .await?;

    println!("========================================");
    println!("CLEARING COMPLETE");
    println!("========================================");
    println!("Records with business data after clearing: {}", verify.0);

    if verify.0 == 0 {
        println!("✓ All business columns successfully cleared");
    } else {
        println!("⚠ Warning: {} records still have business data", verify.0);
    }

    Ok(())
}
