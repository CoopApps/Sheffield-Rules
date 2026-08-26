use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite:D:/projects/Saturday at Three/saturday_at_three.db";
    let pool = SqlitePool::connect(database_url).await?;

    println!("========================================");
    println!("LISTING ALL TABLES IN DATABASE");
    println!("========================================\n");

    let tables: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"
    )
    .fetch_all(&pool)
    .await?;

    println!("Found {} tables:\n", tables.len());

    for (i, (table_name,)) in tables.iter().enumerate() {
        println!("{}. {}", i + 1, table_name);

        // Get count for each table
        let count: i64 = sqlx::query_scalar(&format!("SELECT COUNT(*) FROM {}", table_name))
            .fetch_one(&pool)
            .await
            .unwrap_or(0);

        println!("   Records: {}\n", count);
    }

    Ok(())
}
