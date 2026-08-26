use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("Unmatching all businesses from people...");

    let result = sqlx::query("UPDATE sheffield_businesses SET person_id = NULL WHERE person_id IS NOT NULL")
        .execute(&pool)
        .await?;

    println!("✓ Successfully unmatched {} businesses", result.rows_affected());

    // Verify
    let unmatched: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_businesses WHERE person_id IS NULL"
    )
    .fetch_one(&pool)
    .await?;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_businesses"
    )
    .fetch_one(&pool)
    .await?;

    println!("Verification:");
    println!("  Unmatched businesses: {}", unmatched.0);
    println!("  Total businesses: {}", total.0);

    if unmatched.0 == total.0 {
        println!("✓ All businesses are now unmatched!");
    } else {
        println!("⚠ Warning: {} businesses still have matches", total.0 - unmatched.0);
    }

    Ok(())
}
