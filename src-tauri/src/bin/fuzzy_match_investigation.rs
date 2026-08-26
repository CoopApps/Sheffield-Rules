use sqlx::sqlite::SqlitePool;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
struct CensusRecord {
    id: String,
    name: String,
    birth_year: Option<i64>,
    civil_parish: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
struct GenealogyRecord {
    id: String,
    name: String,
    birth_year: Option<i64>,
    address: Option<String>,
    parish: Option<String>,
}

fn normalize_name(name: &str) -> String {
    name.to_lowercase()
        .replace(".", "")
        .replace(",", "")
        .trim()
        .to_string()
}

// Calculate Levenshtein distance (edit distance) between two strings
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
                    matrix[i][j + 1] + 1,     // deletion
                    matrix[i + 1][j] + 1,     // insertion
                ),
                matrix[i][j] + cost,          // substitution
            );
        }
    }

    matrix[len1][len2]
}

fn name_similarity_score(name1: &str, name2: &str) -> f32 {
    let norm1 = normalize_name(name1);
    let norm2 = normalize_name(name2);

    // Exact match
    if norm1 == norm2 {
        return 1.0;
    }

    // Calculate edit distance
    let distance = levenshtein_distance(&norm1, &norm2);
    let max_len = std::cmp::max(norm1.len(), norm2.len());

    if max_len == 0 {
        return 0.0;
    }

    // Convert distance to similarity (0.0 to 1.0)
    let similarity = 1.0 - (distance as f32 / max_len as f32);

    // Also check if one name contains the other (for nicknames, abbreviations)
    let contains_bonus = if norm1.contains(&norm2) || norm2.contains(&norm1) {
        0.1
    } else {
        0.0
    };

    (similarity + contains_bonus).min(1.0)
}

