use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite:D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(database_url).await?;

    println!("========================================");
    println!("ADDING BUSINESS COLUMNS TO GENEALOGY");
    println!("========================================\n");

    // Add business-related columns to unmatched_genealogy table
    let columns_to_add = vec![
        ("business_surname", "TEXT"),
        ("business_forename", "TEXT"),
        ("business_title", "TEXT"),
        ("business_occupation", "TEXT"),
        ("business_address", "TEXT"),
        ("business_year", "TEXT"),
        ("business_source", "TEXT"),
    ];

    for (column_name, column_type) in &columns_to_add {
        println!("Adding column: {}", column_name);

        let query = format!(
            "ALTER TABLE unmatched_genealogy ADD COLUMN {} {}",
            column_name, column_type
        );

        match sqlx::query(&query).execute(&pool).await {
            Ok(_) => println!("✓ Added column: {}", column_name),
            Err(e) => {
                if e.to_string().contains("duplicate column name") {
                    println!("  Column {} already exists, skipping", column_name);
                } else {
                    return Err(e.into());
                }
            }
        }
    }

    println!("\n========================================");
    println!("COLUMNS ADDED SUCCESSFULLY");
    println!("========================================");
    println!("\nBusiness columns added to unmatched_genealogy:");
    for (column_name, _) in &columns_to_add {
        println!("  - {}", column_name);
    }

    Ok(())
}
