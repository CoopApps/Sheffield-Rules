use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("SORTING CENSUS TABLE");
    println!("========================================\n");

    println!("Current table structure: insertion order");
    println!("Target structure: sorted by piece, folio, name\n");

    // Count records before
    let count_before: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_sheffieldcensus"
    )
    .fetch_one(&pool)
    .await?;
    println!("Records before sorting: {}\n", count_before.0);

    println!("Step 1: Creating temporary sorted table...");

    // Create a new table with sorted data
    sqlx::query(
        "CREATE TABLE unmatched_sheffieldcensus_sorted AS
         SELECT * FROM unmatched_sheffieldcensus
         ORDER BY
            CAST(piece AS INTEGER) ASC,
            CAST(folio AS INTEGER) ASC,
            LOWER(name) ASC"
    )
    .execute(&pool)
    .await?;

    println!("✓ Created sorted table\n");

    // Count records in sorted table
    let count_sorted: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_sheffieldcensus_sorted"
    )
    .fetch_one(&pool)
    .await?;
    println!("Records in sorted table: {}\n", count_sorted.0);

    println!("Step 2: Dropping original table...");
    sqlx::query("DROP TABLE unmatched_sheffieldcensus")
        .execute(&pool)
        .await?;
    println!("✓ Dropped original table\n");

    println!("Step 3: Renaming sorted table...");
    sqlx::query("ALTER TABLE unmatched_sheffieldcensus_sorted RENAME TO unmatched_sheffieldcensus")
        .execute(&pool)
        .await?;
    println!("✓ Renamed table\n");

    // Verify final count
    let count_after: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_sheffieldcensus"
    )
    .fetch_one(&pool)
    .await?;

    println!("========================================");
    println!("SORTING COMPLETE");
    println!("========================================");
    println!("Records after sorting: {}", count_after.0);
    println!("\nTable is now sorted by:");
    println!("  1. Piece (numeric)");
    println!("  2. Folio (numeric)");
    println!("  3. Name (alphabetic)");
    println!("\nDuplicates with the same piece+folio will now be adjacent!");

    Ok(())
}
