use sqlx::sqlite::SqlitePool;
use std::collections::HashMap;

fn normalize_string(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphabetic() || c.is_numeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("ANALYZING UNIQUE NAME MATCHES (V2)");
    println!("(Now including numbers in normalization)");
    println!("========================================\n");

    // Load all businesses into memory
    println!("Loading all businesses...");
    let businesses: Vec<(String, String, String, Option<String>)> = sqlx::query_as(
        "SELECT id, full_name, address, occupation FROM sheffield_businesses"
    )
    .fetch_all(&pool)
    .await?;

    println!("Loaded {} businesses", businesses.len());

    // Create name lookup map
    let mut name_map: HashMap<String, Vec<(String, String, String, Option<String>)>> = HashMap::new();

    for (id, name, address, occupation) in businesses {
        let norm_name = normalize_string(&name);
        name_map.entry(norm_name).or_insert_with(Vec::new).push((id, name, address, occupation));
    }

    // Load all genealogy records
    println!("Loading genealogy records...");
    let genealogy: Vec<(String, String, String, Option<String>)> = sqlx::query_as(
        "SELECT id, name, address, profession FROM unmatched_genealogy"
    )
    .fetch_all(&pool)
    .await?;

    println!("Loaded {} genealogy records\n", genealogy.len());

    // Track how many times each normalized name appears in genealogy
    let mut genealogy_name_counts: HashMap<String, usize> = HashMap::new();
    for (_, gen_name, _, _) in &genealogy {
        let norm_name = normalize_string(gen_name);
        *genealogy_name_counts.entry(norm_name).or_insert(0) += 1;
    }

    println!("========================================");
    println!("FINDING UNIQUE NAME MATCHES");
    println!("========================================\n");

    let mut unique_name_matches = 0;
    let mut duplicate_name_matches = 0;
    let mut shown_unique = 0;

    // Also filter out Rotherham addresses
    let rotherham_keywords = ["rotherham"];

    println!("Examples of UNIQUE name matches (name appears only once in genealogy):");
    println!("(Filtering out Rotherham addresses)\n");

    for (_gen_id, gen_name, gen_addr, gen_prof) in &genealogy {
        let norm_gen_name = normalize_string(gen_name);
        let norm_gen_addr = normalize_string(gen_addr);

        // Check if this name appears in businesses
        if let Some(name_matches) = name_map.get(&norm_gen_name) {
            for (_bus_id, _bus_name, bus_addr, bus_occ) in name_matches {
                let norm_bus_addr = normalize_string(bus_addr);

                // Skip if business address contains Rotherham
                let bus_addr_lower = bus_addr.to_lowercase();
                if rotherham_keywords.iter().any(|&kw| bus_addr_lower.contains(kw)) {
                    continue;
                }

                // Only count if addresses are different
                if norm_gen_addr != norm_bus_addr {
                    // Check if this normalized name appears only once in genealogy
                    if let Some(&count) = genealogy_name_counts.get(&norm_gen_name) {
                        if count == 1 {
                            unique_name_matches += 1;

                            if shown_unique < 30 {
                                println!("Match #{}", unique_name_matches);
                                println!("  Name: {}", gen_name);
                                println!("  Gen Addr (home?): {}", gen_addr);
                                println!("  Bus Addr (work?): {}", bus_addr);
                                println!("  Normalized Gen: {}", norm_gen_addr);
                                println!("  Normalized Bus: {}", norm_bus_addr);
                                println!("  Gen Prof: {} | Bus Occ: {}",
                                    gen_prof.as_deref().unwrap_or("N/A"),
                                    bus_occ.as_deref().unwrap_or("N/A"));
                                println!();
                                shown_unique += 1;
                            }
                        } else {
                            duplicate_name_matches += 1;
                        }
                    }
                }
            }
        }
    }

    println!("\n========================================");
    println!("SUMMARY (V2 with numbers included)");
    println!("========================================");
    println!("Total name matches with different addresses: {}", unique_name_matches + duplicate_name_matches);
    println!("  - Unique names (appearing once in genealogy): {}", unique_name_matches);
    println!("  - Duplicate names (appearing multiple times): {}", duplicate_name_matches);
    println!();
    println!("The {} unique name matches are high-confidence cases where", unique_name_matches);
    println!("the business address is likely the person's work address.");
    println!("(Rotherham addresses filtered out, numbers now included in matching)");
    println!("========================================\n");

    Ok(())
}
