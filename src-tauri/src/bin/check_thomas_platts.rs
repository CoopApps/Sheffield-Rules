use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite:D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(database_url).await?;

    println!("========================================");
    println!("THOMAS PLATTS MATCHING DETAILS");
    println!("========================================\n");

    // Get genealogy record for Thomas Platts
    let genealogy: Vec<(i64, String, String, i64, i64, String, String, String, String, String)> = sqlx::query_as(
        "SELECT id, name, address, birth_year, age, profession, 
                business_surname, business_forename, business_occupation, business_address
         FROM unmatched_genealogy 
         WHERE name LIKE '%Thomas Platts%'"
    )
    .fetch_all(&pool)
    .await?;

    println!("GENEALOGY RECORD:");
    println!("{:-<80}", "");
    for (id, name, address, birth_year, age, profession, b_surname, b_forename, b_occupation, b_address) in &genealogy {
        println!("ID: {}", id);
        println!("Name: {}", name);
        println!("Address: {}", address);
        println!("Birth Year: {}", birth_year);
        println!("Age: {}", age);
        println!("Profession: {}", profession);
        println!("\nMatched Business Info:");
        println!("  Business Name: {} {}", b_forename, b_surname);
        println!("  Business Occupation: {}", b_occupation);
        println!("  Business Address: {}", b_address);
        println!("{:-<80}", "");
    }

    // Get corresponding business record
    println!("\nBUSINESS RECORD:");
    println!("{:-<80}", "");
    let businesses: Vec<(String, String, String, String, String, String, i64, String)> = sqlx::query_as(
        "SELECT id, surname, forename, title, occupation, address, year, source
         FROM sheffield_businesses 
         WHERE surname LIKE '%Platts%' AND forename LIKE '%Thomas%' AND address LIKE '%Woodhouse%'"
    )
    .fetch_all(&pool)
    .await?;

    for (id, surname, forename, title, occupation, address, year, source) in businesses {
        println!("ID: {}", id);
        println!("Name: {} {}", forename, surname);
        println!("Title: {}", title);
        println!("Occupation: {}", occupation);
        println!("Address: {}", address);
        println!("Year: {}", year);
        println!("Source: {}", source);
        println!("{:-<80}", "");
    }

    Ok(())
}
