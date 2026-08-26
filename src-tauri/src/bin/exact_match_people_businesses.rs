use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("EXACT MATCHING: PEOPLE TO BUSINESSES");
    println!("Name & Address must match exactly");
    println!("========================================\n");

    // Find exact matches using SQL JOIN
    let matches: Vec<(String, String, Option<String>, String, String, String)> = sqlx::query_as(
        "SELECT 
            p.id as person_id,
            p.name,
            p.profession,
            b.id as business_id,
            b.full_name,
            b.address
         FROM sheffield_people p
         INNER JOIN sheffield_businesses b
         ON LOWER(TRIM(p.name)) = LOWER(TRIM(b.full_name))
         WHERE p.civil_parish IS NOT NULL 
         AND b.address IS NOT NULL
         AND (
             LOWER(TRIM(p.civil_parish)) = LOWER(TRIM(b.address))
             OR LOWER(TRIM(p.registration_district)) = LOWER(TRIM(b.address))
         )"
    )
    .fetch_all(&pool)
    .await?;

    println!("EXACT MATCHES FOUND: {}\n", matches.len());
    println!("========================================\n");

    for (idx, (person_id, name, profession, business_id, biz_name, biz_address)) in matches.iter().enumerate().take(50) {
        println!("MATCH #{}", idx + 1);
        println!("  Person: {} (ID: {})", name, person_id);
        if let Some(prof) = profession {
            println!("    Profession: {}", prof);
        }
        println!("  Business: {} (ID: {})", biz_name, business_id);
        println!("    Address: {}", biz_address);
        println!();
    }

    if matches.len() > 50 {
        println!("... and {} more matches\n", matches.len() - 50);
    }

    Ok(())
}
