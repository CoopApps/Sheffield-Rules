use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let db_path = "Sheffield1867.db";
    let conn = Connection::open(db_path)?;

    println!("Starting ID assignment process...\n");

    // Check current state
    let count_before: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people WHERE id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;
    println!("People with IDs before: {}", count_before);

    let total_people: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people",
        [],
        |row| row.get(0),
    )?;
    println!("Total people in table: {}\n", total_people);

    // Start transaction
    conn.execute("BEGIN TRANSACTION", [])?;

    println!("Assigning sequential IDs to all people...");

    // Assign sequential IDs based on ROWID
    let changes = conn.execute(
        "UPDATE sheffield_people
         SET id = (
           SELECT COUNT(*)
           FROM sheffield_people AS p2
           WHERE p2.ROWID <= sheffield_people.ROWID
         )",
        [],
    )?;

    println!("Updated {} rows\n", changes);

    // Verify the assignment
    let count_after: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people WHERE id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;

    let min_id: i64 = conn.query_row(
        "SELECT MIN(id) FROM sheffield_people",
        [],
        |row| row.get(0),
    )?;

    let max_id: i64 = conn.query_row(
        "SELECT MAX(id) FROM sheffield_people",
        [],
        |row| row.get(0),
    )?;

    let unique_ids: i64 = conn.query_row(
        "SELECT COUNT(DISTINCT id) FROM sheffield_people",
        [],
        |row| row.get(0),
    )?;

    println!("=== VERIFICATION ===");
    println!("People with IDs after: {}", count_after);
    println!("ID range: {} to {}", min_id, max_id);
    println!("Unique IDs: {}", unique_ids);
    println!("Total people: {}", total_people);

    if unique_ids == total_people && count_after == total_people {
        println!("\n✓ Success! All people have unique IDs.");
        conn.execute("COMMIT", [])?;

        // Show some examples
        println!("\n=== SAMPLE DATA ===");
        let mut stmt = conn.prepare(
            "SELECT id, name, first_name, surname, census_age FROM sheffield_people LIMIT 10"
        )?;

        let mut rows = stmt.query([])?;
        println!("ID | Name | First Name | Surname | Age");
        println!("----------------------------------------------");

        while let Some(row) = rows.next()? {
            let id: i64 = row.get(0)?;
            let name: Option<String> = row.get(1)?;
            let first_name: Option<String> = row.get(2)?;
            let surname: Option<String> = row.get(3)?;
            let age: Option<i64> = row.get(4)?;

            println!(
                "{} | {} | {} | {} | {}",
                id,
                name.unwrap_or_else(|| "NULL".to_string()),
                first_name.unwrap_or_else(|| "NULL".to_string()),
                surname.unwrap_or_else(|| "NULL".to_string()),
                age.map(|a| a.to_string()).unwrap_or_else(|| "NULL".to_string())
            );
        }
    } else {
        println!("\n✗ Error: ID assignment verification failed!");
        println!("Rolling back changes...");
        conn.execute("ROLLBACK", [])?;
    }

    println!("\nDone!");
    Ok(())
}
