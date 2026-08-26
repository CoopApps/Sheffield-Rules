use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite:D:/projects/Saturday at Three/saturday_at_three.db";
    let pool = SqlitePool::connect(database_url).await?;

    println!("========================================");
    println!("REPLACING unmatched_genealogy WITH SORTED VERSION");
    println!("========================================\n");

    // Check both tables exist and have same record count
    let original_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM unmatched_genealogy")
        .fetch_one(&pool)
        .await?;

    let sorted_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM genealogy_sorted")
        .fetch_one(&pool)
        .await?;

    println!("unmatched_genealogy records: {}", original_count);
    println!("genealogy_sorted records: {}", sorted_count);

    if original_count != sorted_count {
        println!("\n⚠ WARNING: Record counts don't match!");
        println!("Aborting to preserve data integrity.");
        return Ok(());
    }

    println!("\n✓ Record counts match. Proceeding with replacement...\n");

    // Drop the original table
    println!("Dropping unmatched_genealogy...");
    sqlx::query("DROP TABLE IF EXISTS unmatched_genealogy")
        .execute(&pool)
        .await?;
    println!("✓ Dropped");

    // Rename sorted table to replace it
    println!("Renaming genealogy_sorted to unmatched_genealogy...");
    sqlx::query("ALTER TABLE genealogy_sorted RENAME TO unmatched_genealogy")
        .execute(&pool)
        .await?;
    println!("✓ Renamed");

    // Verify
    let final_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM unmatched_genealogy")
        .fetch_one(&pool)
        .await?;

    println!("\n========================================");
    println!("REPLACEMENT COMPLETE");
    println!("========================================");
    println!("unmatched_genealogy now contains {} sorted records", final_count);

    Ok(())
}
