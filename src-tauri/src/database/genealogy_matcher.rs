/// Genealogy Data Matcher
///
/// Matches genealogy CSV records (with addresses and professions) to existing
/// players in the Sheffield database based on name, birth year, and parish.

use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct GenealogyRecord {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Address")]
    pub address: String,
    #[serde(rename = "Parish")]
    pub parish: String,
    #[serde(rename = "Age")]
    pub age: Option<String>,
    #[serde(rename = "Born Approx")]
    pub born_approx: Option<String>,
    #[serde(rename = "Birth Place")]
    pub birth_place: String,
    #[serde(rename = "Relation")]
    pub relation: String,
    #[serde(rename = "Profession")]
    pub profession: String,
}

#[derive(Debug, Serialize)]
pub struct MatchResult {
    pub player_id: String,
    pub player_name: String,
    pub player_birth_year: i32,
    pub player_parish: Option<String>,
    pub matched_address: String,
    pub matched_profession: String,
    pub match_confidence: f32, // 0.0 - 1.0
    pub match_reason: String,
}

#[derive(Debug, Serialize)]
pub struct MatchStats {
    pub total_genealogy_records: usize,
    pub total_male_records: usize,
    pub total_players_in_db: usize,
    pub exact_matches: usize,
    pub fuzzy_matches: usize,
    pub no_matches: usize,
    pub avg_confidence: f32,
}

/// Parse a full name into first name and surname
fn parse_name(full_name: &str) -> (String, String) {
    let parts: Vec<&str> = full_name.trim().split_whitespace().collect();

    if parts.is_empty() {
        return (String::new(), String::new());
    }

    if parts.len() == 1 {
        return (String::new(), parts[0].to_string());
    }

    // First name is the first word, surname is the last word
    let first_name = parts[0].to_string();
    let surname = parts.last().unwrap().to_string();

    (first_name, surname)
}

/// Normalize parish name for matching
fn normalize_parish(parish: &str) -> String {
    parish
        .to_lowercase()
        .replace("brightside bierlow", "brightside")
        .replace("ecclesall bierlow", "ecclesall")
        .replace("nether hallam", "hallam")
        .trim()
        .to_string()
}

/// Calculate match confidence based on multiple factors
fn calculate_match_confidence(
    player_name: &str,
    player_birth_year: i32,
    player_parish: &Option<String>,
    gen_name: &str,
    gen_birth_year: i32,
    gen_parish: &str,
) -> (f32, String) {
    let mut score = 0.0;
    let mut reasons = Vec::new();

    // Name match (0.5 weight)
    let (gen_first, gen_surname) = parse_name(gen_name);
    let player_lower = player_name.to_lowercase();
    let gen_surname_lower = gen_surname.to_lowercase();

    if player_lower.contains(&gen_surname_lower) {
        score += 0.5;
        reasons.push("surname match");
    }

    // Birth year match (0.3 weight)
    let year_diff = (player_birth_year - gen_birth_year).abs();
    if year_diff == 0 {
        score += 0.3;
        reasons.push("exact birth year");
    } else if year_diff == 1 {
        score += 0.25;
        reasons.push("birth year ±1");
    } else if year_diff == 2 {
        score += 0.15;
        reasons.push("birth year ±2");
    }

    // Parish match (0.2 weight)
    if let Some(p_parish) = player_parish {
        let p_norm = normalize_parish(p_parish);
        let g_norm = normalize_parish(gen_parish);

        if p_norm == g_norm || p_norm.contains(&g_norm) || g_norm.contains(&p_norm) {
            score += 0.2;
            reasons.push("parish match");
        }
    }

    (score, reasons.join(", "))
}

/// Test matching genealogy records against existing players
pub async fn test_match_genealogy(
    pool: &SqlitePool,
    records: Vec<GenealogyRecord>,
    min_confidence: f32,
) -> Result<(Vec<MatchResult>, MatchStats), Box<dyn std::error::Error>> {

    // Filter to male records only (Head, Son, Lodger, etc.)
    let male_relations = ["Head", "Son", "Lodger", "Boarder", "Nephew", "Brother", "Father"];
    let male_records: Vec<_> = records.iter()
        .filter(|r| male_relations.contains(&r.relation.as_str()))
        .filter(|r| {
            // Exclude if age > 40 (as requested)
            if let Some(age_str) = &r.age {
                if let Ok(age) = age_str.parse::<i32>() {
                    return age <= 40;
                }
            }
            true
        })
        .collect();

    println!("Filtered to {} male records (age <= 40)", male_records.len());

    // Get all players from database
    let players = sqlx::query_as::<_, (String, String, Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>)>(
        "SELECT id, name, first_name, surname, birth_year, ecclesiastical_parish, civil_parish
         FROM sheffield_footballers
         WHERE birth_year IS NOT NULL"
    )
    .fetch_all(pool)
    .await?;

    println!("Found {} players in database", players.len());

    let mut matches = Vec::new();
    let mut exact_count = 0;
    let mut fuzzy_count = 0;
    let mut total_confidence = 0.0;

    // Try to match each male record
    for gen_record in &male_records {
        let gen_birth_year = match &gen_record.born_approx {
            Some(year_str) => year_str.parse::<i32>().unwrap_or(0),
            None => continue,
        };

        if gen_birth_year == 0 {
            continue;
        }

        // Find best matching player
        let mut best_match: Option<(String, String, i32, Option<String>, f32, String)> = None;

        for player in &players {
            let (player_id, player_name, player_first_name, player_surname, player_birth_year, eccl_parish, civil_parish) = player;
            let player_parish = eccl_parish.as_ref().or(civil_parish.as_ref()).cloned();

            let (confidence, reason) = calculate_match_confidence(
                player_name,
                player_birth_year.unwrap_or(0),
                &player_parish,
                &gen_record.name,
                gen_birth_year,
                &gen_record.parish,
            );

            if confidence >= min_confidence {
                if best_match.is_none() || confidence > best_match.as_ref().unwrap().4 {
                    best_match = Some((
                        player_id.clone(),
                        player_name.clone(),
                        player_birth_year.unwrap_or(0),
                        player_parish.clone(),
                        confidence,
                        reason,
                    ));
                }
            }
        }

        // Record match if found
        if let Some((id, name, birth_year, parish, confidence, reason)) = best_match {
            if confidence >= 0.9 {
                exact_count += 1;
            } else {
                fuzzy_count += 1;
            }

            total_confidence += confidence;

            matches.push(MatchResult {
                player_id: id,
                player_name: name,
                player_birth_year: birth_year,
                player_parish: parish,
                matched_address: gen_record.address.clone(),
                matched_profession: gen_record.profession.clone(),
                match_confidence: confidence,
                match_reason: reason,
            });
        }
    }

    let avg_confidence = if matches.is_empty() {
        0.0
    } else {
        total_confidence / matches.len() as f32
    };

    let stats = MatchStats {
        total_genealogy_records: records.len(),
        total_male_records: male_records.len(),
        total_players_in_db: players.len(),
        exact_matches: exact_count,
        fuzzy_matches: fuzzy_count,
        no_matches: male_records.len() - matches.len(),
        avg_confidence,
    };

    Ok((matches, stats))
}
