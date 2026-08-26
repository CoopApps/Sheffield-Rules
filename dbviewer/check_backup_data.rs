use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let old_backup = Connection::open("Sheffield1867_backup_20260305_123217.db")?;
    let new_backup = Connection::open("Sheffield1867-backup-20260305-195707.db")?;

    println!("=== BACKUP COMPARISON ===\n");

    println!("OLD BACKUP (12:32 PM - Sheffield1867_backup_20260305_123217.db):");
    println!("Profession tables:");

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
        let count: i64 = old_backup.query_row(
            &format!("SELECT COUNT(*) FROM {}", table),
            [],
            |row| row.get(0),
        )?;
        let with_ids: i64 = old_backup.query_row(
            &format!("SELECT COUNT(*) FROM {} WHERE sheffield_person_id IS NOT NULL", table),
            [],
            |row| row.get(0),
        )?;
        println!("  {}: {} total, {} with IDs", table, count, with_ids);
    }

    let people_with_ids: i64 = old_backup.query_row(
        "SELECT COUNT(*) FROM sheffield_people WHERE id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;
    println!("\nsheffield_people with IDs: {}\n", people_with_ids);

    println!("NEW BACKUP (7:57 PM - Sheffield1867-backup-20260305-195707.db):");
    println!("Profession tables:");

    for table in &tables {
        let count: i64 = new_backup.query_row(
            &format!("SELECT COUNT(*) FROM {}", table),
            [],
            |row| row.get(0),
        )?;
        let with_ids: i64 = new_backup.query_row(
            &format!("SELECT COUNT(*) FROM {} WHERE sheffield_person_id IS NOT NULL", table),
            [],
            |row| row.get(0),
        )?;
        println!("  {}: {} total, {} with IDs", table, count, with_ids);
    }

    let people_with_ids_new: i64 = new_backup.query_row(
        "SELECT COUNT(*) FROM sheffield_people WHERE id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;
    println!("\nsheffield_people with IDs: {}", people_with_ids_new);

    // Check family and household data
    println!("\n=== FAMILY/HOUSEHOLD GROUPINGS ===\n");

    println!("OLD BACKUP:");
    let old_family_groups: i64 = old_backup.query_row(
        "SELECT COUNT(*) FROM sheffield_family_groups", [], |row| row.get(0))?;
    let old_family_members: i64 = old_backup.query_row(
        "SELECT COUNT(*) FROM sheffield_family_members", [], |row| row.get(0))?;
    let old_households: i64 = old_backup.query_row(
        "SELECT COUNT(*) FROM sheffield_households", [], |row| row.get(0))?;
    let old_household_members: i64 = old_backup.query_row(
        "SELECT COUNT(*) FROM sheffield_household_members", [], |row| row.get(0))?;
    println!("  Family groups: {}, members: {}", old_family_groups, old_family_members);
    println!("  Households: {}, members: {}", old_households, old_household_members);

    println!("\nNEW BACKUP:");
    let new_family_groups: i64 = new_backup.query_row(
        "SELECT COUNT(*) FROM sheffield_family_groups", [], |row| row.get(0))?;
    let new_family_members: i64 = new_backup.query_row(
        "SELECT COUNT(*) FROM sheffield_family_members", [], |row| row.get(0))?;
    let new_households: i64 = new_backup.query_row(
        "SELECT COUNT(*) FROM sheffield_households", [], |row| row.get(0))?;
    let new_household_members: i64 = new_backup.query_row(
        "SELECT COUNT(*) FROM sheffield_household_members", [], |row| row.get(0))?;
    println!("  Family groups: {}, members: {}", new_family_groups, new_family_members);
    println!("  Households: {}, members: {}", new_households, new_household_members);

    Ok(())
}
