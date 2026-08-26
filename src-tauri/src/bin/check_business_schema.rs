use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite:D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(database_url).await?;

    println!("Checking sheffield_businesses table schema...\n");

    // Get table info
    let rows: Vec<(i64, String, String, i64, Option<String>, i64)> = sqlx::query_as(
        "PRAGMA table_info(sheffield_businesses)"
    )
    .fetch_all(&pool)
    .await?;

    println!("Columns in sheffield_businesses:");
    println!("{:-<60}", "");
    for (cid, name, col_type, notnull, _dflt_value, pk) in &rows {
        println!("  {} | {} | {} | NOT NULL: {} | PK: {}",
            cid, name, col_type, notnull, pk);
    }
    println!("{:-<60}", "");

    // Get row count
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_businesses")
        .fetch_one(&pool)
        .await?;
    println!("\nTotal records: {}", count.0);

    // Get sample records
    let samples: Vec<(String, String, String, String, String, String, String)> = sqlx::query_as(
        "SELECT surname, forename, title, occupation, address, year, source FROM sheffield_businesses LIMIT 5"
    )
    .fetch_all(&pool)
    .await?;

    println!("\nSample records:");
    println!("{:=<100}", "");
    for (surname, forename, title, occupation, address, year, source) in samples {
        println!("Name: {} {}", forename, surname);
        println!("Title: {}", title);
        println!("Occupation: {}", occupation);
        println!("Address: {}", address);
        println!("Year: {} | Source: {}", year, source);
        println!("{:-<100}", "");
    }

    Ok(())
}
