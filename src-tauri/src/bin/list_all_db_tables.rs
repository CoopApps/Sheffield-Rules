use sqlx::sqlite::SqlitePool;
use sqlx::Row;

async fn list_tables_in_db(db_path: &str, db_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("sqlite:{}", db_path);

    match SqlitePool::connect(&url).await {
        Ok(pool) => {
            println!("\n========================================");
            println!("{}", db_name);
            println!("========================================");

            let tables: Vec<String> = sqlx::query(
                "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"
            )
            .fetch_all(&pool)
            .await?
            .iter()
            .map(|row| row.get(0))
            .collect();

            println!("Found {} tables:\n", tables.len());

            for table in &tables {
                let count: i64 = sqlx::query(&format!("SELECT COUNT(*) as count FROM {}", table))
                    .fetch_one(&pool)
                    .await?
                    .get(0);

                println!("  {} - {} rows", table, count);

                // Highlight census/genealogy tables
                if table.contains("genealogy") || table.contains("census") {
                    println!("    ^^^ FOUND GENEALOGY/CENSUS TABLE!");
                }
            }

            Ok(())
        }
        Err(e) => {
            println!("\n{}: Could not connect - {}", db_name, e);
            Ok(())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("========================================");
    println!("SEARCHING FOR GENEALOGY/CENSUS TABLES");
    println!("========================================");

    // Check all possible database files
    list_tables_in_db("D:/projects/Saturday at Three/saturday_at_three.db", "saturday_at_three.db").await?;
    list_tables_in_db("D:/projects/Saturday at Three/Sheffield1867.db", "Sheffield1867.db").await?;
    list_tables_in_db("D:/projects/Saturday at Three/Sheffield1867_temp.db", "Sheffield1867_temp.db").await?;

    println!("\n========================================");
    println!("SEARCH COMPLETE");
    println!("========================================");

    Ok(())
}
