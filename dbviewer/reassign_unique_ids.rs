use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;

    println!("=== REASSIGNING UNIQUE IDS ===\n");

    // Check current state
    let people_total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people",
        [],
        |row| row.get(0),
    )?;

    println!("Total people in table: {}\n", people_total);

    // Clear all IDs
    println!("Step 1: Clearing all IDs...");
    conn.execute("UPDATE sheffield_people SET id = NULL", [])?;
    println!("  ✓ Cleared\n");

    // Create a new sequential ID for each row using a window function approach
    println!("Step 2: Assigning unique sequential IDs...");

    // SQLite doesn't have ROW_NUMBER() directly in older versions, so we'll use a different approach
    // We'll create a temporary table with proper sequential IDs, then update the main table

    conn.execute("DROP TABLE IF EXISTS temp_id_mapping", [])?;

    conn.execute(
        "CREATE TEMP TABLE temp_id_mapping AS
         SELECT
           ROWID as old_rowid,
           ROW_NUMBER() OVER (ORDER BY ROWID) as new_id,
           name
         FROM sheffield_people",
        [],
    )?;
    println!("  ✓ Created temporary ID mapping\n");

    // Now update sheffield_people with the new IDs
    println!("Step 3: Applying new IDs to sheffield_people...");
    let updated = conn.execute(
        "UPDATE sheffield_people
         SET id = (
           SELECT new_id FROM temp_id_mapping
           WHERE temp_id_mapping.old_rowid = sheffield_people.ROWID
         )",
        [],
    )?;
    println!("  ✓ Updated {} rows\n", updated);

    // Verify
    println!("Step 4: Verifying...");
    let people_with_ids: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people WHERE id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;

    let unique_ids: i64 = conn.query_row(
        "SELECT COUNT(DISTINCT id) FROM sheffield_people WHERE id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;

    let min_id: i64 = conn.query_row(
        "SELECT MIN(id) FROM sheffield_people WHERE id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;

    let max_id: i64 = conn.query_row(
        "SELECT MAX(id) FROM sheffield_people WHERE id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;

    println!("  People with IDs: {}", people_with_ids);
    println!("  Unique IDs: {}", unique_ids);
    println!("  ID range: {} to {}", min_id, max_id);

    if unique_ids == people_with_ids && people_with_ids == people_total {
        println!("\n  ✓ SUCCESS! All {} people have unique IDs\n", people_total);
    } else {
        println!("\n  ✗ PROBLEM: unique_ids={}, people_with_ids={}, people_total={}\n",
                 unique_ids, people_with_ids, people_total);
    }

    // Clean up
    conn.execute("DROP TABLE IF EXISTS temp_id_mapping", [])?;

    println!("=== COMPLETE ===");

    Ok(())
}
