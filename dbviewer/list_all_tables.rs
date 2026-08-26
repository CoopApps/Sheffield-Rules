use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;

    println!("=== ALL TABLES IN SHEFFIELD1867.DB ===\n");

    let mut stmt = conn.prepare(
        "SELECT name, sql FROM sqlite_master WHERE type='table' ORDER BY name"
    )?;

    let tables = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, Option<String>>(1)?,
        ))
    })?;

    for table in tables {
        let (name, _schema) = table?;

        // Get count
        let count: i64 = conn.query_row(
            &format!("SELECT COUNT(*) FROM {}", name),
            [],
            |row| row.get(0)
        )?;

        println!("{}: {} rows", name, count);
    }

    println!("\n=== BREAKDOWN ===\n");

    // Institutional tables (already copied)
    println!("Institutional (already copied):");
    println!("  - sheffield_asylum");
    println!("  - sheffield_prison");
    println!("  - sheffield_workhouse");
    println!("  - sheffield_patron");

    // Main people table
    println!("\nMain people:");
    println!("  - sheffield_people");
    println!("  - sheffield_footballers (skip this)");

    // Categorization tables
    println!("\nPeople categorization tables:");
    println!("  - sheffield_employers");
    println!("  - sheffield_professionals");
    println!("  - sheffield_tradesmen");
    println!("  - sheffield_publicans");
    println!("  - sheffield_lodgers");
    println!("  - sheffield_german");
    println!("  - sheffield_irish");
    println!("  - sheffield_scottish");
    println!("  - sheffield_welsh");

    // Grouping tables
    println!("\nGrouping tables:");
    println!("  - sheffield_family_groups");
    println!("  - sheffield_family_members");
    println!("  - sheffield_households");
    println!("  - sheffield_household_members");

    // Other tables
    println!("\nOther:");
    println!("  - sheffield_clubs (need to copy)");
    println!("  - sheffield_businesses");
    println!("  - unmatched_ancestry");
    println!("  - unmatched_geni");

    Ok(())
}
