use sqlx::sqlite::SqlitePool;

fn names_match(person_name: &str, business_name: &str) -> bool {
    let person_lower = person_name.to_lowercase();
    let business_lower = business_name.to_lowercase();
    
    // Extract surname from person name (usually last word)
    let person_parts: Vec<&str> = person_lower.split_whitespace().collect();
    let person_surname = person_parts.last().unwrap_or(&"");
    
    // Surname must be at least 4 chars
    if person_surname.len() < 4 {
        return false;
    }
    
    // Check if business name contains person name or vice versa
    if person_lower.contains(&business_lower) || business_lower.contains(&person_lower) {
        return true;
    }
    
    // Check if business name contains the surname
    if business_lower.contains(person_surname) {
        return true;
    }
    
    false
}

fn extract_address_words(text: &str) -> Vec<String> {
    text.split_whitespace()
        .filter(|w| w.len() > 4) // Only words longer than 4 chars
        .map(|w| w.to_lowercase())
        .collect()
}

fn professions_match(person_profession: Option<&str>, business_occupation: &str) -> bool {
    if let Some(prof) = person_profession {
        let prof_lower = prof.to_lowercase();
        let occ_lower = business_occupation.to_lowercase();
        
        // Check if they contain each other
        if prof_lower.contains(&occ_lower) || occ_lower.contains(&prof_lower) {
            return true;
        }
        
        // Check for common profession word overlaps
        let prof_words: Vec<&str> = prof_lower.split_whitespace().collect();
        let occ_words: Vec<&str> = occ_lower.split_whitespace().collect();
        
        for pw in &prof_words {
            if pw.len() > 4 {
                for ow in &occ_words {
                    if pw == ow {
                        return true;
                    }
                }
            }
        }
    }
    false
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("MATCHING SHEFFIELD_PEOPLE TO BUSINESSES");
    println!("By NAME + ADDRESS + PROFESSION");
    println!("========================================\n");

    // Get all people from sheffield_people
    let people: Vec<(String, String, Option<String>, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT id, name, profession, civil_parish, registration_district
         FROM sheffield_people
         WHERE profession IS NOT NULL"
    )
    .fetch_all(&pool)
    .await?;

    println!("Total people with professions: {}\n", people.len());

    // Get all businesses
    let businesses: Vec<(String, String, String, String)> = sqlx::query_as(
        "SELECT id, full_name, address, occupation
         FROM sheffield_businesses
         WHERE full_name IS NOT NULL AND address IS NOT NULL AND occupation IS NOT NULL"
    )
    .fetch_all(&pool)
    .await?;

    println!("Total businesses: {}\n", businesses.len());
    println!("Finding matches (name + address + profession)...\n");

    let mut matches_found = 0;
    let mut unique_matches = Vec::new();

    for (person_id, person_name, profession_opt, parish_opt, district_opt) in &people {
        // Get location words from person
        let mut location_words = Vec::new();
        if let Some(parish) = parish_opt {
            location_words.extend(extract_address_words(parish));
        }
        if let Some(district) = district_opt {
            location_words.extend(extract_address_words(district));
        }

        if location_words.is_empty() {
            continue;
        }

        // Find ALL matching businesses for this person
        let mut all_matches = Vec::new();

        for (biz_id, biz_name, biz_address, biz_occupation) in &businesses {
            // Check if names match
            if !names_match(person_name, biz_name) {
                continue;
            }

            // Check if professions match
            if !professions_match(profession_opt.as_deref(), biz_occupation) {
                continue;
            }

            // Check if any location words appear in business address
            let biz_address_lower = biz_address.to_lowercase();
            let address_words = extract_address_words(&biz_address_lower);

            let mut matching_address_words = Vec::new();
            for loc_word in &location_words {
                if address_words.contains(loc_word) {
                    matching_address_words.push(loc_word.clone());
                }
            }

            if !matching_address_words.is_empty() {
                all_matches.push((
                    biz_id.clone(),
                    biz_name.clone(),
                    biz_address.clone(),
                    biz_occupation.clone(),
                    matching_address_words.clone(),
                ));
            }
        }

        // Only keep if there's exactly ONE match (unique 1-to-1)
        if all_matches.len() == 1 {
            let (biz_id, biz_name, biz_address, biz_occupation, address_words) = &all_matches[0];
            unique_matches.push((
                person_id.clone(),
                person_name.clone(),
                profession_opt.clone(),
                parish_opt.clone(),
                district_opt.clone(),
                biz_id.clone(),
                biz_name.clone(),
                biz_address.clone(),
                biz_occupation.clone(),
                address_words.clone(),
            ));
            matches_found += 1;
        }
    }

    println!("========================================");
    println!("UNIQUE 1-TO-1 MATCHES FOUND: {}", matches_found);
    println!("========================================\n");

    // Show first 20 matches
    for (idx, (person_id, person_name, profession, parish, district, biz_id, biz_name, biz_address, biz_occupation, address_words)) in unique_matches.iter().take(20).enumerate() {
        println!("MATCH #{}", idx + 1);
        println!("  Person: {} ({})", person_name, profession.as_deref().unwrap_or("N/A"));
        if let Some(p) = parish {
            println!("    Parish: {}", p);
        }
        if let Some(d) = district {
            println!("    District: {}", d);
        }
        println!("  Business: {} ({})", biz_name, biz_occupation);
        println!("    Address: {}", biz_address);
        println!("  Matching address words: {}", address_words.join(", "));
        println!("  Person ID: {}", person_id);
        println!("  Business ID: {}", biz_id);
        println!();
    }

    if unique_matches.len() > 20 {
        println!("... and {} more matches\n", unique_matches.len() - 20);
    }

    println!("\n========================================");
    println!("SUMMARY");
    println!("========================================");
    println!("Total people with professions: {}", people.len());
    println!("Total businesses: {}", businesses.len());
    println!("Unique 1-to-1 matches (name + address + profession): {}", matches_found);
    println!("Match rate: {:.2}%", (matches_found as f64 / people.len() as f64) * 100.0);

    Ok(())
}
