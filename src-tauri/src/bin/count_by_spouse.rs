use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("Analyzing unmatched_genealogy by spouse field...\n");

    // Total count
    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy"
    )
    .fetch_one(&pool)
    .await?;

    // Count with spouse data (likely women)
    let with_spouse: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy WHERE spouse IS NOT NULL AND spouse != ''"
    )
    .fetch_one(&pool)
    .await?;

    // Count without spouse data (likely men)
    let without_spouse: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy WHERE spouse IS NULL OR spouse = ''"
    )
    .fetch_one(&pool)
    .await?;

    println!("Total records: {}", total.0);
    println!("With spouse data (likely women): {}", with_spouse.0);
    println!("Without spouse data (likely men): {}", without_spouse.0);
    println!("\nPercentage breakdown:");
    println!("  Women: {:.1}%", (with_spouse.0 as f64 / total.0 as f64) * 100.0);
    println!("  Men: {:.1}%", (without_spouse.0 as f64 / total.0 as f64) * 100.0);

    // Sample records with spouse
    println!("\nSample records WITH spouse (women):");
    let with_spouse_samples: Vec<(String, Option<String>, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT name, spouse, profession, relation FROM unmatched_genealogy WHERE spouse IS NOT NULL AND spouse != '' LIMIT 5"
    )
    .fetch_all(&pool)
    .await?;

    for (name, spouse, profession, relation) in with_spouse_samples {
        println!("  {} - Spouse: {} - Profession: {} - Relation: {}",
            name,
            spouse.unwrap_or("?".to_string()),
            profession.unwrap_or("None".to_string()),
            relation.unwrap_or("?".to_string())
        );
    }

    // Sample records without spouse
    println!("\nSample records WITHOUT spouse (men):");
    let without_spouse_samples: Vec<(String, Option<String>, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT name, spouse, profession, relation FROM unmatched_genealogy WHERE spouse IS NULL OR spouse = '' LIMIT 5"
    )
    .fetch_all(&pool)
    .await?;

    for (name, spouse, profession, relation) in without_spouse_samples {
        println!("  {} - Spouse: {} - Profession: {} - Relation: {}",
            name,
            spouse.as_deref().unwrap_or("None"),
            profession.unwrap_or("None".to_string()),
            relation.unwrap_or("?".to_string())
        );
    }

    Ok(())
}
