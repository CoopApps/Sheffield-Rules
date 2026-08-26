use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("Checking database state...\n");

    // Check if original table exists
    let tables: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM sqlite_master WHERE type='table' AND name LIKE '%unmatched_sheffield%'"
    )
    .fetch_all(&pool)
    .await?;

    println!("Tables found:");
    for (name,) in &tables {
        println!("  - {}", name);
    }

    // Check if the sorted table exists
    if tables.iter().any(|(name,)| name == "unmatched_sheffieldcensus_sorted") {
        println!("\nSorted table exists. Completing the sort process...\n");

        // Just use the sorted table - rename it
        println!("Renaming sorted table...");
        sqlx::query("ALTER TABLE unmatched_sheffieldcensus_sorted RENAME TO unmatched_sheffieldcensus")
            .execute(&pool)
            .await?;

        println!("✓ Table renamed successfully!");

        // Verify
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM unmatched_sheffieldcensus"
        )
        .fetch_one(&pool)
        .await?;

        println!("\n========================================");
        println!("SORTING COMPLETE");
        println!("========================================");
        println!("Records in sorted table: {}", count.0);
        println!("\nTable is now sorted by:");
        println!("  1. Piece (numeric)");
        println!("  2. Folio (numeric)");
        println!("  3. Name (alphabetic)");
    } else {
        println!("\nNo sorted table found. The original table still exists.");
    }

    Ok(())
}
