use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("SHEFFIELD_WORKHOUSE TABLE SCHEMA:");
    println!("========================================\n");

    let schema: Vec<(i64, String, String, i64, Option<String>, i64)> = sqlx::query_as(
        "PRAGMA table_info(sheffield_workhouse)"
    )
    .fetch_all(&pool)
    .await?;

    println!("Columns:");
    for (_, name, col_type, not_null, default_val, pk) in &schema {
        println!("  - {} ({}) | NOT NULL: {} | DEFAULT: {:?} | PK: {}",
            name, col_type, not_null, default_val, pk);
    }

    Ok(())
}
