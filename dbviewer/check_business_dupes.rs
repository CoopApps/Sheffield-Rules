use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867_rescued.db")?;

    eprintln!("=== CHECKING FOR DUPLICATE BUSINESSES ===\n");

    let total: i64 = conn.query_row("SELECT COUNT(*) FROM sheffield_businesses", [], |r| r.get(0))?;
    eprintln!("Total rows: {}", total);

    let unique: i64 = conn.query_row(
        "SELECT COUNT(*) FROM (SELECT DISTINCT surname, forename, occupation, address FROM sheffield_businesses)",
        [],
        |r| r.get(0)
    )?;
    eprintln!("Unique (surname, forename, occupation, address): {}", unique);
    eprintln!("Duplicates: {}", total - unique);

    if total != unique {
        eprintln!("\nSample duplicates:");
        let mut stmt = conn.prepare(
            "SELECT surname, forename, occupation, address, COUNT(*) as cnt
             FROM sheffield_businesses
             GROUP BY surname, forename, occupation, address
             HAVING cnt > 1
             ORDER BY cnt DESC
             LIMIT 10"
        )?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let surname: String = row.get(0)?;
            let forename: String = row.get(1)?;
            let occupation: String = row.get(2)?;
            let address: String = row.get(3)?;
            let cnt: i64 = row.get(4)?;
            eprintln!("  {}x: {} {} - {} @ {}", cnt, forename, surname, occupation, address);
        }
    }

    Ok(())
}
