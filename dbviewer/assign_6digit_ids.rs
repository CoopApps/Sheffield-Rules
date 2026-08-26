use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867b.db")?;

    println!("=== ASSIGNING 6-DIGIT IDS ===\n");

    // Check current state
    let people_total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people",
        [],
        |row| row.get(0),
    )?;

    println!("Total people in table: {}\n", people_total);

    if people_total > 900000 {
        println!("ERROR: Too many people ({}) for 6-digit IDs (max 900000)", people_total);
        return Ok(());
    }

    // Clear all existing IDs
    println!("Step 1: Clearing all existing IDs...");
    conn.execute("UPDATE sheffield_people SET id = NULL", [])?;
    println!("  ✓ Cleared\n");

    // Assign 6-digit IDs starting from 100000
    println!("Step 2: Assigning 6-digit IDs (starting from 100000)...");

    // Get all ROWIDs in order
    let mut stmt = conn.prepare("SELECT ROWID FROM sheffield_people ORDER BY ROWID")?;
    let rowids: Vec<i64> = stmt
        .query_map([], |row| row.get(0))?
        .collect::<Result<Vec<i64>>>()?;

    println!("  Found {} rows to assign IDs to", rowids.len());

    // Assign IDs starting from 100000
    let mut assigned = 0;
    for (index, rowid) in rowids.iter().enumerate() {
        let new_id = 100000 + index as i64;
        conn.execute(
            "UPDATE sheffield_people SET id = ?1 WHERE ROWID = ?2",
            [new_id, *rowid],
        )?;
        assigned += 1;

        if assigned % 10000 == 0 {
            println!("  ... assigned {} IDs", assigned);
        }
    }
    println!("  ✓ Assigned {} IDs\n", assigned);

    // Verify
    println!("Step 3: Verifying...");
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
        println!("\n  ✓ SUCCESS! All {} people have unique 6-digit IDs\n", people_total);
    } else {
        println!("\n  ✗ PROBLEM: unique_ids={}, people_with_ids={}, people_total={}\n",
                 unique_ids, people_with_ids, people_total);
    }

    println!("=== COMPLETE ===");

    Ok(())
}
