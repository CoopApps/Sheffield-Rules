use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("Analyzing unmatched_genealogy table...\n");

    // Check columns in the table
    let columns: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM pragma_table_info('unmatched_genealogy')"
    )
    .fetch_all(&pool)
    .await?;

    println!("Columns in unmatched_genealogy:");
    for (col,) in &columns {
        println!("  - {}", col);
    }

    // Total count
    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy"
    )
    .fetch_one(&pool)
    .await?;

    println!("\nTotal records: {}", total.0);

    // Count by gender if gender column exists
    let gender_exists = columns.iter().any(|(col,)| col.to_lowercase() == "gender" || col.to_lowercase() == "sex");

    if gender_exists {
        // Try different possible gender column names
        let male_count_query = r#"
            SELECT COUNT(*) FROM unmatched_genealogy
            WHERE gender = 'M' OR gender = 'Male' OR gender = 'male'
        "#;

        let male: (i64,) = sqlx::query_as(male_count_query)
            .fetch_one(&pool)
            .await
            .unwrap_or((0,));

        let female_count_query = r#"
            SELECT COUNT(*) FROM unmatched_genealogy
            WHERE gender = 'F' OR gender = 'Female' OR gender = 'female'
        "#;

        let female: (i64,) = sqlx::query_as(female_count_query)
            .fetch_one(&pool)
            .await
            .unwrap_or((0,));

        let unknown = total.0 - male.0 - female.0;

        println!("\nGender breakdown:");
        println!("  Men (M/Male): {}", male.0);
        println!("  Women (F/Female): {}", female.0);
        println!("  Unknown/Other: {}", unknown);
    } else {
        println!("\nNo gender column found in unmatched_genealogy table");
    }

    // Sample a few records
    println!("\nSample records:");
    let samples: Vec<(String,)> = sqlx::query_as(
        "SELECT * FROM unmatched_genealogy LIMIT 3"
    )
    .fetch_all(&pool)
    .await?;

    for (sample,) in samples {
        println!("  {}", sample);
    }

    Ok(())
}
