// Temporary script to parse location data
// Run with: rustc build_location_data.rs && ./build_location_data

use std::collections::HashMap;
use std::fs;

fn main() {
    // Parse postcode file
    let postcode_content = fs::read_to_string("../postcode.txt").expect("Failed to read postcode.txt");
    let mut location_to_postcode: HashMap<String, String> = HashMap::new();

    for line in postcode_content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("S1-S36") || line.starts_with("Postcode") {
            continue;
        }

        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 3 {
            continue;
        }

        let postcode = parts[0].trim();
        let coverage = parts[2].trim();

        // Split coverage by commas
        for location in coverage.split(',') {
            let loc = location.trim();
            if !loc.is_empty() && loc != "City Centre" {
                location_to_postcode.insert(loc.to_lowercase(), postcode.to_string());
            }
        }
    }

    println!("Loaded {} location->postcode mappings", location_to_postcode.len());

    // Parse clubs file
    let clubs_content = fs::read_to_string("../1857-1875.txt").expect("Failed to read 1857-1875.txt");
    let mut matched = 0;
    let mut unmatched = Vec::new();

    println!("\nClub,Year,Area,Postcode");

    for line in clubs_content.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 3 {
            continue;
        }

        let name = parts[0].trim();
        let year = parts[1].trim();
        let location_text = parts[2].trim();

        // Try to find location match
        let (area, postcode) = extract_area(location_text, &location_to_postcode);

        if let (Some(a), Some(p)) = (area, postcode) {
            println!("\"{}\",{},\"{}\",{}", name, year, a, p);
            matched += 1;
        } else {
            unmatched.push((name, location_text));
        }
    }

    eprintln!("\nMatched: {}", matched);
    eprintln!("Unmatched: {}", unmatched.len());
}

fn extract_area(location_text: &str, location_to_postcode: &HashMap<String, String>) -> (Option<String>, Option<String>) {
    let text_lower = location_text.to_lowercase();

    // Find longest matching location
    let mut best_match: Option<(&str, &str)> = None;
    let mut longest_len = 0;

    for (location, postcode) in location_to_postcode.iter() {
        if text_lower.contains(location.as_str()) && location.len() > longest_len {
            best_match = Some((location, postcode));
            longest_len = location.len();
        }
    }

    if let Some((loc, code)) = best_match {
        // Capitalize first letter of each word
        let area = loc.split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
        return (Some(area), Some(code.to_string()));
    }

    (None, None)
}
