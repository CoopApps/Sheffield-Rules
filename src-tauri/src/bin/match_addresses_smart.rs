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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite:D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(database_url).await?;

    println!("========================================");
    println!("MATCHING ADDRESSES - SMART NORMALIZATION");
    println!("Fixes formatting: '70,SnigHill' -> '70 Snig Hill'");
    println!("========================================\n");

    // Test the normalization
    println!("Normalization examples:");
    println!("  '70,SnigHill' -> '{}'", normalize_address("70,SnigHill"));
    println!("  '70, Snig Hill' -> '{}'", normalize_address("70, Snig Hill"));
    println!("  '70 Snig Hill' -> '{}'", normalize_address("70 Snig Hill"));
    println!();

    // Get all businesses
    println!("Loading businesses from sheffield_businesses...");
    let businesses: Vec<(String, String, String, String, String, i64)> = sqlx::query_as(
        "SELECT id, surname, forename, occupation, address, year
         FROM sheffield_businesses 
         WHERE address IS NOT NULL AND address != ''"
    )
    .fetch_all(&pool)
    .await?;

    println!("Loaded {} businesses\n", businesses.len());

    // Create business map
    let mut business_map: HashMap<String, Vec<(String, String, String, String, i64)>> = HashMap::new();
    
    for (id, surname, forename, occupation, address, year) in businesses {
        let normalized = normalize_address(&address);
        business_map.entry(normalized).or_insert_with(Vec::new)
            .push((format!("{} {}", forename, surname), occupation, address.clone(), id, year));
    }

    println!("Created business lookup map with {} unique normalized addresses\n", business_map.len());

    // Get all genealogy records
    println!("Loading genealogy records...");
    let genealogy_records: Vec<(i64, String, String)> = sqlx::query_as(
        "SELECT id, name, address 
         FROM unmatched_genealogy 
         WHERE address IS NOT NULL AND address != ''"
    )
    .fetch_all(&pool)
    .await?;

    println!("Loaded {} genealogy records\n", genealogy_records.len());

    let mut matched = 0;
    let mut total_business_matches = 0;
    let mut examples = Vec::new();

    println!("Matching addresses...\n");

    for (i, (_gen_id, gen_name, gen_address)) in genealogy_records.iter().enumerate() {
        if (i + 1) % 10000 == 0 {
            println!("Processed {}/{} genealogy records...", i + 1, genealogy_records.len());
        }

        let normalized = normalize_address(gen_address);

        if let Some(matches) = business_map.get(&normalized) {
            matched += 1;
            total_business_matches += matches.len();

            if examples.len() < 20 {
                examples.push((
                    gen_name.clone(),
                    gen_address.clone(),
                    matches.len(),
                    matches.iter().map(|(n, o, orig_addr, _, _)| 
                        (n.clone(), o.clone(), orig_addr.clone())
                    ).collect::<Vec<_>>()
                ));
            }
        }
    }

    println!("\n========================================");
    println!("MATCHING COMPLETE");
    println!("========================================");
    println!("Total genealogy records checked: {}", genealogy_records.len());
    println!("Genealogy records with address match: {}", matched);
    println!("Total business records at those addresses: {}", total_business_matches);
    println!("Match rate: {:.2}%", (matched as f64 / genealogy_records.len() as f64) * 100.0);
    println!("Avg businesses per matched address: {:.2}", if matched > 0 { total_business_matches as f64 / matched as f64 } else { 0.0 });

    if !examples.is_empty() {
        println!("\n20 Example address matches:");
        println!("{:=<120}", "");
        for (i, (name, gen_addr, count, businesses)) in examples.iter().enumerate() {
            println!("{}. {} at '{}'", i + 1, name, gen_addr);
            println!("   {} business(es) at this address:", count);
            for (bus_name, occupation, orig_addr) in businesses.iter().take(5) {
                println!("     - {} ({}) [stored as: '{}']", bus_name, occupation, orig_addr);
            }
            if businesses.len() > 5 {
                println!("     ... and {} more", businesses.len() - 5);
            }
            println!("{:-<120}", "");
        }
    }

    Ok(())
}
