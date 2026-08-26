use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("COUNTING RECORDS IN TABLES");
    println!("========================================\n");

    // Count sheffield_people
    let people_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_people"
    )
    .fetch_one(&pool)
    .await?;

    println!("Total records in sheffield_people: {}", people_count.0);

    // Count sheffield_businesses
    let business_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_businesses"
    )
    .fetch_one(&pool)
    .await?;

    println!("Total records in sheffield_businesses: {}", business_count.0);

    // Sample some records from each table
    println!("\n========================================");
    println!("SAMPLE RECORDS FROM SHEFFIELD_PEOPLE:");
    println!("========================================\n");

    let people_samples: Vec<(String, Option<String>, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT name, profession, civil_parish, registration_district
         FROM sheffield_people
         LIMIT 5"
    )
    .fetch_all(&pool)
    .await?;

    for (name, prof, parish, district) in &people_samples {
        println!("Name: {}", name);
        if let Some(p) = prof {
            println!("  Profession: {}", p);
        }
        if let Some(c) = parish {
            println!("  Civil Parish: {}", c);
        }
        if let Some(d) = district {
            println!("  Registration District: {}", d);
        }
        println!();
    }

    println!("========================================");
    println!("SAMPLE RECORDS FROM SHEFFIELD_BUSINESSES:");
    println!("========================================\n");

    let business_samples: Vec<(String, String, String)> = sqlx::query_as(
        "SELECT full_name, address, occupation
         FROM sheffield_businesses
         LIMIT 5"
    )
    .fetch_all(&pool)
    .await?;

    for (name, address, occupation) in &business_samples {
        println!("Name: {}", name);
        println!("  Address: {}", address);
        println!("  Occupation: {}", occupation);
        println!();
    }

    Ok(())
}
