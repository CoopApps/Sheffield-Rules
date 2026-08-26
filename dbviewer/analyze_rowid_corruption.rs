use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    println!("=== THOROUGH ROWID CORRUPTION ANALYSIS ===\n");

    // Try to open Sheffield1867.db
    println!("Step 1: Opening Sheffield1867.db...");
    let result = Connection::open("Sheffield1867.db");

    let conn = match result {
        Ok(c) => {
            println!("  ✓ Database opened\n");
            c
        },
        Err(e) => {
            println!("  ✗ Failed to open: {}\n", e);
            return Err(e);
        }
    };

    // Run integrity check
    println!("Step 2: Running PRAGMA integrity_check...");
    match conn.query_row("PRAGMA integrity_check", [], |row| row.get::<_, String>(0)) {
        Ok(result) => {
            if result == "ok" {
                println!("  ✓ Database integrity: OK\n");
            } else {
                println!("  ✗ Integrity issues found:");
                println!("{}\n", result);
            }
        },
        Err(e) => {
            println!("  ✗ Integrity check failed: {}\n", e);
        }
    }

    // Check ROWID statistics
    println!("Step 3: Analyzing ROWID distribution in sheffield_people...");

    // Count total rows
    let total_rows: Result<i64> = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people",
        [],
        |row| row.get(0)
    );

    match total_rows {
        Ok(count) => println!("  Total rows: {}", count),
        Err(e) => println!("  ✗ Failed to count rows: {}", e),
    }

    // Check ROWID range
    let min_rowid: Result<i64> = conn.query_row(
        "SELECT MIN(ROWID) FROM sheffield_people",
        [],
        |row| row.get(0)
    );

    let max_rowid: Result<i64> = conn.query_row(
        "SELECT MAX(ROWID) FROM sheffield_people",
        [],
        |row| row.get(0)
    );

    match (min_rowid, max_rowid) {
        (Ok(min), Ok(max)) => {
            println!("  ROWID range: {} to {}", min, max);
            println!("  Expected rows if sequential: {}", max - min + 1);
        },
        _ => println!("  ✗ Failed to get ROWID range"),
    }

    // Count unique ROWIDs
    let unique_rowids: Result<i64> = conn.query_row(
        "SELECT COUNT(DISTINCT ROWID) FROM sheffield_people",
        [],
        |row| row.get(0)
    );

    match unique_rowids {
        Ok(count) => println!("  Unique ROWIDs: {}", count),
        Err(e) => println!("  ✗ Failed to count unique ROWIDs: {}", e),
    }

    // Check for duplicate ROWIDs
    println!("\nStep 4: Checking for duplicate ROWIDs...");
    let dup_check = conn.query_row(
        "SELECT COUNT(*) FROM (
            SELECT ROWID, COUNT(*) as cnt
            FROM sheffield_people
            GROUP BY ROWID
            HAVING cnt > 1
        )",
        [],
        |row| row.get::<_, i64>(0)
    );

    match dup_check {
        Ok(0) => println!("  ✓ No duplicate ROWIDs found"),
        Ok(count) => println!("  ✗ {} ROWIDs appear multiple times!", count),
        Err(e) => println!("  ✗ Failed to check duplicates: {}", e),
    }

    // Check current ID column
    println!("\nStep 5: Analyzing custom 'id' column...");
    let ids_total: Result<i64> = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people WHERE id IS NOT NULL",
        [],
        |row| row.get(0)
    );

    let ids_unique: Result<i64> = conn.query_row(
        "SELECT COUNT(DISTINCT id) FROM sheffield_people WHERE id IS NOT NULL",
        [],
        |row| row.get(0)
    );

    match (ids_total, ids_unique) {
        (Ok(total), Ok(unique)) => {
            println!("  Rows with id: {}", total);
            println!("  Unique ids: {}", unique);
            if total != unique {
                println!("  ✗ {} duplicate IDs!", total - unique);
            } else {
                println!("  ✓ All IDs are unique");
            }
        },
        _ => println!("  ✗ Failed to analyze ID column"),
    }

    // Check if we can read data despite corruption
    println!("\nStep 6: Testing data readability...");
    let sample = conn.query_row(
        "SELECT name FROM sheffield_people LIMIT 1",
        [],
        |row| row.get::<_, String>(0)
    );

    match sample {
        Ok(name) => println!("  ✓ Can read data (sample: {})", name),
        Err(e) => println!("  ✗ Cannot read data: {}", e),
    }

    // Check for journal/WAL files
    println!("\nStep 7: Checking for lock files...");
    if std::path::Path::new("Sheffield1867.db-journal").exists() {
        println!("  ⚠ Sheffield1867.db-journal exists (indicates interrupted transaction)");
    }
    if std::path::Path::new("Sheffield1867.db-shm").exists() {
        println!("  ⚠ Sheffield1867.db-shm exists (WAL shared memory)");
    }
    if std::path::Path::new("Sheffield1867.db-wal").exists() {
        println!("  ⚠ Sheffield1867.db-wal exists (write-ahead log)");
    }

    println!("\n=== RECOMMENDATIONS ===");
    println!("The ROWID corruption is internal to SQLite's b-tree structure.");
    println!("The actual data (names, professions, etc.) is likely intact.");
    println!("\nFix options:");
    println!("1. Use .dump/.restore with sqlite3 command-line tool");
    println!("2. Copy data row-by-row to a new database (ignoring ROWID)");
    println!("3. Use VACUUM to rebuild the database file");
    println!("4. Export to CSV and reimport to fresh database");

    Ok(())
}
