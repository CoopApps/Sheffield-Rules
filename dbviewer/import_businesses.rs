use rusqlite::{Connection, Result};
use std::process::Command;

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867_rescued.db")?;

    eprintln!("=== IMPORTING BUSINESSES FROM WHITES ===\n");

    conn.execute("DROP TABLE IF EXISTS sheffield_businesses", [])?;
    conn.execute(
        "CREATE TABLE sheffield_businesses (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            surname TEXT,
            forename TEXT,
            title TEXT,
            occupation TEXT,
            address TEXT,
            year INTEGER,
            source TEXT,
            sheffield_person_id INTEGER
        )",
        [],
    )?;

    // Use sqlite3 .import for bulk import
    let csv_files = std::fs::read_dir("Whites")
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map(|x| x == "csv").unwrap_or(false))
        .map(|e| e.path())
        .collect::<Vec<_>>();

    eprintln!("Found {} CSV files", csv_files.len());

    // Create temp table without id for import
    conn.execute("CREATE TEMP TABLE temp_import (surname TEXT, forename TEXT, title TEXT, occupation TEXT, address TEXT, year TEXT, source TEXT)", [])?;

    for csv in &csv_files {
        let filename = csv.file_name().unwrap().to_str().unwrap();

        // Read file and skip header, insert all at once
        let content = std::fs::read_to_string(csv).unwrap();
        let lines: Vec<&str> = content.lines().skip(1).collect();

        conn.execute("BEGIN", [])?;
        for line in &lines {
            let fields = parse_csv_line(line);
            if fields.len() >= 7 {
                conn.execute(
                    "INSERT INTO sheffield_businesses (surname, forename, title, occupation, address, year, source) VALUES (?1,?2,?3,?4,?5,?6,?7)",
                    rusqlite::params![&fields[0], &fields[1], &fields[2], &fields[3], &fields[4], fields[5].parse::<i64>().unwrap_or(1871), &fields[6]]
                )?;
            }
        }
        conn.execute("COMMIT", [])?;
        eprintln!("{}: {} rows", filename, lines.len());
    }

    let count: i64 = conn.query_row("SELECT COUNT(*) FROM sheffield_businesses", [], |r| r.get(0))?;
    eprintln!("\nTotal: {} businesses imported", count);

    Ok(())
}

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    for c in line.chars() {
        match c {
            '"' => in_quotes = !in_quotes,
            ',' if !in_quotes => { fields.push(current.trim().to_string()); current = String::new(); }
            _ => current.push(c),
        }
    }
    fields.push(current.trim().to_string());
    fields
}
