use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("Analyzing duplicates in sheffield_people...\n");

    // Count total people before
    let total_before: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_people")
        .fetch_one(&pool)
        .await?;
    println!("Total people before: {}", total_before.0);

    // Find duplicates (same name, birth_year, street_address, and profession)
    let duplicates: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) - COUNT(DISTINCT name || COALESCE(birth_year, 0) || COALESCE(street_address, '') || COALESCE(profession, ''))
        FROM sheffield_people
        "#
    )
    .fetch_one(&pool)
    .await?;
    println!("Duplicate records found: {}", duplicates.0);

    if duplicates.0 == 0 {
        println!("\n✓ No duplicates found!");
        return Ok(());
    }

    println!("\nDeleting duplicates, keeping the first occurrence of each person...");

    // Delete duplicates - keep the MIN(id) for each unique combination
    let result = sqlx::query(
        r#"
        DELETE FROM sheffield_people
        WHERE id NOT IN (
            SELECT MIN(id)
            FROM sheffield_people
            GROUP BY
                name,
                COALESCE(birth_year, 0),
                COALESCE(street_address, ''),
                COALESCE(profession, '')
        )
        "#
    )
    .execute(&pool)
    .await?;

    println!("✓ Deleted {} duplicate records", result.rows_affected());

    // Count total people after
    let total_after: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_people")
        .fetch_one(&pool)
        .await?;

    println!("\n========================================");
    println!("RESULTS:");
    println!("  Before: {} people", total_before.0);
    println!("  After:  {} people", total_after.0);
    println!("  Removed: {} duplicates", total_before.0 - total_after.0);
    println!("========================================");

    // Show some example remaining records
    println!("\nSample of remaining records:");
    let samples: Vec<(String, Option<i64>, Option<String>, Option<String>)> = sqlx::query_as(
        r#"
        SELECT name, birth_year, street_address, profession
        FROM sheffield_people
        WHERE name LIKE '%Abraham Adams%'
        LIMIT 5
        "#
    )
    .fetch_all(&pool)
    .await?;

    for (name, birth_year, address, profession) in samples {
        println!("  {} (b.{}) - {} - {}",
            name,
            birth_year.map(|y| y.to_string()).unwrap_or("?".to_string()),
            address.unwrap_or("No address".to_string()),
            profession.unwrap_or("No profession".to_string())
        );
    }

    Ok(())
}
