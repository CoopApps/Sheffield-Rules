use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    println!("=== CREATING FRESH SHEFFIELD1867C.DB WITH 6-DIGIT IDS ===\n");

    // Open source database (using the good backup from 12:32 PM)
    println!("Step 1: Opening Sheffield1867_backup_20260305_123217.db...");
    let source_conn = Connection::open("Sheffield1867_backup_20260305_123217.db")?;
    println!("  ✓ Opened\n");

    // Delete old Sheffield1867c.db if it exists
    println!("Step 2: Removing old Sheffield1867c.db if exists...");
    let _ = std::fs::remove_file("Sheffield1867c.db");
    println!("  ✓ Ready\n");

    // Create new clean database
    println!("Step 3: Creating fresh Sheffield1867c.db...");
    let dest_conn = Connection::open("Sheffield1867c.db")?;
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

    // Copy data with 6-digit IDs assigned using ROW_NUMBER
    println!("Step 6: Copying data with 6-digit IDs (100000+)...");
    source_conn.execute("ATTACH DATABASE 'Sheffield1867c.db' AS dest", [])?;

    // First, get column names (excluding 'id' which we'll generate)
    let mut stmt = source_conn.prepare(
        "SELECT name FROM pragma_table_info('sheffield_people') WHERE name != 'id' ORDER BY cid"
    )?;
    let columns: Vec<String> = stmt
        .query_map([], |row| row.get(0))?
        .collect::<Result<Vec<String>>>()?;

    let column_list = columns.join(", ");

    // Use ROW_NUMBER to assign sequential 6-digit IDs starting from 100000
    let query = format!(
        "INSERT INTO dest.sheffield_people (id, {})
         SELECT
           100000 + (ROW_NUMBER() OVER (ORDER BY ROWID) - 1) as id,
           {}
         FROM sheffield_people",
        column_list, column_list
    );

    let copied = source_conn.execute(&query, [])?;
    println!("  ✓ Copied {} rows with 6-digit IDs\n", copied);

    source_conn.execute("DETACH DATABASE dest", [])?;

    // Verify
    println!("Step 7: Verifying...");
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
