use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("CLEARING TABLES");
    println!("========================================\n");

    // Get current counts before clearing
    println!("Current record counts:");

    let sheffield_people: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_people")
        .fetch_one(&pool)
        .await?;
    println!("  sheffield_people: {}", sheffield_people.0);

    let unmatched_census: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unmatched_sheffieldcensus")
        .fetch_one(&pool)
        .await?;
    println!("  unmatched_sheffieldcensus: {}", unmatched_census.0);

    let unmatched_genealogy: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unmatched_genealogy")
        .fetch_one(&pool)
        .await?;
    println!("  unmatched_genealogy: {}", unmatched_genealogy.0);

    println!("\n========================================");
    println!("DELETING ALL RECORDS...");
    println!("========================================\n");

    // Clear sheffield_people
    sqlx::query("DELETE FROM sheffield_people")
        .execute(&pool)
        .await?;
    println!("✓ Cleared sheffield_people");

    // Clear unmatched_sheffieldcensus
    sqlx::query("DELETE FROM unmatched_sheffieldcensus")
        .execute(&pool)
        .await?;
    println!("✓ Cleared unmatched_sheffieldcensus");

    // Clear unmatched_genealogy
    sqlx::query("DELETE FROM unmatched_genealogy")
        .execute(&pool)
        .await?;
    println!("✓ Cleared unmatched_genealogy");

    println!("\n========================================");
    println!("VERIFICATION");
    println!("========================================\n");

    // Verify counts after clearing
    let sheffield_people_after: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_people")
        .fetch_one(&pool)
        .await?;
    println!("  sheffield_people: {}", sheffield_people_after.0);

    let unmatched_census_after: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unmatched_sheffieldcensus")
        .fetch_one(&pool)
        .await?;
    println!("  unmatched_sheffieldcensus: {}", unmatched_census_after.0);

    let unmatched_genealogy_after: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unmatched_genealogy")
        .fetch_one(&pool)
        .await?;
    println!("  unmatched_genealogy: {}", unmatched_genealogy_after.0);

    println!("\n========================================");
    println!("ALL TABLES CLEARED SUCCESSFULLY");
    println!("========================================");

    Ok(())
}
