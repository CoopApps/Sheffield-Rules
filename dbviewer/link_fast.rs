use rusqlite::{Connection, Result};
use std::collections::HashMap;

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;
    conn.execute("PRAGMA foreign_keys = OFF", [])?;

    println!("Fast profession linking...\n");

    // Step 1: Load all people into HashMap
    println!("Loading all people into memory...");
    let mut people_by_name: HashMap<String, i64> = HashMap::new();

    let mut stmt = conn.prepare("SELECT id, name FROM sheffield_people")?;
    let people = stmt.query_map([], |row| {
        Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
    })?;

    for person in people {
        let (id, name) = person?;
        people_by_name.insert(name, id);
    }

    println!("✓ Loaded {} people\n", people_by_name.len());

    // Tables to process
    let tables = vec![
        "sheffield_employers",
        "sheffield_professionals",
        "sheffield_tradesmen",
        "sheffield_publicans",
        "sheffield_lodgers",
        "sheffield_german",
        "sheffield_irish",
        "sheffield_scottish",
        "sheffield_welsh",
    ];

    for table in &tables {
        println!("Processing {}...", table);

        // Get all rows
        let query = format!("SELECT ROWID, name FROM {}", table);
        let mut stmt = conn.prepare(&query)?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, Option<String>>(1)?))
        })?;

        let mut matched = 0;
        let mut total = 0;

        let update_query = format!("UPDATE {} SET sheffield_person_id = ? WHERE ROWID = ?", table);

        for row in rows {
            let (rowid, name_opt) = row?;
            total += 1;

            if let Some(name) = name_opt {
                if let Some(&person_id) = people_by_name.get(&name) {
                    conn.execute(&update_query, [person_id, rowid])?;
                    matched += 1;
                }
            }
        }

        println!("  ✓ Matched {}/{} records\n", matched, total);
    }

    println!("=== SUMMARY ===");
    for table in &tables {
        let matched: i64 = conn.query_row(
            &format!("SELECT COUNT(*) FROM {} WHERE sheffield_person_id IS NOT NULL", table),
            [],
            |row| row.get(0),
        )?;
        let total: i64 = conn.query_row(
            &format!("SELECT COUNT(*) FROM {}", table),
            [],
            |row| row.get(0),
        )?;
        println!("{}: {}/{} linked", table, matched, total);
    }

    println!("\nDone!");
    Ok(())
}
