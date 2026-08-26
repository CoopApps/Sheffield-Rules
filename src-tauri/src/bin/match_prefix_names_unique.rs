use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("MATCHING COMMON PREFIX NAMES");
    println!("Between Census and Genealogy");
    println!("========================================\n");

    // First, check what prefixes exist in genealogy
    println!("--- Top Name Prefixes in Genealogy ---\n");

    let gen_prefixes: Vec<(String, i64)> = sqlx::query_as(
        "SELECT SUBSTR(name, 1, 3) as prefix, COUNT(*) as count
         FROM unmatched_genealogy
         WHERE LENGTH(name) > 3
         GROUP BY prefix
         ORDER BY count DESC
         LIMIT 20"
    )
    .fetch_all(&pool)
    .await?;

    println!("Most common 3-letter prefixes in genealogy:");
    for (prefix, count) in &gen_prefixes {
        println!("  {}: {}", prefix, count);
    }

    // Now let's find exact name matches for common prefixes
    println!("\n========================================");
    println!("UNIQUE NAME MATCHES BY PREFIX");
    println!("========================================\n");

    let prefixes = vec![
        ("Joh", "John"),
        ("Wil", "William"),
        ("Geo", "George"),
        ("Tho", "Thomas"),
        ("Hen", "Henry"),
        ("Jam", "James"),
        ("Jos", "Joseph"),
        ("Cha", "Charles"),
        ("Edw", "Edward"),
        ("Sam", "Samuel"),
    ];

    for (prefix, full_name) in &prefixes {
        println!("\n--- {} ({}*) ---", full_name, prefix);

        // Find names that appear in both tables (exact match)
        let exact_matches: Vec<(String, i64, i64)> = sqlx::query_as(
            "SELECT c.name,
                    COUNT(DISTINCT c.id) as census_count,
                    COUNT(DISTINCT g.id) as genealogy_count
             FROM unmatched_sheffieldcensus c
             JOIN unmatched_genealogy g ON c.name = g.name
             WHERE c.name LIKE ?
             GROUP BY c.name
             ORDER BY c.name
             LIMIT 50"
        )
        .bind(format!("{}%", prefix))
        .fetch_all(&pool)
        .await?;

        println!("  Exact matches (same name in both tables): {}", exact_matches.len());

        // Show unique matches (appear once in each table)
        let unique_matches: Vec<_> = exact_matches.iter()
            .filter(|(_, c_count, g_count)| *c_count == 1 && *g_count == 1)
            .collect();

        println!("  Unique matches (1 in census, 1 in genealogy): {}", unique_matches.len());

        if !unique_matches.is_empty() {
            println!("  Sample unique matches:");
            for (name, _, _) in unique_matches.iter().take(5) {
                // Get details for this unique match
                let census_details: Option<(String, Option<i64>, String)> = sqlx::query_as(
                    "SELECT relation, age, civil_parish
                     FROM unmatched_sheffieldcensus
                     WHERE name = ?"
                )
                .bind(name)
                .fetch_optional(&pool)
                .await?;

                let gen_details: Option<(String, Option<i64>, String)> = sqlx::query_as(
                    "SELECT relation, age, address
                     FROM unmatched_genealogy
                     WHERE name = ?"
                )
                .bind(name)
                .fetch_optional(&pool)
                .await?;

                if let (Some((c_rel, c_age, c_parish)), Some((g_rel, g_age, g_addr))) =
                    (census_details, gen_details) {
                    println!("    {}", name);
                    println!("      Census: {} (age: {}) at {}",
                        c_rel,
                        c_age.map(|a| a.to_string()).unwrap_or("?".to_string()),
                        c_parish);
                    println!("      Genealogy: {} (age: {}) at {}",
                        g_rel,
                        g_age.map(|a| a.to_string()).unwrap_or("?".to_string()),
                        g_addr);
                }
            }
        }

        // Also count names unique to census
        let census_only: (i64,) = sqlx::query_as(
            "SELECT COUNT(DISTINCT c.name)
             FROM unmatched_sheffieldcensus c
             LEFT JOIN unmatched_genealogy g ON c.name = g.name
             WHERE c.name LIKE ?
             AND g.name IS NULL"
        )
        .bind(format!("{}%", prefix))
        .fetch_one(&pool)
        .await?;

        // And names unique to genealogy
        let genealogy_only: (i64,) = sqlx::query_as(
            "SELECT COUNT(DISTINCT g.name)
             FROM unmatched_genealogy g
             LEFT JOIN unmatched_sheffieldcensus c ON g.name = c.name
             WHERE g.name LIKE ?
             AND c.name IS NULL"
        )
        .bind(format!("{}%", prefix))
        .fetch_one(&pool)
        .await?;

        println!("  Names only in census: {}", census_only.0);
        println!("  Names only in genealogy: {}", genealogy_only.0);
    }

    // Summary statistics
    println!("\n========================================");
    println!("OVERALL SUMMARY");
    println!("========================================\n");

    let total_exact_matches: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT c.name)
         FROM unmatched_sheffieldcensus c
         JOIN unmatched_genealogy g ON c.name = g.name"
    )
    .fetch_one(&pool)
    .await?;

    let total_unique_matches: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT c.name)
         FROM unmatched_sheffieldcensus c
         JOIN unmatched_genealogy g ON c.name = g.name
         WHERE (SELECT COUNT(*) FROM unmatched_sheffieldcensus WHERE name = c.name) = 1
         AND (SELECT COUNT(*) FROM unmatched_genealogy WHERE name = g.name) = 1"
    )
    .fetch_one(&pool)
    .await?;

    println!("Total names appearing in both tables: {}", total_exact_matches.0);
    println!("Total unique matches (1:1): {}", total_unique_matches.0);

    Ok(())
}
