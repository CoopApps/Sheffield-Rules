// Script to generate updated Rust code for clubs with city and region data
// Run with: rustc update_clubs_with_locations.rs && ./update_clubs_with_locations

use std::collections::HashMap;
use std::fs;

fn main() {
    // Parse the CSV file with location data
    let csv_content = fs::read_to_string("../club_locations.csv").expect("Failed to read club_locations.csv");

    let mut club_locations: HashMap<String, (String, String)> = HashMap::new();

    for line in csv_content.lines().skip(3) { // Skip header lines
        if line.trim().is_empty() || line.starts_with("Matched") || line.starts_with("Unmatched") {
            continue;
        }

        // Parse CSV: "Club Name",Year,"Area",Postcode
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() < 4 {
            continue;
        }

        let name = parts[0].trim().trim_matches('"');
        let area = parts[2].trim().trim_matches('"');
        let postcode = parts[3].trim();

        club_locations.insert(name.to_string(), (area.to_string(), postcode.to_string()));
    }

    eprintln!("Loaded {} club locations", club_locations.len());

    // Read the current clubs.rs file
    let clubs_rs = fs::read_to_string("src/sheffield_rules/clubs.rs").expect("Failed to read clubs.rs");

    // Find club definitions and add city/region fields
    let mut output = String::new();
    let mut in_club_def = false;
    let mut current_club_name = String::new();
    let mut club_lines = Vec::new();

    for line in clubs_rs.lines() {
        if line.trim().starts_with("SheffieldClub {") {
            in_club_def = true;
            club_lines.clear();
            club_lines.push(line.to_string());
        } else if in_club_def {
            club_lines.push(line.to_string());

            // Extract club name
            if line.contains("name:") {
                let name_part = line.split("name:").nth(1).unwrap();
                let name = name_part.split('"').nth(1).unwrap_or("");
                current_club_name = name.to_string();
            }

            // Check if we reached the end of the struct
            if line.trim().starts_with("},") || line.trim() == "}" {
                in_club_def = false;

                // Output the modified club definition
                for club_line in &club_lines[..club_lines.len()-1] {
                    // Skip the closing brace line
                    if club_line.contains("origin:") {
                        output.push_str(club_line);
                        output.push('\n');

                        // Add city and region fields
                        if let Some((area, postcode)) = club_locations.get(&current_club_name) {
                            // Get the indentation from the origin line
                            let indent = club_line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
                            output.push_str(&format!("{}city: Some(\"{}\".to_string()),\n", indent, area));
                            output.push_str(&format!("{}region: Some(\"{}\".to_string()),\n", indent, postcode));
                        } else {
                            let indent = club_line.chars().take_while(|c| c.is_whitespace()).collect::<String>();
                            output.push_str(&format!("{}city: None,\n", indent));
                            output.push_str(&format!("{}region: None,\n", indent));
                        }
                    } else {
                        output.push_str(club_line);
                        output.push('\n');
                    }
                }
                // Add the closing brace
                output.push_str(club_lines.last().unwrap());
                output.push('\n');
            }
        } else {
            output.push_str(line);
            output.push('\n');
        }
    }

    // Write the updated file
    fs::write("src/sheffield_rules/clubs_updated.rs", output).expect("Failed to write updated file");
    eprintln!("Updated clubs.rs written to clubs_updated.rs");
    eprintln!("Review the file and then rename it to clubs.rs");
}
