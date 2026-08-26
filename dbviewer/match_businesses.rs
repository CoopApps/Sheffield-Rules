use rusqlite::{Connection, Result};
use std::collections::HashMap;

fn names_match(biz_forename: &str, person_first_name: &str) -> bool {
    // Split into parts (first name and possible middle name/initial)
    let biz_parts: Vec<&str> = biz_forename.split_whitespace().collect();
    let person_parts: Vec<&str> = person_first_name.split_whitespace().collect();

    if biz_parts.is_empty() || person_parts.is_empty() {
        return false;
    }

    // First name must match (exact or initial)
    let biz_first = biz_parts[0];
    let person_first = person_parts[0];

    let first_matches = biz_first == person_first ||
        (biz_first.len() <= 2 && !biz_first.is_empty() && !person_first.is_empty() &&
         biz_first.chars().next() == person_first.chars().next()) ||
        (person_first.len() <= 2 && !person_first.is_empty() && !biz_first.is_empty() &&
         person_first.chars().next() == biz_first.chars().next());

    if !first_matches {
        return false;
    }

    // If both have middle names/initials, they must match too
    if biz_parts.len() > 1 && person_parts.len() > 1 {
        let biz_mid = biz_parts[1];
        let person_mid = person_parts[1];

        let mid_matches = biz_mid == person_mid ||
            (biz_mid.len() <= 2 && !biz_mid.is_empty() && !person_mid.is_empty() &&
             biz_mid.chars().next() == person_mid.chars().next()) ||
            (person_mid.len() <= 2 && !person_mid.is_empty() && !biz_mid.is_empty() &&
             person_mid.chars().next() == biz_mid.chars().next());

        return mid_matches;
    }

    // If only one has a middle name, that's still a match (we matched the first name)
    true
}

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867_rescued.db")?;

    eprintln!("=== MATCHING BUSINESSES TO PEOPLE ===\n");

    // Load all people into memory
    eprintln!("Loading people...");
    let mut people_by_addr: HashMap<(String, String), Vec<(String, String, i64)>> = HashMap::new();
    let mut people_by_prof: HashMap<(String, String), Vec<(String, i64)>> = HashMap::new();
    {
        let mut stmt = conn.prepare("SELECT unique_id, surname, first_name, street_address, profession FROM sheffield_people")?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let uid: i64 = row.get(0)?;
            let surname: String = row.get::<_, Option<String>>(1)?.unwrap_or_default().to_lowercase();
            let first_name: String = row.get::<_, Option<String>>(2)?.unwrap_or_default().to_lowercase();
            let address: String = row.get::<_, Option<String>>(3)?.unwrap_or_default().to_lowercase();
            let profession: String = row.get::<_, Option<String>>(4)?.unwrap_or_default().to_lowercase();

            people_by_addr.entry((surname.clone(), address)).or_default().push((first_name.clone(), profession.clone(), uid));
            if !profession.is_empty() {
                people_by_prof.entry((surname, profession)).or_default().push((first_name, uid));
            }
        }
    }
    eprintln!("Loaded {} surname+address combos, {} surname+profession combos",
              people_by_addr.len(), people_by_prof.len());

    // Load all businesses
    eprintln!("Loading businesses...");
    let mut businesses: Vec<(i64, String, String, String, String)> = Vec::new();
    {
        let mut stmt = conn.prepare("SELECT id, surname, forename, address, occupation FROM sheffield_businesses")?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let id: i64 = row.get(0)?;
            let surname: String = row.get::<_, Option<String>>(1)?.unwrap_or_default().to_lowercase();
            let forename: String = row.get::<_, Option<String>>(2)?.unwrap_or_default().to_lowercase();
            let address: String = row.get::<_, Option<String>>(3)?.unwrap_or_default().to_lowercase();
            let occupation: String = row.get::<_, Option<String>>(4)?.unwrap_or_default().to_lowercase();
            businesses.push((id, surname, forename, address, occupation));
        }
    }
    eprintln!("Loaded {} businesses", businesses.len());

    // PASS 1: Match by surname + address + forename
    eprintln!("\nPass 1: Matching by surname + address + forename...");
    let mut matches: Vec<(i64, i64)> = Vec::new();
    let mut matched_ids: std::collections::HashSet<i64> = std::collections::HashSet::new();

    for (biz_id, surname, forename, address, _) in &businesses {
        let key = (surname.clone(), address.clone());
        if let Some(candidates) = people_by_addr.get(&key) {
            let matching: Vec<i64> = candidates.iter()
                .filter(|(first_name, _, _)| names_match(forename, first_name))
                .map(|(_, _, uid)| *uid)
                .collect();

            if matching.len() == 1 {
                matches.push((*biz_id, matching[0]));
                matched_ids.insert(*biz_id);
            }
        }
    }
    eprintln!("Pass 1: {} matches (by address)", matches.len());

    // PASS 2: For unmatched businesses, match by surname + profession + forename
    eprintln!("Pass 2: Matching remaining by surname + profession + forename...");
    let mut pass2_matches = 0;

    for (biz_id, surname, forename, _, occupation) in &businesses {
        if matched_ids.contains(biz_id) || occupation.is_empty() {
            continue;
        }

        let key = (surname.clone(), occupation.clone());
        if let Some(candidates) = people_by_prof.get(&key) {
            let matching: Vec<i64> = candidates.iter()
                .filter(|(first_name, _)| names_match(forename, first_name))
                .map(|(_, uid)| *uid)
                .collect();

            if matching.len() == 1 {
                matches.push((*biz_id, matching[0]));
                matched_ids.insert(*biz_id);
                pass2_matches += 1;
            }
        }
    }
    eprintln!("Pass 2: {} additional matches (by profession)", pass2_matches);

    eprintln!("\nTotal matches: {}", matches.len());

    // Update database
    eprintln!("Updating database...");
    conn.execute("UPDATE sheffield_businesses SET sheffield_person_id = NULL", [])?;

    conn.execute("BEGIN", [])?;
    let mut update_stmt = conn.prepare("UPDATE sheffield_businesses SET sheffield_person_id = ?1 WHERE id = ?2")?;
    for (biz_id, person_id) in &matches {
        update_stmt.execute(rusqlite::params![person_id, biz_id])?;
    }
    drop(update_stmt);
    conn.execute("COMMIT", [])?;

    // Stats
    let total = businesses.len();
    let matched = matches.len();
    eprintln!("\nTotal businesses: {}", total);
    eprintln!("With match: {} ({:.1}%)", matched, (matched as f64 / total as f64) * 100.0);
    eprintln!("Without match: {}", total - matched);

    // Sample matches
    eprintln!("\nSample matches:");
    let mut stmt = conn.prepare(
        "SELECT b.forename, b.surname, b.occupation, b.address, p.first_name, p.surname, p.profession, p.unique_id
         FROM sheffield_businesses b
         JOIN sheffield_people p ON b.sheffield_person_id = p.unique_id
         LIMIT 15"
    )?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let bf: String = row.get(0)?;
        let bs: String = row.get(1)?;
        let occ: String = row.get(2)?;
        let addr: String = row.get(3)?;
        let pf: String = row.get(4)?;
        let ps: String = row.get(5)?;
        let prof: String = row.get::<_, Option<String>>(6)?.unwrap_or_default();
        let uid: i64 = row.get(7)?;
        eprintln!("  {} {} [{}] @ {} -> {} {} [{}] (id {})", bf, bs, occ, addr, pf, ps, prof, uid);
    }

    Ok(())
}
