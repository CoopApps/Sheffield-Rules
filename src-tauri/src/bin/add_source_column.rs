use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("Adding 'source' column to unmatched_sheffieldcensus table...");

    // Add the source column if it doesn't exist
    sqlx::query(
        "ALTER TABLE unmatched_sheffieldcensus ADD COLUMN source TEXT"
    )
    .execute(&pool)
    .await?;

    println!("✓ Successfully added 'source' column");

    Ok(())
}
