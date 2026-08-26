use sqlx::sqlite::SqlitePool;
use regex::Regex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite:D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(database_url).await?;

    println!("Checking address formats in unmatched_genealogy...\n");

    // Get all addresses
    let addresses: Vec<(String,)> = sqlx::query_as(
        "SELECT DISTINCT address FROM unmatched_genealogy WHERE address IS NOT NULL AND address != ''"
    )
    .fetch_all(&pool)
    .await?;

    // Pattern: number followed by comma with no space, then capital letter (like "70,SnigHill")
    let comma_no_space = Regex::new(r"^\d+,[A-Z]").unwrap();
    
    let mut comma_no_space_count = 0;
    let mut examples = Vec::new();

    for (addr,) in &addresses {
        if comma_no_space.is_match(addr) {
            comma_no_space_count += 1;
            if examples.len() < 20 {
                examples.push(addr.clone());
            }
        }
    }

    println!("Total unique addresses: {}", addresses.len());
    println!("Addresses with format 'number,CapitalLetter': {}", comma_no_space_count);
    println!("Percentage: {:.2}%\n", (comma_no_space_count as f64 / addresses.len() as f64) * 100.0);

    if !examples.is_empty() {
        println!("Examples of 'number,CapitalLetter' format:");
        for (i, addr) in examples.iter().enumerate() {
            println!("  {}. {}", i + 1, addr);
        }
    }

    Ok(())
}