fn calculate_match_score(census: &CensusRecord, gen: &GenealogyRecord) -> (f32, String) {
    let mut score = 0.0;
    let mut reasons = Vec::new();

    // Name similarity (0.0 to 0.5)
    let name_sim = name_similarity_score(&census.name, &gen.name);
    let name_score = name_sim * 0.5;
    score += name_score;
    reasons.push(format!("Name: {:.1}%", name_sim * 100.0));

    // Birth year match (0.0 to 0.4)
    if let (Some(c_year), Some(g_year)) = (census.birth_year, gen.birth_year) {
        if c_year == g_year {
            score += 0.4;
            reasons.push("BirthYear: exact".to_string());
        } else {
            let diff = (c_year - g_year).abs();
            if diff <= 1 {
                score += 0.3;
                reasons.push(format!("BirthYear: ±{}", diff));
            } else if diff <= 2 {
                score += 0.2;
                reasons.push(format!("BirthYear: ±{}", diff));
            } else if diff <= 5 {
                score += 0.1;
                reasons.push(format!("BirthYear: ±{}", diff));
            } else {
                reasons.push(format!("BirthYear: ±{} (low)", diff));
            }
        }
    } else {
        reasons.push("BirthYear: missing".to_string());
    }

    // Parish match (0.0 to 0.1)
    if let (Some(c_parish), Some(g_parish)) = (&census.civil_parish, &gen.parish) {
        let c_norm = c_parish.to_lowercase();
        let g_norm = g_parish.to_lowercase();
        if c_norm == g_norm {
            score += 0.1;
            reasons.push("Parish: exact".to_string());
        } else if c_norm.contains(&g_norm) || g_norm.contains(&c_norm) {
            score += 0.05;
            reasons.push("Parish: partial".to_string());
        }
    }

    (score, reasons.join(", "))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("Loading census records...");
    let census_records: Vec<CensusRecord> = sqlx::query_as(
        "SELECT id, name, birth_year, civil_parish FROM unmatched_sheffieldcensus LIMIT 1000"
    )
    .fetch_all(&pool)
    .await?;

    println!("Loaded {} census records (limited to first 1000 for analysis)", census_records.len());

    println!("Loading genealogy records...");
    let genealogy_records: Vec<GenealogyRecord> = sqlx::query_as(
        "SELECT id, name, birth_year, address, parish FROM unmatched_genealogy"
    )
    .fetch_all(&pool)
    .await?;

    println!("Loaded {} genealogy records\n", genealogy_records.len());

    // Index genealogy by first few chars of normalized name for faster lookup
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

    println!("Finding fuzzy matches...\n");

    let mut high_confidence_matches = Vec::new();
    let mut medium_confidence_matches = Vec::new();
    let mut low_confidence_matches = Vec::new();

    for (i, census) in census_records.iter().enumerate() {
        if i % 100 == 0 && i > 0 {
            println!("Processed {}/{} census records...", i, census_records.len());
        }

        let norm_name = normalize_name(&census.name);
        let prefix = if norm_name.len() >= 4 {
            norm_name[..4].to_string()
        } else {
            norm_name.clone()
        };

        // Only compare with genealogy records that have similar name prefix
        if let Some(gen_candidates) = gen_by_name_prefix.get(&prefix) {
            let mut best_match: Option<(&GenealogyRecord, f32, String)> = None;

            for gen in gen_candidates {
                // Quick filter: only consider if birth years are within 5 years or missing
                if let (Some(c_year), Some(g_year)) = (census.birth_year, gen.birth_year) {
                    if (c_year - g_year).abs() > 5 {
                        continue;
                    }
                }

                let (score, reasons) = calculate_match_score(census, gen);

                if score >= 0.5 {
                    if let Some((_, best_score, _)) = best_match {
                        if score > best_score {
                            best_match = Some((gen, score, reasons));
                        }
                    } else {
                        best_match = Some((gen, score, reasons));
                    }
                }
            }

            if let Some((gen, score, reasons)) = best_match {
                let match_info = (census.clone(), gen.clone(), score, reasons);

                if score >= 0.8 {
                    high_confidence_matches.push(match_info);
                } else if score >= 0.65 {
                    medium_confidence_matches.push(match_info);
                } else {
                    low_confidence_matches.push(match_info);
                }
            }
        }
    }

    println!("\n========================================");
    println!("FUZZY MATCHING INVESTIGATION RESULTS");
    println!("========================================");
    println!("Analyzed: {} census records", census_records.len());
    println!();
    println!("High confidence (≥80%): {}", high_confidence_matches.len());
    println!("Medium confidence (65-79%): {}", medium_confidence_matches.len());
    println!("Low confidence (50-64%): {}", low_confidence_matches.len());
    println!("========================================\n");

    // Show high confidence matches
    println!("HIGH CONFIDENCE MATCHES (≥80%):");
    println!("----------------------------------------");
    high_confidence_matches.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    for (i, (census, gen, score, reasons)) in high_confidence_matches.iter().take(30).enumerate() {
        println!("\n{}. Score: {:.1}% ({})", i + 1, score * 100.0, reasons);
        println!("   Census:    {} (born {})",
            census.name,
            census.birth_year.map(|y| y.to_string()).unwrap_or("?".to_string())
        );
        println!("   Genealogy: {} (born {})",
            gen.name,
            gen.birth_year.map(|y| y.to_string()).unwrap_or("?".to_string())
        );
    }

    println!("\n\nMEDIUM CONFIDENCE MATCHES (65-79%):");
    println!("----------------------------------------");
    medium_confidence_matches.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    for (i, (census, gen, score, reasons)) in medium_confidence_matches.iter().take(20).enumerate() {
        println!("\n{}. Score: {:.1}% ({})", i + 1, score * 100.0, reasons);
        println!("   Census:    {} (born {})",
            census.name,
            census.birth_year.map(|y| y.to_string()).unwrap_or("?".to_string())
        );
        println!("   Genealogy: {} (born {})",
            gen.name,
            gen.birth_year.map(|y| y.to_string()).unwrap_or("?".to_string())
        );
    }

    println!("\n\nLOW CONFIDENCE SAMPLE (50-64%):");
    println!("----------------------------------------");
    low_confidence_matches.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    for (i, (census, gen, score, reasons)) in low_confidence_matches.iter().take(10).enumerate() {
        println!("\n{}. Score: {:.1}% ({})", i + 1, score * 100.0, reasons);
        println!("   Census:    {} (born {})",
            census.name,
            census.birth_year.map(|y| y.to_string()).unwrap_or("?".to_string())
        );
        println!("   Genealogy: {} (born {})",
            gen.name,
            gen.birth_year.map(|y| y.to_string()).unwrap_or("?".to_string())
        );
    }

    println!("\n========================================");
    println!("ANALYSIS SUMMARY");
    println!("========================================");

    // Calculate match rate
    let total_matches = high_confidence_matches.len() + medium_confidence_matches.len() + low_confidence_matches.len();
    let match_rate = (total_matches as f64 / census_records.len() as f64) * 100.0;

    println!("Match rate: {:.1}% ({} of {} census records)", match_rate, total_matches, census_records.len());

    // Extrapolate to full dataset
    let estimated_high = (high_confidence_matches.len() as f64 / census_records.len() as f64) * 7504.0;
    let estimated_medium = (medium_confidence_matches.len() as f64 / census_records.len() as f64) * 7504.0;
    let estimated_low = (low_confidence_matches.len() as f64 / census_records.len() as f64) * 7504.0;

    println!("\nExtrapolated to full 7,504 census records:");
    println!("  High confidence: ~{:.0}", estimated_high);
    println!("  Medium confidence: ~{:.0}", estimated_medium);
    println!("  Low confidence: ~{:.0}", estimated_low);
    println!("  Total potential: ~{:.0}", estimated_high + estimated_medium + estimated_low);
    println!("========================================");

    Ok(())
}
