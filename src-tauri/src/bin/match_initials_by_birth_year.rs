use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("MATCHING INITIALS TO GENEALOGY");
    println!("By Birth Year");
    println!("========================================\n");

    // First, let's see what we have for initials-only names in census
    println!("--- Census Records with Initials Only (3 chars or less) ---\n");

    let initials_census: Vec<(String, String, Option<i64>, Option<i64>)> = sqlx::query_as(
        "SELECT name, relation, age, birth_year
         FROM unmatched_sheffieldcensus
         WHERE LENGTH(name) <= 3
         ORDER BY name, birth_year"
    )
    .fetch_all(&pool)
    .await?;

    println!("Found {} census records with initials only\n", initials_census.len());

    // Show some examples
    println!("Sample census initials:");
    for (name, relation, age, birth_year) in initials_census.iter().take(10) {
        println!("  '{}' - {} (age: {}, birth: {})",
            name,
            relation,
            age.map(|a| a.to_string()).unwrap_or("?".to_string()),
            birth_year.map(|b| b.to_string()).unwrap_or("?".to_string())
        );
    }

    // Now try to match each initial to genealogy records
    println!("\n--- Attempting to Match Initials to Full Names ---\n");

    let mut match_count = 0;
    let mut total_matches = 0;

    for (census_initials, relation, age, birth_year) in &initials_census {
        if let Some(by) = birth_year {
            // Extract the initials (e.g., "J B" -> first="J", last="B")
            let parts: Vec<&str> = census_initials.split_whitespace().collect();

            if parts.len() == 2 {
                let first_initial = parts[0];
                let last_initial = parts[1];

                // Look for genealogy records matching:
                // 1. First name starts with first_initial
                // 2. Last name starts with last_initial
                // 3. Birth year is within 2 years
                let matches: Vec<(String, String, Option<i64>, Option<i64>)> = sqlx::query_as(
                    "SELECT name, relation, age, birth_year
                     FROM unmatched_genealogy
                     WHERE name LIKE ? || '%'
                     AND name LIKE '% ' || ? || '%'
                     AND birth_year BETWEEN ? AND ?
                     LIMIT 10"
                )
                .bind(first_initial)
                .bind(last_initial)
                .bind(by - 2)
                .bind(by + 2)
                .fetch_all(&pool)
                .await?;

                if !matches.is_empty() {
                    match_count += 1;
                    total_matches += matches.len();

                    println!("Census: '{}' ({}, birth: {})", census_initials, relation, by);
                    println!("  Potential matches in genealogy:");
                    for (name, gen_rel, gen_age, gen_birth) in &matches {
                        println!("    {} ({}, age: {}, birth: {})",
                            name,
                            gen_rel,
                            gen_age.map(|a| a.to_string()).unwrap_or("?".to_string()),
                            gen_birth.map(|b| b.to_string()).unwrap_or("?".to_string())
                        );
                    }
                    println!();
                }
            }
        }
    }

    println!("\n========================================");
    println!("SUMMARY");
    println!("========================================");
    println!("Census records with initials: {}", initials_census.len());
    println!("Census initials matched to genealogy: {}", match_count);
    println!("Total potential genealogy matches: {}", total_matches);
    println!("Average matches per census initial: {:.1}",
        if match_count > 0 { total_matches as f64 / match_count as f64 } else { 0.0 });

    // Now expand the common name prefixes
    println!("\n========================================");
    println!("EXPANDING COMMON NAME PREFIXES");
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
        let census_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM unmatched_sheffieldcensus WHERE name LIKE ?"
        )
        .bind(format!("{}%", prefix))
        .fetch_one(&pool)
        .await?;

        let genealogy_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM unmatched_genealogy WHERE name LIKE ?"
        )
        .bind(format!("{}%", prefix))
        .fetch_one(&pool)
        .await?;

        println!("{} ({}*): Census: {}, Genealogy: {}",
            full_name, prefix, census_count.0, genealogy_count.0);

        // Show some examples that start with this prefix
        let examples: Vec<(String,)> = sqlx::query_as(
            "SELECT DISTINCT name FROM unmatched_sheffieldcensus
             WHERE name LIKE ?
             LIMIT 5"
        )
        .bind(format!("{}%", prefix))
        .fetch_all(&pool)
        .await?;

        println!("  Census examples:");
        for (name,) in &examples {
            println!("    {}", name);
        }
    }

    Ok(())
}
