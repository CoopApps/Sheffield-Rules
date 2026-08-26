use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    println!("=== CLUBS IN SHEFFIELD1867B.DB ===\n");

    let conn = Connection::open("Sheffield1867.db")?;

    let count: i64 = conn.query_row("SELECT COUNT(*) FROM sheffield_clubs", [], |r| r.get(0))?;
    println!("Total clubs: {}\n", count);

    let mut stmt = conn.prepare("SELECT id, name FROM sheffield_clubs ORDER BY name")?;
    let mut rows = stmt.query([])?;

    while let Some(row) = rows.next()? {
        let id: Option<String> = row.get(0)?;
        let name: Option<String> = row.get(1)?;
        println!("{:?} - {:?}", id, name);
    }

    Ok(())
}
