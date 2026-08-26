use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("ALL 12 WOMEN IN SHEFFIELD_PEOPLE");
    println!("========================================\n");

    let women: Vec<(
        String,           // name
        Option<String>,   // first_name
        Option<i64>,      // birth_year
        Option<String>,   // profession
        Option<String>,   // street_address
        Option<String>,   // civil_parish
        Option<String>,   // source
    )> = sqlx::query_as(
        "SELECT name, first_name, birth_year, profession, street_address, civil_parish, source
         FROM sheffield_people
         WHERE gender = 'Female'
         ORDER BY name"
    )
    .fetch_all(&pool)
    .await?;

    for (i, (name, first_name, birth_year, profession, address, parish, source)) in women.iter().enumerate() {
        println!("{}. {}", i + 1, name);
        println!("   First name: {}", first_name.as_deref().unwrap_or("?"));
        println!("   Born: {}", birth_year.map(|y| y.to_string()).unwrap_or("?".to_string()));
        println!("   Profession: {}", profession.as_deref().unwrap_or("none"));
        println!("   Address: {}", address.as_deref().unwrap_or("none"));
        println!("   Parish: {}", parish.as_deref().unwrap_or("none"));
        println!("   Source: {}", source.as_deref().unwrap_or("unknown"));
        println!();
    }

    println!("========================================");
    println!("Total: {} women", women.len());
    println!("========================================");

    Ok(())
}
