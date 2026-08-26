use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite:D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(database_url).await?;

    println!("Checking Thomas Platts at Woodhouse - business field values:");
    println!("================================================================\n");

    let result: Option<(String, String, String, String, String, String, String)> = sqlx::query_as(
        "SELECT business_surname, business_forename, business_title, 
                business_occupation, business_address, business_year, business_source
         FROM unmatched_genealogy 
         WHERE name = 'Thomas Platts' AND address = 'Woodhouse'"
    )
    .fetch_optional(&pool)
    .await?;

    if let Some((surname, forename, title, occupation, address, year, source)) = result {
        println!("business_surname: '{}'", surname);
        println!("business_forename: '{}'", forename);
        println!("business_title: '{}'", title);
        println!("business_occupation: '{}'", occupation);
        println!("business_address: '{}'", address);
        println!("business_year: '{}'", year);
        println!("business_source: '{}'", source);
    } else {
        println!("No record found for Thomas Platts at Woodhouse");
    }

    Ok(())
}
