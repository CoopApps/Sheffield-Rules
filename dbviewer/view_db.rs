use rusqlite::{Connection, Result};
use std::env;

fn main() -> Result<()> {
    let db_path = "Sheffield1867.db";
    let conn = Connection::open(db_path)?;

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // Show all tables and their content
        list_tables(&conn)?;
    } else {
        match args[1].as_str() {
            "tables" => list_tables(&conn)?,
            "schema" => {
                if args.len() > 2 {
                    show_schema(&conn, &args[2])?;
                } else {
                    println!("Usage: view_db schema [table_name]");
                }
            }
            "show" => {
                if args.len() > 2 {
                    let limit = if args.len() > 3 {
                        args[3].parse().unwrap_or(10)
                    } else {
                        10
                    };
                    show_table(&conn, &args[2], limit)?;
                } else {
                    println!("Usage: view_db show [table_name] [limit]");
                }
            }
            "query" => {
                if args.len() > 2 {
                    run_query(&conn, &args[2])?;
                } else {
                    println!("Usage: view_db query \"SELECT ...\"");
                }
            }
            _ => println!("Unknown command. Use: tables, schema, show, or query"),
        }
    }

    Ok(())
}

fn list_tables(conn: &Connection) -> Result<()> {
    println!("\n=== DATABASE TABLES ===");
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")?;
    let tables = stmt.query_map([], |row| row.get::<_, String>(0))?;

    for table in tables {
        let table_name = table?;
        println!("  - {}", table_name);
        show_schema(conn, &table_name)?;
        println!();
    }

    Ok(())
}

fn show_schema(conn: &Connection, table_name: &str) -> Result<()> {
    println!("=== SCHEMA: {} ===", table_name);
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table_name))?;
    let columns = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(1)?, // name
            row.get::<_, String>(2)?, // type
            row.get::<_, i32>(3)?,    // notnull
            row.get::<_, i32>(5)?,    // pk
        ))
    })?;

    for col in columns {
        let (name, col_type, notnull, pk) = col?;
        print!("  {} ({})", name, col_type);
        if pk > 0 {
            print!(" PRIMARY KEY");
        }
        if notnull > 0 {
            print!(" NOT NULL");
        }
        println!();
    }

    Ok(())
}

fn show_table(conn: &Connection, table_name: &str, limit: usize) -> Result<()> {
    println!("\n=== DATA: {} (first {} rows) ===", table_name, limit);

    // Get column names
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table_name))?;
    let column_names: Vec<String> = stmt
        .query_map([], |row| row.get::<_, String>(1))?
        .collect::<Result<Vec<_>, _>>()?;

    // Print header
    println!("{}", column_names.join(" | "));
    println!("{}", "-".repeat(column_names.len() * 15));

    // Get data
    let query = format!("SELECT * FROM {} LIMIT {}", table_name, limit);
    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query([])?;

    let mut count = 0;
    while let Some(row) = rows.next()? {
        let values: Vec<String> = (0..column_names.len())
            .map(|i| {
                row.get::<_, Option<String>>(i)
                    .unwrap_or(None)
                    .unwrap_or_else(|| "NULL".to_string())
            })
            .collect();
        println!("{}", values.join(" | "));
        count += 1;
    }

    // Get total count
    let total: i64 = conn.query_row(
        &format!("SELECT COUNT(*) FROM {}", table_name),
        [],
        |row| row.get(0),
    )?;
    println!("\nShowing {} of {} total rows", count, total);

    Ok(())
}

fn run_query(conn: &Connection, sql: &str) -> Result<()> {
    println!("\n=== RUNNING QUERY ===\n{}\n", sql);

    if sql.trim().to_uppercase().starts_with("SELECT") {
        let mut stmt = conn.prepare(sql)?;
        let column_count = stmt.column_count();
        let column_names: Vec<&str> = stmt.column_names();

        println!("{}", column_names.join(" | "));
        println!("{}", "-".repeat(column_count * 15));

        let mut rows = stmt.query([])?;
        let mut count = 0;
        while let Some(row) = rows.next()? {
            let values: Vec<String> = (0..column_count)
                .map(|i| {
                    row.get::<_, Option<String>>(i)
                        .unwrap_or(None)
                        .unwrap_or_else(|| "NULL".to_string())
                })
                .collect();
            println!("{}", values.join(" | "));
            count += 1;
        }
        println!("\nRows returned: {}", count);
    } else {
        let changes = conn.execute(sql, [])?;
        println!("Changes: {}", changes);
    }

    Ok(())
}
