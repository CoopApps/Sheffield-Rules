use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("Checking all tables in database...\n");

    let tables: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"
    )
    .fetch_all(&pool)
    .await?;

    println!("Tables in database:");
    for (name,) in &tables {
        let count: (i64,) = sqlx::query_as(&format!("SELECT COUNT(*) FROM {}", name))
            .fetch_one(&pool)
            .await?;
        println!("  {} - {} records", name, count.0);
    }

    Ok(())
}
