use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;

    // Disable foreign key constraints temporarily
    conn.execute("PRAGMA foreign_keys = OFF", [])?;

    println!("Linking profession tables to sheffield_people...\n");

    // List of profession tables to link
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

        // Update based on exact name match
        let _updated = conn.execute(
            &format!(
                "UPDATE {}
                 SET sheffield_person_id = (
                   SELECT id
                   FROM sheffield_people
                   WHERE sheffield_people.name = {}.name
                   LIMIT 1
                 )
                 WHERE EXISTS (
                   SELECT 1
                   FROM sheffield_people
                   WHERE sheffield_people.name = {}.name
                 )",
                table, table, table
            ),
            [],
        )?;

        // Count matches
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

        println!("  ✓ Matched {}/{} records", matched, total);
    }

    println!("\n=== SUMMARY ===");
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

    Ok(())
}
