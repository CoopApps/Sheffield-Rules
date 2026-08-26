use sqlx::sqlite::SqlitePool;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
struct CensusRecord {
    id: String,
    name: String,
    civil_parish: Option<String>,
    household_members: Option<String>,
    birth_year: Option<i64>,
    relation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
struct GenealogyRecord {
    id: String,
    name: String,
    address: Option<String>,
    profession: Option<String>,
    spouse: Option<String>,
    relation: Option<String>,
    birth_year: Option<i64>,
    parish: Option<String>,
}

fn normalize_name(name: &str) -> String {
    name.to_lowercase()
        .replace(".", "")
        .replace(",", "")
        .trim()
        .to_string()
}

fn calculate_match_score(census: &CensusRecord, gen: &GenealogyRecord) -> f32 {
    let mut score = 0.0;

    // Name already matches (we filtered by name), give base score
    score += 0.5;

    // Birth year match (40%)
    if let (Some(c_year), Some(g_year)) = (census.birth_year, gen.birth_year) {
        if c_year == g_year {
            score += 0.4;
        } else if (c_year - g_year).abs() <= 2 {
            score += 0.2;
        }
    }

    // Parish match (10%)
    if let (Some(c_parish), Some(g_parish)) = (&census.civil_parish, &gen.parish) {
        let c_norm = c_parish.to_lowercase();
        let g_norm = g_parish.to_lowercase();
        if c_norm == g_norm {
            score += 0.1;
        } else if c_norm.contains(&g_norm) || g_norm.contains(&c_norm) {
            score += 0.05;
        }
    }

    score
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("Loading unmatched genealogy records and indexing by name...");
    let genealogy_records: Vec<GenealogyRecord> = sqlx::query_as(
        "SELECT id, name, address, profession, spouse, relation, birth_year, parish FROM unmatched_genealogy"
    )
    .fetch_all(&pool)
    .await?;

    // Index genealogy records by normalized name
    let mut gen_by_name: HashMap<String, Vec<GenealogyRecord>> = HashMap::new();
    for gen in genealogy_records {
        let norm_name = normalize_name(&gen.name);
        gen_by_name.entry(norm_name).or_insert_with(Vec::new).push(gen);
    }

    println!("Indexed {} unique names from {} genealogy records", gen_by_name.len(), gen_by_name.values().map(|v| v.len()).sum::<usize>());

    println!("\nLoading unmatched census records...");
    let census_records: Vec<CensusRecord> = sqlx::query_as(
        "SELECT id, name, civil_parish, household_members, birth_year, relation FROM unmatched_sheffieldcensus"
    )
    .fetch_all(&pool)
    .await?;

    println!("Loaded {} census records\n", census_records.len());

    // Build index of census records by name and birth year to check uniqueness
    let mut census_by_name_year: HashMap<(String, Option<i64>), Vec<&CensusRecord>> = HashMap::new();
    for census in &census_records {
        let norm_name = normalize_name(&census.name);
        let key = (norm_name, census.birth_year);
        census_by_name_year.entry(key).or_insert_with(Vec::new).push(census);
    }

    // Build index of genealogy records by name and birth year to check uniqueness
    let mut gen_by_name_year: HashMap<(String, Option<i64>), Vec<&GenealogyRecord>> = HashMap::new();
    for gen_list in gen_by_name.values() {
        for gen in gen_list {
            let norm_name = normalize_name(&gen.name);
            let key = (norm_name, gen.birth_year);
            gen_by_name_year.entry(key).or_insert_with(Vec::new).push(gen);
        }
    }

    println!("Finding matches...");
    let mut matches_found = 0;
    let mut high_confidence_matches = Vec::new();

    for (i, census) in census_records.iter().enumerate() {
        if i % 1000 == 0 && i > 0 {
            println!("Processed {}/{} census records, found {} high confidence matches so far...",
                i, census_records.len(), high_confidence_matches.len());
        }

        let norm_name = normalize_name(&census.name);

        // Only compare with genealogy records that have the same name!
        if let Some(gen_candidates) = gen_by_name.get(&norm_name) {
            let mut best_match: Option<(&GenealogyRecord, f32)> = None;

            for gen in gen_candidates {
                let score = calculate_match_score(census, gen);

                if score >= 0.6 {
                    if let Some((_, best_score)) = best_match {
                        if score > best_score {
                            best_match = Some((gen, score));
                        }
                    } else {
                        best_match = Some((gen, score));
                    }
                }
            }

            if let Some((gen, score)) = best_match {
                // Check uniqueness: both census and genealogy should have only ONE record
                // with this exact name and birth year
                let census_key = (norm_name.clone(), census.birth_year);
                let gen_key = (norm_name.clone(), gen.birth_year);

                let census_is_unique = census_by_name_year.get(&census_key)
                    .map(|v| v.len() == 1)
                    .unwrap_or(false);

                let gen_is_unique = gen_by_name_year.get(&gen_key)
                    .map(|v| v.len() == 1)
                    .unwrap_or(false);

                // Only count as a match if BOTH are unique
                if census_is_unique && gen_is_unique {
                    matches_found += 1;
                    if score >= 0.7 {
                        high_confidence_matches.push((census.clone(), gen.clone(), score));
                    }
                }
            }
        }
    }

    println!("\n========================================");
    println!("MATCHING RESULTS:");
    println!("========================================");
    println!("Total potential matches found: {}", matches_found);
    println!("High confidence matches (≥70%): {}", high_confidence_matches.len());

    println!("\n\nTop 20 high confidence matches:");
    high_confidence_matches.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    for (i, (census, gen, score)) in high_confidence_matches.iter().take(20).enumerate() {
        println!("\n{}. Match Score: {:.1}%", i + 1, score * 100.0);
        println!("   Census: {} (born {}, parish: {})",
            census.name,
            census.birth_year.map(|y| y.to_string()).unwrap_or("?".to_string()),
            census.civil_parish.as_deref().unwrap_or("no parish")
        );
        println!("   Genealogy: {} (born {}, {}, spouse: {})",
            gen.name,
            gen.birth_year.map(|y| y.to_string()).unwrap_or("?".to_string()),
            gen.address.as_deref().unwrap_or("no address"),
            gen.spouse.as_deref().unwrap_or("none")
        );
    }

    println!("\n\n========================================");
    println!("Ready to apply {} high confidence matches", high_confidence_matches.len());
    println!("(This is a DRY RUN - no changes will be made yet)");
    println!("========================================");

    Ok(())
}
