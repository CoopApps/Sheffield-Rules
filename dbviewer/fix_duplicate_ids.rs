use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;

    println!("=== FIXING DUPLICATE IDS ===\n");

    // First, check the current state
    let people_total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people",
        [],
        |row| row.get(0),
    )?;

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

    println!("Current state:");
    println!("  Total people: {}", people_total);
    println!("  People with IDs: {}", people_with_ids);
    println!("  Unique IDs: {}", unique_ids);
    println!("  Duplicates: {}\n", people_with_ids - unique_ids);

    // Clear all IDs and reassign sequentially
    println!("Clearing all IDs...");
    conn.execute("UPDATE sheffield_people SET id = NULL", [])?;

    println!("Assigning new sequential IDs based on ROWID...");
    let updated = conn.execute(
        "UPDATE sheffield_people SET id = ROWID",
        [],
    )?;
    println!("  ✓ Assigned {} IDs\n", updated);

    // Verify the fix
    let people_with_ids_after: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people WHERE id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;

    let unique_ids_after: i64 = conn.query_row(
        "SELECT COUNT(DISTINCT id) FROM sheffield_people WHERE id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;

    println!("After fix:");
    println!("  People with IDs: {}", people_with_ids_after);
    println!("  Unique IDs: {}", unique_ids_after);

    if unique_ids_after == people_with_ids_after && people_with_ids_after == people_total {
        println!("  ✓ SUCCESS! All IDs are now unique\n");
    } else {
        println!("  ✗ PROBLEM STILL EXISTS!\n");
    }

    println!("=== COMPLETE ===");

    Ok(())
}
