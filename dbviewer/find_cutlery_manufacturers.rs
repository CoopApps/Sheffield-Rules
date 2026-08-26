use rusqlite::{Connection, Result};

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

    // Exact match
    if na == nb {
        return true;
    }

    // One contains the other
    if na.contains(&nb) || nb.contains(&na) {
        return true;
    }

    // Check if key parts match (street name + number)
    let a_parts: Vec<&str> = na.split_whitespace().collect();
    let b_parts: Vec<&str> = nb.split_whitespace().collect();

    // Look for shared significant words (3+ chars, not common words)
    let common_words = ["the", "and", "park", "house", "villa"];
    let significant_a: Vec<&str> = a_parts.iter()
        .filter(|p| p.len() >= 3 && !common_words.contains(p))
        .cloned()
        .collect();
    let significant_b: Vec<&str> = b_parts.iter()
        .filter(|p| p.len() >= 3 && !common_words.contains(p))
        .cloned()
        .collect();

    // Count matches
    let matches = significant_a.iter()
        .filter(|p| significant_b.contains(p))
        .count();

    // If we have at least 2 matching significant words, consider it a match
    matches >= 2
}

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867_rescued.db")?;

    eprintln!("=== SEARCHING FOR CUTLERY MANUFACTURERS (FUZZY ADDRESS) ===\n");

    let manufacturers = vec![
        ("Askam", "J.", "Osborne Villa, Ranmoor Park"),
        ("Baker", "J.", "29, Redhill"),
        ("Barnes", "V.", "144, Ecclesall Rd."),
        ("Barston", "J.", "Shorham St."),
        ("Beardshaw", "G.", "39, Brunswick Rd."),
        ("Blyde", "Wm.", "118, Hanover St."),
        ("Brooksbank", "A.", "Moor Lodge, Clarkehouse Rd."),
        ("Burnand", "J.", "40, Leafygreave Rd."),
        ("Cantrell", "E.", "Shelburn Pl."),
        ("Copley", "J.", "Carr Rd., Walkley"),
        ("Crossland", "J.", "Norfolk Rd."),
        ("Dawson", "W.", "25, Evans St."),
        ("Elliott", "R.", "32, Chippinghouse Rd."),
        ("Epworth", "T.", "63, Highfield"),
        ("Greaves", "F.", "74, Upperthorpe"),
        ("Hardy", "F.T.", "Norton"),
        ("Haxton", "R.", "164, Carr Rd., Walkley"),
        ("Holmes", "T.", "Scotland St."),
        ("Hunter", "M.", "135, Scotland St."),
        ("Ibberson", "G.", "135, Scotland St."),
        ("Masterton", "J.", "184, Witham Rd."),
        ("Nowill", "H.", "Westbourne Rd."),
        ("Nowill", "J.", "Westbourne Rd. East"),
        ("Paterson", "A.", "195, Ecclesall Rd."),
        ("Peace", "W.K.", "Dam House, 237, Glossop Rd."),
        ("Pearce", "H.K.", "Wadsley Bridge"),
        ("Petty", "Jos.", "Netherthorpe St."),
        ("Pryor", "M.", "69, Havelock Sq."),
        ("Renshaw", "T.", "76, Nether Edge Rd."),
        ("Renton", "G.", "41, Parkers Rd."),
        ("Richardson", "W.", "Broomhall St."),
        ("Roberts", "L.", "116, Broad La."),
        ("Rowland", "L.", "Solly St."),
        ("Ryalls", "J.", "73, William St."),
        ("Scaife", "F.", "73, Eyre St."),
        ("Schofield", "J.", "101, Woodhead"),
        ("Shaw", "J.", "Orchard La."),
        ("Schemeld", "J.", "23, Broad La."),
        ("Slinn", "W.", "Eagle House, Owlerton"),
        ("Taylor", "H.H.", "Nicholson Rd., Heeley"),
        ("Taylor", "W.", "1, Blake St."),
        ("Townsend", "F.", "Solly St."),
        ("Twigg", "F.", "25A, Owlerton Rd."),
        ("Watson", "G.", "2, Shorham St."),
        ("Whitham", "J.", "Cambridge St."),
        ("Wragg", "W.", "118, Cemetery Rd."),
    ];

    let mut found = 0;
    let mut not_found = 0;

    for (surname, forename, address) in &manufacturers {
        let initial = forename.chars().next().unwrap_or(' ').to_lowercase().next().unwrap();

        let mut stmt = conn.prepare(
            "SELECT unique_id, first_name, surname, street_address, profession
             FROM sheffield_people
             WHERE LOWER(surname) = LOWER(?1)
             AND LOWER(SUBSTR(first_name, 1, 1)) = ?2"
        )?;

        let initial_str = initial.to_string();
        let mut rows = stmt.query(rusqlite::params![surname, initial_str])?;

        let mut all_matches: Vec<(i64, String, String, String, String)> = Vec::new();
        while let Some(row) = rows.next()? {
            let uid: i64 = row.get(0)?;
            let first: String = row.get::<_, Option<String>>(1)?.unwrap_or_default();
            let sur: String = row.get::<_, Option<String>>(2)?.unwrap_or_default();
            let addr: String = row.get::<_, Option<String>>(3)?.unwrap_or_default();
            let prof: String = row.get::<_, Option<String>>(4)?.unwrap_or_default();
            all_matches.push((uid, first, sur, addr, prof));
        }

        if all_matches.is_empty() {
            eprintln!("NOT FOUND: {} {} @ {}", forename, surname, address);
            not_found += 1;
            continue;
        }

        // Filter by address match
        let addr_matches: Vec<_> = all_matches.iter()
            .filter(|(_, _, _, a, _)| addresses_match(address, a))
            .collect();

        if addr_matches.len() == 1 {
            let m = addr_matches[0];
            eprintln!("ADDRESS MATCH: {} {} @ {} -> {} {} @ {} [{}] (id {})",
                     forename, surname, address, m.1, m.2, m.3, m.4, m.0);
            found += 1;
        } else if addr_matches.len() > 1 {
            eprintln!("MULTIPLE ADDR MATCHES ({}): {} {} @ {}", addr_matches.len(), forename, surname, address);
            for m in addr_matches.iter().take(5) {
                eprintln!("    -> {} {} @ {} [{}] (id {})", m.1, m.2, m.3, m.4, m.0);
            }
            not_found += 1;
        } else if all_matches.len() == 1 {
            let m = &all_matches[0];
            eprintln!("UNIQUE (no addr): {} {} @ {} -> {} {} @ {} [{}] (id {})",
                     forename, surname, address, m.1, m.2, m.3, m.4, m.0);
            found += 1;
        } else {
            // Multiple matches, no address match - look for cutlery-related professions
            let cutlery_matches: Vec<_> = all_matches.iter()
                .filter(|(_, _, _, _, p)| {
                    let pl = p.to_lowercase();
                    pl.contains("cutl") || pl.contains("knife") || pl.contains("blade") ||
                    pl.contains("fork") || pl.contains("manufacturer")
                })
                .collect();

            if cutlery_matches.len() == 1 {
                let m = cutlery_matches[0];
                eprintln!("CUTLERY MATCH: {} {} @ {} -> {} {} @ {} [{}] (id {})",
                         forename, surname, address, m.1, m.2, m.3, m.4, m.0);
                found += 1;
            } else if !cutlery_matches.is_empty() {
                eprintln!("MULTIPLE CUTLERY ({}): {} {} @ {}", cutlery_matches.len(), forename, surname, address);
                for m in cutlery_matches.iter().take(5) {
                    eprintln!("    -> {} {} @ {} [{}] (id {})", m.1, m.2, m.3, m.4, m.0);
                }
                not_found += 1;
            } else {
                eprintln!("NO MATCH ({}): {} {} @ {}", all_matches.len(), forename, surname, address);
                not_found += 1;
            }
        }
    }

    eprintln!("\n=== SUMMARY ===");
    eprintln!("Found: {}", found);
    eprintln!("Not found/multiple: {}", not_found);
    eprintln!("Total: {}", manufacturers.len());

    Ok(())
}
