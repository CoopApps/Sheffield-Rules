use sqlx::sqlite::SqlitePool;
use std::collections::HashMap;
use regex::Regex;

fn normalize_address(addr: &str) -> String {
    let mut result = addr.to_string();

    // Replace comma followed by capital letter with space and capital letter
    // "70,SnigHill" -> "70 SnigHill"
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite:D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(database_url).await?;

    println!("========================================");
    println!("MATCHING NAME AND ADDRESS EXACTLY");
    println!("Smart normalization for both");
    println!("========================================\n");

    // Get all businesses with addresses
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
    let mut business_map: HashMap<(String, String), Vec<(String, String, String, String, String, i64)>> = HashMap::new();

    for (id, surname, forename, occupation, address, year) in businesses {
        let full_name = format!("{} {}", forename, surname);
        let normalized_name = normalize_name(&full_name);
        let normalized_address = normalize_address(&address);

        let key = (normalized_name, normalized_address);
        business_map.entry(key).or_insert_with(Vec::new)
            .push((full_name.clone(), occupation, address.clone(), id, surname, year));
    }

    println!("Created business lookup map with {} unique (name, address) combinations\n", business_map.len());

    // Get all genealogy records
    println!("Loading genealogy records...");
    let genealogy_records: Vec<(i64, String, String)> = sqlx::query_as(
        "SELECT id, name, address
         FROM unmatched_genealogy
         WHERE address IS NOT NULL AND address != ''
         AND name IS NOT NULL AND name != ''"
    )
    .fetch_all(&pool)
    .await?;

    println!("Loaded {} genealogy records\n", genealogy_records.len());

    let mut matched = 0;
    let mut unique_matches = 0;
    let mut multiple_matches = 0;
    let mut total_business_matches = 0;
    let mut examples = Vec::new();

    println!("Matching name and address pairs...\n");

    for (i, (gen_id, gen_name, gen_address)) in genealogy_records.iter().enumerate() {
        if (i + 1) % 10000 == 0 {
            println!("Processed {}/{} genealogy records...", i + 1, genealogy_records.len());
        }

        let normalized_name = normalize_name(gen_name);
        let normalized_address = normalize_address(gen_address);

        let key = (normalized_name, normalized_address);

        if let Some(matches) = business_map.get(&key) {
            matched += 1;
            total_business_matches += matches.len();

            if matches.len() == 1 {
                unique_matches += 1;

                // Extract surname and forename from genealogy name (last word is surname)
                let name_parts: Vec<&str> = gen_name.split_whitespace().collect();
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

                let (_full_name, occupation, business_address, _id, business_surname, year) = &matches[0];

                // Update the genealogy record with business info
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
                .bind(occupation)
                .bind(business_address)
                .bind(year.to_string())
                .bind("White's 1871")
                .bind(gen_id)
                .execute(&pool)
                .await?;

                if examples.len() < 20 {
                    examples.push((
                        gen_name.clone(),
                        gen_address.clone(),
                        occupation.clone(),
                        business_address.clone()
                    ));
                }
            } else {
                multiple_matches += 1;

                if examples.len() < 20 {
                    examples.push((
                        format!("{} [MULTIPLE: {}]", gen_name, matches.len()),
                        gen_address.clone(),
                        format!("{} businesses", matches.len()),
                        "Not updated - multiple matches".to_string()
                    ));
                }
            }
        }
    }

    println!("\n========================================");
    println!("MATCHING COMPLETE");
    println!("========================================");
    println!("Total genealogy records checked: {}", genealogy_records.len());
    println!("Genealogy records with name+address match: {}", matched);
    println!("  - Unique matches (updated): {}", unique_matches);
    println!("  - Multiple matches (not updated): {}", multiple_matches);
    println!("Total business records matched: {}", total_business_matches);
    println!("Match rate: {:.2}%", (matched as f64 / genealogy_records.len() as f64) * 100.0);
    println!("Update rate: {:.2}%", (unique_matches as f64 / genealogy_records.len() as f64) * 100.0);
    if matched > 0 {
        println!("Avg businesses per matched pair: {:.2}", total_business_matches as f64 / matched as f64);
    }

    if !examples.is_empty() {
        println!("\n20 Example matches:");
        println!("{:=<120}", "");
        for (i, (name, gen_addr, occupation, business_addr)) in examples.iter().enumerate() {
            println!("{}. {}", i + 1, name);
            println!("   Genealogy address: {}", gen_addr);
            println!("   Business address:  {}", business_addr);
            println!("   Occupation: {}", occupation);
            println!("{:-<120}", "");
        }
    }

    Ok(())
}
