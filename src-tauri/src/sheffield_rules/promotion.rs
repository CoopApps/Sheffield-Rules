/// Promotion and Relegation System for Sheffield & Hallamshire League (1867 Fantasy Mode)
/// Handles automatic promotion/relegation based on final standings
/// with regional distribution logic for Divisions 6 & 7

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Standing {
    pub club_id: String,
    pub club_name: String,
    pub position: i32,
    pub played: i32,
    pub won: i32,
    pub drawn: i32,
    pub lost: i32,
    pub goals_for: i32,
    pub goals_against: i32,
    pub points: i32,
}

#[derive(Debug, Clone)]
pub struct PromotionEvent {
    pub club_id: String,
    pub club_name: String,
    pub from_division_id: String,
    pub to_division_id: String,
    pub final_position: i32,
}

#[derive(Debug, Clone)]
pub struct RelegationEvent {
    pub club_id: String,
    pub club_name: String,
    pub from_division_id: String,
    pub to_division_id: String,
    pub final_position: i32,
}

#[derive(Debug, Clone)]
pub struct SeasonEndReport {
    pub season: u16,
    pub promotions: Vec<PromotionEvent>,
    pub relegations: Vec<RelegationEvent>,
    pub divisions_affected: Vec<String>,
    pub processed_at: String,
}

/// Calculate which teams promote from a division
/// Returns list of (club_id, new_position) tuples
pub fn calculate_promotions_from_division(
    division_id: &str,
    standings: &[Standing],
) -> Vec<(String, i32)> {
    let promotion_slots = match division_id {
        "div-2" => 4,  // Top 4 from Div 2 → Div 1
        "div-3" => 3,  // Top 3 from Div 3 → Div 2
        "div-4" => 3,  // Top 3 from Div 4 → Div 3
        "div-5a" | "div-5b" => 2,  // Top 2 from each → Div 4
        "div-6a" | "div-6b" | "div-6c" | "div-6d" => 1,  // Top 1 from each → Div 5
        "div-7a" | "div-7b" | "div-7c" | "div-7d" => 1,  // Top 1 from each → Div 6 (same region)
        _ => 0,
    };

    standings
        .iter()
        .take(promotion_slots as usize)
        .enumerate()
        .map(|(idx, standing)| {
            (standing.club_id.clone(), (idx + 1) as i32)
        })
        .collect()
}

/// Calculate which teams relegate from a division
/// Returns list of (club_id, new_position) tuples
pub fn calculate_relegations_from_division(
    division_id: &str,
    standings: &[Standing],
) -> Vec<(String, i32)> {
    let total_clubs = standings.len() as i32;
    let relegation_slots = match division_id {
        "div-1" => 4,  // Bottom 4 from Div 1 → Div 2
        "div-2" => 3,  // Bottom 3 from Div 2 → Div 3
        "div-3" => 3,  // Bottom 3 from Div 3 → Div 4
        "div-4" => 4,  // Bottom 4 from Div 4 → Div 5 (split: 2→5A, 2→5B)
        "div-5a" | "div-5b" => 2,  // Bottom 2 from each → Div 6 (distributed)
        "div-6a" | "div-6b" | "div-6c" | "div-6d" => 1,  // Bottom 1 from each → Div 7 (same region)
        "div-7a" | "div-7b" | "div-7c" | "div-7d" => 0,  // No relegation from Div 7
        _ => 0,
    };

    let start_idx = (total_clubs - relegation_slots) as usize;
    standings
        .iter()
        .skip(start_idx)
        .enumerate()
        .map(|(idx, standing)| {
            (standing.club_id.clone(), (idx + 1) as i32)
        })
        .collect()
}

/// Get destination division for promoted clubs from a source division
pub fn get_promotion_destination(from_division: &str) -> String {
    match from_division {
        "div-2" => "div-1".to_string(),
        "div-3" => "div-2".to_string(),
        "div-4" => "div-3".to_string(),
        "div-5a" | "div-5b" => "div-4".to_string(),
        "div-6a" | "div-6b" | "div-6c" | "div-6d" => "div-5a".to_string(), // Will be split
        "div-7a" => "div-6a".to_string(),  // Same region
        "div-7b" => "div-6b".to_string(),
        "div-7c" => "div-6c".to_string(),
        "div-7d" => "div-6d".to_string(),
        _ => from_division.to_string(),
    }
}

/// Get destination divisions for relegated clubs from a source division
/// Returns a map of position -> destination division
pub fn get_relegation_destinations(from_division: &str, count: usize) -> Vec<String> {
    match from_division {
        "div-1" => (1..=count).map(|i| {
            // Bottom 4 from Div 1 go to Div 2
            "div-2".to_string()
        }).collect(),
        "div-2" => (1..=count).map(|_| {
            // Bottom 3 from Div 2 go to Div 3
            "div-3".to_string()
        }).collect(),
        "div-3" => (1..=count).map(|_| {
            // Bottom 3 from Div 3 go to Div 4
            "div-4".to_string()
        }).collect(),
        "div-4" => {
            // Bottom 4 from Div 4 split: 2→5A, 2→5B
            let mut dests = vec![];
            for i in 1..=count {
                if i <= 2 {
                    dests.push("div-5a".to_string());
                } else {
                    dests.push("div-5b".to_string());
                }
            }
            dests
        }
        "div-5a" | "div-5b" => {
            // Bottom 2 from Div 5 go to Div 6 (distributed: 6A/B or 6C/D)
            // Distribution logic handled elsewhere
            vec!["div-6-scattered".to_string(); count]
        }
        "div-6a" => vec!["div-7a".to_string(); count],  // Same region
        "div-6b" => vec!["div-7b".to_string(); count],
        "div-6c" => vec!["div-7c".to_string(); count],
        "div-6d" => vec!["div-7d".to_string(); count],
        _ => vec![],
    }
}

