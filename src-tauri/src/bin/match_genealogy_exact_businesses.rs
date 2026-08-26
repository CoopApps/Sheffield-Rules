use sqlx::sqlite::SqlitePool;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite:D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(database_url).await?;

    println!("========================================");
    println!("MATCHING GENEALOGY TO BUSINESSES");
    println!("Exact name and address matching");
    println!("========================================\n");

    // Get all businesses with their full names from sheffield_businesses
    println!("Loading businesses from sheffield_businesses...");
    let businesses: Vec<(String, String, String, String, String, i64, String)> = sqlx::query_as(
        "SELECT id, surname, forename, title, occupation, year, address 
         FROM sheffield_businesses 
         WHERE surname IS NOT NULL AND forename IS NOT NULL AND address IS NOT NULL"
    )
    .fetch_all(&pool)
    .await?;

    println!("Loaded {} businesses\n", businesses.len());

    // Create a map of businesses by (full_name, address)
    // Construct full name as "forename surname" to match genealogy format
    let mut business_map: HashMap<(String, String), Vec<(String, String, String, i64, String)>> = HashMap::new();
    
    for (id, surname, forename, title, occupation, year, address) in businesses {
        let full_name = format!("{} {}", forename, surname);
        let key = (
            full_name.to_lowercase().trim().to_string(),
            address.to_lowercase().trim().to_string()
        );
        business_map.entry(key).or_insert_with(Vec::new).push((id, title, occupation, year, address));
    }

    println!("Created business lookup map with {} unique name+address combinations\n", business_map.len());

    // Get all genealogy records
    println!("Loading genealogy records...");
    let genealogy_records: Vec<(i64, String, String)> = sqlx::query_as(
        "SELECT id, name, address 
         FROM unmatched_genealogy 
         WHERE name IS NOT NULL AND address IS NOT NULL"
    )
    .fetch_all(&pool)
    .await?;

    println!("Loaded {} genealogy records\n", genealogy_records.len());

    let mut matched = 0;
    let mut unique_matches = 0;
    let mut multiple_matches = 0;
    let mut examples = Vec::new();

    println!("Matching records...\n");

    for (i, (gen_id, gen_name, gen_address)) in genealogy_records.iter().enumerate() {
        if (i + 1) % 10000 == 0 {
            println!("Processed {}/{} genealogy records...", i + 1, genealogy_records.len());
        }

        let key = (
            gen_name.to_lowercase().trim().to_string(),
            gen_address.to_lowercase().trim().to_string()
        );

        if let Some(matches) = business_map.get(&key) {
            matched += 1;

            if matches.len() == 1 {
                unique_matches += 1;
                let (_bus_id, title, occupation, year, address) = &matches[0];

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

                // Update the genealogy record with business info
                sqlx::query(
                    "UPDATE unmatched_genealogy 
                     SET business_surname = ?,
                         business_forename = ?,
                         business_title = ?,
                         business_occupation = ?,
                         business_address = ?,
                         business_year = ?,
                         business_source = ?
                     WHERE id = ?"
                )
                .bind(surname)
                .bind(&forename)
                .bind(title)
                .bind(occupation)
                .bind(address)
                .bind(year.to_string())
                .bind("White's 1871")
                .bind(gen_id)
                .execute(&pool)
                .await?;

                if examples.len() < 10 {
                    examples.push((gen_name.clone(), gen_address.clone(), occupation.clone()));
                }
            } else {
                multiple_matches += 1;
            }
        }
    }

    println!("\n========================================");
    println!("MATCHING COMPLETE");
    println!("========================================");
    println!("Total genealogy records checked: {}", genealogy_records.len());
    println!("Records with at least one business match: {}", matched);
    println!("Records with unique business match (updated): {}", unique_matches);
    println!("Records with multiple business matches (not updated): {}", multiple_matches);
    println!("Update rate: {:.2}%", (unique_matches as f64 / genealogy_records.len() as f64) * 100.0);

    if !examples.is_empty() {
        println!("\n10 Example matches:");
        println!("{:=<100}", "");
        for (i, (name, address, occupation)) in examples.iter().enumerate() {
            println!("{}. {}", i + 1, name);
            println!("   Address: {}", address);
            println!("   Business: {}", occupation);
            println!("{:-<100}", "");
        }
    }

    Ok(())
}
