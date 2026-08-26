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

fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(
                    matrix[i][j + 1] + 1,
                    matrix[i + 1][j] + 1,
                ),
                matrix[i][j] + cost,
            );
        }
    }

    matrix[len1][len2]
}

fn name_similarity_score(name1: &str, name2: &str) -> f32 {
    let norm1 = normalize_name(name1);
    let norm2 = normalize_name(name2);

    if norm1 == norm2 {
        return 1.0;
    }

    let distance = levenshtein_distance(&norm1, &norm2);
    let max_len = std::cmp::max(norm1.len(), norm2.len());

    if max_len == 0 {
        return 0.0;
    }

    let similarity = 1.0 - (distance as f32 / max_len as f32);
    let contains_bonus = if norm1.contains(&norm2) || norm2.contains(&norm1) {
        0.1
    } else {
        0.0
    };

    (similarity + contains_bonus).min(1.0)
}

fn calculate_match_score(census: &CensusRecord, gen: &GenealogyRecord) -> f32 {
    let mut score = 0.0;

    let name_sim = name_similarity_score(&census.name, &gen.name);
    let name_score = name_sim * 0.5;
    score += name_score;

    if let (Some(c_year), Some(g_year)) = (census.birth_year, gen.birth_year) {
        if c_year == g_year {
            score += 0.4;
        } else {
            let diff = (c_year - g_year).abs();
            if diff <= 1 {
                score += 0.3;
            } else if diff <= 2 {
                score += 0.2;
            } else if diff <= 5 {
                score += 0.1;
            }
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

    println!("Loading genealogy records...");
    let genealogy_records: Vec<GenealogyRecord> = sqlx::query_as(
        "SELECT id, name, address, profession, spouse, relation, birth_year, parish FROM unmatched_genealogy"
    )
    .fetch_all(&pool)
    .await?;

    println!("Loaded {} genealogy records", genealogy_records.len());

    // Index genealogy by name prefix for faster lookup
    let mut gen_by_name_prefix: HashMap<String, Vec<GenealogyRecord>> = HashMap::new();
    for gen in genealogy_records {
        let norm_name = normalize_name(&gen.name);
        let prefix = if norm_name.len() >= 4 {
            norm_name[..4].to_string()
        } else {
            norm_name.clone()
        };
        gen_by_name_prefix.entry(prefix).or_insert_with(Vec::new).push(gen);
    }

    println!("Loading census records...");
    let census_records: Vec<CensusRecord> = sqlx::query_as(
        "SELECT id, name, civil_parish, household_members, birth_year, relation FROM unmatched_sheffieldcensus"
    )
    .fetch_all(&pool)
    .await?;

    println!("Loaded {} census records\n", census_records.len());

    println!("Finding high confidence matches (≥80%)...");
    let mut matches_to_apply = Vec::new();

    for (i, census) in census_records.iter().enumerate() {
        if i % 1000 == 0 && i > 0 {
            println!("Processed {}/{} census records, found {} matches so far...",
                i, census_records.len(), matches_to_apply.len());
        }

        let norm_name = normalize_name(&census.name);
        let prefix = if norm_name.len() >= 4 {
            norm_name[..4].to_string()
        } else {
            norm_name.clone()
        };

        if let Some(gen_candidates) = gen_by_name_prefix.get(&prefix) {
            let mut best_match: Option<(&GenealogyRecord, f32)> = None;

            for gen in gen_candidates {
                // Quick filter: only consider if birth years are within 5 years or missing
                if let (Some(c_year), Some(g_year)) = (census.birth_year, gen.birth_year) {
                    if (c_year - g_year).abs() > 5 {
                        continue;
                    }
                }

                let score = calculate_match_score(census, gen);

                if score >= 0.8 {
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
                matches_to_apply.push((census.clone(), gen.clone(), score));
            }
        }
    }

    println!("\n========================================");
    println!("Found {} high confidence matches (≥80%)", matches_to_apply.len());
    println!("========================================\n");

    if matches_to_apply.is_empty() {
        println!("No matches to apply!");
        return Ok(());
    }

    println!("Applying matches to sheffield_people...\n");

    let mut applied = 0;
    let mut failed = 0;

    for (census, gen, score) in &matches_to_apply {
        if applied % 100 == 0 && applied > 0 {
            println!("Applied {}/{} matches...", applied, matches_to_apply.len());
        }

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
        .bind("fuzzy_matched_census_genealogy")
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
            }
            Err(e) => {
                failed += 1;
                if failed <= 10 {
                    println!("  ✗ Failed to match {} ({}): {}", census.name, score, e);
                }
            }
        }
    }

    println!("\n========================================");
    println!("RESULTS:");
    println!("========================================");
    println!("Matches applied: {}", applied);
    println!("Failed: {}", failed);
    println!("Success rate: {:.1}%", (applied as f64 / matches_to_apply.len() as f64) * 100.0);
    println!("========================================");

    // Show sample of what was matched
    println!("\nSample of matched records:");
    matches_to_apply.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    for (i, (census, gen, score)) in matches_to_apply.iter().take(10).enumerate() {
        println!("\n{}. Score: {:.1}%", i + 1, score * 100.0);
        println!("   Census:    {} (born {})",
            census.name,
            census.birth_year.map(|y| y.to_string()).unwrap_or("?".to_string())
        );
        println!("   Genealogy: {} (born {})",
            gen.name,
            gen.birth_year.map(|y| y.to_string()).unwrap_or("?".to_string())
        );
    }

    Ok(())
}
