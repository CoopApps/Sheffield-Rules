use sqlx::sqlite::SqlitePool;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
struct WomanRecord {
    id: String,
    name: String,
    birth_year: Option<i64>,
    address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
struct HouseholdRecord {
    id: String,
    name: String,
    census_household_members: Option<String>,
    birth_year: Option<i64>,
}

fn normalize_name(name: &str) -> String {
    name.to_lowercase()
        .replace(".", "")
        .replace(",", "")
        .trim()
        .to_string()
}

fn get_first_name(full_name: &str) -> String {
    // Extract the first word as the first name
    full_name.split_whitespace()
        .next()
        .unwrap_or(full_name)
        .to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("Loading women from unmatched_genealogy (without spouse)...");

    // Women are those without spouse data
    let women: Vec<WomanRecord> = sqlx::query_as(
        "SELECT id, name, birth_year, address FROM unmatched_genealogy WHERE spouse IS NULL OR spouse = ''"
    )
    .fetch_all(&pool)
    .await?;

    println!("Found {} potential women in unmatched_genealogy\n", women.len());

    // Index women by full normalized name for efficient lookup
    println!("Indexing women by name...");
    let mut women_by_name: HashMap<String, Vec<WomanRecord>> = HashMap::new();

    for woman in women {
        let normalized_name = normalize_name(&woman.name);
        women_by_name
            .entry(normalized_name)
            .or_insert_with(Vec::new)
            .push(woman);
    }

    println!("Indexed {} unique names\n", women_by_name.len());

    println!("Loading sheffield_people with household members...");

    let households: Vec<HouseholdRecord> = sqlx::query_as(
        "SELECT id, name, census_household_members, birth_year FROM sheffield_people WHERE census_household_members IS NOT NULL AND census_household_members != ''"
    )
    .fetch_all(&pool)
    .await?;

    println!("Found {} sheffield_people records with household members\n", households.len());

    println!("Searching for women in household members...\n");

    // Track matches: key is (woman_id, birth_year), value is list of matching households
    let mut woman_matches: HashMap<(String, Option<i64>), Vec<HouseholdRecord>> = HashMap::new();

    let mut processed = 0;
    for household in &households {
        processed += 1;
        if processed % 10000 == 0 {
            println!("Processed {} / {} households...", processed, households.len());
        }

        if let Some(members) = &household.census_household_members {
            // Parse household members - format is: "Name\tAge | Name\tAge | ..."
            // Split by '|' to get individual member entries
            let member_entries: Vec<&str> = members
                .split('|')
                .map(|entry| entry.trim())
                .filter(|entry| !entry.is_empty())
                .collect();

            for entry in &member_entries {
                // Split by tab to get name and age
                let parts: Vec<&str> = entry.split('\t').collect();
                if parts.len() >= 2 {
                    let member_name = parts[0].trim();
                    let member_age_str = parts[1].trim();

                    // Parse the age
                    let member_age: Option<i64> = if member_age_str.contains('/') {
                        // Handle fractional ages like "3/12" (3 months old)
                        Some(0)
                    } else {
                        member_age_str.parse().ok()
                    };

                    // Normalize the full member name
                    let normalized_member_name = normalize_name(member_name);

                    // Look up this full name in our women index
                    if let Some(women_with_this_name) = women_by_name.get(&normalized_member_name) {
                        // Found women with this exact name
                        for woman in women_with_this_name {
                            // Calculate expected birth year from member's age
                            if let Some(age) = member_age {
                                let expected_birth = 1871 - age; // Census year is 1871

                                // Check if birth years match (within 2 years tolerance)
                                if let Some(woman_birth) = woman.birth_year {
                                    if (woman_birth - expected_birth).abs() <= 2 {
                                        // Birth year matches! This is a match
                                        woman_matches
                                            .entry((woman.id.clone(), woman.birth_year))
                                            .or_insert_with(Vec::new)
                                            .push(household.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("\n========================================");
    println!("HOUSEHOLD MATCHING RESULTS:");
    println!("========================================");
    println!("Total women found in households: {}", woman_matches.len());

    // Count unique vs multiple matches
    println!("Analyzing match uniqueness...");
    let mut unique_matches = Vec::new();
    let mut multiple_matches = Vec::new();

    let mut count = 0;
    for ((woman_id, _birth_year), households_list) in &woman_matches {
        count += 1;
        if count % 1000 == 0 {
            println!("Analyzed {} / {} matches...", count, woman_matches.len());
        }

        // Find the woman record
        let woman = women_by_name.values()
            .flatten()
            .find(|w| &w.id == woman_id)
            .unwrap();

        if households_list.len() == 1 {
            unique_matches.push((woman.clone(), households_list[0].clone()));
        } else {
            multiple_matches.push((woman.clone(), households_list.len()));
        }
    }
    println!("UNIQUE matches (woman in exactly 1 household): {}", unique_matches.len());
    println!("Multiple matches (woman in 2+ households): {}", multiple_matches.len());
    println!("========================================\n");

    println!("SAMPLE UNIQUE MATCHES (first 20):");
    println!("----------------------------------------");
    for (i, (woman, household)) in unique_matches.iter().take(20).enumerate() {
        println!("\n{}. Woman: {} (born {})",
            i + 1,
            woman.name,
            woman.birth_year.map(|y| y.to_string()).unwrap_or("?".to_string())
        );
        println!("   Household head: {} (born {})",
            household.name,
            household.birth_year.map(|y| y.to_string()).unwrap_or("?".to_string())
        );
        println!("   Members: {}",
            household.census_household_members.as_deref().unwrap_or("none")
        );
    }

    println!("\n\nSAMPLE MULTIPLE MATCHES (first 10):");
    println!("----------------------------------------");
    multiple_matches.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by count descending
    for (i, (woman, count)) in multiple_matches.iter().take(10).enumerate() {
        println!("{}. {} (born {}) - found in {} households",
            i + 1,
            woman.name,
            woman.birth_year.map(|y| y.to_string()).unwrap_or("?".to_string()),
            count
        );
    }

    println!("\n========================================");
    println!("RECOMMENDATION:");
    println!("========================================");
    println!("Unique matches that can be auto-applied: {}", unique_matches.len());
    println!("These women appear in exactly one household's member list");
    println!("and can be safely matched to that household.");
    println!("========================================");

    Ok(())
}
