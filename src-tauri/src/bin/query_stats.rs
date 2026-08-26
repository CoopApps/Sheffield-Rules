use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    // People with matched businesses
    let matched_people: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT person_id) FROM sheffield_businesses WHERE person_id IS NOT NULL"
    )
    .fetch_one(&pool)
    .await?;

    // Unmatched businesses
    let unmatched_businesses: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_businesses WHERE person_id IS NULL"
    )
    .fetch_one(&pool)
    .await?;

    // Total businesses
    let total_businesses: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_businesses"
    )
    .fetch_one(&pool)
    .await?;

    // Total people
    let total_people: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_people"
    )
    .fetch_one(&pool)
    .await?;

    println!("========================================");
    println!("DATABASE MATCHING STATISTICS");
    println!("========================================");
    println!("People with matched businesses: {}", matched_people.0);
    println!("Unmatched businesses: {}", unmatched_businesses.0);
    println!("Matched businesses: {}", total_businesses.0 - unmatched_businesses.0);
    println!("Total businesses: {}", total_businesses.0);
    println!("Total people: {}", total_people.0);
    println!("========================================");

    Ok(())
}
