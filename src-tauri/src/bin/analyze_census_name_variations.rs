use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("ANALYZING NAME VARIATIONS IN CENSUS");
    println!("========================================\n");

    // Find similar names that might be spelling variations
    // Look for names that differ by 1-2 characters

    // First, let's find names that appear multiple times
    println!("--- Names Appearing Multiple Times ---\n");

    let duplicate_names: Vec<(String, i64)> = sqlx::query_as(
        "SELECT name, COUNT(*) as count
         FROM unmatched_sheffieldcensus
         GROUP BY name
         HAVING count > 1
         ORDER BY count DESC
         LIMIT 30"
    )
    .fetch_all(&pool)
    .await?;

    println!("Top duplicate names:");
    for (name, count) in &duplicate_names {
        println!("  {}: {} occurrences", name, count);
    }

    // Find names with very similar patterns (same length, mostly same characters)
    println!("\n--- Potential Spelling Variations (Same Length) ---\n");

    // This is a simple approach - find names that are the same length
    // and might be variations of each other
    let similar_names: Vec<(String, String, i64, i64)> = sqlx::query_as(
        "SELECT
            a.name as name1,
            b.name as name2,
            COUNT(DISTINCT a.id) as count1,
            COUNT(DISTINCT b.id) as count2
         FROM unmatched_sheffieldcensus a
         JOIN unmatched_sheffieldcensus b
            ON LENGTH(a.name) = LENGTH(b.name)
            AND a.name < b.name
            AND ABS(a.age - b.age) <= 5
         WHERE LENGTH(a.name) >= 10
         GROUP BY a.name, b.name
         HAVING count1 = 1 AND count2 = 1
         LIMIT 50"
    )
    .fetch_all(&pool)
    .await?;

    println!("Name pairs with same length and similar ages:");
    for (name1, name2, count1, count2) in &similar_names {
        println!("  '{}' ({}) vs '{}' ({})", name1, count1, name2, count2);
    }

    // Look for common name patterns and prefixes
    println!("\n--- Common Name Prefixes ---\n");

    let prefixes: Vec<(String, i64)> = sqlx::query_as(
        "SELECT SUBSTR(name, 1, 3) as prefix, COUNT(*) as count
         FROM unmatched_sheffieldcensus
         WHERE LENGTH(name) > 3
         GROUP BY prefix
         ORDER BY count DESC
         LIMIT 30"
    )
    .fetch_all(&pool)
    .await?;

    println!("Most common 3-letter prefixes:");
    for (prefix, count) in &prefixes {
        println!("  {}: {}", prefix, count);
    }

    // Find unusual characters or patterns
    println!("\n--- Names with Unusual Characters ---\n");

    let unusual: Vec<(String, String)> = sqlx::query_as(
        "SELECT name, relation
         FROM unmatched_sheffieldcensus
         WHERE name LIKE '%?%'
            OR name LIKE '%[%'
            OR name LIKE '%]%'
            OR name LIKE '%{%'
            OR name LIKE '%}%'
            OR name LIKE '%0%'
            OR name LIKE '%1%'
            OR name LIKE '%2%'
            OR name LIKE '%3%'
            OR name LIKE '%4%'
            OR name LIKE '%5%'
            OR name LIKE '%6%'
            OR name LIKE '%7%'
            OR name LIKE '%8%'
            OR name LIKE '%9%'
         LIMIT 50"
    )
    .fetch_all(&pool)
    .await?;

    println!("Names with numbers or special characters:");
    for (name, relation) in &unusual {
        println!("  {} ({})", name, relation);
    }

    // Find very short or very long names
    println!("\n--- Name Length Distribution ---\n");

    let lengths: Vec<(i64, i64)> = sqlx::query_as(
        "SELECT LENGTH(name) as len, COUNT(*) as count
         FROM unmatched_sheffieldcensus
         GROUP BY len
         ORDER BY len"
    )
    .fetch_all(&pool)
    .await?;

    println!("Name length distribution:");
    for (len, count) in &lengths {
        println!("  {} characters: {} names", len, count);
    }

    // Find names with multiple spaces or unusual spacing
    println!("\n--- Names with Multiple Spaces ---\n");

    let multi_space: Vec<(String, String)> = sqlx::query_as(
        "SELECT name, relation
         FROM unmatched_sheffieldcensus
         WHERE name LIKE '%  %'
         LIMIT 30"
    )
    .fetch_all(&pool)
    .await?;

    println!("Names with multiple consecutive spaces:");
    for (name, relation) in &multi_space {
        println!("  '{}' ({})", name, relation);
    }

    Ok(())
}
