use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("PEOPLE IN BARRACKS (first 10):");
    println!("========================================\n");

    let barracks_people: Vec<(String, String, Option<i64>, Option<String>)> = sqlx::query_as(
        "SELECT name, address, birth_year, profession
         FROM unmatched_genealogy
         WHERE address LIKE '%barracks%' OR address LIKE '%barrick%'
         LIMIT 10"
    )
    .fetch_all(&pool)
    .await?;

    for (i, (name, address, birth_year, profession)) in barracks_people.iter().enumerate() {
        println!("{}. {}", i + 1, name);
        println!("   Address: {}", address);
        println!("   Birth year: {}", birth_year.map(|y| y.to_string()).unwrap_or("Unknown".to_string()));
        println!("   Profession: {}", profession.as_deref().unwrap_or("Unknown"));
        println!();
    }

    Ok(())
}
