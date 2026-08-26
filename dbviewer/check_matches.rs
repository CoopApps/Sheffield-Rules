use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867_rescued.db")?;

    let total: i64 = conn.query_row("SELECT COUNT(*) FROM sheffield_businesses", [], |r| r.get(0))?;
    let matched: i64 = conn.query_row("SELECT COUNT(*) FROM sheffield_businesses WHERE sheffield_person_id IS NOT NULL", [], |r| r.get(0))?;

    eprintln!("Total businesses: {}", total);
    eprintln!("Matched: {} ({:.1}%)", matched, (matched as f64 / total as f64) * 100.0);
    eprintln!("Unmatched: {}", total - matched);

    if matched > 0 {
        eprintln!("\nSample matches:");
        let mut stmt = conn.prepare(
            "SELECT b.forename, b.surname, b.occupation, b.address, p.first_name, p.surname, p.unique_id
             FROM sheffield_businesses b
             JOIN sheffield_people p ON b.sheffield_person_id = p.unique_id
             LIMIT 10"
        )?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let bf: String = row.get(0)?;
            let bs: String = row.get(1)?;
            let occ: String = row.get(2)?;
            let addr: String = row.get(3)?;
            let pf: String = row.get(4)?;
            let ps: String = row.get(5)?;
            let uid: i64 = row.get(6)?;
            eprintln!("  {} {} ({}) @ {} -> {} {} (id {})", bf, bs, occ, addr, pf, ps, uid);
        }
    }

    Ok(())
}
