use sqlx::sqlite::SqlitePool;
use sqlx::Row;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let source_db = "sqlite:D:/projects/Saturday at Three/saturday_at_three.db";
    let target_db = "sqlite:D:/projects/Saturday at Three/Sheffield1867.db";

    println!("========================================");
    println!("CHECKING TABLES IN saturday_at_three.db");
    println!("========================================\n");

    let source_pool = SqlitePool::connect(source_db).await?;

    // Get all tables
    let tables: Vec<String> = sqlx::query(
        "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"
    )
    .fetch_all(&source_pool)
    .await?
    .iter()
    .map(|row| row.get(0))
    .collect();

    println!("Found {} tables in saturday_at_three.db:\n", tables.len());

    let mut genealogy_found = false;
    let mut census_found = false;
    let mut genealogy_count = 0;
    let mut census_count = 0;

    for table in &tables {
        // Get row count
        let count: i64 = sqlx::query(&format!("SELECT COUNT(*) as count FROM {}", table))
            .fetch_one(&source_pool)
            .await?
            .get(0);

        println!("  {} - {} rows", table, count);

        if table == "unmatched_genealogy" {
            genealogy_found = true;
            genealogy_count = count;
        }
        if table == "unmatched_sheffield_census" {
            census_found = true;
            census_count = count;
        }
    }

    println!("\n========================================");
    println!("TABLE CHECK RESULTS");
    println!("========================================");
    println!("unmatched_genealogy: {} (rows: {})", if genealogy_found { "FOUND" } else { "NOT FOUND" }, genealogy_count);
    println!("unmatched_sheffield_census: {} (rows: {})\n", if census_found { "FOUND" } else { "NOT FOUND" }, census_count);

    if !genealogy_found && !census_found {
        println!("Neither table found in saturday_at_three.db. Exiting.");
        return Ok(());
    }

    // Get schema for found tables
    if genealogy_found {
        println!("Schema for unmatched_genealogy:");
        let schema: String = sqlx::query("SELECT sql FROM sqlite_master WHERE name='unmatched_genealogy'")
            .fetch_one(&source_pool)
            .await?
            .get(0);
        println!("{}\n", schema);
    }

    if census_found {
        println!("Schema for unmatched_sheffield_census:");
        let schema: String = sqlx::query("SELECT sql FROM sqlite_master WHERE name='unmatched_sheffield_census'")
            .fetch_one(&source_pool)
            .await?
            .get(0);
        println!("{}\n", schema);
    }

    println!("========================================");
    println!("MOVING TABLES TO Sheffield1867.db");
    println!("========================================\n");

    let target_pool = SqlitePool::connect(target_db).await?;

    // Move unmatched_genealogy
    if genealogy_found {
        println!("Moving unmatched_genealogy...");

        // Get schema
        let create_sql: String = sqlx::query("SELECT sql FROM sqlite_master WHERE name='unmatched_genealogy'")
            .fetch_one(&source_pool)
            .await?
            .get(0);

        // Drop if exists in target
        sqlx::query("DROP TABLE IF EXISTS unmatched_genealogy")
            .execute(&target_pool)
            .await?;

        // Create table in target
        sqlx::query(&create_sql)
            .execute(&target_pool)
            .await?;

        // Copy data
        let rows: Vec<(i64, String, Option<String>, Option<i64>, Option<i64>, Option<String>, Option<String>, Option<String>)> =
            sqlx::query_as("SELECT id, name, address, birth_year, age, profession, relation, spouse FROM unmatched_genealogy")
            .fetch_all(&source_pool)
            .await?;

        for row in rows {
            sqlx::query(
                "INSERT INTO unmatched_genealogy (id, name, address, birth_year, age, profession, relation, spouse)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(row.0)
            .bind(row.1)
            .bind(row.2)
            .bind(row.3)
            .bind(row.4)
            .bind(row.5)
            .bind(row.6)
            .bind(row.7)
            .execute(&target_pool)
            .await?;
        }

        println!("  ✓ Moved {} rows", genealogy_count);
    }

    // Move unmatched_sheffield_census
    if census_found {
        println!("Moving unmatched_sheffield_census...");

        // Get schema
        let create_sql: String = sqlx::query("SELECT sql FROM sqlite_master WHERE name='unmatched_sheffield_census'")
            .fetch_one(&source_pool)
            .await?
            .get(0);

        // Drop if exists in target
        sqlx::query("DROP TABLE IF EXISTS unmatched_sheffield_census")
            .execute(&target_pool)
            .await?;

        // Create table in target
        sqlx::query(&create_sql)
            .execute(&target_pool)
            .await?;

        // Copy data - adjust columns based on actual schema
        let rows: Vec<(i64, String, Option<String>, Option<i64>, Option<i64>, Option<String>, Option<String>, Option<String>)> =
            sqlx::query_as("SELECT id, name, address, birth_year, age, profession, relation, spouse FROM unmatched_sheffield_census")
            .fetch_all(&source_pool)
            .await?;

        for row in rows {
            sqlx::query(
                "INSERT INTO unmatched_sheffield_census (id, name, address, birth_year, age, profession, relation, spouse)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(row.0)
            .bind(row.1)
            .bind(row.2)
            .bind(row.3)
            .bind(row.4)
            .bind(row.5)
            .bind(row.6)
            .bind(row.7)
            .execute(&target_pool)
            .await?;
        }

        println!("  ✓ Moved {} rows", census_count);
    }

    println!("\n========================================");
    println!("MIGRATION COMPLETE");
    println!("========================================");
    println!("Tables successfully moved to Sheffield1867.db");

    Ok(())
}
