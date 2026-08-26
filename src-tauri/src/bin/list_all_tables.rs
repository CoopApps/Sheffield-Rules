use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("ALL TABLES IN DATABASE");
    println!("========================================\n");

    let tables: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"
    )
    .fetch_all(&pool)
    .await?;

    println!("Found {} tables:\n", tables.len());
    for (name,) in &tables {
        println!("  {}", name);
    }

    // Now let's check for tables that might contain piece/folio information
    println!("\n========================================");
    println!("CHECKING TABLE SCHEMAS");
    println!("========================================\n");

    for (table_name,) in &tables {
        if table_name.contains("sheffield") ||
           table_name.contains("genealogy") ||
           table_name.contains("census") ||
           table_name.contains("people") ||
           table_name.contains("ancestry") {

            println!("\n--- {} ---", table_name);

            let columns: Vec<(i64, String, String, i64, Option<String>, i64)> = sqlx::query_as(
                &format!("PRAGMA table_info({})", table_name)
            )
            .fetch_all(&pool)
            .await?;

            for (_, col_name, col_type, _, _, _) in &columns {
                println!("  {} ({})", col_name, col_type);
            }
        }
    }

    Ok(())
}