/// Distribute promoted teams from Div 6 to Div 5
/// Takes top 1 from each of 6A, 6B, 6C, 6D (4 teams total)
/// Distributes 2 to 5A, 2 to 5B based on pairing
pub fn distribute_div6_to_div5(
    promoted_teams: Vec<(String, String, i32)>, // (club_id, from_division, position)
) -> Vec<(String, String, i32)> {
    // Group by region
    let mut by_region: HashMap<String, Vec<(String, i32)>> = HashMap::new();

    for (club_id, div_id, pos) in promoted_teams {
        let region = div_id.split('-').last().unwrap_or("").to_string();
        by_region.entry(region).or_insert_with(Vec::new).push((club_id, pos));
    }

    // Distribute: A&B → 5A, C&D → 5B
    let mut result = vec![];
    let mut order = 1;

    // From 6A and 6B
    for region in &["a", "b"] {
        if let Some(teams) = by_region.get(*region) {
            for (club_id, _) in teams {
                result.push((club_id.clone(), "div-5a".to_string(), order));
                order += 1;
            }
        }
    }

    order = 1;
    // From 6C and 6D
    for region in &["c", "d"] {
        if let Some(teams) = by_region.get(*region) {
            for (club_id, _) in teams {
                result.push((club_id.clone(), "div-5b".to_string(), order));
                order += 1;
            }
        }
    }

    result
}

/// Distribute relegated teams from Div 5 to Div 6
/// Takes bottom 2 from 5A and 5B (4 teams total)
/// Scatters to 6A, 6B, 6C, 6D
pub fn distribute_div5_to_div6(
    relegated_teams: Vec<(String, String, i32)>, // (club_id, from_division, position)
) -> Vec<(String, String, i32)> {
    // Simple round-robin distribution to 6A, 6B, 6C, 6D
    let divs = ["div-6a", "div-6b", "div-6c", "div-6d"];
    let mut result = vec![];

    for (idx, (club_id, _from_div, _pos)) in relegated_teams.iter().enumerate() {
        let target_div = divs[idx % 4];
        result.push((club_id.clone(), target_div.to_string(), (idx / 4 + 1) as i32));
    }

    result
}

/// Process complete season end with promotions and relegations
/// Takes standings for all divisions and returns complete report
pub fn process_season_end(
    standings_by_division: HashMap<String, Vec<Standing>>,
    season: u16,
) -> SeasonEndReport {
    let mut promotions = vec![];
    let mut relegations = vec![];
    let mut divisions_affected = vec![];

    let division_order = vec![
        "div-1", "div-2", "div-3", "div-4",
        "div-5a", "div-5b",
        "div-6a", "div-6b", "div-6c", "div-6d",
        "div-7a", "div-7b", "div-7c", "div-7d",
    ];

    // Process promotions top-down
    for div_id in &division_order {
        if let Some(standings) = standings_by_division.get(*div_id) {
            let promoted = calculate_promotions_from_division(div_id, standings);

            if !promoted.is_empty() {
                divisions_affected.push(div_id.to_string());

                for (idx, (club_id, new_pos)) in promoted.iter().enumerate() {
                    if let Some(standing) = standings.get(idx) {
                        let dest_div = get_promotion_destination(div_id);
                        promotions.push(PromotionEvent {
                            club_id: club_id.clone(),
                            club_name: standing.club_name.clone(),
                            from_division_id: div_id.to_string(),
                            to_division_id: dest_div,
                            final_position: standing.position,
                        });
                    }
                }
            }
        }
    }

    // Process relegations bottom-up
    for div_id in division_order.iter().rev() {
        if let Some(standings) = standings_by_division.get(*div_id) {
            let relegated = calculate_relegations_from_division(div_id, standings);

            if !relegated.is_empty() {
                if !divisions_affected.contains(&div_id.to_string()) {
                    divisions_affected.push(div_id.to_string());
                }

                for (idx, (club_id, _new_pos)) in relegated.iter().enumerate() {
                    let total = standings.len();
                    if let Some(standing) = standings.get(total - relegated.len() + idx) {
                        let dest_divs = get_relegation_destinations(div_id, relegated.len());
                        if idx < dest_divs.len() {
                            relegations.push(RelegationEvent {
                                club_id: club_id.clone(),
                                club_name: standing.club_name.clone(),
                                from_division_id: div_id.to_string(),
                                to_division_id: dest_divs[idx].clone(),
                                final_position: standing.position,
                            });
                        }
                    }
                }
            }
        }
    }

    SeasonEndReport {
        season,
        promotions,
        relegations,
        divisions_affected,
        processed_at: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_promotion_slots() {
        assert_eq!(calculate_promotions_from_division("div-2", &[]).len(), 0);

        let standings = vec![
            Standing {
                club_id: "a".to_string(),
                club_name: "Club A".to_string(),
                position: 1,
                played: 10,
                won: 8,
                drawn: 1,
                lost: 1,
                goals_for: 20,
                goals_against: 5,
                points: 17,
            },
        ];

        assert_eq!(calculate_promotions_from_division("div-2", &standings).len(), 1);
    }

    #[test]
    fn test_destination_divisions() {
        assert_eq!(get_promotion_destination("div-2"), "div-1");
        assert_eq!(get_promotion_destination("div-5a"), "div-4");
        assert_eq!(get_promotion_destination("div-7a"), "div-6a");
    }
}
