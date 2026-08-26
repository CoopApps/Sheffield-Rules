use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;

    let with_ids: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people WHERE id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;

    println!("People with IDs: {}", with_ids);

    let mut stmt = conn.prepare("SELECT id, name FROM sheffield_people LIMIT 5")?;
    let mut rows = stmt.query([])?;

    println!("\nFirst 5 people:");
    while let Some(row) = rows.next()? {
        let id: Option<i64> = row.get(0)?;
        let name: String = row.get(1)?;
        println!("  ID: {:?}, Name: {}", id, name);
    }

    Ok(())
}
