use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    println!("=== COPYING DATA TO SHEFFIELD1867C.DB ===\n");

    // Open source database
    let source_conn = Connection::open("Sheffield1867b.db")?;

    // Create new clean database
    println!("Step 1: Creating new clean database (Sheffield1867c.db)...");
    let dest_conn = Connection::open("Sheffield1867c.db")?;
    println!("  ✓ Created\n");

    // Get the schema for sheffield_people
    println!("Step 2: Getting schema from sheffield_people...");
    let schema: String = source_conn.query_row(
        "SELECT sql FROM sqlite_master WHERE type='table' AND name='sheffield_people'",
        [],
        |row| row.get(0),
    )?;
    println!("  ✓ Schema retrieved\n");

    // Create the table in the new database
    println!("Step 3: Creating sheffield_people table in new database...");
    dest_conn.execute(&schema, [])?;
    println!("  ✓ Table created\n");

    // Copy all data
    println!("Step 4: Copying all data from Sheffield1867b.db...");
    source_conn.execute("ATTACH DATABASE 'Sheffield1867c.db' AS dest", [])?;

    let copied = source_conn.execute(
        "INSERT INTO dest.sheffield_people SELECT * FROM sheffield_people",
        [],
    )?;
    println!("  ✓ Copied {} rows\n", copied);

    source_conn.execute("DETACH DATABASE dest", [])?;

    // Verify
    println!("Step 5: Verifying...");
    let dest_count: i64 = dest_conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people",
        [],
        |row| row.get(0),
    )?;

    let dest_with_ids: i64 = dest_conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people WHERE id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;

    println!("  Total rows in Sheffield1867c.db: {}", dest_count);
    println!("  Rows with IDs: {}", dest_with_ids);

    if dest_count > 0 {
        let min_id: Result<i64> = dest_conn.query_row(
            "SELECT MIN(id) FROM sheffield_people WHERE id IS NOT NULL",
            [],
            |row| row.get(0),
        );

        let max_id: Result<i64> = dest_conn.query_row(
            "SELECT MAX(id) FROM sheffield_people WHERE id IS NOT NULL",
            [],
            |row| row.get(0),
        );

        if let (Ok(min), Ok(max)) = (min_id, max_id) {
            println!("  ID range: {} to {}", min, max);
        }
    }

    println!("\n=== SHEFFIELD1867C.DB CREATED SUCCESSFULLY ===");

    Ok(())
}
