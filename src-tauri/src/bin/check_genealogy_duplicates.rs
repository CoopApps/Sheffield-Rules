use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("CHECKING FOR DUPLICATES IN UNMATCHED_GENEALOGY");
    println!("========================================\n");

    // First check the schema to see what columns are available
    println!("Checking table schema...");
    let schema: Vec<(i64, String, String, i64, Option<String>, i64)> = sqlx::query_as(
        "PRAGMA table_info(unmatched_genealogy)"
    )
    .fetch_all(&pool)
    .await?;

    println!("Columns in unmatched_genealogy:");
    for (_, name, col_type, not_null, default_val, pk) in &schema {
        println!("  - {} ({}) | NOT NULL: {} | DEFAULT: {:?} | PK: {}",
            name, col_type, not_null, default_val, pk);
    }

    // Check total records
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unmatched_genealogy")
        .fetch_one(&pool)
        .await?;
    println!("\nTotal records in unmatched_genealogy: {}", total.0);

    // Find duplicates based on name, address, and birth_year
    println!("\n========================================");
    println!("FINDING DUPLICATES (same name, address, birth_year)");
    println!("========================================\n");

    let duplicates: Vec<(String, String, Option<i64>, i64)> = sqlx::query_as(
        "SELECT name, address, birth_year, COUNT(*) as count
         FROM unmatched_genealogy
         GROUP BY name, address, birth_year
         HAVING COUNT(*) > 1
         ORDER BY count DESC, name"
    )
    .fetch_all(&pool)
    .await?;

    println!("Found {} groups of duplicates\n", duplicates.len());

    if duplicates.is_empty() {
        println!("No duplicates found!");
    } else {
        println!("Top 30 duplicate groups:\n");
        for (i, (name, address, birth_year, count)) in duplicates.iter().take(30).enumerate() {
            println!("Group #{}: {} duplicates", i + 1, count);
            println!("  Name: {}", name);
            println!("  Address: {}", address);
            println!("  Birth Year: {}", birth_year.map(|y| y.to_string()).unwrap_or("N/A".to_string()));
            println!();
        }

        // Calculate total duplicate records
        let total_duplicates: i64 = duplicates.iter().map(|(_, _, _, count)| count).sum();
        let duplicate_groups = duplicates.len();
        let extra_records = total_duplicates - duplicate_groups as i64;

        println!("\n========================================");
        println!("SUMMARY");
        println!("========================================");
        println!("Total duplicate groups: {}", duplicate_groups);
        println!("Total records in duplicate groups: {}", total_duplicates);
        println!("Extra/redundant records: {}", extra_records);
        println!("(If we kept one from each group, we'd remove {} records)", extra_records);
        println!("========================================\n");
    }

    // Also check for exact duplicates (all fields)
    println!("\n========================================");
    println!("CHECKING FOR EXACT DUPLICATES (all fields identical)");
    println!("========================================\n");

    let exact_duplicates: Vec<(i64,)> = sqlx::query_as(
        "SELECT COUNT(*) as count
         FROM (
             SELECT name, address, birth_year, profession, COUNT(*) as dup_count
             FROM unmatched_genealogy
             GROUP BY name, address, birth_year, profession
             HAVING COUNT(*) > 1
         )"
    )
    .fetch_all(&pool)
    .await?;

    if let Some((count,)) = exact_duplicates.first() {
        println!("Found {} groups with exact duplicates (all fields match)", count);
    }

    Ok(())
}
