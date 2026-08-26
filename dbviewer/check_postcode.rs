use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867_rescued.db")?;

    println!("=== CHECKING POSTCODE ===\n");

    let total: i64 = conn.query_row("SELECT COUNT(*) FROM sheffield_people", [], |r| r.get(0))?;
    let with_postcode: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people WHERE postcode IS NOT NULL AND postcode != ''",
        [],
        |r| r.get(0)
    )?;
    let without_postcode: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people WHERE postcode IS NULL OR postcode = ''",
        [],
        |r| r.get(0)
    )?;

    println!("Total people: {}", total);
    println!("With postcode: {}", with_postcode);
    println!("Without postcode: {}", without_postcode);

    if with_postcode > 0 {
        println!("\nSample people WITH postcode:");
        let mut stmt = conn.prepare(
            "SELECT unique_id, name, street_address, postcode FROM sheffield_people WHERE postcode IS NOT NULL AND postcode != '' LIMIT 10"
        )?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let uid: i64 = row.get(0)?;
            let name: Option<String> = row.get(1)?;
            let addr: Option<String> = row.get(2)?;
            let postcode: Option<String> = row.get(3)?;
            println!("  {} - {:?} - {:?} - {:?}", uid, name, addr, postcode);
        }
    }

    Ok(())
}
