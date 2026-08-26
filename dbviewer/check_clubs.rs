use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;

    println!("Checking sheffield_clubs in Sheffield1867.db...");

    let count: i64 = conn.query_row("SELECT COUNT(*) FROM sheffield_clubs", [], |r| r.get(0))?;
    println!("Count: {}", count);

    let mut stmt = conn.prepare("SELECT id, name FROM sheffield_clubs LIMIT 5")?;
    let mut rows = stmt.query([])?;

    while let Some(row) = rows.next()? {
        let id: Option<String> = row.get(0)?;
        let name: Option<String> = row.get(1)?;
        println!("  {:?} - {:?}", id, name);
    }

    Ok(())
}
