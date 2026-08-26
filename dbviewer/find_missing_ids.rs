use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867_rescued.db")?;

    eprintln!("=== FINDING MISSING IDS ===\n");

    // Get all existing unique_ids
    let mut stmt = conn.prepare("SELECT unique_id FROM sheffield_people ORDER BY unique_id")?;
    let mut rows = stmt.query([])?;

    let mut existing: Vec<i64> = Vec::new();
    while let Some(row) = rows.next()? {
        existing.push(row.get(0)?);
    }

    let max_id = *existing.last().unwrap_or(&0);
    eprintln!("Total existing: {}", existing.len());
    eprintln!("Max ID: {}", max_id);

    // Find gaps
    let mut missing: Vec<i64> = Vec::new();
    let mut expected = 1i64;
    for &id in &existing {
        while expected < id {
            missing.push(expected);
            expected += 1;
        }
        expected = id + 1;
    }

    eprintln!("Missing IDs ({}):", missing.len());
    for id in &missing {
        eprintln!("  {}", id);
    }

    Ok(())
}
