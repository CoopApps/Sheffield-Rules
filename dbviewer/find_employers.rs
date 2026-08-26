use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;

    // Check sheffield_people for "employs" in profession
    eprintln!("=== SEARCHING FOR 'EMPLOYS' IN SHEFFIELD_PEOPLE PROFESSION ===\n");

    let mut stmt = conn.prepare(
        "SELECT unique_id, first_name, surname, profession, street_address
         FROM sheffield_people
         WHERE profession LIKE '%employ%' OR profession LIKE '%men%' OR profession LIKE '%hands%'
         LIMIT 50"
    )?;

    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let id: i64 = row.get(0)?;
        let first: String = row.get::<_, Option<String>>(1)?.unwrap_or_default();
        let surname: String = row.get::<_, Option<String>>(2)?.unwrap_or_default();
        let prof: String = row.get::<_, Option<String>>(3)?.unwrap_or_default();
        let addr: String = row.get::<_, Option<String>>(4)?.unwrap_or_default();
        eprintln!("{}: {} {} - [{}] @ {}", id, first, surname, prof, addr);
    }

    eprintln!("\n=== SEARCHING FOR EMPLOYEE-RELATED TERMS IN SHEFFIELD_BUSINESSES ===\n");

    // Search all text columns for various employee-related terms
    let mut stmt = conn.prepare(
        "SELECT id, surname, forename, occupation, address
         FROM sheffield_businesses
         WHERE occupation LIKE '%men%' OR occupation LIKE '%hand%' OR occupation LIKE '%worker%'
            OR occupation LIKE '%boy%' OR occupation LIKE '%person%' OR occupation LIKE '%staff%'
            OR occupation LIKE '% 10 %' OR occupation LIKE '% 20 %' OR occupation LIKE '% 50 %'
            OR occupation LIKE '% 100 %' OR occupation LIKE '%employ%'
         LIMIT 100"
    )?;

    let mut rows = stmt.query([])?;
    let mut count = 0;

    while let Some(row) = rows.next()? {
        let id: i64 = row.get(0)?;
        let surname: String = row.get::<_, Option<String>>(1)?.unwrap_or_default();
        let forename: String = row.get::<_, Option<String>>(2)?.unwrap_or_default();
        let occupation: String = row.get::<_, Option<String>>(3)?.unwrap_or_default();
        let address: String = row.get::<_, Option<String>>(4)?.unwrap_or_default();

        eprintln!("{}: {} {} - {} @ {}", id, forename, surname, occupation, address);
        count += 1;
    }

    eprintln!("\nFound {} entries", count);

    // Show ALL columns in the table
    eprintln!("\n=== TABLE SCHEMA ===\n");
    let mut stmt = conn.prepare("PRAGMA table_info(sheffield_businesses)")?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let name: String = row.get(1)?;
        let dtype: String = row.get(2)?;
        eprintln!("  {} ({})", name, dtype);
    }

    // Show a few full rows
    eprintln!("\n=== SAMPLE FULL ROWS ===\n");
    let mut stmt = conn.prepare("SELECT * FROM sheffield_businesses LIMIT 5")?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        eprintln!("{:?}", (0..9).map(|i| row.get::<_, Option<String>>(i).ok().flatten().unwrap_or_default()).collect::<Vec<_>>());
    }

    Ok(())
}
