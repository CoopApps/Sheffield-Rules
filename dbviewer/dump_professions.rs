use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;

    // Search for "employ" in sheffield_people profession
    let mut stmt = conn.prepare(
        "SELECT unique_id, first_name, surname, profession, street_address
         FROM sheffield_people
         WHERE profession LIKE '%employ%'
         LIMIT 100"
    )?;
    let mut rows = stmt.query([])?;

    eprintln!("=== PEOPLE WITH 'EMPLOY' IN PROFESSION ===\n");
    while let Some(row) = rows.next()? {
        let id: i64 = row.get(0)?;
        let first: String = row.get::<_, Option<String>>(1)?.unwrap_or_default();
        let surname: String = row.get::<_, Option<String>>(2)?.unwrap_or_default();
        let prof: String = row.get::<_, Option<String>>(3)?.unwrap_or_default();
        let addr: String = row.get::<_, Option<String>>(4)?.unwrap_or_default();
        println!("{}: {} {} - [{}] @ {}", id, first, surname, prof, addr);
    }

    Ok(())
}
