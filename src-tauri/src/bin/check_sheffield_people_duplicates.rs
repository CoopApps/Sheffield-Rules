use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";

    println!("Opening database: {}", db_path);
    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?;
    let pool = SqlitePool::connect_with(connect_options).await?;

    println!("\n=== Checking sheffield_people table schema ===\n");

    // First check the schema
    let columns: Vec<(i32, String, String, i32, Option<String>, i32)> = sqlx::query_as(
        "PRAGMA table_info(sheffield_people)"
    )
    .fetch_all(&pool)
    .await?;

    println!("{:<5} {:<30} {:<15}", "cid", "name", "type");
    println!("{}", "-".repeat(55));
    for (cid, name, col_type, _, _, _) in &columns {
        println!("{:<5} {:<30} {:<15}", cid, name, col_type);
    }

    println!("\n=== Checking for duplicate unique_id values in sheffield_people ===\n");

    // Check for duplicate unique_id values
    let duplicate_ids: Vec<(i64, i64)> = sqlx::query_as(
        "SELECT unique_id, COUNT(*) as count
         FROM sheffield_people
         GROUP BY unique_id
         HAVING COUNT(*) > 1
         ORDER BY count DESC"
    )
    .fetch_all(&pool)
    .await?;

    if duplicate_ids.is_empty() {
        println!("✓ No duplicate unique_id values in sheffield_people");
    } else {
        println!("⚠ Found {} duplicate unique_id values:\n", duplicate_ids.len());
        for (unique_id, count) in duplicate_ids.iter().take(10) {
            println!("unique_id {} appears {} times", unique_id, count);
        }
    }

    println!("\n=== Checking for duplicate person records (same first_name + surname) ===\n");

    // Find people with duplicate names
    let duplicate_names: Vec<(String, String, i64)> = sqlx::query_as(
        "SELECT first_name, surname, COUNT(*) as count
         FROM sheffield_people
         GROUP BY first_name, surname
         HAVING COUNT(*) > 1
         ORDER BY count DESC
         LIMIT 20"
    )
    .fetch_all(&pool)
    .await?;

    if duplicate_names.is_empty() {
        println!("No duplicate name combinations found in sheffield_people");
    } else {
        println!("Found duplicate name combinations (top 20):\n");
        println!("{:<25} {:<25} {:<10}", "First Name", "Surname", "Count");
        println!("{}", "-".repeat(65));

        for (first_name, surname, count) in &duplicate_names {
            println!("{:<25} {:<25} {:<10}", first_name, surname, count);
        }

        // Show details for the most common duplicate (John Smith)
        println!("\n=== Details for 'John Smith' entries (first 10) ===\n");

        let john_smiths: Vec<(i64, String, String, Option<i32>, Option<String>, Option<String>)> = sqlx::query_as(
            "SELECT unique_id, first_name, surname, birth_year, where_born, profession
             FROM sheffield_people
             WHERE first_name = 'John' AND surname = 'Smith'
             LIMIT 10"
        )
        .fetch_all(&pool)
        .await?;

        println!("{:<12} {:<25} {:<12} {:<30} {:<25}", "unique_id", "Name", "Birth Year", "Where Born", "Profession");
        println!("{}", "-".repeat(110));

        for (uid, fname, sname, birth_year, where_born, profession) in john_smiths {
            let name = format!("{} {}", fname, sname);
            let by_str = birth_year.map(|y| y.to_string()).unwrap_or_else(|| "NULL".to_string());
            let wb_str = where_born.unwrap_or_else(|| "NULL".to_string());
            let prof_str = profession.unwrap_or_else(|| "NULL".to_string());
            println!("{:<12} {:<25} {:<12} {:<30} {:<25}", uid, name, by_str, &wb_str[..wb_str.len().min(30)], &prof_str[..prof_str.len().min(25)]);
        }
    }

    // Get total counts
    let total_people: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_people")
        .fetch_one(&pool)
        .await?;

    let unique_ids: (i64,) = sqlx::query_as("SELECT COUNT(DISTINCT unique_id) FROM sheffield_people")
        .fetch_one(&pool)
        .await?;

    println!("\n=== Summary ===");
    println!("Total records in sheffield_people: {}", total_people.0);
    println!("Unique unique_id values: {}", unique_ids.0);
    println!("Difference: {}", total_people.0 - unique_ids.0);

    Ok(())
}
