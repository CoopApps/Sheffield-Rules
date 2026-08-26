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

fn calculate_match_score(census: &CensusRecord, gen: &GenealogyRecord) -> (f32, String, bool, bool) {
    let mut score = 0.0;
    let mut reasons = Vec::new();

    let name_sim = name_similarity_score(&census.name, &gen.name);
    let name_score = name_sim * 0.5;
    score += name_score;
    reasons.push(format!("Name: {:.1}%", name_sim * 100.0));

    let mut has_parish_data = false;
    let mut parish_matches = false;

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
            }
        }
    }

    if let (Some(c_parish), Some(g_parish)) = (&census.civil_parish, &gen.parish) {
        has_parish_data = true;
        let c_norm = c_parish.to_lowercase();
        let g_norm = g_parish.to_lowercase();
        if c_norm == g_norm {
            score += 0.1;
            parish_matches = true;
            reasons.push("Parish: exact".to_string());
        } else if c_norm.contains(&g_norm) || g_norm.contains(&c_norm) {
            score += 0.05;
            parish_matches = true;
            reasons.push("Parish: partial".to_string());
        } else {
            reasons.push("Parish: MISMATCH".to_string());
        }
    }

    (score, reasons.join(", "), has_parish_data, parish_matches)
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

    println!("Loaded {} census records", census_records.len());

    println!("Loading genealogy records...");
    let genealogy_records: Vec<GenealogyRecord> = sqlx::query_as(
        "SELECT id, name, birth_year, address, parish FROM unmatched_genealogy"
    )
    .fetch_all(&pool)
    .await?;

    println!("Loaded {} genealogy records\n", genealogy_records.len());

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

    println!("Finding medium confidence matches...\n");

    let mut medium_confidence_matches = Vec::new();

    for census in census_records.iter() {
        let norm_name = normalize_name(&census.name);
        let prefix = if norm_name.len() >= 4 {
            norm_name[..4].to_string()
        } else {
            norm_name.clone()
        };

        if let Some(gen_candidates) = gen_by_name_prefix.get(&prefix) {
            let mut best_match: Option<(&GenealogyRecord, f32, String, bool, bool)> = None;

            for gen in gen_candidates {
                if let (Some(c_year), Some(g_year)) = (census.birth_year, gen.birth_year) {
                    if (c_year - g_year).abs() > 5 {
                        continue;
                    }
                }

                let (score, reasons, has_parish, parish_match) = calculate_match_score(census, gen);

                if score >= 0.65 && score < 0.8 {
                    if let Some((_, best_score, _, _, _)) = best_match {
                        if score > best_score {
                            best_match = Some((gen, score, reasons, has_parish, parish_match));
                        }
                    } else {
                        best_match = Some((gen, score, reasons, has_parish, parish_match));
                    }
                }
            }

            if let Some((gen, score, reasons, has_parish, parish_match)) = best_match {
                medium_confidence_matches.push((census.clone(), gen.clone(), score, reasons, has_parish, parish_match));
            }
        }
    }

    println!("========================================");
    println!("MEDIUM CONFIDENCE ANALYSIS (65-79%)");
    println!("========================================");
    println!("Total medium confidence: {}", medium_confidence_matches.len());

    let with_parish_data: Vec<_> = medium_confidence_matches.iter()
        .filter(|(_, _, _, _, has_parish, _)| *has_parish)
        .collect();

    let parish_matches: Vec<_> = medium_confidence_matches.iter()
        .filter(|(_, _, _, _, _, parish_match)| *parish_match)
        .collect();

    let parish_mismatches: Vec<_> = medium_confidence_matches.iter()
        .filter(|(_, _, _, _, has_parish, parish_match)| *has_parish && !*parish_match)
        .collect();

    println!("With parish data: {} ({:.1}%)",
        with_parish_data.len(),
        (with_parish_data.len() as f64 / medium_confidence_matches.len() as f64) * 100.0
    );
    println!("Parish matches: {} ({:.1}%)",
        parish_matches.len(),
        (parish_matches.len() as f64 / medium_confidence_matches.len() as f64) * 100.0
    );
    println!("Parish mismatches: {} ({:.1}%)",
        parish_mismatches.len(),
        (parish_mismatches.len() as f64 / medium_confidence_matches.len() as f64) * 100.0
    );
    println!();

    println!("SAMPLE WITH PARISH MATCHES (likely correct):");
    println!("----------------------------------------");
    for (i, (census, gen, score, reasons, _, _)) in parish_matches.iter().take(15).enumerate() {
        println!("\n{}. Score: {:.1}% ({})", i + 1, score * 100.0, reasons);
        println!("   Census:    {} (born {}, {})",
            census.name,
            census.birth_year.map(|y| y.to_string()).unwrap_or("?".to_string()),
            census.civil_parish.as_deref().unwrap_or("no parish")
        );
        println!("   Genealogy: {} (born {}, {})",
            gen.name,
            gen.birth_year.map(|y| y.to_string()).unwrap_or("?".to_string()),
            gen.parish.as_deref().unwrap_or("no parish")
        );
    }

    println!("\n\nSAMPLE WITH PARISH MISMATCHES (likely wrong):");
    println!("----------------------------------------");
    for (i, (census, gen, score, reasons, _, _)) in parish_mismatches.iter().take(15).enumerate() {
        println!("\n{}. Score: {:.1}% ({})", i + 1, score * 100.0, reasons);
        println!("   Census:    {} (born {}, parish: {})",
            census.name,
            census.birth_year.map(|y| y.to_string()).unwrap_or("?".to_string()),
            census.civil_parish.as_deref().unwrap_or("no parish")
        );
        println!("   Genealogy: {} (born {}, parish: {})",
            gen.name,
            gen.birth_year.map(|y| y.to_string()).unwrap_or("?".to_string()),
            gen.parish.as_deref().unwrap_or("no parish")
        );
    }

    println!("\n========================================");
    println!("RECOMMENDATION:");
    println!("========================================");
    println!("Medium confidence matches WITH parish match: {} (likely good matches)", parish_matches.len());
    println!("Medium confidence matches WITH parish MISMATCH: {} (likely false positives)", parish_mismatches.len());
    println!("Medium confidence matches WITHOUT parish data: {} (uncertain)",
        medium_confidence_matches.len() - with_parish_data.len());
    println!();
    println!("Extrapolated to full dataset:");
    let ratio = medium_confidence_matches.len() as f64 / 1000.0;
    println!("  Parish matches: ~{:.0}", (parish_matches.len() as f64 / ratio));
    println!("  Parish mismatches: ~{:.0}", (parish_mismatches.len() as f64 / ratio));
    println!("========================================");

    Ok(())
}
