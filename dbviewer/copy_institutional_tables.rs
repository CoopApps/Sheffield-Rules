use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let dest_conn = Connection::open("Sheffield1867_rescued.db")?;

    println!("=== COPYING INSTITUTIONAL TABLES ===\n");

    // Drop existing tables if they exist
    println!("1. Cleaning up existing tables...");
    dest_conn.execute("DROP TABLE IF EXISTS sheffield_asylum", [])?;
    dest_conn.execute("DROP TABLE IF EXISTS sheffield_prison", [])?;
    dest_conn.execute("DROP TABLE IF EXISTS sheffield_workhouse", [])?;
    dest_conn.execute("DROP TABLE IF EXISTS sheffield_clergy", [])?;
    println!("   ✓ Cleaned up\n");

    // Attach source database
    dest_conn.execute("ATTACH DATABASE 'Sheffield1867.db' AS source", [])?;

    // Copy asylum table
    println!("2. Copying sheffield_asylum...");
    dest_conn.execute(
        "CREATE TABLE sheffield_asylum AS SELECT * FROM source.sheffield_asylum",
        [],
    )?;
    let asylum_count: i64 = dest_conn.query_row(
        "SELECT COUNT(*) FROM sheffield_asylum",
        [],
        |row| row.get(0)
    )?;
    println!("   ✓ Copied {} people\n", asylum_count);

    // Copy prison table
    println!("3. Copying sheffield_prison...");
    dest_conn.execute(
        "CREATE TABLE sheffield_prison AS SELECT * FROM source.sheffield_prison",
        [],
    )?;
    let prison_count: i64 = dest_conn.query_row(
        "SELECT COUNT(*) FROM sheffield_prison",
        [],
        |row| row.get(0)
    )?;
    println!("   ✓ Copied {} people\n", prison_count);

    // Copy workhouse table
    println!("4. Copying sheffield_workhouse...");
    dest_conn.execute(
        "CREATE TABLE sheffield_workhouse AS SELECT * FROM source.sheffield_workhouse",
        [],
    )?;
    let workhouse_count: i64 = dest_conn.query_row(
        "SELECT COUNT(*) FROM sheffield_workhouse",
        [],
        |row| row.get(0)
    )?;
    println!("   ✓ Copied {} people\n", workhouse_count);

    // Detach source database
    dest_conn.execute("DETACH DATABASE source", [])?;

    println!("=== COPY COMPLETE ===");
    println!("Total institutional people: {}", asylum_count + prison_count + workhouse_count);

    Ok(())
}
