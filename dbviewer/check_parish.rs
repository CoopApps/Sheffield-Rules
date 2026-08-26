use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867_rescued.db")?;

    println!("=== CHECKING ECCLESIASTICAL PARISH ===\n");

    let total: i64 = conn.query_row("SELECT COUNT(*) FROM sheffield_people", [], |r| r.get(0))?;
    let with_parish: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people WHERE ecclesiastical_parish IS NOT NULL AND ecclesiastical_parish != ''",
        [],
        |r| r.get(0)
    )?;
    let without_parish: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people WHERE ecclesiastical_parish IS NULL OR ecclesiastical_parish = ''",
        [],
        |r| r.get(0)
    )?;

    println!("Total people: {}", total);
    println!("With ecclesiastical_parish: {}", with_parish);
    println!("Without ecclesiastical_parish: {}", without_parish);

    if without_parish > 0 {
        println!("\nSample people without parish:");
        let mut stmt = conn.prepare(
            "SELECT unique_id, name, street_address FROM sheffield_people WHERE ecclesiastical_parish IS NULL OR ecclesiastical_parish = '' LIMIT 5"
        )?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let uid: i64 = row.get(0)?;
            let name: Option<String> = row.get(1)?;
            let addr: Option<String> = row.get(2)?;
            println!("  {} - {:?} - {:?}", uid, name, addr);
        }
    }

    Ok(())
}
