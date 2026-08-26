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
    println!("FINDING UNIQUE MATCHES");
    println!("========================================");

    // Find unique matches only
    let mut unique_matches: Vec<(WomanRecord, HouseholdRecord)> = Vec::new();

    for ((woman_id, _birth_year), households_list) in &woman_matches {
        if households_list.len() == 1 {
            // Find the woman record
            let woman = women_by_name.values()
                .flatten()
                .find(|w| &w.id == woman_id)
                .unwrap();

            unique_matches.push((woman.clone(), households_list[0].clone()));
        }
    }

    println!("Found {} unique matches to apply\n", unique_matches.len());

    println!("========================================");
    println!("APPLYING MATCHES TO DATABASE");
    println!("========================================\n");

    let mut success_count = 0;
    let mut error_count = 0;

    for (i, (woman, household)) in unique_matches.iter().enumerate() {
        if (i + 1) % 500 == 0 {
            println!("Applied {} / {} matches...", i + 1, unique_matches.len());
        }

        // Insert the genealogy record into sheffield_people with household_head set
        let result = sqlx::query(
            "INSERT INTO sheffield_people (
                id, name, birth_year, profession, gender, source, household_head
            )
            SELECT
                id, name, birth_year, NULL, NULL, 'genealogy', ?
            FROM unmatched_genealogy
            WHERE id = ?"
        )
        .bind(&household.id)
        .bind(&woman.id)
        .execute(&pool)
        .await;

        match result {
            Ok(_) => {
                success_count += 1;

                // Delete from unmatched_genealogy
                sqlx::query("DELETE FROM unmatched_genealogy WHERE id = ?")
                    .bind(&woman.id)
                    .execute(&pool)
                    .await?;
            }
            Err(e) => {
                error_count += 1;
                println!("Error applying match for {}: {}", woman.name, e);
            }
        }
    }

    println!("\n========================================");
    println!("RESULTS:");
    println!("========================================");
    println!("Successfully applied: {}", success_count);
    println!("Errors: {}", error_count);
    println!("========================================");

    // Show updated counts
    let sheffield_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_people")
        .fetch_one(&pool)
        .await?;

    let unmatched_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unmatched_genealogy")
        .fetch_one(&pool)
        .await?;

    println!("\nUpdated database counts:");
    println!("sheffield_people: {}", sheffield_count.0);
    println!("unmatched_genealogy: {}", unmatched_count.0);
    println!("========================================");

    Ok(())
}
