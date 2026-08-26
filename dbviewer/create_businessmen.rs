use rusqlite::{Connection, Result};
use std::collections::HashMap;

fn extract_employee_count(profession: &str) -> i32 {
    // Parse employee counts from profession strings like:
    // "Manufacturer Employing 202 Man 25 Woman 37 Boys"
    // "Builder Employing 30 Men"
    // "Carpet Manufacturing Employing At the Carpet Factory 504 Hand"

    let prof_lower = profession.to_lowercase();

    // Find "employ" and extract numbers after it
    let mut total = 0;
    let mut in_employ_section = false;

    for word in profession.split_whitespace() {
        let word_lower = word.to_lowercase();
        if word_lower.contains("employ") {
            in_employ_section = true;
            continue;
        }

        if in_employ_section {
            // Try to parse as number
            if let Ok(n) = word.parse::<i32>() {
                total += n;
            }
        }
    }

    total
}

fn normalize_address(addr: &str) -> String {
    addr.to_lowercase()
        .replace("street", "st")
        .replace("road", "rd")
        .replace("lane", "la")
        .replace("place", "pl")
        .replace("square", "sq")
        .replace(",", "")
        .replace(".", "")
        .replace("~", "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn addresses_match(a: &str, b: &str) -> bool {
    let na = normalize_address(a);
    let nb = normalize_address(b);

    if na.is_empty() || nb.is_empty() {
        return false;
    }

    if na == nb {
        return true;
    }

    if na.contains(&nb) || nb.contains(&na) {
        return true;
    }

    let a_parts: Vec<&str> = na.split_whitespace().collect();
    let b_parts: Vec<&str> = nb.split_whitespace().collect();

    let common_words = ["the", "and", "park", "house", "villa"];
    let significant_a: Vec<&str> = a_parts.iter()
        .filter(|p| p.len() >= 3 && !common_words.contains(p))
        .cloned()
        .collect();
    let significant_b: Vec<&str> = b_parts.iter()
        .filter(|p| p.len() >= 3 && !common_words.contains(p))
        .cloned()
        .collect();

    let matches = significant_a.iter()
        .filter(|p| significant_b.contains(p))
        .count();

    matches >= 2
}

fn names_match(first_name: &str, target_forename: &str) -> bool {
    let first_lower = first_name.to_lowercase();
    let target_lower = target_forename.to_lowercase().replace(".", "");

    // Exact match
    if first_lower == target_lower {
        return true;
    }

    // Initial match
    if let (Some(f), Some(t)) = (first_lower.chars().next(), target_lower.chars().next()) {
        if f == t {
            // Either is an initial (1-2 chars)
            if target_lower.len() <= 2 || first_lower.len() <= 2 {
                return true;
            }
            // First name starts with target
            if first_lower.starts_with(&target_lower) || target_lower.starts_with(&first_lower) {
                return true;
            }
        }
    }

    false
}

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;

    eprintln!("=== CREATING SHEFFIELD_BUSINESSMEN TABLE ===\n");

    // Create the table
    conn.execute("DROP TABLE IF EXISTS sheffield_businessmen", [])?;
    conn.execute(
        "CREATE TABLE sheffield_businessmen (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            surname TEXT NOT NULL,
            forename TEXT,
            home_address TEXT,
            work_address TEXT,
            business_type TEXT,
            source TEXT,
            person_id INTEGER,
            business_id INTEGER,
            FOREIGN KEY (person_id) REFERENCES sheffield_people(unique_id),
            FOREIGN KEY (business_id) REFERENCES sheffield_businesses(id)
        )",
        [],
    )?;

    // 1876 Cutlery Manufacturers from the table
    let cutlers_1876: Vec<(&str, &str, &str, &str)> = vec![
        ("Askam", "J.", "Osborne Villa, Ranmoor Park", "Broad Lane Works"),
        ("Baker", "J.", "29, Redhill", "Wheeldon Works"),
        ("Barnes", "V.", "144, Ecclesall Rd.", "103, Arundel St."),
        ("Barston", "J.", "Shorham St.", "Harwood Works"),
        ("Beardshaw", "G.", "39, Brunswick Rd.", "2, Marcus St."),
        ("Blyde", "Wm.", "118, Hanover St.", "96, Carver St."),
        ("Brooksbank", "A.", "Moor Lodge, Clarkehouse Rd.", "Malinda St. Works"),
        ("Burnand", "J.", "40, Leafygreave Rd.", "Leicester St."),
        ("Cantrell", "E.", "Shelburn Pl.", "68, Napier St."),
        ("Copley", "J.", "Carr Rd., Walkley", "Richmond Wks., Walkley"),
        ("Crossland", "J.", "Norfolk Rd.", "Eclipse Wks., Edward St."),
        ("Dawson", "W.", "25, Evans St.", "Pool Wks., Burgess St."),
        ("Elliott", "R.", "32, Chippinghouse Rd.", "151, Arundel St."),
        ("Epworth", "T.", "63, Highfield", "Truss Wks., New George St."),
        ("Greaves", "F.", "74, Upperthorpe", "Radford Wks., Radford St."),
        ("Hardy", "F.T.", "Norton", "Marsden's Wheel, Love St."),
        ("Haxton", "R.", "164, Carr Rd., Walkley", ""),
        ("Holmes", "T.", "Scotland St.", "Scotland St."),
        ("Hunter", "M.", "135, Scotland St.", "Andrew St."),
        ("Ibberson", "G.", "135, Scotland St.", "Central Wks., West St."),
        ("Masterton", "J.", "184, Witham Rd.", "Central Wks., West St."),
        ("Mosley", "R.", "", "Portland Wks., West St."),
        ("Nadin", "A.", "", "Court 8, Radford St."),
        ("Nowill", "H.", "Westbourne Rd.", "Central Wks., West St."),
        ("Nowill", "J.", "Westbourne Rd. East", "Central Wks., West St."),
        ("Paterson", "A.", "195, Ecclesall Rd.", "Forth Wks., Glossop Rd."),
        ("Peace", "W.K.", "Dam House, 237, Glossop Rd.", "Mowbray St."),
        ("Pearce", "H.K.", "Wadsley Bridge", "20B, West St."),
        ("Petty", "Jos.", "Netherthorpe St.", "58, Garden St."),
        ("Pryor", "M.", "69, Havelock Sq.", "Scotland St."),
        ("Renshaw", "T.", "76, Nether Edge Rd.", "32, Birkendale"),
        ("Renton", "G.", "41, Parkers Rd.", "Carver St."),
        ("Richardson", "W.", "Broomhall St.", "Broomhall St."),
        ("Roberts", "L.", "116, Broad La.", "Rockingham St."),
        ("Rowland", "L.", "Solly St.", "Solly St."),
        ("Ryalls", "J.", "73, William St.", "Solly St."),
        ("Scaife", "F.", "73, Eyre St.", "73, Eyre St."),
        ("Schofield", "J.", "101, Woodhead", "39, Broomspring La."),
        ("Shaw", "J.", "Orchard La.", "Orchard La."),
        ("Schemeld", "J.", "23, Broad La.", "47, Chester St."),
        ("Slinn", "W.", "Eagle House, Owlerton", "36, Thomas St."),
        ("Taylor", "H.H.", "Nicholson Rd., Heeley", "Times Wks., Paradise Sq."),
        ("Taylor", "W.", "1, Blake St.", "188, Rockingham St."),
        ("Townsend", "F.", "Solly St.", "Solly St."),
        ("Twigg", "F.", "25A, Owlerton Rd.", "25A, Owlerton Rd."),
        ("Watson", "G.", "2, Shorham St.", "25, Cornhill"),
        ("Webster", "W.", "", "Jessop St."),
        ("Whitham", "J.", "Cambridge St.", ""),
        ("Wragg", "W.", "118, Cemetery Rd.", ""),
    ];

    eprintln!("Adding {} cutlery manufacturers from 1876...", cutlers_1876.len());

    for (surname, forename, home_addr, work_addr) in &cutlers_1876 {
        conn.execute(
            "INSERT INTO sheffield_businessmen (surname, forename, home_address, work_address, business_type, source)
             VALUES (?1, ?2, ?3, ?4, 'Cutlery Manufacturer', '1876 Cutlery Manufacturers Table')",
            rusqlite::params![surname, forename, home_addr, work_addr],
        )?;
    }

    // Find people from sheffield_people who employ 10+ people
    eprintln!("\nFinding employers with 10+ employees from sheffield_people...");

    let mut stmt = conn.prepare(
        "SELECT unique_id, first_name, surname, profession, street_address
         FROM sheffield_people
         WHERE profession LIKE '%employ%'"
    )?;

    let mut rows = stmt.query([])?;
    let mut large_employers: Vec<(i64, String, String, String, String)> = Vec::new();

    while let Some(row) = rows.next()? {
        let uid: i64 = row.get(0)?;
        let first: String = row.get::<_, Option<String>>(1)?.unwrap_or_default();
        let surname: String = row.get::<_, Option<String>>(2)?.unwrap_or_default();
        let profession: String = row.get::<_, Option<String>>(3)?.unwrap_or_default();
        let address: String = row.get::<_, Option<String>>(4)?.unwrap_or_default();

        // Parse number of employees from profession
        // Look for patterns like "Employing 30 Men" or "Employing 10 Man"
        let prof_lower = profession.to_lowercase();
        if prof_lower.contains("unemploy") {
            continue; // Skip unemployed people
        }

        // Extract numbers after "employ"
        let total_employees = extract_employee_count(&profession);
        if total_employees >= 10 {
            large_employers.push((uid, first, surname, profession, address));
        }
    }
    drop(rows);
    drop(stmt);

    eprintln!("Found {} employers with 10+ employees", large_employers.len());

    for (uid, forename, surname, profession, address) in &large_employers {
        // Check if already exists (from cutlers)
        let exists: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sheffield_businessmen
             WHERE LOWER(surname) = LOWER(?1) AND LOWER(forename) = LOWER(?2)",
            rusqlite::params![surname, forename],
            |row| row.get(0)
        )?;

        if exists == 0 {
            conn.execute(
                "INSERT INTO sheffield_businessmen (surname, forename, home_address, business_type, source, person_id)
                 VALUES (?1, ?2, ?3, ?4, 'Census 1867 (10+ employees)', ?5)",
                rusqlite::params![surname, forename, address, profession, uid],
            )?;
            eprintln!("  Added: {} {} - {} @ {}", forename, surname, profession, address);
        }
    }

    // Count total
    let total: i64 = conn.query_row("SELECT COUNT(*) FROM sheffield_businessmen", [], |r| r.get(0))?;
    eprintln!("\nTotal businessmen in table: {}", total);

    // Now match to sheffield_people
    eprintln!("\n=== MATCHING BUSINESSMEN TO SHEFFIELD_PEOPLE ===\n");

    // Load all people into memory for faster matching
    let mut people: HashMap<String, Vec<(i64, String, String, String, String)>> = HashMap::new();
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
            people.entry(key).or_default().push((uid, first, surname, addr, prof));
        }
    }

    // Get all businessmen to match
    let mut to_match: Vec<(i64, String, String, String, String)> = Vec::new();
    {
        let mut stmt = conn.prepare(
            "SELECT id, surname, forename, home_address, work_address FROM sheffield_businessmen"
        )?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let id: i64 = row.get(0)?;
            let surname: String = row.get(1)?;
            let forename: String = row.get::<_, Option<String>>(2)?.unwrap_or_default();
            let home: String = row.get::<_, Option<String>>(3)?.unwrap_or_default();
            let work: String = row.get::<_, Option<String>>(4)?.unwrap_or_default();
            to_match.push((id, surname, forename, home, work));
        }
    }

    let mut matched = 0;
    let mut unmatched = 0;

    for (bm_id, surname, forename, home_addr, work_addr) in &to_match {
        let key = surname.to_lowercase();

        if let Some(candidates) = people.get(&key) {
            // Try to find a match by name + address
            let mut best_match: Option<i64> = None;

            for (uid, first, _sur, addr, _prof) in candidates {
                // Check forename
                if !forename.is_empty() && !names_match(first, forename) {
                    continue;
                }

                // Check address (try both home and work)
                let addr_matched = (!home_addr.is_empty() && addresses_match(home_addr, addr)) ||
                                  (!work_addr.is_empty() && addresses_match(work_addr, addr));

                if addr_matched {
                    best_match = Some(*uid);
                    break;
                }
            }

            // If no address match, try unique name match
            if best_match.is_none() && !forename.is_empty() {
                let name_matches: Vec<_> = candidates.iter()
                    .filter(|(_, first, _, _, _)| names_match(first, forename))
                    .collect();

                if name_matches.len() == 1 {
                    best_match = Some(name_matches[0].0);
                }
            }

            if let Some(person_id) = best_match {
                conn.execute(
                    "UPDATE sheffield_businessmen SET person_id = ?1 WHERE id = ?2",
                    [person_id, *bm_id],
                )?;
                matched += 1;
            } else {
                unmatched += 1;
            }
        } else {
            unmatched += 1;
        }
    }

    eprintln!("Matched: {}", matched);
    eprintln!("Unmatched: {}", unmatched);

    // Show some matches
    eprintln!("\n=== SAMPLE MATCHES ===\n");
    let mut stmt = conn.prepare(
        "SELECT bm.surname, bm.forename, bm.home_address, bm.business_type, bm.source,
                p.first_name, p.surname, p.street_address, p.profession
         FROM sheffield_businessmen bm
         JOIN sheffield_people p ON bm.person_id = p.unique_id
         LIMIT 20"
    )?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let bm_sur: String = row.get(0)?;
        let bm_fore: String = row.get::<_, Option<String>>(1)?.unwrap_or_default();
        let bm_addr: String = row.get::<_, Option<String>>(2)?.unwrap_or_default();
        let bm_type: String = row.get::<_, Option<String>>(3)?.unwrap_or_default();
        let source: String = row.get::<_, Option<String>>(4)?.unwrap_or_default();
        let p_first: String = row.get::<_, Option<String>>(5)?.unwrap_or_default();
        let p_sur: String = row.get::<_, Option<String>>(6)?.unwrap_or_default();
        let p_addr: String = row.get::<_, Option<String>>(7)?.unwrap_or_default();
        let p_prof: String = row.get::<_, Option<String>>(8)?.unwrap_or_default();

        eprintln!("{} {} ({}) -> {} {} @ {} [{}]",
                 bm_fore, bm_sur, bm_type, p_first, p_sur, p_addr, p_prof);
    }

    eprintln!("\n=== UNMATCHED BUSINESSMEN ===\n");
    let mut stmt = conn.prepare(
        "SELECT surname, forename, home_address, work_address, source
         FROM sheffield_businessmen
         WHERE person_id IS NULL
         LIMIT 20"
    )?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let sur: String = row.get(0)?;
        let fore: String = row.get::<_, Option<String>>(1)?.unwrap_or_default();
        let home: String = row.get::<_, Option<String>>(2)?.unwrap_or_default();
        let work: String = row.get::<_, Option<String>>(3)?.unwrap_or_default();
        let source: String = row.get::<_, Option<String>>(4)?.unwrap_or_default();
        eprintln!("{} {} @ {} / {} ({})", fore, sur, home, work, source);
    }

    Ok(())
}
