use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("Adding household_head column to sheffield_people table...");

    sqlx::query("ALTER TABLE sheffield_people ADD COLUMN household_head TEXT")
        .execute(&pool)
        .await?;

    println!("✓ Successfully added household_head column");

    Ok(())
}
