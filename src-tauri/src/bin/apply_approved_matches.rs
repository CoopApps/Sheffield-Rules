use sqlx::sqlite::SqlitePool;
use std::collections::HashMap;
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
    match_num: usize,
    gen_id: i64,
    gen_name: String,
    gen_address: String,
    business_occupation: String,
    business_address: String,
    business_year: i64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite:D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(database_url).await?;

    println!("========================================");
    println!("APPLYING APPROVED MATCHES");
    println!("========================================\n");

    // Approved match numbers (skipping: 9, 14, 17, 24, 30, 34)
    let approved_matches: Vec<usize> = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 10,  // Batch 1
        11, 12, 13, 15, 16, 18, 19, 20,  // Batch 2
        21, 22, 23, 25, 26, 27, 28, 29,  // Batch 3
        31, 32, 33, 35, 36, 37, 38, 39, 40,  // Batch 4
        41, 42, 43, 44, 45, 46, 47, 48, 49, 50,  // Batch 5
        51, 52, 53, 54, 55, 56, 57, 58, 59, 60,  // Batch 6
        61, 62, 63, 64, 65, 66, 67, 68, 69, 70,  // Batch 7
        71, 72, 73, 74, 75, 76, 77, 78, 79, 80,  // Batch 8
        81, 82, 83, 84, 85, 86, 87, 88, 89, 90,  // Batch 9
        91, 92, 93, 94,  // Batch 10
    ];

    println!("Total approved matches to apply: {}\n", approved_matches.len());

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

    println!("Created business lookup map\n");

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
                    match_num: unique_matches.len() + 1,
                    gen_id: *gen_id,
                    gen_name: gen_name.clone(),
                    gen_address: gen_address.clone(),
                    business_occupation: business_occupation.clone(),
                    business_address: business_address.clone(),
                    business_year: *business_year,
                });
            }
        }
    }

    println!("Found {} unique matches total\n", unique_matches.len());

    // Apply only approved matches
    let mut applied_count = 0;

    for m in unique_matches.iter() {
        if approved_matches.contains(&m.match_num) {
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

            applied_count += 1;

            if applied_count % 10 == 0 {
                println!("Applied {} matches...", applied_count);
            }
        }
    }

    println!("\n========================================");
    println!("APPLICATION COMPLETE");
    println!("========================================");
    println!("Total unique matches found: {}", unique_matches.len());
    println!("Approved matches: {}", approved_matches.len());
    println!("Successfully applied: {}", applied_count);

    Ok(())
}
