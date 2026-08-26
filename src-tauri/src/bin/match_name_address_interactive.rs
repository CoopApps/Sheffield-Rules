use sqlx::sqlite::SqlitePool;
use std::collections::HashMap;
use std::io::{self, Write};
use regex::Regex;

fn normalize_address(addr: &str) -> String {
    let mut result = addr.to_string();

    // Replace comma followed by capital letter with space and capital letter
    let re = Regex::new(r",([A-Z])").unwrap();
    result = re.replace_all(&result, " $1").to_string();

    // Remove other commas
    result = result.replace(',', " ");

    // Normalize multiple spaces to single space
    let re_spaces = Regex::new(r"\s+").unwrap();
    result = re_spaces.replace_all(&result, " ").to_string();

    // Trim and convert to lowercase for comparison
    result.trim().to_lowercase()
}

fn normalize_name(name: &str) -> String {
    let mut result = name.to_string();

    // Common abbreviation expansions
    result = result.replace("Wm.", "William");
    result = result.replace("Wm ", "William ");
    result = result.replace("Thos.", "Thomas");
    result = result.replace("Thos ", "Thomas ");
    result = result.replace("Jas.", "James");
    result = result.replace("Jas ", "James ");
    result = result.replace("Robt.", "Robert");
    result = result.replace("Robt ", "Robert ");
    result = result.replace("Geo.", "George");
    result = result.replace("Geo ", "George ");
    result = result.replace("Chas.", "Charles");
    result = result.replace("Chas ", "Charles ");
    result = result.replace("Jos.", "Joseph");
    result = result.replace("Jos ", "Joseph ");
    result = result.replace("Edw.", "Edward");
    result = result.replace("Edw ", "Edward ");
    result = result.replace("Benj.", "Benjamin");
    result = result.replace("Benj ", "Benjamin ");
    result = result.replace("Saml.", "Samuel");
    result = result.replace("Saml ", "Samuel ");

    // Normalize multiple spaces to single space
    let re_spaces = Regex::new(r"\s+").unwrap();
    result = re_spaces.replace_all(&result, " ").to_string();

    // Trim and convert to lowercase for comparison
    result.trim().to_lowercase()
}

#[derive(Debug)]
struct Match {
    gen_id: i64,
    gen_name: String,
    gen_address: String,
    gen_profession: String,
    business_name: String,
    business_occupation: String,
    business_address: String,
    business_id: String,
    business_year: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite:D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(database_url).await?;

    println!("========================================");
    println!("INTERACTIVE NAME AND ADDRESS MATCHING");
    println!("Match individually with approval");
    println!("========================================\n");

    // Get all businesses
    println!("Loading businesses from sheffield_businesses...");
    let businesses: Vec<(String, String, String, String, String, i64)> = sqlx::query_as(
        "SELECT id, surname, forename, occupation, address, year
         FROM sheffield_businesses
         WHERE address IS NOT NULL AND address != ''
         AND surname IS NOT NULL AND forename IS NOT NULL"
    )
    .fetch_all(&pool)
    .await?;

    println!("Loaded {} businesses\n", businesses.len());

    // Create a map of businesses by (normalized_name, normalized_address)
    let mut business_map: HashMap<(String, String), Vec<(String, String, String, String, i64)>> = HashMap::new();

    for (id, surname, forename, occupation, address, year) in businesses {
        let full_name = format!("{} {}", forename, surname);
        let normalized_name = normalize_name(&full_name);
        let normalized_address = normalize_address(&address);

        let key = (normalized_name, normalized_address);
        business_map.entry(key).or_insert_with(Vec::new)
            .push((full_name.clone(), occupation, address.clone(), id, year));
    }

    println!("Created business lookup map with {} unique (name, address) combinations\n", business_map.len());

    // Get all genealogy records
    println!("Loading genealogy records...");
    let genealogy_records: Vec<(i64, String, String, String)> = sqlx::query_as(
        "SELECT id, name, address, profession
         FROM unmatched_genealogy
         WHERE address IS NOT NULL AND address != ''
         AND name IS NOT NULL AND name != ''"
    )
    .fetch_all(&pool)
    .await?;

