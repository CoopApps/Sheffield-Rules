use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("SAMPLE HOUSEHOLD MEMBERS (first 10):");
    println!("========================================\n");

    let households: Vec<(String, String)> = sqlx::query_as(
        "SELECT name, census_household_members FROM sheffield_people WHERE census_household_members IS NOT NULL AND census_household_members != '' LIMIT 10"
    )
    .fetch_all(&pool)
    .await?;

    for (i, (name, members)) in households.iter().enumerate() {
        println!("{}. Household head: {}", i + 1, name);
        println!("   Members: {}\n", members);
    }

    println!("\nSAMPLE WOMEN FROM UNMATCHED_GENEALOGY (first 10):");
    println!("========================================\n");

    let women: Vec<(String, Option<i64>)> = sqlx::query_as(
        "SELECT name, birth_year FROM unmatched_genealogy WHERE spouse IS NULL OR spouse = '' LIMIT 10"
    )
    .fetch_all(&pool)
    .await?;

    for (i, (name, birth_year)) in women.iter().enumerate() {
        println!("{}. {} (born {})", i + 1, name, birth_year.map(|y| y.to_string()).unwrap_or("?".to_string()));
    }

    Ok(())
}
