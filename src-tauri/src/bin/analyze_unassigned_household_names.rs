use sqlx::sqlite::SqlitePool;
use sqlx::Row;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("SCHEMA FOR unmatched_sheffieldcensus");
    println!("========================================\n");

    // Get table schema
    let schema: Vec<(i64, String, String, i64, Option<String>, i64)> = sqlx::query_as(
        "PRAGMA table_info(unmatched_sheffieldcensus)"
    )
    .fetch_all(&pool)
    .await?;

    println!("Columns:");
    for (_, name, col_type, not_null, default_val, pk) in &schema {
        println!("  {} ({}) - NOT NULL: {}, PK: {}, DEFAULT: {:?}",
            name, col_type, not_null, pk, default_val);
    }

    println!("\n========================================");
    println!("SAMPLE DATA");
    println!("========================================\n");

    // Get a few sample rows to see the data
    let sample = sqlx::query("SELECT * FROM unmatched_sheffieldcensus LIMIT 3")
        .fetch_all(&pool)
        .await?;

    for row in sample {
        println!("Row:");
        for (i, col) in schema.iter().enumerate() {
            let value: Option<String> = row.try_get(i).ok();
            println!("  {}: {:?}", col.1, value);
        }
        println!();
    }

    Ok(())
}
