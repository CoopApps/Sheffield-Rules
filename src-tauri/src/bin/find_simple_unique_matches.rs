use sqlx::sqlite::SqlitePool;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
struct CensusRecord {
    id: String,
    name: String,
    birth_year: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
struct GenealogyRecord {
    id: String,
    name: String,
    birth_year: Option<i64>,
}

fn normalize_name(name: &str) -> String {
    name.to_lowercase()
        .replace(".", "")
        .replace(",", "")
        .trim()
        .to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("Loading census records...");
    let census_records: Vec<CensusRecord> = sqlx::query_as(
        "SELECT id, name, birth_year FROM unmatched_sheffieldcensus"
    )
    .fetch_all(&pool)
    .await?;

    println!("Loaded {} census records", census_records.len());

    println!("Loading genealogy records...");
    let genealogy_records: Vec<GenealogyRecord> = sqlx::query_as(
        "SELECT id, name, birth_year FROM unmatched_genealogy"
    )
    .fetch_all(&pool)
    .await?;

    println!("Loaded {} genealogy records\n", genealogy_records.len());

    // Build index of census records by (normalized name, birth_year)
    let mut census_by_name_year: HashMap<(String, Option<i64>), Vec<&CensusRecord>> = HashMap::new();
    for census in &census_records {
        let norm_name = normalize_name(&census.name);
        let key = (norm_name, census.birth_year);
        census_by_name_year.entry(key).or_insert_with(Vec::new).push(census);
    }

    // Build index of genealogy records by (normalized name, birth_year)
    let mut gen_by_name_year: HashMap<(String, Option<i64>), Vec<&GenealogyRecord>> = HashMap::new();
    for gen in &genealogy_records {
        let norm_name = normalize_name(&gen.name);
        let key = (norm_name, gen.birth_year);
        gen_by_name_year.entry(key).or_insert_with(Vec::new).push(gen);
    }

    println!("Finding unique matches (name + birth year appears exactly once in each table)...\n");

    let mut unique_matches = Vec::new();

    for census in &census_records {
        let norm_name = normalize_name(&census.name);
        let key = (norm_name.clone(), census.birth_year);

        // Check if this census record is unique
        let census_is_unique = census_by_name_year.get(&key)
            .map(|v| v.len() == 1)
            .unwrap_or(false);

        if !census_is_unique {
            continue;
        }

        // Check if there's a matching genealogy record that is also unique
        if let Some(gen_candidates) = gen_by_name_year.get(&key) {
            if gen_candidates.len() == 1 {
                // We have a unique match!
                unique_matches.push((census, gen_candidates[0]));
            }
        }
    }

    println!("========================================");
    println!("UNIQUE MATCH RESULTS:");
    println!("========================================");
    println!("Total census records: {}", census_records.len());
    println!("Total genealogy records: {}", genealogy_records.len());
    println!("Unique matches found: {}", unique_matches.len());
    println!("Percentage of census matched: {:.1}%",
        (unique_matches.len() as f64 / census_records.len() as f64) * 100.0);
    println!("========================================\n");

    println!("Sample of first 20 unique matches:\n");
    for (i, (census, gen)) in unique_matches.iter().take(20).enumerate() {
        println!("{}. {} (born {})",
            i + 1,
            census.name,
            census.birth_year.map(|y| y.to_string()).unwrap_or("?".to_string())
        );
    }

    // Analyze non-matches
    let mut census_unique_but_no_gen_match = 0;
    let mut census_unique_gen_not_unique = 0;
    let mut census_not_unique = 0;

    for census in &census_records {
        let norm_name = normalize_name(&census.name);
        let key = (norm_name.clone(), census.birth_year);

        let census_is_unique = census_by_name_year.get(&key)
            .map(|v| v.len() == 1)
            .unwrap_or(false);

        if !census_is_unique {
            census_not_unique += 1;
        } else {
            // Census is unique
            if let Some(gen_candidates) = gen_by_name_year.get(&key) {
                if gen_candidates.len() > 1 {
                    census_unique_gen_not_unique += 1;
                }
            } else {
                census_unique_but_no_gen_match += 1;
            }
        }
    }

    println!("\n========================================");
    println!("NON-MATCH BREAKDOWN:");
    println!("========================================");
    println!("Census records that are NOT unique (duplicates in census): {}", census_not_unique);
    println!("Census unique but no genealogy record with same name+age: {}", census_unique_but_no_gen_match);
    println!("Census unique but genealogy has duplicates: {}", census_unique_gen_not_unique);
    println!("Total non-matches: {}", census_records.len() - unique_matches.len());
    println!("========================================");

    Ok(())
}
