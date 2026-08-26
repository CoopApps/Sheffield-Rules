use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("Finding incorrectly gendered records...\n");

    // Common male first names that are marked as Female
    let male_names = vec![
        "James", "John", "William", "George", "Thomas", "Charles", "Robert",
        "Joseph", "Henry", "Edward", "Samuel", "Benjamin", "Richard", "Alfred",
        "Frederick", "David", "Arthur", "Francis", "Edwin", "Abraham", "Aaron"
    ];

    // Build query to find misgendered males
    let placeholders: Vec<String> = male_names.iter().map(|_| "?".to_string()).collect();
    let query = format!(
        "SELECT id, name, first_name, gender FROM sheffield_people WHERE gender = 'Female' AND first_name IN ({})",
        placeholders.join(", ")
    );

    let mut query_builder = sqlx::query_as::<_, (String, String, Option<String>, Option<String>)>(&query);
    for name in &male_names {
        query_builder = query_builder.bind(name);
    }

    let misgendered_males = query_builder.fetch_all(&pool).await?;

    println!("Found {} males incorrectly marked as Female:", misgendered_males.len());
    for (id, name, first_name, _) in &misgendered_males {
        println!("  {} ({})", name, first_name.as_deref().unwrap_or("?"));
    }

    if misgendered_males.is_empty() {
        println!("No corrections needed!");
        return Ok(());
    }

    println!("\nCorrecting gender to 'Male'...");

    let mut corrected = 0;
    for (id, name, _, _) in &misgendered_males {
        let result = sqlx::query(
            "UPDATE sheffield_people SET gender = 'Male' WHERE id = ?"
        )
        .bind(id)
        .execute(&pool)
        .await;

        match result {
            Ok(_) => {
                corrected += 1;
                println!("  ✓ Corrected: {}", name);
            }
            Err(e) => {
                println!("  ✗ Failed to correct {}: {}", name, e);
            }
        }
    }

    println!("\n========================================");
    println!("RESULTS:");
    println!("========================================");
    println!("Records corrected: {}", corrected);
    println!("========================================");

    // Show updated counts
    let gender_counts: Vec<(Option<String>, i64)> = sqlx::query_as(
        "SELECT gender, COUNT(*) FROM sheffield_people GROUP BY gender"
    )
    .fetch_all(&pool)
    .await?;

    println!("\nUpdated gender breakdown:");
    for (gender, count) in gender_counts {
        println!("  {:10} {}", gender.unwrap_or("NULL".to_string()), count);
    }

    Ok(())
}
