use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("CHECKING ALL INSTITUTIONAL TABLE SCHEMAS");
    println!("========================================\n");

    // Get all table names
    let tables: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"
    )
    .fetch_all(&pool)
    .await?;

    // Check for institutional tables
    let institutional_keywords = vec![
        "workhouse", "asylum", "hospital", "prison", "jail", "gaol",
        "orphanage", "barracks", "poorhouse", "poor_house", "institution",
        "infirmary"
    ];

    for (table_name,) in &tables {
        let lower_name = table_name.to_lowercase();
        let mut is_institutional = false;

        for keyword in &institutional_keywords {
            if lower_name.contains(keyword) {
                is_institutional = true;
                break;
            }
        }

        if is_institutional {
            println!("========================================");
            println!("TABLE: {}", table_name);
            println!("========================================");

            let schema: Vec<(i64, String, String, i64, Option<String>, i64)> = sqlx::query_as(
                &format!("PRAGMA table_info({})", table_name)
            )
            .fetch_all(&pool)
            .await?;

            for (_, name, col_type, _, _, _) in &schema {
                println!("  - {} ({})", name, col_type);
            }

            let count: (i64,) = sqlx::query_as(&format!("SELECT COUNT(*) FROM {}", table_name))
                .fetch_one(&pool)
                .await?;
            println!("  Records: {}", count.0);
            println!();
        }
    }

    Ok(())
}
