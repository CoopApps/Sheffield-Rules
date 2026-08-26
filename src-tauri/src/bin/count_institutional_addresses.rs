use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    // Get total count with addresses
    let total_with_address: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy WHERE address IS NOT NULL AND address != ''"
    )
    .fetch_one(&pool)
    .await?;

    // Count workhouse addresses
    let workhouse_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy
         WHERE address LIKE '%workhouse%' OR address LIKE '%work house%'"
    )
    .fetch_one(&pool)
    .await?;

    // Count asylum addresses
    let asylum_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy
         WHERE address LIKE '%asylum%'"
    )
    .fetch_one(&pool)
    .await?;

    // Count hospital addresses
    let hospital_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy
         WHERE address LIKE '%hospital%' OR address LIKE '%infirmary%'"
    )
    .fetch_one(&pool)
    .await?;

    // Count prison/jail addresses
    let prison_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy
         WHERE address LIKE '%prison%' OR address LIKE '%jail%' OR address LIKE '%gaol%'"
    )
    .fetch_one(&pool)
    .await?;

    // Count orphanage/children's home addresses
    let orphanage_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy
         WHERE address LIKE '%orphanage%' OR address LIKE '%children%home%' OR address LIKE '%orphan%'"
    )
    .fetch_one(&pool)
    .await?;

    // Count barracks/military addresses
    let barracks_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy
         WHERE address LIKE '%barracks%' OR address LIKE '%barrick%'"
    )
    .fetch_one(&pool)
    .await?;

    // Count poor house/union addresses
    let poorhouse_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy
         WHERE address LIKE '%poor%house%' OR address LIKE '%union%workhouse%' OR address LIKE '%poor law%'"
    )
    .fetch_one(&pool)
    .await?;

    // Count institution addresses (general)
    let institution_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy
         WHERE address LIKE '%institution%' OR address LIKE '%institute%'"
    )
    .fetch_one(&pool)
    .await?;

    println!("========================================");
    println!("INSTITUTIONAL ADDRESSES IN UNMATCHED_GENEALOGY");
    println!("========================================");
    println!("Total with addresses: {}", total_with_address.0);
    println!();
    println!("Workhouse: {}", workhouse_count.0);
    println!("Asylum: {}", asylum_count.0);
    println!("Hospital/Infirmary: {}", hospital_count.0);
    println!("Prison/Jail/Gaol: {}", prison_count.0);
    println!("Orphanage/Children's Home: {}", orphanage_count.0);
    println!("Barracks (military): {}", barracks_count.0);
    println!("Poor House/Union: {}", poorhouse_count.0);
    println!("Institution (general): {}", institution_count.0);
    println!("========================================");

    // Show some sample institutional addresses
    println!("\nSAMPLE WORKHOUSE ADDRESSES:");
    println!("----------------------------------------");
    let workhouse_samples: Vec<(String, String)> = sqlx::query_as(
        "SELECT name, address FROM unmatched_genealogy
         WHERE address LIKE '%workhouse%' OR address LIKE '%work house%'
         LIMIT 10"
    )
    .fetch_all(&pool)
    .await?;

    for (name, address) in workhouse_samples {
        println!("{}: {}", name, address);
    }

    println!("\nSAMPLE ASYLUM ADDRESSES:");
    println!("----------------------------------------");
    let asylum_samples: Vec<(String, String)> = sqlx::query_as(
        "SELECT name, address FROM unmatched_genealogy
         WHERE address LIKE '%asylum%'
         LIMIT 10"
    )
    .fetch_all(&pool)
    .await?;

    for (name, address) in asylum_samples {
        println!("{}: {}", name, address);
    }

    println!("\nSAMPLE HOSPITAL ADDRESSES:");
    println!("----------------------------------------");
    let hospital_samples: Vec<(String, String)> = sqlx::query_as(
        "SELECT name, address FROM unmatched_genealogy
         WHERE address LIKE '%hospital%' OR address LIKE '%infirmary%'
         LIMIT 10"
    )
    .fetch_all(&pool)
    .await?;

    for (name, address) in hospital_samples {
        println!("{}: {}", name, address);
    }

    Ok(())
}
