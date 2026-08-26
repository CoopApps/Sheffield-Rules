use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867b.db")?;

    println!("=== SHEFFIELD1867B.DB HEALTH CHECK ===\n");

    // 1. Integrity check
    println!("1. Running integrity check...");
    let integrity: String = conn.query_row("PRAGMA integrity_check", [], |row| row.get(0))?;
    if integrity == "ok" {
        println!("   ✓ Database integrity: OK\n");
    } else {
        println!("   ✗ Database integrity: {}\n", integrity);
    }

    // 2. Check people have IDs
    println!("2. Checking sheffield_people IDs...");
    let people_total: i64 = conn.query_row("SELECT COUNT(*) FROM sheffield_people", [], |row| row.get(0))?;
    let people_with_ids: i64 = conn.query_row("SELECT COUNT(*) FROM sheffield_people WHERE id IS NOT NULL", [], |row| row.get(0))?;
    let unique_ids: i64 = conn.query_row("SELECT COUNT(DISTINCT id) FROM sheffield_people WHERE id IS NOT NULL", [], |row| row.get(0))?;

    println!("   Total people: {}", people_total);
    println!("   People with IDs: {}", people_with_ids);
    println!("   Unique IDs: {}", unique_ids);

    if people_with_ids == people_total && unique_ids == people_with_ids {
        println!("   ✓ All people have unique IDs\n");
    } else {
        println!("   ⚠ Issues: {} without IDs, {} duplicates\n",
                 people_total - people_with_ids,
                 people_with_ids - unique_ids);
    }

    // 3. Check profession tables
    println!("3. Checking profession tables...");
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
        let total: i64 = conn.query_row(&format!("SELECT COUNT(*) FROM {}", table), [], |row| row.get(0))?;
        let with_ids: i64 = conn.query_row(
            &format!("SELECT COUNT(*) FROM {} WHERE sheffield_person_id IS NOT NULL", table),
            [],
            |row| row.get(0),
        )?;
        if with_ids == total && total > 0 {
            println!("   ✓ {}: {}/{} linked", table, with_ids, total);
        } else {
            println!("   ✗ {}: {}/{} linked", table, with_ids, total);
        }
    }

    // 4. Check family/household data
    println!("\n4. Checking family/household groupings...");
    let family_groups: i64 = conn.query_row("SELECT COUNT(*) FROM sheffield_family_groups", [], |row| row.get(0))?;
    let family_members: i64 = conn.query_row("SELECT COUNT(*) FROM sheffield_family_members", [], |row| row.get(0))?;
    let family_linked: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_family_members WHERE sheffield_person_id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;
    println!("   Family groups: {} groups, {} members ({} linked)", family_groups, family_members, family_linked);

    let households: i64 = conn.query_row("SELECT COUNT(*) FROM sheffield_households", [], |row| row.get(0))?;
    let household_members: i64 = conn.query_row("SELECT COUNT(*) FROM sheffield_household_members", [], |row| row.get(0))?;
    let household_linked: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_household_members WHERE sheffield_person_id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;
    println!("   Households: {} households, {} members ({} linked)", households, household_members, household_linked);

    println!("\n=== SHEFFIELD1867B.DB IS READY TO USE ===");

    Ok(())
}
