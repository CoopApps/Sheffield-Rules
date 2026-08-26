use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;

    let terms = [
        "vicar", "curate", "rector", "incumbent", "clergyman",
        "reverend", "rev.", "m.a.", "d.d.", "b.a."
    ];

    for term in &terms {
        eprintln!("\n=== Searching for '{}' ===", term);
        let query = format!(
            "SELECT unique_id, first_name, surname, profession, street_address
             FROM sheffield_people
             WHERE LOWER(profession) LIKE '%{}%'
             LIMIT 10",
            term
        );
        let mut stmt = conn.prepare(&query)?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let uid: i64 = row.get(0)?;
            let first: String = row.get::<_, Option<String>>(1)?.unwrap_or_default();
            let surname: String = row.get::<_, Option<String>>(2)?.unwrap_or_default();
            let prof: String = row.get::<_, Option<String>>(3)?.unwrap_or_default();
            let addr: String = row.get::<_, Option<String>>(4)?.unwrap_or_default();
            eprintln!("[{}] {} {} - {} @ {}", uid, first, surname, prof, addr);
        }
    }

    Ok(())
}
