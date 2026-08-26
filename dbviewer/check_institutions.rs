use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;

    println!("=== INSTITUTIONAL TABLES IN ORIGINAL DB ===\n");

    // Check each institutional table
    let tables = vec![
        "sheffield_asylum",
        "sheffield_prison",
        "sheffield_workhouse",
        "sheffield_patron",
    ];

    for table in &tables {
        let count: i64 = conn.query_row(
            &format!("SELECT COUNT(*) FROM {}", table),
            [],
            |row| row.get(0)
        )?;
        println!("{}: {} people", table, count);

        // Show sample addresses
        if count > 0 {
            let mut stmt = conn.prepare(&format!(
                "SELECT DISTINCT street_address FROM {} LIMIT 5",
                table
            ))?;
            let addresses: Vec<String> = stmt.query_map([], |row| row.get(0))?
                .filter_map(|r| r.ok())
                .collect();

            for addr in addresses {
                println!("  - {}", addr);
            }
        }
        println!();
    }

    // Check if there's a clergy table
    let clergy_check = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name LIKE '%clergy%'",
        [],
        |row| row.get::<_, i64>(0)
    )?;

    if clergy_check > 0 {
        println!("Found clergy table!");
    } else {
        println!("No clergy table found - checking for clergy in professions...");
        let clergy_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sheffield_people WHERE profession LIKE '%clergy%' OR profession LIKE '%minister%' OR profession LIKE '%vicar%' OR profession LIKE '%priest%'",
            [],
            |row| row.get(0)
        )?;
        println!("People with clergy professions: {}", clergy_count);
    }

    Ok(())
}
