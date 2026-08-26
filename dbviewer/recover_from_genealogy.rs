use rusqlite::{Connection, Result};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    for c in line.chars() {
        match c {
            '"' => in_quotes = !in_quotes,
            ',' if !in_quotes => {
                fields.push(current.trim().to_string());
                current = String::new();
            }
            _ => current.push(c),
        }
    }
    fields.push(current.trim().to_string());
    fields
}

fn normalize(s: &str) -> String {
    // First, insert spaces before capital letters (handles "TrinityStreet" -> "Trinity Street")
    let mut spaced = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && c.is_uppercase() {
            spaced.push(' ');
        }
        spaced.push(c);
    }

    // Remove punctuation, extra spaces, lowercase
    spaced.to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867_rescued.db")?;

    eprintln!("=== FINDING PEOPLE IN GENEALOGY NOT IN DATABASE ===\n");

    // Get all existing people (normalized name + address combos)
    let mut existing: HashSet<(String, String)> = HashSet::new();
    {
        let mut stmt = conn.prepare("SELECT first_name, surname, street_address FROM sheffield_people")?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let first: String = row.get::<_, Option<String>>(0)?.unwrap_or_default();
            let surname: String = row.get::<_, Option<String>>(1)?.unwrap_or_default();
            let addr: String = row.get::<_, Option<String>>(2)?.unwrap_or_default();
            let name = normalize(&format!("{} {}", first, surname));
            let addr_norm = normalize(&addr);
            existing.insert((name, addr_norm));
        }
    }
    eprintln!("Loaded {} existing people", existing.len());

    // Get max unique_id
    let max_id: i64 = conn.query_row("SELECT MAX(unique_id) FROM sheffield_people", [], |r| r.get(0))?;
    eprintln!("Max unique_id: {}", max_id);

    // Read all genealogy CSVs
    let genealogy_path = Path::new("genealogy");
    let mut missing_people: Vec<(String, String, String, String, String, String, String)> = Vec::new();
    let mut seen: HashSet<(String, String)> = HashSet::new();

    let entries = fs::read_dir(genealogy_path).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().map(|e| e == "csv").unwrap_or(false) {
            let content = fs::read_to_string(&path).unwrap_or_default();
            let lines: Vec<&str> = content.lines().collect();

            // Skip header
            for line in lines.iter().skip(1) {
                let fields = parse_csv_line(line);
                if fields.len() >= 9 {
                    // Name,Spouse,Address,Parish,Area,Age,Born Approx,Birth Place,Relation,Profession
                    let name = &fields[0];
                    let address = &fields[2];
                    let parish = &fields[3];
                    let birth_year = &fields[6];
                    let birth_place = &fields[7];
                    let _relation = &fields[8];
                    let profession = fields.get(9).map(|s| s.as_str()).unwrap_or("");

                    // Parse name into first_name and surname
                    let name_parts: Vec<&str> = name.split_whitespace().collect();
                    if name_parts.len() >= 2 {
                        let first_name = name_parts[0];
                        let surname = name_parts[name_parts.len() - 1];

                        let name_norm = normalize(name);
                        let addr_norm = normalize(address);
                        let key = (name_norm.clone(), addr_norm.clone());

                        if !existing.contains(&key) && !seen.contains(&key) && !name.is_empty() && !surname.is_empty() {
                            missing_people.push((
                                first_name.to_string(),
                                surname.to_string(),
                                address.to_string(),
                                parish.to_string(),
                                birth_year.to_string(),
                                birth_place.to_string(),
                                profession.to_string(),
                            ));
                            seen.insert(key);
                        }
                    }
                }
            }
        }
    }

    eprintln!("Found {} people in genealogy not in database", missing_people.len());

    // Show sample
    eprintln!("\nSample (first 20):");
    for (i, (first, surname, addr, parish, birth, place, prof)) in missing_people.iter().take(20).enumerate() {
        eprintln!("  {}: {} {} @ {} ({}, born {} in {}) [{}]",
                 i + 1, first, surname, addr, parish, birth, place, prof);
    }

    eprintln!("\nTo add these, run with --commit flag");

    Ok(())
}
