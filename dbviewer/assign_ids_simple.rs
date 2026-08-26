use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let db_path = "Sheffield1867.db";
    let conn = Connection::open(db_path)?;

    println!("Assigning IDs to all people...");

    // Simple update with ROWID
    let changes = conn.execute(
        "UPDATE sheffield_people SET id = ROWID",
        [],
    )?;

    println!("✓ Assigned IDs to {} people", changes);

    // Verify
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people WHERE id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;

    println!("✓ Verified: {} people now have IDs", count);

    // Show sample
    println!("\nSample:");
    let mut stmt = conn.prepare("SELECT id, name FROM sheffield_people LIMIT 3")?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let id: i64 = row.get(0)?;
        let name: String = row.get(1)?;
        println!("  {} - {}", id, name);
    }

    Ok(())
}
