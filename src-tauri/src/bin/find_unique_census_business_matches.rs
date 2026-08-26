use sqlx::sqlite::SqlitePool;

fn names_match(census_name: &str, business_name: &str) -> bool {
    let census_lower = census_name.to_lowercase();
    let business_lower = business_name.to_lowercase();

    // Extract surname from census (usually last word)
    let census_parts: Vec<&str> = census_lower.split_whitespace().collect();
    let census_surname = census_parts.last().unwrap_or(&"");

    // Check if surnames match (must be at least 4 chars to avoid false positives like "A. Harrop" matching "Benjamin Harrop")
    if census_surname.len() < 4 {
        return false;
    }

    // Check if business name contains census name or vice versa
    if census_lower.contains(&business_lower) || business_lower.contains(&census_lower) {
        return true;
    }

    // More strict surname matching
    if business_lower.contains(census_surname) {
        return true;
    }

    false
}

fn extract_street_words(text: &str) -> Vec<String> {
    text.split_whitespace()
        .filter(|w| w.len() > 4) // Only words longer than 4 chars (avoid "road", "lane", etc)
        .map(|w| w.to_lowercase())
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("UNIQUE CENSUS TO BUSINESS MATCHES");
    println!("Name + Specific Street (1-to-1 matches only)");
    println!("========================================\n");

    // Get all census records
    let census_records: Vec<(String, String, i64, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT id, name, age, civil_parish, registration_district
         FROM unmatched_sheffieldcensus"
    )
    .fetch_all(&pool)
    .await?;

    println!("Total census records: {}\n", census_records.len());

    // Get all businesses
    let businesses: Vec<(String, String, String)> = sqlx::query_as(
        "SELECT id, full_name, address
         FROM sheffield_businesses
         WHERE full_name IS NOT NULL AND address IS NOT NULL"
    )
    .fetch_all(&pool)
    .await?;

    println!("Total businesses: {}\n", businesses.len());
    println!("Looking for UNIQUE NAME + STREET matches (1-to-1 only)...\n");

    let mut unique_matches = Vec::new();

    for (census_id, census_name, census_age, parish_opt, district_opt) in &census_records {
        // Get location words from census
        let mut location_words = Vec::new();
        if let Some(parish) = parish_opt {
            location_words.extend(extract_street_words(parish));
        }
        if let Some(district) = district_opt {
            location_words.extend(extract_street_words(district));
        }

        if location_words.is_empty() {
            continue;
        }

        // Find ALL matching businesses for this census person
        let mut all_matches = Vec::new();

        for (biz_id, biz_name, biz_address) in &businesses {
            // Check if names match
            if !names_match(census_name, biz_name) {
                continue;
            }

            // Check if any location words appear in business address
            let biz_address_lower = biz_address.to_lowercase();
            let address_words = extract_street_words(&biz_address_lower);

            let mut matching_street_words = Vec::new();
            for loc_word in &location_words {
                if address_words.contains(loc_word) {
                    matching_street_words.push(loc_word.clone());
                }
            }

            if !matching_street_words.is_empty() {
                all_matches.push((
                    biz_id.clone(),
                    biz_name.clone(),
                    biz_address.clone(),
                    matching_street_words.clone(),
                ));
            }
        }

        // Only keep if there's exactly ONE match
        if all_matches.len() == 1 {
            let (biz_id, biz_name, biz_address, street_words) = &all_matches[0];
            unique_matches.push((
                census_id.clone(),
                census_name.clone(),
                *census_age,
                parish_opt.clone(),
                district_opt.clone(),
                biz_id.clone(),
                biz_name.clone(),
                biz_address.clone(),
                street_words.clone(),
            ));
        }
    }

    println!("========================================");
    println!("UNIQUE MATCHES FOUND: {}", unique_matches.len());
    println!("========================================\n");

    // Show all unique matches
    for (idx, (census_id, census_name, census_age, parish, district, biz_id, biz_name, biz_address, street_words)) in unique_matches.iter().enumerate() {
        println!("UNIQUE MATCH #{}", idx + 1);
        println!("  Census Person: {} (age: {})", census_name, census_age);
        if let Some(p) = parish {
            println!("    Parish: {}", p);
        }
        if let Some(d) = district {
            println!("    District: {}", d);
        }
        println!("  Business: {}", biz_name);
        println!("    Address: {}", biz_address);
        println!("  Matching street words: {}", street_words.join(", "));
        println!("  Census ID: {}", census_id);
        println!("  Business ID: {}", biz_id);
        println!();
    }

    println!("\n========================================");
    println!("SUMMARY");
    println!("========================================");
    println!("Total census records: {}", census_records.len());
    println!("Total businesses: {}", businesses.len());
    println!("Unique 1-to-1 matches (name + street): {}", unique_matches.len());
    println!("Match rate: {:.2}%", (unique_matches.len() as f64 / census_records.len() as f64) * 100.0);

    Ok(())
}
