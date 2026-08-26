use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("hold/Sheffield1867b.db")?;

    // Check if sheffield_clubs exists
    let exists: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='sheffield_clubs'",
        [],
        |r| r.get(0),
    )?;

    if exists == 0 {
        eprintln!("sheffield_clubs table does not exist");
        return Ok(());
    }

    // Show columns
    eprintln!("=== COLUMNS ===\n");
    let mut stmt = conn.prepare("PRAGMA table_info(sheffield_clubs)")?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let name: String = row.get(1)?;
        let dtype: String = row.get(2)?;
        eprintln!("{}: {}", name, dtype);
    }

    // Show all clubs
    eprintln!("\n=== ALL CLUBS ===\n");
    let mut stmt = conn.prepare("SELECT * FROM sheffield_clubs")?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let id: i64 = row.get(0)?;
        let name: String = row.get::<_, Option<String>>(1)?.unwrap_or_default();
        eprintln!("[{}] {}", id, name);
    }

    Ok(())
}