    println!("Loaded {} genealogy records\n", genealogy_records.len());

    // Find all unique matches
    let mut unique_matches = Vec::new();

    for (gen_id, gen_name, gen_address, gen_profession) in genealogy_records.iter() {
        let normalized_name = normalize_name(gen_name);
        let normalized_address = normalize_address(gen_address);

        let key = (normalized_name, normalized_address);

        if let Some(matches) = business_map.get(&key) {
            if matches.len() == 1 {
                let (business_name, business_occupation, business_address, business_id, business_year) = &matches[0];

                unique_matches.push(Match {
                    gen_id: *gen_id,
                    gen_name: gen_name.clone(),
                    gen_address: gen_address.clone(),
                    gen_profession: gen_profession.clone(),
                    business_name: business_name.clone(),
                    business_occupation: business_occupation.clone(),
                    business_address: business_address.clone(),
                    business_id: business_id.clone(),
                    business_year: *business_year,
                });
            }
        }
    }

    println!("Found {} unique matches\n", unique_matches.len());
    println!("========================================\n");

    // Process matches individually
    let mut approved_count = 0;
    let mut rejected_count = 0;
    let mut batch_approved = 0;
    let mut current_batch = 1;

    for (i, m) in unique_matches.iter().enumerate() {
        let match_num = i + 1;
        let batch_position = (match_num - 1) % 20 + 1;

        if batch_position == 1 {
            println!("\n========================================");
            println!("BATCH {} - Matches {}-{}", current_batch, match_num, (match_num + 19).min(unique_matches.len()));
            println!("========================================\n");
            current_batch += 1;
            batch_approved = 0;
        }

        println!("Match {}/{} (Batch position {})", match_num, unique_matches.len(), batch_position);
        println!("  GENEALOGY: {} at {}", m.gen_name, m.gen_address);
        println!("  Profession: {}", m.gen_profession);
        println!("  BUSINESS:   {} at {}", m.business_name, m.business_address);
        println!("  Occupation: {} ({})", m.business_occupation, m.business_year);
        print!("\n  Approve? (y/n/q to quit): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice = input.trim().to_lowercase();

        if choice == "q" {
            println!("\nQuitting...");
            break;
        }

        if choice == "y" {
            // Extract surname and forename from genealogy name
            let name_parts: Vec<&str> = m.gen_name.split_whitespace().collect();
            let surname = if !name_parts.is_empty() {
                name_parts[name_parts.len() - 1]
            } else {
                ""
            };
            let forename = if name_parts.len() > 1 {
                name_parts[..name_parts.len() - 1].join(" ")
            } else {
                String::new()
            };

            sqlx::query(
                "UPDATE unmatched_genealogy
                 SET business_surname = ?,
                     business_forename = ?,
                     business_occupation = ?,
                     business_address = ?,
                     business_year = ?,
                     business_source = ?
                 WHERE id = ?"
            )
            .bind(surname)
            .bind(&forename)
            .bind(&m.business_occupation)
            .bind(&m.business_address)
            .bind(m.business_year.to_string())
            .bind("White's 1871")
            .bind(m.gen_id)
            .execute(&pool)
            .await?;

            approved_count += 1;
            batch_approved += 1;
            println!("  ✓ Approved (Batch: {}/20, Total: {})", batch_approved, approved_count);
        } else {
            rejected_count += 1;
            println!("  ✗ Rejected (Total rejected: {})", rejected_count);
        }

        // Show batch summary every 20 matches
        if batch_position == 20 || match_num == unique_matches.len() {
            println!("\n  --- Batch Summary: {}/20 approved ---", batch_approved);
        }
    }

    println!("\n========================================");
    println!("MATCHING COMPLETE");
    println!("========================================");
    println!("Total unique matches found: {}", unique_matches.len());
    println!("Approved and applied: {}", approved_count);
    println!("Rejected: {}", rejected_count);
    println!("Match rate: {:.1}%", if unique_matches.len() > 0 {
        (approved_count as f64 / unique_matches.len() as f64) * 100.0
    } else {
        0.0
    });

    Ok(())
}
