use csv::{ReaderBuilder, Writer};
use regex::Regex;
use std::collections::HashMap;

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

fn reorder_address(parts: Vec<String>) -> (String, String, String, String) {
    if parts.is_empty() {
        return ("".to_string(), "".to_string(), "".to_string(), "".to_string());
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
    let number = all_numbers.join(" ");
    let building = buildings.join(", ");
    let street = streets.join(", ");
    let city = cities.join(", ");

    (number, building, street, city)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_file = "D:/projects/Saturday at Three/unique_streets.csv";
    let output_file = "D:/projects/Saturday at Three/unique_streets_cleaned.csv";

    println!("========================================");
    println!("CLEANING STREET ADDRESS DATA");
    println!("========================================\n");

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(input_file)?;

    let mut writer = Writer::from_path(output_file)?;

    // Write header
    writer.write_record(&[
        "Example Person",
        "Number",
        "Building/Yard",
        "Street Name",
        "Area/City",
        "Original Address"
    ])?;

    let mut processed = 0;
    let mut cleaned_count = 0;

    for result in reader.records() {
        let record = result?;
        processed += 1;

        if processed % 1000 == 0 {
            println!("Processed {} records...", processed);
        }

        let person = record.get(0).unwrap_or("");
        let original_address = record.get(4).unwrap_or("");

        // Split address by commas
        let parts: Vec<String> = original_address
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        // Reorder and clean
        let (number, building, street, city) = reorder_address(parts);

        // Count if we made changes
        let new_format = format!("{},{},{},{}", number, building, street, city);
        if new_format != original_address {
            cleaned_count += 1;
        }

        // Write cleaned record
        writer.write_record(&[
            person,
            &number,
            &building,
            &street,
            &city,
            original_address
        ])?;
    }

    writer.flush()?;

    println!("\n========================================");
    println!("CLEANING COMPLETE");
    println!("========================================");
    println!("Total records processed: {}", processed);
    println!("Records cleaned: {}", cleaned_count);
    println!("Output file: {}\n", output_file);

    // Show some examples
    println!("Example transformations:");
    println!("  Yard,CumberlandLane,Sheffield");
    println!("  → Number: | Building: Yard | Street: Cumberland Lane | City: Sheffield\n");
    println!("  WoodStreet,ScottsBuildings");
    println!("  → Number: | Building: Scotts Buildings | Street: Wood Street\n");
    println!("  NewHerefordStreet56");
    println!("  → Number: 56 | Building: | Street: New Hereford Street\n");

    Ok(())
}
