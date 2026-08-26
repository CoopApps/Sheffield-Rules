use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;

    eprintln!("=== CREATING SHEFFIELD_CLERGY TABLE ===\n");

    // Create table
    conn.execute("DROP TABLE IF EXISTS sheffield_clergy", [])?;
    conn.execute(
        "CREATE TABLE sheffield_clergy (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            person_id INTEGER NOT NULL,
            first_name TEXT,
            surname TEXT,
            profession TEXT,
            denomination TEXT,
            street_address TEXT,
            FOREIGN KEY (person_id) REFERENCES sheffield_people(unique_id)
        )",
        [],
    )?;

    // Search for clergy in sheffield_people
    let clergy_terms = [
        "vicar", "curate", "minister", "rector", "reverend", "rev ",
        "clergyman", "clergy", "pastor", "chaplain", "preacher",
        "priest", "deacon", "bishop", "canon", "wesleyan", "methodist",
        "baptist", "congregational", "primitive methodist", "church of england",
        "dissenting", "nonconformist", "incumbent", "missionary",
    ];

    let mut stmt = conn.prepare(
        "SELECT unique_id, first_name, surname, profession, street_address
         FROM sheffield_people
         WHERE profession IS NOT NULL AND profession != ''"
    )?;

    let mut rows = stmt.query([])?;
    let mut clergy: Vec<(i64, String, String, String, String, String)> = Vec::new();

    while let Some(row) = rows.next()? {
        let uid: i64 = row.get(0)?;
        let first: String = row.get::<_, Option<String>>(1)?.unwrap_or_default();
        let surname: String = row.get::<_, Option<String>>(2)?.unwrap_or_default();
        let prof: String = row.get::<_, Option<String>>(3)?.unwrap_or_default();
        let addr: String = row.get::<_, Option<String>>(4)?.unwrap_or_default();

        let prof_lower = prof.to_lowercase();

        // Check if this is clergy
        let is_clergy = clergy_terms.iter().any(|term| prof_lower.contains(term));

        if is_clergy {
            // Try to determine denomination
            let denomination = if prof_lower.contains("wesleyan") {
                "Wesleyan Methodist"
            } else if prof_lower.contains("primitive methodist") {
                "Primitive Methodist"
            } else if prof_lower.contains("methodist") {
                "Methodist"
            } else if prof_lower.contains("baptist") {
                "Baptist"
            } else if prof_lower.contains("congregational") {
                "Congregational"
            } else if prof_lower.contains("church of england") || prof_lower.contains("c of e") {
                "Church of England"
            } else if prof_lower.contains("roman catholic") || prof_lower.contains("catholic") {
                "Roman Catholic"
            } else if prof_lower.contains("dissenting") || prof_lower.contains("nonconformist") {
                "Nonconformist"
            } else if prof_lower.contains("quaker") || prof_lower.contains("friends") {
                "Quaker"
            } else if prof_lower.contains("unitarian") {
                "Unitarian"
            } else {
                ""
            };

            clergy.push((uid, first, surname, prof, denomination.to_string(), addr));
        }
    }

    eprintln!("Found {} clergy members\n", clergy.len());

    // Insert clergy
    for (uid, first, surname, prof, denom, addr) in &clergy {
        conn.execute(
            "INSERT INTO sheffield_clergy (person_id, first_name, surname, profession, denomination, street_address)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![uid, first, surname, prof, denom, addr],
        )?;
    }

    // Show sample
    eprintln!("=== SAMPLE CLERGY ===\n");
    let mut stmt = conn.prepare(
        "SELECT person_id, first_name, surname, profession, denomination, street_address
         FROM sheffield_clergy LIMIT 30"
    )?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let uid: i64 = row.get(0)?;
        let first: String = row.get::<_, Option<String>>(1)?.unwrap_or_default();
        let surname: String = row.get::<_, Option<String>>(2)?.unwrap_or_default();
        let prof: String = row.get::<_, Option<String>>(3)?.unwrap_or_default();
        let denom: String = row.get::<_, Option<String>>(4)?.unwrap_or_default();
        let addr: String = row.get::<_, Option<String>>(5)?.unwrap_or_default();
        eprintln!("[{}] {} {} - {} ({}) @ {}", uid, first, surname, prof, denom, addr);
    }

    // Summary by denomination
    eprintln!("\n=== BY DENOMINATION ===\n");
    let mut stmt = conn.prepare(
        "SELECT denomination, COUNT(*) as cnt FROM sheffield_clergy
         GROUP BY denomination ORDER BY cnt DESC"
    )?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let denom: String = row.get::<_, Option<String>>(0)?.unwrap_or_else(|| "Unknown".to_string());
        let cnt: i64 = row.get(1)?;
        eprintln!("{}: {}", if denom.is_empty() { "Unknown" } else { &denom }, cnt);
    }

    let total: i64 = conn.query_row("SELECT COUNT(*) FROM sheffield_clergy", [], |r| r.get(0))?;
    eprintln!("\n=== TOTAL: {} clergy members ===", total);

    Ok(())
}
