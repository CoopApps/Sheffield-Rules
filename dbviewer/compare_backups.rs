use rusqlite::{Connection, Result};
use std::collections::HashSet;

fn main() -> Result<()> {
    // Wait a moment for any locks to clear
    std::thread::sleep(std::time::Duration::from_secs(2));

    let current_conn = Connection::open("Sheffield1867-backup-20260305-195707.db")?;
    let backup_conn = Connection::open("Sheffield1867_backup_20260305_123217.db")?;

    println!("=== COMPARING DATABASES ===\n");

    // Get tables from current database
    let mut current_stmt = current_conn.prepare(
        "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name"
    )?;
    let current_tables: HashSet<String> = current_stmt
        .query_map([], |row| row.get(0))?
        .collect::<Result<HashSet<String>>>()?;

    // Get tables from backup database
    let mut backup_stmt = backup_conn.prepare(
        "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name"
    )?;
    let backup_tables: HashSet<String> = backup_stmt
        .query_map([], |row| row.get(0))?
        .collect::<Result<HashSet<String>>>()?;

    // Tables only in current database
    let new_tables: Vec<_> = current_tables.difference(&backup_tables).collect();
    if !new_tables.is_empty() {
        println!("TABLES ONLY IN CURRENT DATABASE (not in backup):");
        for table in &new_tables {
            let count: i64 = current_conn.query_row(
                &format!("SELECT COUNT(*) FROM {}", table),
                [],
                |row| row.get(0),
            )?;
            println!("  - {} ({} rows)", table, count);
        }
        println!();
    } else {
        println!("No new tables in current database.\n");
    }

    // Tables only in backup database
    let removed_tables: Vec<_> = backup_tables.difference(&current_tables).collect();
    if !removed_tables.is_empty() {
        println!("TABLES ONLY IN BACKUP (removed from current):");
        for table in &removed_tables {
            let count: i64 = backup_conn.query_row(
                &format!("SELECT COUNT(*) FROM {}", table),
                [],
                |row| row.get(0),
            )?;
            println!("  - {} ({} rows)", table, count);
        }
        println!();
    } else {
        println!("No tables removed from current database.\n");
    }

    // Tables in both - compare row counts
    let common_tables: Vec<_> = current_tables.intersection(&backup_tables).collect();
    if !common_tables.is_empty() {
        println!("TABLES IN BOTH (row count comparison):");
        for table in &common_tables {
            let current_count: i64 = current_conn.query_row(
                &format!("SELECT COUNT(*) FROM {}", table),
                [],
                |row| row.get(0),
            )?;
            let backup_count: i64 = backup_conn.query_row(
                &format!("SELECT COUNT(*) FROM {}", table),
                [],
                |row| row.get(0),
            )?;

            if current_count != backup_count {
                println!("  {} - Current: {}, Backup: {} (DIFFERENT by {})",
                         table, current_count, backup_count,
                         (current_count as i64 - backup_count as i64).abs());
            }
        }
        println!();
    }

    println!("=== SUMMARY ===");
    println!("Total tables in current: {}", current_tables.len());
    println!("Total tables in backup: {}", backup_tables.len());
    println!("New tables: {}", new_tables.len());
    println!("Removed tables: {}", removed_tables.len());
    println!("Common tables: {}", common_tables.len());

    Ok(())
}
