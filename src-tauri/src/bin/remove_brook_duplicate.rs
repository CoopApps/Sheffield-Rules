use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("Finding Elizabeth Brook duplicate records...\n");

    // Get all Elizabeth Brook records
    let brook_records: Vec<(String, String, Option<String>, Option<i64>)> = sqlx::query_as(
        "SELECT id, name, profession, birth_year FROM sheffield_people WHERE name = 'Elizabeth Brook' ORDER BY profession DESC"
    )
    .fetch_all(&pool)
    .await?;

    println!("Found {} Elizabeth Brook records:", brook_records.len());
    for (id, name, profession, birth_year) in &brook_records {
        println!("  ID: {} - {} (born {}, profession: {})",
            id,
            name,
            birth_year.map(|y| y.to_string()).unwrap_or("?".to_string()),
            profession.as_deref().unwrap_or("none")
        );
    }

    if brook_records.len() <= 1 {
        println!("\nNo duplicates to remove!");
        return Ok(());
    }

    // Keep the one WITH profession (Bricklayer), delete the one without
    println!("\nKeeping the record with profession (Bricklayer), deleting the one without profession...");

    // Delete the record without profession
    let deleted = sqlx::query(
        "DELETE FROM sheffield_people WHERE name = 'Elizabeth Brook' AND (profession IS NULL OR profession = '')"
    )
    .execute(&pool)
    .await?;

    println!("✓ Deleted {} duplicate Elizabeth Brook record(s)", deleted.rows_affected());

    // Verify
    let remaining: Vec<(String, Option<String>)> = sqlx::query_as(
        "SELECT name, profession FROM sheffield_people WHERE name = 'Elizabeth Brook'"
    )
    .fetch_all(&pool)
    .await?;

    println!("\nRemaining Elizabeth Brook records:");
    for (name, profession) in remaining {
        println!("  {} - {}", name, profession.unwrap_or("none".to_string()));
    }

    // Show updated female count
    let female_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_people WHERE gender = 'Female'"
    )
    .fetch_one(&pool)
    .await?;

    println!("\n========================================");
    println!("Updated female count: {}", female_count.0);
    println!("========================================");

    Ok(())
}
