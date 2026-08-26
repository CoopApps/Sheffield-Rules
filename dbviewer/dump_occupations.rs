use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;

    let mut stmt = conn.prepare("SELECT DISTINCT occupation FROM sheffield_businesses WHERE occupation IS NOT NULL AND occupation != '' ORDER BY occupation")?;
    let mut rows = stmt.query([])?;

    while let Some(row) = rows.next()? {
        let occ: String = row.get(0)?;
        println!("{}", occ);
    }

    Ok(())
}
