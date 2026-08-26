use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    let sheffield_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_people")
        .fetch_one(&pool)
        .await?;

    let unmatched_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unmatched_genealogy")
        .fetch_one(&pool)
        .await?;

    let with_household: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_people WHERE household_head IS NOT NULL")
        .fetch_one(&pool)
        .await?;

    println!("========================================");
    println!("DATABASE COUNTS:");
    println!("========================================");
    println!("sheffield_people: {}", sheffield_count.0);
    println!("unmatched_genealogy: {}", unmatched_count.0);
    println!("sheffield_people with household_head: {}", with_household.0);
    println!("========================================");

    Ok(())
}
