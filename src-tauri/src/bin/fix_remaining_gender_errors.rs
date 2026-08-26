use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("Fixing remaining gender errors...\n");

    // Fix Benjamine (variant of Benjamin - male name)
    let benjamine_count = sqlx::query(
        "UPDATE sheffield_people SET gender = 'Male' WHERE first_name = 'Benjamine' AND gender = 'Female'"
    )
    .execute(&pool)
    .await?;

    println!("✓ Fixed {} Benjamine records (Male name)", benjamine_count.rows_affected());

    // Fix Walter (clearly male name)
    let walter_count = sqlx::query(
        "UPDATE sheffield_people SET gender = 'Male' WHERE first_name = 'Walter' AND gender = 'Female'"
    )
    .execute(&pool)
    .await?;

    println!("✓ Fixed {} Walter records (Male name)", walter_count.rows_affected());

    // Show updated counts
    println!("\n========================================");
    println!("UPDATED GENDER BREAKDOWN:");
    println!("========================================");

    let gender_counts: Vec<(Option<String>, i64)> = sqlx::query_as(
        "SELECT gender, COUNT(*) FROM sheffield_people GROUP BY gender"
    )
    .fetch_all(&pool)
    .await?;

    let total: i64 = gender_counts.iter().map(|(_, count)| count).sum();

    for (gender, count) in &gender_counts {
        let percentage = (*count as f64 / total as f64) * 100.0;
        println!("{:10} {:6} ({:.2}%)",
            gender.as_deref().unwrap_or("NULL"),
            count,
            percentage
        );
    }

    println!("\n========================================");
    println!("REMAINING FEMALE RECORDS:");
    println!("========================================");

    let remaining_women: Vec<(String, Option<String>, Option<i64>, Option<String>)> = sqlx::query_as(
        "SELECT name, first_name, birth_year, profession FROM sheffield_people WHERE gender = 'Female' ORDER BY name"
    )
    .fetch_all(&pool)
    .await?;

    println!("Total: {} women\n", remaining_women.len());

    for (name, first_name, birth_year, profession) in remaining_women {
        println!("{} ({}, born {}, {})",
            name,
            first_name.unwrap_or("?".to_string()),
            birth_year.map(|y| y.to_string()).unwrap_or("?".to_string()),
            profession.unwrap_or("no profession".to_string())
        );
    }

    println!("\n========================================");

    Ok(())
}
