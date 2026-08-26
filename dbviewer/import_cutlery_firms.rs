use rusqlite::{Connection, Result};
use std::fs;
use std::collections::HashMap;

fn normalize_address(addr: &str) -> String {
    addr.to_lowercase()
        .replace("street", "st")
        .replace("road", "rd")
        .replace("lane", "la")
        .replace("place", "pl")
        .replace("square", "sq")
        .replace(",", "")
        .replace(".", "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn extract_street_name(addr: &str) -> Option<String> {
    // Extract the main street name from an address
    let addr_lower = addr.to_lowercase();
    let street_indicators = ["street", "road", "lane", "place", "works", "row"];

    for indicator in &street_indicators {
        if let Some(pos) = addr_lower.find(indicator) {
            // Get words before the indicator
            let before = &addr[..pos];
            let words: Vec<&str> = before.split_whitespace().collect();
            if let Some(last_word) = words.last() {
                // Return the street name (e.g., "Rockingham" from "Rockingham Street")
                return Some(last_word.to_lowercase());
            }
        }
    }
    None
}

fn addresses_match(firm_addr: &str, person_addr: &str) -> bool {
    if firm_addr.is_empty() || person_addr.is_empty() {
        return false;
    }

    let firm_norm = normalize_address(firm_addr);
    let person_norm = normalize_address(person_addr);

    // Exact match
    if firm_norm == person_norm {
        return true;
    }

    // Check if one contains the other
    if firm_norm.contains(&person_norm) || person_norm.contains(&firm_norm) {
        return true;
    }

    // Extract and compare street names
    if let (Some(firm_street), Some(person_street)) = (extract_street_name(firm_addr), extract_street_name(person_addr)) {
        if firm_street == person_street {
            return true;
        }
    }

    false
}

fn extract_year(text: &str) -> Option<i32> {
    // Look for 4-digit years between 1700 and 1900
    for word in text.split(|c: char| !c.is_numeric()) {
        if word.len() == 4 {
            if let Ok(year) = word.parse::<i32>() {
                if year >= 1700 && year <= 1900 {
                    return Some(year);
                }
            }
        }
    }
    None
}

fn extract_person_names(firm_name: &str) -> Vec<(Option<String>, String)> {
    // Extract potential person names from firm name
    // Returns list of (first_name, surname) pairs
    let mut results = Vec::new();

    // Remove common suffixes
    let clean = firm_name
        .replace(" Limited", "")
        .replace(" Ltd", "")
        .replace(" Ltd.", "")
        .replace(" and Sons", "")
        .replace(" & Sons", "")
        .replace(" and Company", "")
        .replace(" & Company", "")
        .replace(" & Co", "")
        .replace(" and Co", "")
        .replace(" Brothers", "")
        .replace(" Bros", "");

    // Split by "and" or "&" for partnerships
    let partners: Vec<&str> = clean.split(|c| c == '&')
        .flat_map(|s| s.split(" and "))
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    for partner in partners {
        let words: Vec<&str> = partner.split_whitespace().collect();

        match words.len() {
            1 => {
                // Just surname: "Ashberry"
                results.push((None, words[0].to_string()));
            }
            2 => {
                // Could be "John Smith" or "J. Smith" or "I. Barber"
                let first = words[0].trim_matches(|c: char| !c.is_alphabetic());
                let surname = words[1].to_string();
                if first.len() <= 2 {
                    // Initial
                    results.push((Some(first.to_string()), surname));
                } else {
                    // Full first name
                    results.push((Some(first.to_string()), surname));
                }
            }
            n if n >= 3 => {
                // "Philip Ashberry" or longer - take first and last
                let first = words[0].trim_matches(|c: char| !c.is_alphabetic());
                let surname = words[n-1].to_string();
                results.push((Some(first.to_string()), surname));
            }
            _ => {}
        }
    }

    results
}

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;

    eprintln!("=== IMPORTING CUTLERY FIRMS ===\n");

    // Create table
    conn.execute("DROP TABLE IF EXISTS sheffield_cutlery_firms", [])?;
    conn.execute(
        "CREATE TABLE sheffield_cutlery_firms (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            firm_name TEXT NOT NULL,
            business_type TEXT,
            address TEXT,
            earliest_year INTEGER,
            notes TEXT,
            person_id INTEGER,
            FOREIGN KEY (person_id) REFERENCES sheffield_people(unique_id)
        )",
        [],
    )?;

    // Read the file
    let content = fs::read_to_string("cutler info.txt").expect("Could not read file");
    let lines: Vec<&str> = content.lines().collect();

    let mut firms: Vec<(String, String, String, Option<i32>, String)> = Vec::new();
    let mut current_firm = String::new();
    let mut current_type = String::new();
    let mut current_address = String::new();
    let mut current_notes = String::new();
    let mut earliest_year: Option<i32> = None;

    let skip_patterns = [
        "SHEFFIELD LIBRARIES",
        "Sheffield City Archives and Sheffield Local Studies",
        "© Sheffield",
        "Page ",
        "http://",
        "(accessed",
    ];

    let business_type_keywords = [
        "Manufacturer", "Silversmith", "Cutler", "Plater", "maker", "Maker",
        "Metal", "Forger", "Designer", "Scissors", "Knives", "Silver",
        "Electro", "Britannia", "goods", "Pen,", "pocket,",
    ];

    for line in &lines {
        let trimmed = line.trim();

        // Skip headers and empty lines
        if trimmed.is_empty() || skip_patterns.iter().any(|p| trimmed.contains(p)) {
            continue;
        }

        // Check if this is a bullet point (note/archive reference)
        if trimmed.starts_with("•") || trimmed.starts_with("Note:") {
            // Extract years from notes
            if let Some(year) = extract_year(trimmed) {
                if earliest_year.is_none() || year < earliest_year.unwrap() {
                    earliest_year = Some(year);
                }
            }
            if !current_notes.is_empty() {
                current_notes.push_str("; ");
            }
            current_notes.push_str(trimmed);
            continue;
        }

        // Check if line starts with uppercase (potential firm name or business type)
        let first_char = trimmed.chars().next().unwrap_or(' ');
        if !first_char.is_uppercase() {
            continue;
        }

        // Is this a business type line?
        let is_business_type = business_type_keywords.iter().any(|k| trimmed.contains(k));

        // Is this an address line?
        let is_address = (trimmed.contains("Street") || trimmed.contains("Road") ||
                        trimmed.contains("Lane") || trimmed.contains("Works") ||
                        trimmed.contains("Place") || trimmed.contains("Row")) &&
                        trimmed.contains("Sheffield");

        if is_business_type && !current_firm.is_empty() {
            // This is the business type for current firm
            current_type = trimmed.to_string();
        } else if is_address && !current_firm.is_empty() {
            // This is the address for current firm
            current_address = trimmed.to_string();
        } else if !is_business_type && !is_address {
            // This looks like a new firm name
            // Save previous firm if exists
            if !current_firm.is_empty() {
                // Only include firms with dates before/including 1867
                if earliest_year.map(|y| y <= 1867).unwrap_or(false) {
                    firms.push((
                        current_firm.clone(),
                        current_type.clone(),
                        current_address.clone(),
                        earliest_year,
                        current_notes.clone(),
                    ));
                }
            }

            current_firm = trimmed.to_string();
            current_type = String::new();
            current_address = String::new();
            current_notes = String::new();
            earliest_year = None;
        }
    }

    // Save last firm
    if !current_firm.is_empty() && earliest_year.map(|y| y <= 1867).unwrap_or(false) {
        firms.push((
            current_firm,
            current_type,
            current_address,
            earliest_year,
            current_notes,
        ));
    }

    eprintln!("Found {} firms with dates before/including 1867", firms.len());

    // Insert firms
    for (name, btype, addr, year, notes) in &firms {
        conn.execute(
            "INSERT INTO sheffield_cutlery_firms (firm_name, business_type, address, earliest_year, notes)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![name, btype, addr, year, notes],
        )?;
    }

    // Now try to match to sheffield_people
    eprintln!("\n=== MATCHING FIRMS TO PEOPLE ===\n");

    // Load people into memory, indexed by surname
    let mut people_by_surname: HashMap<String, Vec<(i64, String, String, String, String)>> = HashMap::new();
    {
        let mut stmt = conn.prepare(
            "SELECT unique_id, first_name, surname, street_address, profession FROM sheffield_people"
        )?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let uid: i64 = row.get(0)?;
            let first: String = row.get::<_, Option<String>>(1)?.unwrap_or_default();
            let surname: String = row.get::<_, Option<String>>(2)?.unwrap_or_default();
            let addr: String = row.get::<_, Option<String>>(3)?.unwrap_or_default();
            let prof: String = row.get::<_, Option<String>>(4)?.unwrap_or_default();

            let key = surname.to_lowercase();
            people_by_surname.entry(key).or_default().push((uid, first, surname, addr, prof));
        }
    }

    // Get firms to match
    let mut to_match: Vec<(i64, String, String)> = Vec::new();
    {
        let mut stmt = conn.prepare(
            "SELECT id, firm_name, address FROM sheffield_cutlery_firms"
        )?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let id: i64 = row.get(0)?;
            let name: String = row.get(1)?;
            let addr: String = row.get::<_, Option<String>>(2)?.unwrap_or_default();
            to_match.push((id, name, addr));
        }
    }

    let mut matched = 0;
    let mut unmatched: Vec<(String, String)> = Vec::new();

    for (firm_id, firm_name, firm_addr) in &to_match {
        let person_names = extract_person_names(firm_name);
        let mut found_match = false;

        for (first_opt, surname) in &person_names {
            let key = surname.to_lowercase();

            if let Some(candidates) = people_by_surname.get(&key) {
                for (uid, person_first, person_surname, person_addr, person_prof) in candidates {
                    // Check if first name matches (if provided)
                    let first_matches = match first_opt {
                        Some(first) => {
                            let first_lower = first.to_lowercase();
                            let person_first_lower = person_first.to_lowercase();
                            // Match full name or initial
                            first_lower == person_first_lower ||
                            (first.len() == 1 && person_first_lower.starts_with(&first_lower))
                        }
                        None => true // No first name to match
                    };

                    // Check address match
                    let addr_matches = addresses_match(firm_addr, person_addr);

                    // Match if: (first name matches AND address matches) OR (exact name AND cutlery profession)
                    let is_cutlery_prof = person_prof.to_lowercase().contains("cutl") ||
                                          person_prof.to_lowercase().contains("knife") ||
                                          person_prof.to_lowercase().contains("silver") ||
                                          person_prof.to_lowercase().contains("plat");

                    if (first_matches && addr_matches) || (first_matches && is_cutlery_prof) {
                        conn.execute(
                            "UPDATE sheffield_cutlery_firms SET person_id = ?1 WHERE id = ?2",
                            [*uid, *firm_id],
                        )?;
                        matched += 1;
                        eprintln!("MATCHED: {} -> {} {} @ {} [{}]",
                            firm_name, person_first, person_surname, person_addr, person_prof);
                        found_match = true;
                        break;
                    }
                }
            }

            if found_match {
                break;
            }
        }

        if !found_match {
            unmatched.push((firm_name.clone(), firm_addr.clone()));
        }
    }

    eprintln!("\nMatched {} firms to people", matched);

    // Show summary
    let total: i64 = conn.query_row("SELECT COUNT(*) FROM sheffield_cutlery_firms", [], |r| r.get(0))?;
    let matched_count: i64 = conn.query_row("SELECT COUNT(*) FROM sheffield_cutlery_firms WHERE person_id IS NOT NULL", [], |r| r.get(0))?;

    eprintln!("\n=== SUMMARY ===");
    eprintln!("Total firms (pre-1867): {}", total);
    eprintln!("Matched to people: {}", matched_count);

    // Show matched firms
    eprintln!("\n=== MATCHED FIRMS ===\n");
    let mut stmt = conn.prepare(
        "SELECT f.firm_name, f.business_type, f.address, f.earliest_year, p.first_name, p.surname, p.street_address
         FROM sheffield_cutlery_firms f
         JOIN sheffield_people p ON f.person_id = p.unique_id
         LIMIT 30"
    )?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let name: String = row.get(0)?;
        let btype: String = row.get::<_, Option<String>>(1)?.unwrap_or_default();
        let addr: String = row.get::<_, Option<String>>(2)?.unwrap_or_default();
        let year: Option<i32> = row.get(3)?;
        let pfirst: String = row.get::<_, Option<String>>(4)?.unwrap_or_default();
        let psur: String = row.get::<_, Option<String>>(5)?.unwrap_or_default();
        let paddr: String = row.get::<_, Option<String>>(6)?.unwrap_or_default();
        eprintln!("{} [{}] -> {} {} @ {}",
            name, year.map(|y| y.to_string()).unwrap_or_default(),
            pfirst, psur, paddr);
    }

    // Show unmatched
    eprintln!("\n=== UNMATCHED FIRMS (first 20) ===\n");
    for (name, addr) in unmatched.iter().take(20) {
        eprintln!("{} @ {}", name, addr);
    }

    Ok(())
}
