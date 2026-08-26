use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let db_path = "../Sheffield1867.db";
    let conn = Connection::open(db_path)?;

    println!("================================================================================");
    println!("ALL TABLES IN SHEFFIELD1867.DB");
    println!("================================================================================\n");

    let mut stmt = conn.prepare(
        "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"
    )?;

    let tables: Vec<String> = stmt
        .query_map([], |row| row.get(0))?
        .collect::<Result<Vec<_>, _>>()?;

    println!("Total tables: {}\n", tables.len());

    for table in &tables {
        // Get row count
        let count: i64 = conn.query_row(
            &format!("SELECT COUNT(*) FROM {}", table),
            [],
            |r| r.get(0),
        ).unwrap_or(0);

        let marker = if count > 0 { "✓" } else { "○" };
        println!("  {} {:<40} ({} rows)", marker, table, count);
    }

    println!("\n================================================================================\n");

    Ok(())
}
