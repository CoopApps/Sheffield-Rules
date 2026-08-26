use sqlx::sqlite::SqlitePool;
use std::collections::HashMap;

fn normalize_string(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphabetic() || c.is_numeric())
        .flat_map(|c| c.to_lowercase())
        .collect()
}

fn profession_similarity(prof1: &str, prof2: &str) -> bool {
    if prof1.is_empty() || prof2.is_empty() {
        return false;
    }

    let norm1 = normalize_string(prof1);
    let norm2 = normalize_string(prof2);

    // Check for exact match
    if norm1 == norm2 {
        return true;
    }

    // Check if one contains the other (for variations like "Grocer" vs "Grocer & Shopkeeper")
    if norm1.contains(&norm2) || norm2.contains(&norm1) {
        return true;
    }

    // Check for common profession keywords
    let keywords1: Vec<&str> = vec!["grocer", "baker", "butcher", "blacksmith", "shopkeeper",
        "maker", "dealer", "manufacturer", "smith", "cutler", "grinder", "forger"];

    for keyword in keywords1 {
        if norm1.contains(keyword) && norm2.contains(keyword) {
            return true;
        }
    }

    false
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("ANALYZING MATCHES WITH PROFESSION CHECK");
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
    println!("FINDING MATCHES WITH PROFESSION CHECK");
    println!("========================================\n");

    let mut unique_with_prof_match = 0;
    let mut unique_without_prof_match = 0;
    let mut shown_with_prof = 0;

    let rotherham_keywords = ["rotherham"];

    println!("HIGH CONFIDENCE matches (unique name + similar profession):\n");

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
                            let gen_prof_str = gen_prof.as_deref().unwrap_or("");
                            let bus_occ_str = bus_occ.as_deref().unwrap_or("");

                            if profession_similarity(gen_prof_str, bus_occ_str) {
                                unique_with_prof_match += 1;

                                if shown_with_prof < 50 {
                                    println!("Match #{}", unique_with_prof_match);
                                    println!("  Name: {}", gen_name);
                                    println!("  Gen Addr: {}", gen_addr);
                                    println!("  Bus Addr: {}", bus_addr);
                                    println!("  Normalized Gen: {}", norm_gen_addr);
                                    println!("  Normalized Bus: {}", norm_bus_addr);
                                    println!("  Gen Prof: {} | Bus Occ: {}", gen_prof_str, bus_occ_str);
                                    println!("  ✓ Professions match!");
                                    println!();
                                    shown_with_prof += 1;
                                }
                            } else {
                                unique_without_prof_match += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("\n========================================");
    println!("SUMMARY (with profession similarity)");
    println!("========================================");
    println!("Total unique name matches: {}", unique_with_prof_match + unique_without_prof_match);
    println!("  - With matching/similar profession: {} (HIGH CONFIDENCE)", unique_with_prof_match);
    println!("  - Without profession match: {} (LOWER CONFIDENCE)", unique_without_prof_match);
    println!();
    println!("The {} matches with similar professions are very high-confidence", unique_with_prof_match);
    println!("cases where the business address is the person's work location.");
    println!("========================================\n");

    Ok(())
}
