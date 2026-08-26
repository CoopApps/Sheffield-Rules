use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("Checking for examples of what WAS duplicated (before deletion)...\n");

    // Check if there are any census year or source columns
    let columns: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM pragma_table_info('sheffield_people')"
    )
    .fetch_all(&pool)
    .await?;

    println!("Available columns in sheffield_people:");
    for (col,) in &columns {
        println!("  - {}", col);
    }

    // Show some examples of remaining people with their full details
    println!("\nExamples of people after deduplication:");
    let samples: Vec<(String, String, Option<i64>, Option<String>, Option<String>, Option<String>)> = sqlx::query_as(
        r#"
        SELECT id, name, birth_year, street_address, profession, civil_parish
        FROM sheffield_people
        WHERE name LIKE '%Abraham Adams%'
        LIMIT 10
        "#
    )
    .fetch_all(&pool)
    .await?;

    for (id, name, birth_year, address, profession, parish) in samples {
        println!("\nID: {}", id);
        println!("  Name: {}", name);
        println!("  Birth: {}", birth_year.map(|y| y.to_string()).unwrap_or("?".to_string()));
        println!("  Address: {}", address.unwrap_or("None".to_string()));
        println!("  Profession: {}", profession.unwrap_or("None".to_string()));
        println!("  Parish: {}", parish.unwrap_or("None".to_string()));
    }

    Ok(())
}
