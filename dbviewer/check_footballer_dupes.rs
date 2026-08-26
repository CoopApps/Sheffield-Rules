use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;

    // Check for duplicate person_ids
    let dupe_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM (
            SELECT person_id FROM sheffield_footballers
            GROUP BY person_id HAVING COUNT(*) > 1
        )",
        [],
        |r| r.get(0),
    )?;

    eprintln!("Duplicate person_ids: {}", dupe_count);

    // Check total vs distinct
    let total: i64 = conn.query_row("SELECT COUNT(*) FROM sheffield_footballers", [], |r| r.get(0))?;
    let distinct: i64 = conn.query_row("SELECT COUNT(DISTINCT person_id) FROM sheffield_footballers", [], |r| r.get(0))?;

    eprintln!("Total rows: {}", total);
    eprintln!("Distinct person_ids: {}", distinct);

    Ok(())
}
