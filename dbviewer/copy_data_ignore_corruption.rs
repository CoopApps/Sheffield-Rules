use rusqlite::{Connection, Result, params};

fn main() -> Result<()> {
    println!("=== COPYING DATA TO SHEFFIELD1867C.DB (IGNORING ROWID CORRUPTION) ===\n");

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

    // Get column names
    println!("Step 6: Getting column list...");
    let mut stmt = source_conn.prepare(
        "SELECT name FROM pragma_table_info('sheffield_people') WHERE name != 'id' ORDER BY cid"
    )?;
    let columns: Vec<String> = stmt
        .query_map([], |row| row.get(0))?
        .collect::<Result<Vec<String>>>()?;

    let column_count = columns.len();
    println!("  ✓ Found {} columns (excluding id)\n", column_count);

    // Copy data row by row, assigning new 6-digit IDs
    println!("Step 7: Copying data row by row with new 6-digit IDs...");

    let column_list = columns.join(", ");
    let select_query = format!("SELECT {} FROM sheffield_people", column_list);

    let placeholders: Vec<String> = (0..=column_count).map(|_| "?".to_string()).collect();
    let insert_query = format!(
        "INSERT INTO sheffield_people (id, {}) VALUES ({})",
        column_list,
        placeholders.join(", ")
    );

    let mut select_stmt = source_conn.prepare(&select_query)?;
    let mut rows = select_stmt.query([])?;

    let mut copied = 0;
    let mut new_id = 100000i64;

    while let Some(row) = rows.next()? {
        // Get all column values
        let mut values: Vec<rusqlite::types::Value> = vec![rusqlite::types::Value::Integer(new_id)];

        for i in 0..column_count {
            values.push(row.get(i)?);
        }

        // Insert into destination
        dest_conn.execute(&insert_query, rusqlite::params_from_iter(values.iter()))?;

        copied += 1;
        new_id += 1;

        if copied % 10000 == 0 {
            println!("  ... copied {} rows", copied);
        }
    }

    println!("  ✓ Copied {} rows with 6-digit IDs\n", copied);

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
