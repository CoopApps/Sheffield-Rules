/// Whites Directory Matcher
///
/// Matches Whites Directory business entries (sheffield_businesses table) to existing people
/// in the Sheffield database based on name, address, and profession. Supports both automatic
/// matching and manual drag-and-drop matching through the GUI.

use sqlx::{SqlitePool, FromRow};
use serde::{Deserialize, Serialize};

/// Represents a business entry from White's Directory (sheffield_businesses table)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WhitesEntry {
    pub id: String,
    pub surname: Option<String>,
    pub forename: Option<String>,
    pub full_name: Option<String>,
    pub title: Option<String>,
    pub occupation: Option<String>,
    pub address: Option<String>,
    pub year: i64,
    pub source: Option<String>,
    pub person_id: Option<String>,
}

/// Represents a candidate person who could be matched to a business
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PlayerCandidate {
    pub id: String,
    pub name: String,
    pub first_name: Option<String>,
    pub surname: Option<String>,
    pub birth_year: Option<i64>,
    pub profession: Option<String>,
    pub street_address: Option<String>,
    pub civil_parish: Option<String>,
    pub ecclesiastical_parish: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WhitesMatchSuggestion {
    pub whites_entry: WhitesEntry,
    pub candidates: Vec<PlayerCandidateWithScore>,
}

#[derive(Debug, Serialize)]
pub struct PlayerCandidateWithScore {
    pub player: PlayerCandidate,
    pub confidence_score: f32,
    pub match_factors: MatchFactors,
    pub match_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchFactors {
    pub name_score: f32,
    pub address_score: f32,
    pub profession_score: f32,
    pub parish_score: f32,
}

#[derive(Debug, Serialize)]
pub struct WhitesMatchStats {
    pub total_whites_entries: usize,
    pub matched_entries: usize,
    pub unmatched_entries: usize,
    pub suggestions_generated: usize,
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

    let first_name = parts[0].to_string();
    let surname = parts.last().unwrap().to_string();

    (first_name, surname)
}

/// Normalize text for fuzzy matching
fn normalize_text(text: &str) -> String {
    text.to_lowercase()
        .replace("street", "st")
        .replace("road", "rd")
        .replace("lane", "ln")
        .replace("avenue", "ave")
        .replace(".", "")
        .replace(",", "")
        .trim()
        .to_string()
}

/// Normalize parish name for matching
fn normalize_parish(parish: &str) -> String {
    parish
        .to_lowercase()
        .replace("brightside bierlow", "brightside")
        .replace("ecclesall bierlow", "ecclesall")
        .replace("nether hallam", "hallam")
        .replace("upper hallam", "hallam")
        .replace("st philip", "philip")
        .replace("st george", "george")
        .replace("st mary", "mary")
        .replace("st paul", "paul")
        .trim()
        .to_string()
}

/// Calculate name similarity score (0.0 - 1.0)
fn calculate_name_score(whites_name: &str, player_name: &str) -> f32 {
    let (w_first, w_surname) = parse_name(whites_name);
    let (p_first, p_surname) = parse_name(player_name);

    let w_surname_norm = normalize_text(&w_surname);
    let p_surname_norm = normalize_text(&p_surname);
    let w_first_norm = normalize_text(&w_first);
    let p_first_norm = normalize_text(&p_first);

    let mut score: f32 = 0.0;

    // Surname match is most important (0.7 weight)
    if w_surname_norm == p_surname_norm {
        score += 0.7;
    } else if !w_surname_norm.is_empty() && !p_surname_norm.is_empty() {
        if p_surname_norm.contains(&w_surname_norm) || w_surname_norm.contains(&p_surname_norm) {
            score += 0.5;
        }
    }

    // First name match (0.3 weight)
    if !w_first_norm.is_empty() && !p_first_norm.is_empty() {
        if w_first_norm == p_first_norm {
            score += 0.3;
        } else if w_first_norm.starts_with(&p_first_norm.chars().next().unwrap_or(' ').to_string())
            || p_first_norm.starts_with(&w_first_norm.chars().next().unwrap_or(' ').to_string())
        {
            score += 0.15; // Initial match
        }
    }

    score
}

/// Calculate address similarity score (0.0 - 1.0)
fn calculate_address_score(whites_address: &str, player_address: Option<&String>) -> f32 {
    let Some(player_addr) = player_address else {
        return 0.0;
    };

    let w_norm = normalize_text(whites_address);
    let p_norm = normalize_text(player_addr);

    if w_norm == p_norm {
        return 1.0;
    }

    // Extract house numbers
    let w_number: Option<i32> = w_norm
        .split_whitespace()
        .next()
        .and_then(|s| s.parse().ok());
    let p_number: Option<i32> = p_norm
        .split_whitespace()
        .next()
        .and_then(|s| s.parse().ok());

    let mut score: f32 = 0.0;

    // Check if addresses contain each other
    if w_norm.contains(&p_norm) || p_norm.contains(&w_norm) {
        score += 0.8;
    } else {
        // Check street name similarity
        let w_parts: Vec<&str> = w_norm.split_whitespace().skip(1).collect();
        let p_parts: Vec<&str> = p_norm.split_whitespace().skip(1).collect();

        let w_street = w_parts.join(" ");
        let p_street = p_parts.join(" ");

        if w_street == p_street && !w_street.is_empty() {
            score += 0.5;
        }
    }

    // House number match bonus
    if let (Some(w_num), Some(p_num)) = (w_number, p_number) {
        if w_num == p_num {
            score += 0.5;
        }
    }

    score.min(1.0)
}

/// Calculate profession similarity score (0.0 - 1.0)
fn calculate_profession_score(
    whites_profession: Option<&String>,
    player_profession: Option<&String>,
) -> f32 {
    let Some(w_prof) = whites_profession else {
        return 0.0;
    };
    let Some(p_prof) = player_profession else {
        return 0.0;
    };

    let w_norm = normalize_text(w_prof);
    let p_norm = normalize_text(p_prof);

    if w_norm == p_norm {
        return 1.0;
    }

    if w_norm.contains(&p_norm) || p_norm.contains(&w_norm) {
        return 0.7;
    }

    // Check for common profession synonyms
    let synonyms = vec![
        (vec!["cutler", "knife maker"], vec!["cutler", "knife maker"]),
        (vec!["innkeeper", "publican", "landlord"], vec!["innkeeper", "publican", "landlord"]),
        (vec!["grocer", "shopkeeper"], vec!["grocer", "shopkeeper"]),
        (vec!["blacksmith", "farrier"], vec!["blacksmith", "farrier"]),
    ];

    for (group1, group2) in synonyms {
        let w_in_group = group1.iter().any(|s| w_norm.contains(s));
        let p_in_group = group2.iter().any(|s| p_norm.contains(s));
        if w_in_group && p_in_group {
            return 0.8;
        }
    }

    0.0
}

/// Calculate parish similarity score (0.0 - 1.0)
fn calculate_parish_score(
    whites_parish: Option<&String>,
    player_parish: Option<&String>,
) -> f32 {
    let Some(w_parish) = whites_parish else {
        return 0.0;
    };
    let Some(p_parish) = player_parish else {
        return 0.0;
    };

    let w_norm = normalize_parish(w_parish);
    let p_norm = normalize_parish(p_parish);

    if w_norm == p_norm {
        return 1.0;
    }

    if w_norm.contains(&p_norm) || p_norm.contains(&w_norm) {
        return 0.7;
    }

    0.0
}

/// Calculate overall match confidence with weighted factors
fn calculate_match_confidence(
    business: &WhitesEntry,
    player: &PlayerCandidate,
) -> (f32, MatchFactors, String) {
    // Build full name from business forename + surname
    let business_name = match (&business.forename, &business.surname) {
        (Some(f), Some(s)) => format!("{} {}", f, s),
        (None, Some(s)) => s.clone(),
        (Some(f), None) => f.clone(),
        (None, None) => business.full_name.clone().unwrap_or_default(),
    };

    // Calculate individual factor scores
    let name_score = calculate_name_score(&business_name, &player.name);
    let address_score = calculate_address_score(
        business.address.as_deref().unwrap_or(""),
        player.street_address.as_ref(),
    );
    let profession_score = calculate_profession_score(
        business.occupation.as_ref(),
        player.profession.as_ref(),
    );

    // Business table doesn't have parish, so parish score is always 0
    let parish_score = 0.0;

    let factors = MatchFactors {
        name_score,
        address_score,
        profession_score,
        parish_score,
    };

    // Weighted combination - parish removed so readjust weights
    // Name: 40%, Address: 35%, Profession: 25%
    let confidence = name_score * 0.4 + address_score * 0.35 + profession_score * 0.25;

    // Build reason string
    let mut reasons = Vec::new();
    if name_score >= 0.7 {
        reasons.push("strong name match");
    } else if name_score >= 0.5 {
        reasons.push("partial name match");
    }
    if address_score >= 0.8 {
        reasons.push("address match");
    }
    if profession_score >= 0.7 {
        reasons.push("profession match");
    }

    let reason = if reasons.is_empty() {
        "weak match".to_string()
    } else {
        reasons.join(", ")
    };

    (confidence, factors, reason)
}

/// Get all unmatched businesses from Whites Directory
pub async fn get_unmatched_whites_entries(
    pool: &SqlitePool,
) -> Result<Vec<WhitesEntry>, sqlx::Error> {
    sqlx::query_as::<_, WhitesEntry>(
        r#"
        SELECT
            id, surname, forename, full_name, title, occupation, address,
            year, source, person_id
        FROM sheffield_businesses
        WHERE person_id IS NULL
        ORDER BY surname, forename
        "#
    )
    .fetch_all(pool)
    .await
}

/// Get all people candidates (excluding those already matched to businesses)
pub async fn get_player_candidates(
    pool: &SqlitePool,
) -> Result<Vec<PlayerCandidate>, sqlx::Error> {
    // Return empty initially - will be populated by search
    Ok(Vec::new())
}

/// Search for player candidates by name, profession, or address
pub async fn search_player_candidates(
    pool: &SqlitePool,
    search_term: &str,
    limit: i64,
) -> Result<Vec<PlayerCandidate>, sqlx::Error> {
    // Simplified query - just search by surname which is indexed
    let search_pattern = format!("{}%", search_term); // Start with instead of contains

    sqlx::query_as::<_, PlayerCandidate>(
        r#"
        SELECT DISTINCT
            id,
            name,
            first_name,
            surname,
            birth_year,
            profession,
            street_address,
            civil_parish,
            ecclesiastical_parish
        FROM sheffield_people
        WHERE surname IS NOT NULL
        AND surname LIKE ?1
        LIMIT ?2
        "#
    )
    .bind(&search_pattern)
    .bind(limit)
    .fetch_all(pool)
    .await
}

/// Create indexes on sheffield_people for faster searching
pub async fn create_people_indexes(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_people_surname ON sheffield_people(surname)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_people_name ON sheffield_people(name)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_people_first_name ON sheffield_people(first_name)")
        .execute(pool)
        .await?;

    Ok(())
}

/// Generate match suggestions for all unmatched whites entries
pub async fn generate_match_suggestions(
    pool: &SqlitePool,
    min_confidence: f32,
    max_suggestions_per_entry: usize,
) -> Result<Vec<WhitesMatchSuggestion>, sqlx::Error> {
    let whites_entries = get_unmatched_whites_entries(pool).await?;
    let players = get_player_candidates(pool).await?;

    let mut suggestions = Vec::new();

    for whites_entry in whites_entries {
        let mut candidates = Vec::new();

        for player in &players {
            let (confidence, factors, reason) =
                calculate_match_confidence(&whites_entry, player);

            if confidence >= min_confidence {
                candidates.push(PlayerCandidateWithScore {
                    player: player.clone(),
                    confidence_score: confidence,
                    match_factors: factors,
                    match_reason: reason,
                });
            }
        }

        // Sort by confidence descending
        candidates.sort_by(|a, b| b.confidence_score.partial_cmp(&a.confidence_score).unwrap());

        // Take top N candidates
        candidates.truncate(max_suggestions_per_entry);

        if !candidates.is_empty() {
            suggestions.push(WhitesMatchSuggestion {
                whites_entry,
                candidates,
            });
        }
    }

    Ok(suggestions)
}

/// Accept a manual match (from drag-and-drop or suggestion)
pub async fn accept_manual_match(
    pool: &SqlitePool,
    business_id: &str,
    person_id: &str,
) -> Result<(), sqlx::Error> {
    // Simply update the business's person_id field
    sqlx::query(
        "UPDATE sheffield_businesses SET person_id = ? WHERE id = ?"
    )
    .bind(person_id)
    .bind(business_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Reject a match suggestion (currently does nothing, but could store rejection history)
pub async fn reject_match(
    _pool: &SqlitePool,
    _business_id: &str,
    _person_id: &str,
    _notes: Option<String>,
) -> Result<(), sqlx::Error> {
    // Could implement a rejection history table here if needed
    // For now, just do nothing - the business remains unmatched
    Ok(())
}

/// Get match statistics
pub async fn get_match_stats(pool: &SqlitePool) -> Result<WhitesMatchStats, sqlx::Error> {
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_businesses")
        .fetch_one(pool)
        .await?;

    let matched: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_businesses WHERE person_id IS NOT NULL"
    )
    .fetch_one(pool)
    .await?;

    Ok(WhitesMatchStats {
        total_whites_entries: total.0 as usize,
        matched_entries: matched.0 as usize,
        unmatched_entries: (total.0 - matched.0) as usize,
        suggestions_generated: 0, // Will be updated when suggestions are generated
    })
}
