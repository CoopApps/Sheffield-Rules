use sqlx::sqlite::SqlitePool;
use regex::Regex;

fn split_camel_case(s: &str) -> String {
    let re = Regex::new(r"([a-z])([A-Z])").unwrap();
    re.replace_all(s, "$1 $2").to_string()
}

fn extract_number_and_text(s: &str) -> (String, String) {
    // Try to extract trailing number
    let re = Regex::new(r"^(.+?)(\d+)$").unwrap();
    if let Some(caps) = re.captures(s) {
        let text = caps.get(1).map_or("", |m| m.as_str());
        let num = caps.get(2).map_or("", |m| m.as_str());
        return (num.to_string(), text.to_string());
    }

    // Try to extract leading number
    let re = Regex::new(r"^(\d+)(.+)$").unwrap();
    if let Some(caps) = re.captures(s) {
        let num = caps.get(1).map_or("", |m| m.as_str());
        let text = caps.get(2).map_or("", |m| m.as_str());
        return (num.to_string(), text.to_string());
    }

    ("".to_string(), s.to_string())
}

fn classify_address_part(part: &str) -> &str {
    let lower = part.to_lowercase();

    // Cities/areas
    if lower.contains("sheffield") || lower.contains("rotherham") ||
       lower.contains("doncaster") || lower.contains("barnsley") {
        return "city";
    }

    // Building types
    if lower.contains("yard") || lower.contains("court") || lower.contains("building") ||
       lower.contains("cottage") || lower.contains("house") || lower.contains("place") ||
       lower.contains("terrace") || lower.contains("row") || lower.contains("square") {
        return "building";
    }

    // Default to street
    "street"
}

fn clean_address_part(part: &str) -> (String, String) {
    let trimmed = part.trim();

    // Extract number if present
    let (number, text) = extract_number_and_text(trimmed);

    // Split camel case
    let spaced = split_camel_case(&text);

    // Return number separately and cleaned text
    (number, spaced.trim().to_string())
}

fn reorder_address(parts: Vec<String>) -> String {
    if parts.is_empty() {
        return String::new();
    }

    // Clean all parts and extract numbers
    let mut all_numbers = Vec::new();
    let mut streets = Vec::new();
    let mut buildings = Vec::new();
    let mut cities = Vec::new();

    for part in parts {
        let (number, text) = clean_address_part(&part);

        // Collect numbers separately
        if !number.is_empty() {
            all_numbers.push(number);
        }

        // Classify the text part if not empty
        if !text.is_empty() {
            match classify_address_part(&text) {
                "city" => cities.push(text),
                "building" => buildings.push(text),
                _ => streets.push(text),
            }
        }
    }

    // Build ordered address: Number FIRST, then Building/Yard, then Street, then Area/City
    let mut result = Vec::new();

    if !all_numbers.is_empty() {
        result.push(all_numbers.join(" "));
    }
    if !buildings.is_empty() {
        result.push(buildings.join(", "));
    }
    if !streets.is_empty() {
        result.push(streets.join(", "));
    }
    if !cities.is_empty() {
        result.push(cities.join(", "));
    }

    result.join(", ")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite:D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(database_url).await?;

    println!("========================================");
    println!("UPDATING GENEALOGY ADDRESS DATA");
    println!("========================================\n");

    // Get all records with addresses
    let records: Vec<(i64, String)> = sqlx::query_as(
        "SELECT id, address FROM unmatched_genealogy WHERE address IS NOT NULL AND address != ''"
    )
    .fetch_all(&pool)
    .await?;

    println!("Found {} records with addresses to clean\n", records.len());

    let mut updated = 0;
    let mut unchanged = 0;

    for (i, (id, original_address)) in records.iter().enumerate() {
        if (i + 1) % 1000 == 0 {
            println!("Processed {}/{} records...", i + 1, records.len());
        }

        // Split address by commas
        let parts: Vec<String> = original_address
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        // Reorder and clean
        let cleaned_address = reorder_address(parts);

        // Only update if changed
        if &cleaned_address != original_address {
            sqlx::query("UPDATE unmatched_genealogy SET address = ? WHERE id = ?")
                .bind(&cleaned_address)
                .bind(id)
                .execute(&pool)
                .await?;

            updated += 1;
        } else {
            unchanged += 1;
        }
    }

    println!("\n========================================");
    println!("UPDATE COMPLETE");
    println!("========================================");
    println!("Total records processed: {}", records.len());
    println!("Records updated: {}", updated);
    println!("Records unchanged: {}", unchanged);
    println!("\nExample transformations:");
    println!("  Yard,CumberlandLane,Sheffield");
    println!("  → Yard, Cumberland Lane, Sheffield\n");
    println!("  WoodStreet,ScottsBuildings");
    println!("  → Scotts Buildings, Wood Street\n");
    println!("  NewHerefordStreet56");
    println!("  → 56, New Hereford Street\n");

    Ok(())
}
