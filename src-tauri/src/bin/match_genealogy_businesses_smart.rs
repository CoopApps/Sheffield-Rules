use sqlx::sqlite::SqlitePool;
use std::collections::HashMap;

fn normalize_string(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("SMART MATCHING: GENEALOGY TO BUSINESSES");
    println!("========================================\n");

    // Load all businesses into memory
    println!("Loading all businesses...");
    let businesses: Vec<(String, String, String, Option<String>)> = sqlx::query_as(
        "SELECT id, full_name, address, occupation FROM sheffield_businesses"
    )
    .fetch_all(&pool)
    .await?;

    println!("Loaded {} businesses", businesses.len());

    // Create lookup maps by normalized address and name
    let mut address_map: HashMap<String, Vec<(String, String, String, Option<String>)>> = HashMap::new();
    let mut name_map: HashMap<String, Vec<(String, String, String, Option<String>)>> = HashMap::new();

    for (id, name, address, occupation) in businesses {
        let norm_addr = normalize_string(&address);
        let norm_name = normalize_string(&name);

        address_map.entry(norm_addr).or_insert_with(Vec::new).push((id.clone(), name.clone(), address.clone(), occupation.clone()));
        name_map.entry(norm_name).or_insert_with(Vec::new).push((id, name, address, occupation));
    }

    println!("Created lookup maps");
    println!("  Unique normalized addresses: {}", address_map.len());
    println!("  Unique normalized names: {}", name_map.len());

    // Load all unmatched genealogy records
    println!("\nLoading unmatched genealogy records...");
    let genealogy: Vec<(String, String, String, Option<String>)> = sqlx::query_as(
        "SELECT id, name, address, profession FROM unmatched_genealogy"
    )
    .fetch_all(&pool)
    .await?;

    println!("Loaded {} genealogy records\n", genealogy.len());

    // Find matches
    println!("========================================");
    println!("FINDING MATCHES");
    println!("========================================\n");

    let mut exact_name_and_address = 0;
    let mut exact_name_diff_address = 0;
    let mut exact_address_diff_name = 0;

    println!("Examples of exact name + address matches:\n");
    let mut shown_exact = 0;

    for (gen_id, gen_name, gen_addr, gen_prof) in &genealogy {
        let norm_gen_name = normalize_string(gen_name);
        let norm_gen_addr = normalize_string(gen_addr);

        // Check for name AND address match
        if let Some(name_matches) = name_map.get(&norm_gen_name) {
            for (bus_id, bus_name, bus_addr, bus_occ) in name_matches {
                let norm_bus_addr = normalize_string(bus_addr);

                if norm_gen_addr == norm_bus_addr {
                    exact_name_and_address += 1;

                    if shown_exact < 20 {
                        println!("Match #{}", exact_name_and_address);
                        println!("  Genealogy: {} - {}", gen_name, gen_addr);
                        println!("  Business:  {} - {}", bus_name, bus_addr);
                        println!("  Gen Prof: {} | Bus Occ: {}",
                            gen_prof.as_deref().unwrap_or("N/A"),
                            bus_occ.as_deref().unwrap_or("N/A"));
                        println!();
                        shown_exact += 1;
                    }
                }
            }
        }
    }

    println!("\n========================================");
    println!("Examples of exact name matches (different address):\n");
    shown_exact = 0;

    for (gen_id, gen_name, gen_addr, gen_prof) in &genealogy {
        let norm_gen_name = normalize_string(gen_name);
        let norm_gen_addr = normalize_string(gen_addr);

        if let Some(name_matches) = name_map.get(&norm_gen_name) {
            for (bus_id, bus_name, bus_addr, bus_occ) in name_matches {
                let norm_bus_addr = normalize_string(bus_addr);

                if norm_gen_addr != norm_bus_addr {
                    exact_name_diff_address += 1;

                    if shown_exact < 20 {
                        println!("Match #{}", exact_name_diff_address);
                        println!("  Name: {}", gen_name);
                        println!("  Gen Addr: {}", gen_addr);
                        println!("  Bus Addr: {}", bus_addr);
                        println!("  Gen Prof: {} | Bus Occ: {}",
                            gen_prof.as_deref().unwrap_or("N/A"),
                            bus_occ.as_deref().unwrap_or("N/A"));
                        println!();
                        shown_exact += 1;
                    }
                }
            }
        }
    }

    println!("\n========================================");
    println!("Examples of exact address matches (different name):\n");
    shown_exact = 0;

    for (gen_id, gen_name, gen_addr, gen_prof) in &genealogy {
        let norm_gen_name = normalize_string(gen_name);
        let norm_gen_addr = normalize_string(gen_addr);

        if let Some(addr_matches) = address_map.get(&norm_gen_addr) {
            for (bus_id, bus_name, bus_addr, bus_occ) in addr_matches {
                let norm_bus_name = normalize_string(bus_name);

                if norm_gen_name != norm_bus_name {
                    exact_address_diff_name += 1;

                    if shown_exact < 20 {
                        println!("Match #{}", exact_address_diff_name);
                        println!("  Address: {}", gen_addr);
                        println!("  Gen Name: {}", gen_name);
                        println!("  Bus Name: {}", bus_name);
                        println!("  Gen Prof: {} | Bus Occ: {}",
                            gen_prof.as_deref().unwrap_or("N/A"),
                            bus_occ.as_deref().unwrap_or("N/A"));
                        println!();
                        shown_exact += 1;
                    }
                }
            }
        }
    }

    println!("\n========================================");
    println!("SUMMARY");
    println!("========================================");
    println!("Exact name + address matches: {}", exact_name_and_address);
    println!("Exact name, different address: {}", exact_name_diff_address);
    println!("Exact address, different name: {}", exact_address_diff_name);
    println!("========================================\n");

    Ok(())
}
