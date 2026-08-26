use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("UNMATCHED_SHEFFIELDCENSUS TABLE\n");
    println!("Columns:");
    let census_cols: Vec<(String, String)> = sqlx::query_as(
        "SELECT name, type FROM pragma_table_info('unmatched_sheffieldcensus')"
    )
    .fetch_all(&pool)
    .await?;

    for (name, type_) in &census_cols {
        println!("  {} ({})", name, type_);
    }

    println!("\nSample records:");
    let census_samples: Vec<(String, Option<String>, Option<String>, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT id, name, address, profession, parish FROM unmatched_sheffieldcensus LIMIT 3"
    )
    .fetch_all(&pool)
    .await?;

    for (id, name, address, profession, parish) in census_samples {
        println!("\nID: {}", id);
        println!("  Name: {}", name.unwrap_or("None".to_string()));
        println!("  Address: {}", address.unwrap_or("None".to_string()));
        println!("  Profession: {}", profession.unwrap_or("None".to_string()));
        println!("  Parish: {}", parish.unwrap_or("None".to_string()));
    }

    println!("\n\n================================================");
    println!("UNMATCHED_GENEALOGY TABLE\n");
    println!("Columns:");
    let gen_cols: Vec<(String, String)> = sqlx::query_as(
        "SELECT name, type FROM pragma_table_info('unmatched_genealogy')"
    )
    .fetch_all(&pool)
    .await?;

    for (name, type_) in &gen_cols {
        println!("  {} ({})", name, type_);
    }

    println!("\nSample records:");
    let gen_samples: Vec<(String, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT id, name, address, profession, spouse, relation FROM unmatched_genealogy LIMIT 3"
    )
    .fetch_all(&pool)
    .await?;

    for (id, name, address, profession, spouse, relation) in gen_samples {
        println!("\nID: {}", id);
        println!("  Name: {}", name.unwrap_or("None".to_string()));
        println!("  Address: {}", address.unwrap_or("None".to_string()));
        println!("  Profession: {}", profession.unwrap_or("None".to_string()));
        println!("  Spouse: {}", spouse.unwrap_or("None".to_string()));
        println!("  Relation: {}", relation.unwrap_or("None".to_string()));
    }

    Ok(())
}
