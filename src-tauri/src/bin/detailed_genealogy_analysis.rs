use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("DETAILED GENEALOGY ANALYSIS");
    println!("========================================\n");

    // Check total records
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unmatched_genealogy")
        .fetch_one(&pool)
        .await?;
    println!("Total records in unmatched_genealogy: {}\n", total.0);

    // Check records with only one name (forename or surname only)
    println!("========================================");
    println!("CHECKING NAME COMPLETENESS");
    println!("========================================\n");

    // Count names without spaces (single name only)
    let single_name: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy WHERE name NOT LIKE '% %'"
    )
    .fetch_one(&pool)
    .await?;

    println!("Records with single name (no space): {}", single_name.0);

    // Show examples
    let single_examples: Vec<(String, String, Option<String>)> = sqlx::query_as(
        "SELECT name, address, profession
         FROM unmatched_genealogy
         WHERE name NOT LIKE '% %'
         LIMIT 20"
    )
    .fetch_all(&pool)
    .await?;

    println!("\nExamples of single-name records:");
    for (name, addr, prof) in &single_examples {
        println!("  {} | {} | {}", name, addr, prof.as_deref().unwrap_or("N/A"));
    }

    // Analyze the duplicates more carefully
    println!("\n========================================");
    println!("ANALYZING DUPLICATES IN DETAIL");
    println!("========================================\n");

    let duplicate_details: Vec<(String, String, Option<i64>, i64, String, String)> = sqlx::query_as(
        "SELECT g1.name, g1.address, g1.birth_year, COUNT(*) as count,
                GROUP_CONCAT(DISTINCT g1.profession) as professions,
                GROUP_CONCAT(DISTINCT g1.relation) as relations
         FROM unmatched_genealogy g1
         GROUP BY g1.name, g1.address, g1.birth_year
         HAVING COUNT(*) > 1
         ORDER BY count DESC
         LIMIT 30"
    )
    .fetch_all(&pool)
    .await?;

    println!("Top duplicate groups with profession/relation info:\n");
    for (i, (name, address, birth_year, count, professions, relations)) in duplicate_details.iter().enumerate() {
        println!("Group #{}: {} duplicates", i + 1, count);
        println!("  Name: {}", name);
        println!("  Address: {}", address);
        println!("  Birth Year: {}", birth_year.map(|y| y.to_string()).unwrap_or("N/A".to_string()));
        println!("  Professions: {}", professions);
        println!("  Relations: {}", relations);
        println!();
    }

    // Get specific details for a few duplicate groups to see if they're parent/child
    println!("\n========================================");
    println!("DETAILED VIEW OF FIRST DUPLICATE GROUP");
    println!("========================================\n");

    let first_dup: Vec<(String, String, Option<i64>, Option<String>, Option<String>, Option<String>, Option<i64>)> = sqlx::query_as(
        "SELECT name, address, birth_year, profession, relation, spouse, age
         FROM unmatched_genealogy
         WHERE name = 'Ann Almond' AND address = '19,BennettStreet'
         ORDER BY birth_year"
    )
    .fetch_all(&pool)
    .await?;

    for (name, addr, birth_year, prof, rel, spouse, age) in &first_dup {
        println!("Record:");
        println!("  Name: {}", name);
        println!("  Address: {}", addr);
        println!("  Birth Year: {}", birth_year.map(|y| y.to_string()).unwrap_or("N/A".to_string()));
        println!("  Age: {}", age.map(|a| a.to_string()).unwrap_or("N/A".to_string()));
        println!("  Profession: {}", prof.as_deref().unwrap_or("N/A"));
        println!("  Relation: {}", rel.as_deref().unwrap_or("N/A"));
        println!("  Spouse: {}", spouse.as_deref().unwrap_or("N/A"));
        println!();
    }

    // Count truly exact duplicates (where all important fields match)
    println!("\n========================================");
    println!("EXACT DUPLICATES (all fields match)");
    println!("========================================\n");

    let exact_dup_groups: Vec<(i64,)> = sqlx::query_as(
        "SELECT COUNT(*) as dup_count
         FROM (
             SELECT name, address, birth_year, profession, relation, spouse, COUNT(*) as cnt
             FROM unmatched_genealogy
             GROUP BY name, address, birth_year, profession, relation, spouse
             HAVING COUNT(*) > 1
         )"
    )
    .fetch_all(&pool)
    .await?;

    if let Some((count,)) = exact_dup_groups.first() {
        println!("Groups with exact duplicates (all fields identical): {}", count);
    }

    // Show how many records would be removed
    let exact_dup_records: Vec<(i64,)> = sqlx::query_as(
        "SELECT SUM(cnt - 1) as removable
         FROM (
             SELECT COUNT(*) as cnt
             FROM unmatched_genealogy
             GROUP BY name, address, birth_year, profession, relation, spouse
             HAVING COUNT(*) > 1
         )"
    )
    .fetch_all(&pool)
    .await?;

    if let Some((removable,)) = exact_dup_records.first() {
        println!("Records that could be safely removed: {}", removable);
    }

    println!("\n========================================");
    println!("SUMMARY");
    println!("========================================");
    println!("Total records: {}", total.0);
    println!("Single-name records (incomplete): {}", single_name.0);
    println!("Complete name records: {}", total.0 - single_name.0);
    println!("========================================\n");

    Ok(())
}
