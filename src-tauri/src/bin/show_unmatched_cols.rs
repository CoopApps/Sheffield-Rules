use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("UNMATCHED_SHEFFIELDCENSUS columns:");
    let census_cols: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM pragma_table_info('unmatched_sheffieldcensus')"
    )
    .fetch_all(&pool)
    .await?;

    for (col,) in &census_cols {
        println!("  {}", col);
    }

    println!("\nUNMATCHED_GENEALOGY columns:");
    let gen_cols: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM pragma_table_info('unmatched_genealogy')"
    )
    .fetch_all(&pool)
    .await?;

    for (col,) in &gen_cols {
        println!("  {}", col);
    }

    Ok(())
}
