use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("Checking genealogy matching status...\n");

    // List all tables
    let tables: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"
    )
    .fetch_all(&pool)
    .await?;

    println!("All tables in database:");
    for (table,) in &tables {
        println!("  - {}", table);
    }

    // Check if there's a genealogy-related table
    println!("\nSearching for genealogy-related tables...");
    let genealogy_tables: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM sqlite_master WHERE type='table' AND name LIKE '%genealog%'"
    )
    .fetch_all(&pool)
    .await?;

    if genealogy_tables.is_empty() {
        println!("  No genealogy tables found");
    } else {
        for (table,) in &genealogy_tables {
            println!("  Found: {}", table);

            // Count records
            let count_query = format!("SELECT COUNT(*) FROM {}", table);
            let count: (i64,) = sqlx::query_as(&count_query)
                .fetch_one(&pool)
                .await?;
            println!("    Records: {}", count.0);
        }
    }

    // Check for any matching/linking tables
    println!("\nSearching for match-related tables...");
    let match_tables: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM sqlite_master WHERE type='table' AND name LIKE '%match%'"
    )
    .fetch_all(&pool)
    .await?;

    if match_tables.is_empty() {
        println!("  No match tables found");
    } else {
        for (table,) in &match_tables {
            println!("  Found: {}", table);

            // Count records
            let count_query = format!("SELECT COUNT(*) FROM {}", table);
            let count: (i64,) = sqlx::query_as(&count_query)
                .fetch_one(&pool)
                .await?;
            println!("    Records: {}", count.0);
        }
    }

    // Check if player_id column in sheffield_people has any values
    let player_linked: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_people WHERE player_id IS NOT NULL"
    )
    .fetch_one(&pool)
    .await?;

    println!("\nPeople with player_id links: {}", player_linked.0);

    // Check total people now
    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_people"
    )
    .fetch_one(&pool)
    .await?;

    println!("Total people remaining: {}", total.0);

    Ok(())
}
