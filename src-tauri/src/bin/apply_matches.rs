use sqlx::sqlite::SqlitePool;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

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
    let mut score = 0.5; // Base score for name match

    if let (Some(c_year), Some(g_year)) = (census.birth_year, gen.birth_year) {
        if c_year == g_year {
            score += 0.4;
        } else if (c_year - g_year).abs() <= 2 {
            score += 0.2;
        }
    }

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

    println!("Loading and indexing data...\n");

    // Load and index genealogy
    let genealogy_records: Vec<GenealogyRecord> = sqlx::query_as(
        "SELECT id, name, address, profession, spouse, relation, birth_year, parish FROM unmatched_genealogy"
    )
    .fetch_all(&pool)
    .await?;

    let mut gen_by_name: HashMap<String, Vec<GenealogyRecord>> = HashMap::new();
    for gen in genealogy_records {
        let norm_name = normalize_name(&gen.name);
        gen_by_name.entry(norm_name).or_insert_with(Vec::new).push(gen);
    }

    // Load census
    let census_records: Vec<CensusRecord> = sqlx::query_as(
        "SELECT id, name, civil_parish, household_members, birth_year, relation FROM unmatched_sheffieldcensus"
    )
    .fetch_all(&pool)
    .await?;

    // Build uniqueness indexes
    let mut census_by_name_year: HashMap<(String, Option<i64>), Vec<&CensusRecord>> = HashMap::new();
    for census in &census_records {
        let norm_name = normalize_name(&census.name);
        let key = (norm_name, census.birth_year);
        census_by_name_year.entry(key).or_insert_with(Vec::new).push(census);
    }

    let mut gen_by_name_year: HashMap<(String, Option<i64>), Vec<&GenealogyRecord>> = HashMap::new();
    for gen_list in gen_by_name.values() {
        for gen in gen_list {
            let norm_name = normalize_name(&gen.name);
            let key = (norm_name, gen.birth_year);
            gen_by_name_year.entry(key).or_insert_with(Vec::new).push(gen);
        }
    }

    // Find matches
    println!("Finding high confidence matches...");
    let mut matches_to_apply = Vec::new();

    for census in &census_records {
        let norm_name = normalize_name(&census.name);

        if let Some(gen_candidates) = gen_by_name.get(&norm_name) {
            let mut best_match: Option<(&GenealogyRecord, f32)> = None;

            for gen in gen_candidates {
                let score = calculate_match_score(census, gen);

                if score >= 0.7 {
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
                // Check uniqueness
                let census_key = (norm_name.clone(), census.birth_year);
                let gen_key = (norm_name, gen.birth_year);

                let census_is_unique = census_by_name_year.get(&census_key)
                    .map(|v| v.len() == 1)
                    .unwrap_or(false);

                let gen_is_unique = gen_by_name_year.get(&gen_key)
                    .map(|v| v.len() == 1)
                    .unwrap_or(false);

                if census_is_unique && gen_is_unique {
                    matches_to_apply.push((census.clone(), gen.clone(), score));
                }
            }
        }
    }

    println!("Found {} high confidence unique matches\n", matches_to_apply.len());

    if matches_to_apply.is_empty() {
        println!("No matches to apply!");
        return Ok(());
    }

    println!("Applying matches to sheffield_people...\n");

    let mut applied = 0;
    let mut failed = 0;

    for (census, gen, score) in &matches_to_apply {
        println!("Merging: {} (born {}) - Score: {:.0}%",
            census.name,
            census.birth_year.map(|y| y.to_string()).unwrap_or("?".to_string()),
            score * 100.0
        );

        // Create merged record
        let new_id = Uuid::new_v4().to_string();

        // Parse name components
        let name_parts: Vec<&str> = census.name.split_whitespace().collect();
        let first_name = name_parts.first().map(|s| s.to_string());
        let surname = name_parts.last().map(|s| s.to_string());

        // Insert into sheffield_people
        let result = sqlx::query(
            r#"
            INSERT INTO sheffield_people (
                id, name, first_name, surname, source, birth_year,
                civil_parish, street_address, profession,
                census_relation, census_household_members
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&new_id)
        .bind(&census.name)
        .bind(&first_name)
        .bind(&surname)
        .bind("matched_census_genealogy")
        .bind(census.birth_year)
        .bind(&census.civil_parish)
        .bind(&gen.address)
        .bind(&gen.profession)
        .bind(&census.relation)
        .bind(&census.household_members)
        .execute(&pool)
        .await;

        match result {
            Ok(_) => {
                // Delete from unmatched tables
                sqlx::query("DELETE FROM unmatched_sheffieldcensus WHERE id = ?")
                    .bind(&census.id)
                    .execute(&pool)
                    .await?;

                sqlx::query("DELETE FROM unmatched_genealogy WHERE id = ?")
                    .bind(&gen.id)
                    .execute(&pool)
                    .await?;

                applied += 1;
                println!("  ✓ Applied successfully");
            }
            Err(e) => {
                failed += 1;
                println!("  ✗ Failed: {}", e);
            }
        }
    }

    println!("\n========================================");
    println!("RESULTS:");
    println!("========================================");
    println!("Matches applied: {}", applied);
    println!("Failed: {}", failed);
    println!("========================================");

    Ok(())
}
