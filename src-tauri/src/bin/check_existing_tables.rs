use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("CHECKING EXISTING TABLES");
    println!("========================================\n");

    // Get all table names
    let tables: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"
    )
    .fetch_all(&pool)
    .await?;

    println!("All tables in database:");
    for (table_name,) in &tables {
        println!("  - {}", table_name);
    }

    println!("\n========================================");
    println!("INSTITUTIONAL TABLES:");
    println!("========================================\n");

    // Check for specific institutional tables
    let institutional_keywords = vec![
        "workhouse", "asylum", "hospital", "prison", "jail", "gaol",
        "orphanage", "barracks", "poorhouse", "poor_house", "institution",
        "infirmary"
    ];

    let mut found_institutional = Vec::new();

    for (table_name,) in &tables {
        let lower_name = table_name.to_lowercase();
        for keyword in &institutional_keywords {
            if lower_name.contains(keyword) {
                found_institutional.push(table_name.clone());
                break;
            }
        }
    }

    if found_institutional.is_empty() {
        println!("No institutional tables found.");
    } else {
        println!("Found institutional tables:");
        for table in &found_institutional {
            // Get count
            let count: (i64,) = sqlx::query_as(&format!("SELECT COUNT(*) FROM {}", table))
                .fetch_one(&pool)
                .await?;
            println!("  - {} ({} records)", table, count.0);
        }
    }

    println!("\n========================================");

    Ok(())
}
