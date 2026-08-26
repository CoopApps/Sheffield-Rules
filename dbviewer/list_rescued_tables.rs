use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867_rescued.db")?;

    println!("=== TABLES IN SHEFFIELD1867_RESCUED.DB ===\n");

    let mut stmt = conn.prepare(
        "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"
    )?;

    let tables: Vec<String> = stmt.query_map([], |row| row.get(0))?
        .filter_map(|r| r.ok())
        .collect();

    for table in &tables {
        let count: i64 = conn.query_row(
            &format!("SELECT COUNT(*) FROM {}", table),
            [],
            |row| row.get(0)
        )?;
        println!("{}: {} rows", table, count);
    }

    Ok(())
}
