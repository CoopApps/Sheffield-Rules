use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    let census_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_sheffieldcensus"
    )
    .fetch_one(&pool)
    .await?;

    let genealogy_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy"
    )
    .fetch_one(&pool)
    .await?;

    let people_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_people"
    )
    .fetch_one(&pool)
    .await?;

    let matched_source_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_people WHERE source = 'matched_census_genealogy'"
    )
    .fetch_one(&pool)
    .await?;

    println!("========================================");
    println!("UPDATED COUNTS:");
    println!("========================================");
    println!("unmatched_sheffieldcensus: {}", census_count.0);
    println!("unmatched_genealogy: {}", genealogy_count.0);
    println!("sheffield_people (total): {}", people_count.0);
    println!("sheffield_people (matched_census_genealogy): {}", matched_source_count.0);
    println!("========================================");

    Ok(())
}
