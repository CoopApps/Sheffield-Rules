use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("UNIQUE NAME ANALYSIS");
    println!("========================================\n");

    // Total records
    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy"
    )
    .fetch_one(&pool)
    .await?;

    println!("Total records: {}", total.0);

    // Unique names
    let unique_names: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT name) FROM unmatched_genealogy"
    )
    .fetch_one(&pool)
    .await?;

    println!("Unique names: {}", unique_names.0);

    // Breakdown by relation
    println!("\n--- Breakdown by Relation ---");

    let by_relation: Vec<(String, i64, i64)> = sqlx::query_as(
        "SELECT relation, COUNT(*) as total, COUNT(DISTINCT name) as unique_names
         FROM unmatched_genealogy
         GROUP BY relation
         ORDER BY total DESC"
    )
    .fetch_all(&pool)
    .await?;

    for (relation, total, unique) in by_relation {
        println!("{}: {} records, {} unique names", relation, total, unique);
    }

    // Most common names
    println!("\n--- Most Common Names (Top 20) ---");

    let common_names: Vec<(String, i64)> = sqlx::query_as(
        "SELECT name, COUNT(*) as count
         FROM unmatched_genealogy
         GROUP BY name
         ORDER BY count DESC
         LIMIT 20"
    )
    .fetch_all(&pool)
    .await?;

    for (name, count) in common_names {
        println!("{}: {} occurrences", name, count);
    }

    // Names that appear only once
    let singles: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM (
            SELECT name
            FROM unmatched_genealogy
            GROUP BY name
            HAVING COUNT(*) = 1
         )"
    )
    .fetch_one(&pool)
    .await?;

    println!("\n--- Name Frequency ---");
    println!("Names appearing only once: {}", singles.0);
    println!("Names appearing multiple times: {}", unique_names.0 - singles.0);

    Ok(())
}
