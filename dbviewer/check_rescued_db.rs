use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867_rescued.db")?;

    println!("=== CHECKING RESCUED DATABASE ===\n");

    // 1. Integrity check
    println!("1. Running integrity check...");
    let integrity: String = conn.query_row("PRAGMA integrity_check", [], |row| row.get(0))?;
    if integrity == "ok" {
        println!("   ✓ Database integrity: OK\n");
    } else {
        println!("   ✗ Database integrity: {}\n", integrity);
    }

    // 2. Check rowid
    println!("2. Checking rowid consistency...");
    let rowid_count: i64 = conn.query_row(
        "SELECT COUNT(DISTINCT rowid) FROM sheffield_people",
        [],
        |row| row.get(0)
    )?;
    let total_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people",
        [],
        |row| row.get(0)
    )?;

    println!("   Total rows: {}", total_count);
    println!("   Distinct rowids: {}", rowid_count);

    if rowid_count == total_count {
        println!("   ✓ All rowids are unique\n");
    } else {
        println!("   ✗ WARNING: rowid mismatch! Possible corruption\n");
    }

    // 3. Check unique_id
    println!("3. Checking unique_id...");
    let unique_id_count: i64 = conn.query_row(
        "SELECT COUNT(DISTINCT unique_id) FROM sheffield_people",
        [],
        |row| row.get(0)
    )?;

    println!("   Distinct unique_ids: {}", unique_id_count);

    if unique_id_count == total_count {
        println!("   ✓ All unique_ids are unique\n");
    } else {
        println!("   ✗ WARNING: unique_id duplicates found\n");
    }

    // 4. Check for gaps in unique_id
    println!("4. Checking for gaps in unique_id sequence...");
    let min_id: i64 = conn.query_row(
        "SELECT MIN(unique_id) FROM sheffield_people",
        [],
        |row| row.get(0)
    )?;
    let max_id: i64 = conn.query_row(
        "SELECT MAX(unique_id) FROM sheffield_people",
        [],
        |row| row.get(0)
    )?;

    let expected_count = max_id - min_id + 1;
    println!("   Range: {} to {} (expected {} rows)", min_id, max_id, expected_count);
    println!("   Actual rows: {}", total_count);

    if expected_count == total_count {
        println!("   ✓ No gaps in unique_id sequence\n");
    } else {
        println!("   ⚠ {} gaps detected in sequence\n", expected_count - total_count);
    }

    // 5. Compare rowid and unique_id
    println!("5. Checking if rowid matches unique_id...");
    let mismatches: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people WHERE rowid != unique_id",
        [],
        |row| row.get(0)
    )?;

    if mismatches == 0 {
        println!("   ✓ rowid and unique_id are identical\n");
    } else {
        println!("   ⓘ {} rows where rowid != unique_id (this is OK)\n", mismatches);
    }

    println!("=== CHECK COMPLETE ===");

    Ok(())
}
