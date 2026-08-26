use sqlx::sqlite::SqlitePool;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("EXPANDING NAME PREFIXES");
    println!("Creating Standardized Full Names");
    println!("========================================\n");

    // Create comprehensive prefix expansion map
    let mut expansions: HashMap<&str, &str> = HashMap::new();

    // Male names
    expansions.insert("Joh", "John");
    expansions.insert("Wil", "William");
    expansions.insert("Geo", "George");
    expansions.insert("Tho", "Thomas");
    expansions.insert("Hen", "Henry");
    expansions.insert("Jam", "James");
    expansions.insert("Jos", "Joseph");
    expansions.insert("Cha", "Charles");
    expansions.insert("Edw", "Edward");
    expansions.insert("Sam", "Samuel");
    expansions.insert("Rob", "Robert");
    expansions.insert("Ric", "Richard");
    expansions.insert("Fre", "Frederick");
    expansions.insert("Fra", "Francis");
    expansions.insert("Wal", "Walter");
    expansions.insert("Ben", "Benjamin");
    expansions.insert("Alf", "Alfred");
    expansions.insert("Art", "Arthur");
    expansions.insert("Alb", "Albert");
    expansions.insert("Har", "Harry");
    expansions.insert("Dav", "David");
    expansions.insert("Ale", "Alexander");
    expansions.insert("Her", "Herbert");
    expansions.insert("Edm", "Edmund");
    expansions.insert("Mat", "Matthew");

    // Female names
    expansions.insert("Mar", "Mary");
    expansions.insert("Eli", "Elizabeth");
    expansions.insert("Ann", "Ann");
    expansions.insert("Sar", "Sarah");
    expansions.insert("Jan", "Jane");
    expansions.insert("Emm", "Emma");
    expansions.insert("Han", "Hannah");
    expansions.insert("Ell", "Ellen");
    expansions.insert("Isa", "Isabella");
    expansions.insert("Har", "Harriet"); // Note: conflicts with Harry
    expansions.insert("Cha", "Charlotte"); // Note: conflicts with Charles

    println!("Created {} prefix expansion rules\n", expansions.len());

    // Create expanded_names table for census
    println!("Creating expanded names for census...");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS census_expanded_names (
            original_id TEXT,
            original_name TEXT,
            expanded_name TEXT,
            prefix TEXT,
            expansion TEXT
        )"
    )
    .execute(&pool)
    .await?;

    // Clear existing data
    sqlx::query("DELETE FROM census_expanded_names")
        .execute(&pool)
        .await?;

    // Get all census names
    let census_names: Vec<(String, String)> = sqlx::query_as(
        "SELECT id, name FROM unmatched_sheffieldcensus"
    )
    .fetch_all(&pool)
    .await?;

    println!("Processing {} census names...", census_names.len());

    let mut census_expanded = 0;
    for (id, name) in &census_names {
        if name.len() >= 3 {
            let prefix = &name[0..3];
            if let Some(&expansion) = expansions.get(prefix) {
                // Create expanded name by replacing prefix
                let expanded = name.replacen(prefix, expansion, 1);

                sqlx::query(
                    "INSERT INTO census_expanded_names
                     (original_id, original_name, expanded_name, prefix, expansion)
                     VALUES (?, ?, ?, ?, ?)"
                )
                .bind(id)
                .bind(name)
                .bind(&expanded)
                .bind(prefix)
                .bind(expansion)
                .execute(&pool)
                .await?;

                census_expanded += 1;
            }
        }
    }

    println!("Expanded {} census names\n", census_expanded);

    // Create expanded_names table for genealogy
    println!("Creating expanded names for genealogy...");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS genealogy_expanded_names (
            original_id TEXT,
            original_name TEXT,
            expanded_name TEXT,
            prefix TEXT,
            expansion TEXT
        )"
    )
    .execute(&pool)
    .await?;

    // Clear existing data
    sqlx::query("DELETE FROM genealogy_expanded_names")
        .execute(&pool)
        .await?;

    // Get all genealogy names
    let gen_names: Vec<(String, String)> = sqlx::query_as(
        "SELECT id, name FROM unmatched_genealogy"
    )
    .fetch_all(&pool)
    .await?;

    println!("Processing {} genealogy names...", gen_names.len());

    let mut gen_expanded = 0;
    for (id, name) in &gen_names {
        if name.len() >= 3 {
            let prefix = &name[0..3];
            if let Some(&expansion) = expansions.get(prefix) {
                // Create expanded name by replacing prefix
                let expanded = name.replacen(prefix, expansion, 1);

                sqlx::query(
                    "INSERT INTO genealogy_expanded_names
                     (original_id, original_name, expanded_name, prefix, expansion)
                     VALUES (?, ?, ?, ?, ?)"
                )
                .bind(id)
                .bind(name)
                .bind(&expanded)
                .bind(prefix)
                .bind(expansion)
                .execute(&pool)
                .await?;

                gen_expanded += 1;
            }
        }
    }

    println!("Expanded {} genealogy names\n", gen_expanded);

    // Now find matches using expanded names
    println!("========================================");
    println!("FINDING MATCHES WITH EXPANDED NAMES");
    println!("========================================\n");

    let matches: Vec<(String, String, String, String)> = sqlx::query_as(
        "SELECT
            c.expanded_name,
            c.original_name as census_original,
            g.original_name as gen_original,
            c.expansion
         FROM census_expanded_names c
         JOIN genealogy_expanded_names g ON c.expanded_name = g.expanded_name
         ORDER BY c.expanded_name
         LIMIT 100"
    )
    .fetch_all(&pool)
    .await?;

    println!("Found {} matches with expanded names (showing first 100)\n", matches.len());

    // Show examples
    println!("Sample matches:");
    for (expanded, census_orig, gen_orig, expansion) in matches.iter().take(20) {
        println!("  Expanded: {}", expanded);
        println!("    Census original: {}", census_orig);
        println!("    Genealogy original: {}", gen_orig);
        println!("    Expansion used: {}", expansion);
        println!();
    }

    // Count total matches
    let total_matches: (i64,) = sqlx::query_as(
        "SELECT COUNT(*)
         FROM census_expanded_names c
         JOIN genealogy_expanded_names g ON c.expanded_name = g.expanded_name"
    )
    .fetch_one(&pool)
    .await?;

    println!("\n========================================");
    println!("SUMMARY");
    println!("========================================");
    println!("Census names expanded: {}", census_expanded);
    println!("Genealogy names expanded: {}", gen_expanded);
    println!("Total matches found: {}", total_matches.0);

    // Breakdown by expansion
    println!("\nMatches by expansion:");
    let by_expansion: Vec<(String, i64)> = sqlx::query_as(
        "SELECT c.expansion, COUNT(*) as count
         FROM census_expanded_names c
         JOIN genealogy_expanded_names g ON c.expanded_name = g.expanded_name
         GROUP BY c.expansion
         ORDER BY count DESC"
    )
    .fetch_all(&pool)
    .await?;

    for (expansion, count) in &by_expansion {
        println!("  {}: {} matches", expansion, count);
    }

    Ok(())
}
