use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("REMOVING DUPLICATE CENSUS RECORDS (FAST)");
    println!("========================================\n");

    // Count before
    let count_before: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_sheffieldcensus"
    )
    .fetch_one(&pool)
    .await?;
    println!("Records before deduplication: {}\n", count_before.0);

    println!("Removing all duplicates in one query...\n");

    // Delete all duplicates except the one with the smallest ID (first inserted) for each group
    // This uses a single DELETE query that's much faster than looping
    let deleted = sqlx::query(
        "DELETE FROM unmatched_sheffieldcensus
         WHERE id NOT IN (
             SELECT MIN(id)
             FROM unmatched_sheffieldcensus
             WHERE name IS NOT NULL AND TRIM(name) != ''
             GROUP BY LOWER(TRIM(name)), TRIM(COALESCE(piece, '')), TRIM(COALESCE(folio, '')), age
         )"
    )
    .execute(&pool)
    .await?
    .rows_affected();

    println!("✓ Removed {} duplicate records\n", deleted);

    // Count after
    let count_after: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_sheffieldcensus"
    )
    .fetch_one(&pool)
    .await?;

    // Verify no duplicates remain
    let remaining_duplicates: Vec<(i64,)> = sqlx::query_as(
        "SELECT COUNT(*) as count
         FROM unmatched_sheffieldcensus
         WHERE name IS NOT NULL AND TRIM(name) != ''
         GROUP BY LOWER(TRIM(name)), TRIM(COALESCE(piece, '')), TRIM(COALESCE(folio, '')), age
         HAVING COUNT(*) > 1"
    )
    .fetch_all(&pool)
    .await?;

    println!("========================================");
    println!("DEDUPLICATION COMPLETE");
    println!("========================================");
    println!("Records before: {}", count_before.0);
    println!("Records after:  {}", count_after.0);
    println!("Records removed: {}", count_before.0 - count_after.0);
    println!();

    if remaining_duplicates.is_empty() {
        println!("✓ No duplicate records remain!");
    } else {
        println!("⚠ Warning: {} duplicate groups still remain", remaining_duplicates.len());
    }

    Ok(())
}
