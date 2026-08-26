use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite:D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(database_url).await?;

    println!("20 Sample Cleaned Addresses:\n");
    println!("{:=<60}", "");

    let addresses: Vec<(String,)> = sqlx::query_as(
        "SELECT address FROM unmatched_genealogy
         WHERE address IS NOT NULL AND address != ''
         LIMIT 20"
    )
    .fetch_all(&pool)
    .await?;

    for (i, (address,)) in addresses.iter().enumerate() {
        println!("{}. {}", i + 1, address);
    }

    println!("{:=<60}", "");

    Ok(())
}
