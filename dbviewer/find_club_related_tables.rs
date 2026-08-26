use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let db_path = "../Sheffield1867.db";
    let conn = Connection::open(db_path)?;

    println!("================================================================================");
    println!("FINDING ALL TABLES RELATED TO CLUBS IN SHEFFIELD1867.DB");
    println!("================================================================================\n");

    // Get all tables
    let mut stmt = conn.prepare(
        "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"
    )?;

    let tables: Vec<String> = stmt
        .query_map([], |row| row.get(0))?
        .collect::<Result<Vec<String>, _>>()?;

    println!("Total tables in database: {}\n", tables.len());

    // Check each table for club-related columns
    println!("================================================================================");
    println!("TABLES WITH CLUB-RELATED COLUMNS:");
    println!("================================================================================\n");

    let mut club_related_tables: Vec<(String, Vec<String>)> = Vec::new();

    for table in &tables {
        let mut club_columns: Vec<String> = Vec::new();

        // Get table info
        let mut info_stmt = conn.prepare(&format!("PRAGMA table_info({})", table))?;
        let mut rows = info_stmt.query([])?;

        while let Some(row) = rows.next()? {
            let col_name: String = row.get(1)?;

            // Check if column name contains 'club'
            if col_name.to_lowercase().contains("club") {
                club_columns.push(col_name);
            }
        }

        if !club_columns.is_empty() {
            club_related_tables.push((table.clone(), club_columns));
        }
    }

    // Display results
    for (table, columns) in &club_related_tables {
        println!("📋 {}", table);
        println!("   Club-related columns:");
        for col in columns {
            println!("     - {}", col);
        }

        // Check if it has data
        let count: i64 = conn.query_row(
            &format!("SELECT COUNT(*) FROM {}", table),
            [],
            |r| r.get(0),
        ).unwrap_or(0);

        println!("   Rows: {}", count);

        // Show sample foreign key relationships
        let fk_info = conn.prepare(&format!("PRAGMA foreign_key_list({})", table));
        if let Ok(mut fk_stmt) = fk_info {
            let mut fk_rows = fk_stmt.query([])?;
            let mut has_fks = false;

            while let Some(fk_row) = fk_rows.next()? {
                if !has_fks {
                    println!("   Foreign keys:");
                    has_fks = true;
                }
                let from_col: String = fk_row.get(3)?;
                let to_table: String = fk_row.get(2)?;
                let to_col: String = fk_row.get(4)?;
                println!("     {} → {}.{}", from_col, to_table, to_col);
            }
        }

        println!();
    }

    // Check for tables that reference sheffield_clubs
    println!("================================================================================");
    println!("TABLES WITH FOREIGN KEYS TO sheffield_clubs:");
    println!("================================================================================\n");

    for table in &tables {
        let mut fk_stmt = conn.prepare(&format!("PRAGMA foreign_key_list({})", table))?;
        let mut rows = fk_stmt.query([])?;
        let mut found_fk = false;

        while let Some(row) = rows.next()? {
            let to_table: String = row.get(2)?;
            if to_table == "sheffield_clubs" {
                if !found_fk {
                    println!("📌 {}", table);
                    found_fk = true;
                }
                let from_col: String = row.get(3)?;
                let to_col: String = row.get(4)?;
                println!("   {} → sheffield_clubs.{}", from_col, to_col);
            }
        }

        if found_fk {
            let count: i64 = conn.query_row(
                &format!("SELECT COUNT(*) FROM {}", table),
                [],
                |r| r.get(0),
            ).unwrap_or(0);
            println!("   Rows: {}\n", count);
        }
    }

    // Specific important tables
    println!("================================================================================");
    println!("KEY CLUB TABLES ANALYSIS:");
    println!("================================================================================\n");

    let key_tables = vec![
        "sheffield_clubs",
        "sheffield_league_clubs",
        "sheffield_league_divisions",
        "sheffield_players",
        "sheffield_standings",
        "sheffield_fixtures",
        "sheffield_matches",
    ];

    for table in &key_tables {
        if tables.contains(&table.to_string()) {
            println!("✓ {}", table);

            // Get row count
            let count: i64 = conn.query_row(
                &format!("SELECT COUNT(*) FROM {}", table),
                [],
                |r| r.get(0),
            ).unwrap_or(0);

            println!("  Rows: {}", count);

            // Get columns
            let mut col_stmt = conn.prepare(&format!("PRAGMA table_info({})", table))?;
            let mut col_rows = col_stmt.query([])?;

            print!("  Columns: ");
            let mut cols = Vec::new();
            while let Some(row) = col_rows.next()? {
                let name: String = row.get(1)?;
                cols.push(name);
            }
            println!("{}", cols.join(", "));

            // Show sample data for small tables
            if count > 0 && count <= 5 {
                println!("  Sample data:");
                let query = format!("SELECT * FROM {} LIMIT 3", table);
                let mut sample_stmt = conn.prepare(&query)?;
                let mut sample_rows = sample_stmt.query([])?;

                while let Some(_row) = sample_rows.next()? {
                    println!("    (row data)");
                }
            }

            println!();
        } else {
            println!("✗ {} (does not exist)", table);
        }
    }

    // Summary
    println!("================================================================================");
    println!("SUMMARY:");
    println!("================================================================================\n");

    println!("Tables with club-related columns: {}", club_related_tables.len());
    println!("Key club tables present: {}/{}",
        key_tables.iter().filter(|t| tables.contains(&t.to_string())).count(),
        key_tables.len()
    );

    // Check if clubs can be safely deleted/modified
    println!("\n⚠ DEPENDENCIES TO CONSIDER:");
    println!("If you modify/delete clubs in sheffield_clubs, these tables are affected:");

    for table in &tables {
        let mut fk_stmt = conn.prepare(&format!("PRAGMA foreign_key_list({})", table))?;
        let mut rows = fk_stmt.query([])?;

        while let Some(row) = rows.next()? {
            let to_table: String = row.get(2)?;
            if to_table == "sheffield_clubs" {
                let count: i64 = conn.query_row(
                    &format!("SELECT COUNT(*) FROM {}", table),
                    [],
                    |r| r.get(0),
                ).unwrap_or(0);

                if count > 0 {
                    println!("  - {} ({} rows)", table, count);
                }
            }
        }
    }

    println!("\n================================================================================\n");

    Ok(())
}
