use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867_rescued.db")?;

    println!("=== SEARCHING FOR INSTITUTIONAL ADDRESSES ===\n");

    // Search for asylum-like addresses
    println!("1. Searching for asylum addresses...");
    let mut stmt = conn.prepare(
        "SELECT DISTINCT street_address FROM sheffield_people
         WHERE street_address LIKE '%asylum%'
            OR street_address LIKE '%lunatic%'
            OR street_address LIKE '%lunstie%'
         LIMIT 10"
    )?;
    let addresses: Vec<String> = stmt.query_map([], |row| row.get(0))?
        .filter_map(|r| r.ok())
        .collect();
    println!("   Found {} distinct addresses:", addresses.len());
    for addr in &addresses {
        println!("   - {}", addr);
    }
    println!();

    // Search for prison-like addresses
    println!("2. Searching for prison addresses...");
    let mut stmt = conn.prepare(
        "SELECT DISTINCT street_address FROM sheffield_people
         WHERE street_address LIKE '%prison%'
            OR street_address LIKE '%gaol%'
            OR street_address LIKE '%jail%'
         LIMIT 10"
    )?;
    let addresses: Vec<String> = stmt.query_map([], |row| row.get(0))?
        .filter_map(|r| r.ok())
        .collect();
    println!("   Found {} distinct addresses:", addresses.len());
    for addr in &addresses {
        println!("   - {}", addr);
    }
    println!();

    // Search for workhouse addresses
    println!("3. Searching for workhouse addresses...");
    let mut stmt = conn.prepare(
        "SELECT DISTINCT street_address FROM sheffield_people
         WHERE street_address LIKE '%workhouse%'
         LIMIT 10"
    )?;
    let addresses: Vec<String> = stmt.query_map([], |row| row.get(0))?
        .filter_map(|r| r.ok())
        .collect();
    println!("   Found {} distinct addresses:", addresses.len());
    for addr in &addresses {
        println!("   - {}", addr);
    }
    println!();

    // Check what's currently in institutional tables
    println!("4. Current institutional table counts:");
    let asylum: i64 = conn.query_row("SELECT COUNT(*) FROM sheffield_asylum", [], |row| row.get(0))?;
    let prison: i64 = conn.query_row("SELECT COUNT(*) FROM sheffield_prison", [], |row| row.get(0))?;
    let workhouse: i64 = conn.query_row("SELECT COUNT(*) FROM sheffield_workhouse", [], |row| row.get(0))?;
    let clergy: i64 = conn.query_row("SELECT COUNT(*) FROM sheffield_clergy", [], |row| row.get(0))?;

    println!("   Asylum: {}", asylum);
    println!("   Prison: {}", prison);
    println!("   Workhouse: {}", workhouse);
    println!("   Clergy: {}", clergy);

    Ok(())
}
