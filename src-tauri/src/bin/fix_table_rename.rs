use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("Checking for temp table...\n");

    // Check what tables exist
    let tables: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM sqlite_master WHERE type='table' AND name LIKE '%genealogy%'"
    )
    .fetch_all(&pool)
    .await?;

    println!("Genealogy-related tables:");
    for (name,) in &tables {
        println!("  {}", name);
    }

    // If temp table exists, rename it
    if tables.iter().any(|(name,)| name == "unmatched_genealogy_temp") {
        println!("\nFound temp table! Renaming...");
        sqlx::query("ALTER TABLE unmatched_genealogy_temp RENAME TO unmatched_genealogy")
            .execute(&pool)
            .await?;
        println!("Successfully renamed!");

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unmatched_genealogy")
            .fetch_one(&pool)
            .await?;
        println!("Records in unmatched_genealogy: {}", count.0);
    } else {
        println!("\nNo temp table found.");
    }

    Ok(())
}
