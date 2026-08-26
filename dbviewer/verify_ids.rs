use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;

    println!("=== CHECKING ID INTEGRITY ===\n");
    println!("Verifying that every sheffield_person_id in all tables matches an id in sheffield_people...\n");

    // Check profession tables
    println!("1. PROFESSION TABLES:");
    let profession_tables = vec![
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

    for table in &profession_tables {
        let total: i64 = conn.query_row(
            &format!("SELECT COUNT(*) FROM {} WHERE sheffield_person_id IS NOT NULL", table),
            [],
            |row| row.get(0),
        )?;

        let orphaned: i64 = conn.query_row(
            &format!(
                "SELECT COUNT(*) FROM {}
                 WHERE sheffield_person_id IS NOT NULL
                 AND sheffield_person_id NOT IN (SELECT id FROM sheffield_people WHERE id IS NOT NULL)",
                table
            ),
            [],
            |row| row.get(0),
        )?;

        if orphaned == 0 {
            println!("   ✓ {}: {}/{} IDs valid (0 orphaned)", table, total, total);
        } else {
            println!("   ✗ {}: {}/{} IDs valid ({} ORPHANED!)", table, total - orphaned, total, orphaned);
        }
    }

    // Check special population tables
    println!("\n2. SPECIAL POPULATION TABLES:");
    let special_tables = vec![
        ("sheffield_asylum", "matched_to_ancestry_id"),
        ("sheffield_prison", "matched_to_ancestry_id"),
        ("sheffield_workhouse", "matched_to_ancestry_id"),
        ("sheffield_patron", "matched_to_ancestry_id"),
    ];

    for (table, id_col) in &special_tables {
        let total: i64 = conn.query_row(
            &format!("SELECT COUNT(*) FROM {} WHERE {} IS NOT NULL", table, id_col),
            [],
            |row| row.get(0),
        )?;

        let orphaned: i64 = conn.query_row(
            &format!(
                "SELECT COUNT(*) FROM {}
                 WHERE {} IS NOT NULL
                 AND {} NOT IN (SELECT id FROM sheffield_people WHERE id IS NOT NULL)",
                table, id_col, id_col
            ),
            [],
            |row| row.get(0),
        )?;

        if total == 0 {
            println!("   - {}: No IDs assigned yet", table);
        } else if orphaned == 0 {
            println!("   ✓ {}: {}/{} IDs valid (0 orphaned)", table, total, total);
        } else {
            println!("   ✗ {}: {}/{} IDs valid ({} ORPHANED!)", table, total - orphaned, total, orphaned);
        }
    }

    // Check family members
    println!("\n3. FAMILY MEMBERS:");
    let family_total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_family_members WHERE sheffield_person_id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;

    let family_orphaned: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_family_members
         WHERE sheffield_person_id IS NOT NULL
         AND sheffield_person_id NOT IN (SELECT id FROM sheffield_people WHERE id IS NOT NULL)",
        [],
        |row| row.get(0),
    )?;

    if family_orphaned == 0 {
        println!("   ✓ sheffield_family_members: {}/{} IDs valid (0 orphaned)", family_total, family_total);
    } else {
        println!("   ✗ sheffield_family_members: {}/{} IDs valid ({} ORPHANED!)", family_total - family_orphaned, family_total, family_orphaned);
    }

    // Check household members
    println!("\n4. HOUSEHOLD MEMBERS:");
    let household_total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_household_members WHERE sheffield_person_id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;

    let household_orphaned: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_household_members
         WHERE sheffield_person_id IS NOT NULL
         AND sheffield_person_id NOT IN (SELECT id FROM sheffield_people WHERE id IS NOT NULL)",
        [],
        |row| row.get(0),
    )?;

    if household_orphaned == 0 {
        println!("   ✓ sheffield_household_members: {}/{} IDs valid (0 orphaned)", household_total, household_total);
    } else {
        println!("   ✗ sheffield_household_members: {}/{} IDs valid ({} ORPHANED!)", household_total - household_orphaned, household_total, household_orphaned);
    }

    // Check household heads
    let heads_total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_households WHERE head_person_id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;

    let heads_orphaned: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_households
         WHERE head_person_id IS NOT NULL
         AND head_person_id NOT IN (SELECT id FROM sheffield_people WHERE id IS NOT NULL)",
        [],
        |row| row.get(0),
    )?;

    if heads_orphaned == 0 {
        println!("   ✓ sheffield_households (heads): {}/{} IDs valid (0 orphaned)", heads_total, heads_total);
    } else {
        println!("   ✗ sheffield_households (heads): {}/{} IDs valid ({} ORPHANED!)", heads_total - heads_orphaned, heads_total, heads_orphaned);
    }

    // Check sheffield_people ID uniqueness
    println!("\n5. SHEFFIELD_PEOPLE ID UNIQUENESS:");
    let people_total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people",
        [],
        |row| row.get(0),
    )?;

    let people_with_ids: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people WHERE id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;

    let unique_ids: i64 = conn.query_row(
        "SELECT COUNT(DISTINCT id) FROM sheffield_people WHERE id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;

    println!("   Total people: {}", people_total);
    println!("   People with IDs: {}", people_with_ids);
    println!("   Unique IDs: {}", unique_ids);

    if unique_ids == people_with_ids && people_with_ids == people_total {
        println!("   ✓ All IDs are unique and all people have IDs");
    } else if unique_ids == people_with_ids {
        println!("   ⚠ All IDs are unique, but {} people missing IDs", people_total - people_with_ids);
    } else {
        println!("   ✗ DUPLICATE IDs FOUND! {} duplicates", people_with_ids - unique_ids);
    }

    println!("\n=== ID INTEGRITY CHECK COMPLETE ===");

    Ok(())
}
