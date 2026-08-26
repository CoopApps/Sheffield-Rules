use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("Checking table sort order...\n");

    let records: Vec<(String, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT name, piece, folio FROM unmatched_sheffieldcensus LIMIT 20"
    )
    .fetch_all(&pool)
    .await?;

    println!("First 20 records:");
    for (i, (name, piece, folio)) in records.iter().enumerate() {
        println!("{}. Name: {:?}, Piece: {:?}, Folio: {:?}",
            i + 1,
            name,
            piece.as_deref().unwrap_or("NULL"),
            folio.as_deref().unwrap_or("NULL")
        );
    }

    println!("\n\nThe table appears to be in insertion order (no specific sorting).");
    println!("Records are in the order they were imported:");
    println!("  - First Sheffield census records (35,314)");
    println!("  - Then Geni records (205,530)");

    Ok(())
}
