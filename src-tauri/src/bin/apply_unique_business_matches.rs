use sqlx::sqlite::SqlitePool;
use sqlx::Row;

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
    println!("APPLYING UNIQUE BUSINESS MATCHES");
    println!("Moving matched census records to sheffield_people");
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
    println!("Finding UNIQUE NAME + STREET matches (1-to-1 only)...\n");

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

    println!("Found {} unique 1-to-1 matches\n", unique_matches.len());

    // First, add business columns to sheffield_people if they don't exist
    println!("Adding business columns to sheffield_people table...\n");

    sqlx::query("ALTER TABLE sheffield_people ADD COLUMN business_id TEXT")
        .execute(&pool)
        .await
        .ok(); // Ignore error if column already exists

    sqlx::query("ALTER TABLE sheffield_people ADD COLUMN business_name TEXT")
        .execute(&pool)
        .await
        .ok();

    sqlx::query("ALTER TABLE sheffield_people ADD COLUMN business_address TEXT")
        .execute(&pool)
        .await
        .ok();

    println!("Business columns added (or already exist).\n");

    // Now move these records to sheffield_people
    let mut moved_count = 0;
    let mut failed_count = 0;

    for (census_id, _census_name, _census_age, _parish, _district, biz_id, biz_name, biz_address, street_words) in &unique_matches {
        // Map unmatched_sheffieldcensus columns to sheffield_people columns
        // Note: sheffield_people uses census_ prefixes for census data
        let result = sqlx::query(
            "INSERT INTO sheffield_people (
                id, name, source, birth_year, gender, census_age, census_birth_place, census_relation,
                census_household_members, civil_parish, ecclesiastical_parish, registration_district,
                census_county, census_piece, census_folio, census_page, created_at,
                business_id, business_name, business_address
             )
             SELECT
                id, name, 'census' as source, birth_year, gender, age as census_age,
                birth_place as census_birth_place, relation as census_relation,
                household_members as census_household_members, civil_parish, ecclesiastical_parish,
                registration_district, county as census_county, piece as census_piece,
                folio as census_folio, page as census_page, created_at,
                ? as business_id, ? as business_name, ? as business_address
             FROM unmatched_sheffieldcensus WHERE id = ?"
        )
        .bind(biz_id)
        .bind(biz_name)
        .bind(biz_address)
        .bind(census_id)
        .execute(&pool)
        .await;

        match result {
            Ok(_) => {
                // Delete from unmatched_sheffieldcensus
                sqlx::query("DELETE FROM unmatched_sheffieldcensus WHERE id = ?")
                    .bind(census_id)
                    .execute(&pool)
                    .await?;

                moved_count += 1;

                if moved_count <= 10 {
                    println!("✓ Moved: {} → Business: {}", census_id, biz_name);
                    println!("  Location match: {}", street_words.join(", "));
                    println!("  Business address: {}", biz_address);
                    println!();
                }
            }
            Err(e) => {
                failed_count += 1;
                eprintln!("✗ Failed to move {}: {}", census_id, e);
            }
        }
    }

    println!("\n========================================");
    println!("OPERATION COMPLETE");
    println!("========================================");
    println!("Total unique matches found: {}", unique_matches.len());
    println!("Successfully moved to sheffield_people: {}", moved_count);
    println!("Failed to move: {}", failed_count);
    println!("Deleted from unmatched_sheffieldcensus: {}", moved_count);

    // Check final counts
    let remaining_unmatched: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_sheffieldcensus"
    )
    .fetch_one(&pool)
    .await?;

    let total_sheffield_people: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_people"
    )
    .fetch_one(&pool)
    .await?;

    println!("\n========================================");
    println!("FINAL COUNTS");
    println!("========================================");
    println!("Remaining in unmatched_sheffieldcensus: {}", remaining_unmatched.0);
    println!("Total in sheffield_people: {}", total_sheffield_people.0);

    Ok(())
}
