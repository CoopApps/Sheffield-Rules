use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite:D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(database_url).await?;

    println!("========================================");
    println!("BACKING UP CLEANED TABLES");
    println!("========================================\n");

    // Drop backup tables if they exist
    println!("Dropping old backup tables if they exist...");
    sqlx::query("DROP TABLE IF EXISTS unmatched_sheffieldcensus_backup")
        .execute(&pool)
        .await?;
    sqlx::query("DROP TABLE IF EXISTS unmatched_genealogy_backup")
        .execute(&pool)
        .await?;
    println!("Old backups dropped.\n");

    // Backup census table
    println!("Creating backup of unmatched_sheffieldcensus...");
    sqlx::query("CREATE TABLE unmatched_sheffieldcensus_backup AS SELECT * FROM unmatched_sheffieldcensus")
        .execute(&pool)
        .await?;

    let census_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unmatched_sheffieldcensus_backup")
        .fetch_one(&pool)
        .await?;
    println!("✓ Census backup created: {} records\n", census_count.0);

    // Backup genealogy table
    println!("Creating backup of unmatched_genealogy...");
    sqlx::query("CREATE TABLE unmatched_genealogy_backup AS SELECT * FROM unmatched_genealogy")
        .execute(&pool)
        .await?;

    let genealogy_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unmatched_genealogy_backup")
        .fetch_one(&pool)
        .await?;
    println!("✓ Genealogy backup created: {} records\n", genealogy_count.0);

    println!("========================================");
    println!("BACKUP COMPLETE");
    println!("========================================");
    println!("Tables backed up:");
    println!("  - unmatched_sheffieldcensus → unmatched_sheffieldcensus_backup");
    println!("  - unmatched_genealogy → unmatched_genealogy_backup");

    Ok(())
}
