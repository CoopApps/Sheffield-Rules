use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    println!("=== FAST BULK COPY TO SHEFFIELD1867C.DB ===\n");

    // Open source database
    println!("Step 1: Opening source database...");
    let source_conn = Connection::open("Sheffield1867_backup_20260305_123217.db")?;
    println!("  ✓ Opened\n");

    // Delete old Sheffield1867c.db if it exists
    println!("Step 2: Removing old Sheffield1867c.db if exists...");
    let _ = std::fs::remove_file("Sheffield1867c.db");
    println!("  ✓ Ready\n");

    // Create new clean database
    println!("Step 3: Creating fresh Sheffield1867c.db...");
    let dest_conn = Connection::open("Sheffield1867c.db")?;
    dest_conn.execute("PRAGMA journal_mode=WAL", [])?;
    dest_conn.execute("PRAGMA synchronous=NORMAL", [])?;
    println!("  ✓ Created\n");

    // Get the schema
    println!("Step 4: Getting schema...");
    let schema: String = source_conn.query_row(
        "SELECT sql FROM sqlite_master WHERE type='table' AND name='sheffield_people'",
        [],
        |row| row.get(0),
    )?;
    println!("  ✓ Schema retrieved\n");

    // Create table in new db
    println!("Step 5: Creating sheffield_people table...");
    dest_conn.execute(&schema, [])?;
    println!("  ✓ Table created\n");

    // Get column names (excluding id)
    println!("Step 6: Getting column list...");
    let mut stmt = source_conn.prepare(
        "SELECT name FROM pragma_table_info('sheffield_people') WHERE name != 'id' ORDER BY cid"
    )?;
    let columns: Vec<String> = stmt
        .query_map([], |row| row.get(0))?
        .collect::<Result<Vec<String>>>()?;

    let column_list = columns.join(", ");
    println!("  ✓ Found {} columns (excluding id)\n", columns.len());

    // Try bulk copy using INSERT INTO SELECT with ROW_NUMBER
    println!("Step 7: Attempting fast bulk copy with 6-digit IDs...");

    // Attach the destination database
    source_conn.execute("ATTACH DATABASE 'Sheffield1867c.db' AS dest", [])?;

    // Try the bulk insert - if it fails due to corruption, we'll fall back
    let result = source_conn.execute(
        &format!(
            "INSERT INTO dest.sheffield_people (id, {})
             SELECT
               100000 + (ROW_NUMBER() OVER (ORDER BY ROWID) - 1) as id,
               {}
             FROM sheffield_people",
            column_list, column_list
        ),
        [],
    );

    match result {
        Ok(copied) => {
            println!("  ✓ Copied {} rows with bulk insert!\n", copied);
        }
        Err(e) => {
            println!("  ✗ Bulk copy failed: {}", e);
            println!("  Database corruption prevents bulk operations.");
            println!("  Try using sqlite3 to dump and restore the data.\n");
            return Err(e);
        }
    }

    source_conn.execute("DETACH DATABASE dest", [])?;

    // Verify
    println!("Step 8: Verifying...");
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

    let unique_ids: i64 = dest_conn.query_row(
        "SELECT COUNT(DISTINCT id) FROM sheffield_people",
        [],
        |row| row.get(0),
    )?;

    let min_id: i64 = dest_conn.query_row(
        "SELECT MIN(id) FROM sheffield_people",
        [],
        |row| row.get(0),
    )?;

    let max_id: i64 = dest_conn.query_row(
        "SELECT MAX(id) FROM sheffield_people",
        [],
        |row| row.get(0),
    )?;

    println!("  Total rows: {}", dest_count);
    println!("  Rows with IDs: {}", dest_with_ids);
    println!("  Unique IDs: {}", unique_ids);
    println!("  ID range: {} to {}", min_id, max_id);

    if unique_ids == dest_with_ids && dest_with_ids == dest_count {
        println!("\n  ✓ SUCCESS! All rows have unique 6-digit IDs\n");
    } else {
        println!("\n  ✗ WARNING: Some inconsistency detected\n");
    }

    println!("=== SHEFFIELD1867C.DB CREATED SUCCESSFULLY ===");

    Ok(())
}
