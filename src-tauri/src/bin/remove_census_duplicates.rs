use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("REMOVING DUPLICATE CENSUS RECORDS");
    println!("========================================\n");

    // Count before
    let count_before: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_sheffieldcensus"
    )
    .fetch_one(&pool)
    .await?;
    println!("Records before deduplication: {}\n", count_before.0);

    // Find duplicates
    println!("Finding duplicates based on: name + piece + folio + age\n");

    let duplicates: Vec<(String, String, String, Option<i64>, i64)> = sqlx::query_as(
        "SELECT
            LOWER(TRIM(name)) as name,
            TRIM(COALESCE(piece, '')) as piece,
            TRIM(COALESCE(folio, '')) as folio,
            age,
            COUNT(*) as count
         FROM unmatched_sheffieldcensus
         WHERE name IS NOT NULL AND TRIM(name) != ''
         GROUP BY LOWER(TRIM(name)), TRIM(COALESCE(piece, '')), TRIM(COALESCE(folio, '')), age
         HAVING COUNT(*) > 1
         ORDER BY count DESC"
    )
    .fetch_all(&pool)
    .await?;

    println!("Found {} unique combinations with duplicates\n", duplicates.len());

    if duplicates.is_empty() {
        println!("✓ No duplicates found!");
        return Ok(());
    }

    let total_duplicates: i64 = duplicates.iter().map(|(_, _, _, _, count)| count - 1).sum();
    println!("Total duplicate records to remove: {}\n", total_duplicates);

    println!("Removing duplicates (keeping the first record of each group)...\n");

    let mut removed_count = 0;

    for (i, (name, piece, folio, age, count)) in duplicates.iter().enumerate() {
        if (i + 1) % 1000 == 0 {
            println!("Processing group {}/{}... (Removed: {})", i + 1, duplicates.len(), removed_count);
        }

        // For each duplicate group, keep only the record with the smallest ID (first inserted)
        // and delete all others
        let deleted: u64 = sqlx::query(
            "DELETE FROM unmatched_sheffieldcensus
             WHERE id NOT IN (
                 SELECT id FROM unmatched_sheffieldcensus
                 WHERE LOWER(TRIM(name)) = LOWER(TRIM(?))
                 AND TRIM(COALESCE(piece, '')) = TRIM(?)
                 AND TRIM(COALESCE(folio, '')) = TRIM(?)
                 AND (age = ? OR (age IS NULL AND ? IS NULL))
                 ORDER BY id ASC
                 LIMIT 1
             )
             AND LOWER(TRIM(name)) = LOWER(TRIM(?))
             AND TRIM(COALESCE(piece, '')) = TRIM(?)
             AND TRIM(COALESCE(folio, '')) = TRIM(?)
             AND (age = ? OR (age IS NULL AND ? IS NULL))"
        )
        .bind(name)
        .bind(piece)
        .bind(folio)
        .bind(age)
        .bind(age)
        .bind(name)
        .bind(piece)
        .bind(folio)
        .bind(age)
        .bind(age)
        .execute(&pool)
        .await?
        .rows_affected();

        removed_count += deleted;
    }

    println!("\n✓ Removed {} duplicate records\n", removed_count);

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
