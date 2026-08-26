use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("REMOVING DUPLICATES FROM UNMATCHED_GENEALOGY");
    println!("========================================\n");

    // Check total records before
    let total_before: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unmatched_genealogy")
        .fetch_one(&pool)
        .await?;
    println!("Total records before cleanup: {}\n", total_before.0);

    // Count duplicates that will be removed
    let duplicates_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) - COUNT(DISTINCT name || '|' || address || '|' || COALESCE(birth_year, 0))
         FROM unmatched_genealogy"
    )
    .fetch_one(&pool)
    .await?;

    println!("Duplicate records to be removed: {}\n", duplicates_count.0);

    // Create a temporary table with unique records only
    println!("Creating temporary table with unique records...");
    sqlx::query(
        "CREATE TABLE unmatched_genealogy_temp AS
         SELECT MIN(id) as id, name, address, birth_year, age, profession, relation, spouse
         FROM unmatched_genealogy
         GROUP BY name, address, birth_year"
    )
    .execute(&pool)
    .await?;

    println!("Temporary table created successfully!\n");

    // Check count in temp table
    let temp_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unmatched_genealogy_temp")
        .fetch_one(&pool)
        .await?;
    println!("Unique records in temp table: {}\n", temp_count.0);

    // Drop old table and rename temp table
    println!("Replacing original table with deduplicated version...");
    sqlx::query("DROP TABLE unmatched_genealogy")
        .execute(&pool)
        .await?;

    sqlx::query("ALTER TABLE unmatched_genealogy_temp RENAME TO unmatched_genealogy")
        .execute(&pool)
        .await?;

    println!("Table replaced successfully!\n");

    // Check total records after
    let total_after: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unmatched_genealogy")
        .fetch_one(&pool)
        .await?;

    println!("\n========================================");
    println!("SUMMARY");
    println!("========================================");
    println!("Records before: {}", total_before.0);
    println!("Records after: {}", total_after.0);
    println!("Records removed: {}", total_before.0 - total_after.0);
    println!("========================================\n");

    Ok(())
}
