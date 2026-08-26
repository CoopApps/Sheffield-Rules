use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;

    let mut stmt = conn.prepare("PRAGMA table_info(sheffield_people)")?;
    let mut rows = stmt.query([])?;

    eprintln!("=== COLUMNS IN SHEFFIELD_PEOPLE ===\n");
    while let Some(row) = rows.next()? {
        let name: String = row.get(1)?;
        let dtype: String = row.get(2)?;
        eprintln!("{}: {}", name, dtype);
    }

    Ok(())
}
