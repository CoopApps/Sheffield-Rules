use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("SHEFFIELD_PEOPLE TABLE STRUCTURE\n");
    println!("Columns:");
    let cols: Vec<(i32, String, String, i32, Option<String>, i32)> = sqlx::query_as(
        "SELECT * FROM pragma_table_info('sheffield_people')"
    )
    .fetch_all(&pool)
    .await?;

    for (_, name, type_, notnull, default_val, pk) in &cols {
        let pk_marker = if *pk > 0 { " [PRIMARY KEY]" } else { "" };
        let notnull_marker = if *notnull > 0 { " NOT NULL" } else { "" };
        println!("  {}{}{} ({})", name, pk_marker, notnull_marker, type_);
    }

    println!("\n\nSample records showing combined data:");
    let samples: Vec<(
        String,      // id
        String,      // name
        Option<String>, // address
        Option<String>, // profession
        Option<String>, // spouse (from genealogy)
        Option<String>, // civil_parish (from census)
        Option<String>, // source
        Option<i64>,    // birth_year
    )> = sqlx::query_as(
        r#"
        SELECT
            id, name, street_address, profession,
            (SELECT spouse FROM unmatched_genealogy WHERE name = sheffield_people.name LIMIT 1) as spouse,
            civil_parish, source, birth_year
        FROM sheffield_people
        WHERE profession IS NOT NULL
        LIMIT 5
        "#
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    for (id, name, address, profession, spouse, parish, source, birth_year) in samples {
        println!("\nID: {}", id);
        println!("  Name: {}", name);
        println!("  Birth Year: {}", birth_year.map(|y| y.to_string()).unwrap_or("?".to_string()));
        println!("  Address: {}", address.unwrap_or("None".to_string()));
        println!("  Profession: {}", profession.unwrap_or("None".to_string()));
        println!("  Spouse: {}", spouse.unwrap_or("None".to_string()));
        println!("  Parish: {}", parish.unwrap_or("None".to_string()));
        println!("  Source: {}", source.unwrap_or("None".to_string()));
    }

    println!("\n\nSource field values:");
    let sources: Vec<(Option<String>, i64)> = sqlx::query_as(
        "SELECT source, COUNT(*) as count FROM sheffield_people GROUP BY source"
    )
    .fetch_all(&pool)
    .await?;

    for (source, count) in sources {
        println!("  {}: {}", source.unwrap_or("NULL".to_string()), count);
    }

    Ok(())
}
