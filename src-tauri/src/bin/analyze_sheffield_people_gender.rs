use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_people"
    )
    .fetch_one(&pool)
    .await?;

    // Check if there's a gender column
    let gender_col: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM pragma_table_info('sheffield_people') WHERE name = 'gender' OR name = 'sex'"
    )
    .fetch_all(&pool)
    .await?;

    println!("========================================");
    println!("SHEFFIELD_PEOPLE GENDER ANALYSIS");
    println!("========================================");
    println!("Total records: {}", total.0);
    println!();

    if !gender_col.is_empty() {
        let col_name = &gender_col[0].0;
        println!("Gender column found: {}", col_name);

        let gender_breakdown: Vec<(Option<String>, i64)> = sqlx::query_as(
            &format!("SELECT {}, COUNT(*) FROM sheffield_people GROUP BY {}", col_name, col_name)
        )
        .fetch_all(&pool)
        .await?;

        println!("\nGender breakdown:");
        for (gender, count) in &gender_breakdown {
            let percentage = (*count as f64 / total.0 as f64) * 100.0;
            println!("  {:10} {:6} ({:.1}%)",
                gender.as_deref().unwrap_or("NULL"),
                count,
                percentage
            );
        }

        // Show sample of each gender
        for (gender, _) in &gender_breakdown {
            let gender_filter = match gender {
                Some(g) => format!("WHERE {} = '{}'", col_name, g),
                None => format!("WHERE {} IS NULL", col_name),
            };

            println!("\nSample {} records:", gender.as_deref().unwrap_or("NULL"));
            let samples: Vec<(String, Option<String>, Option<i64>)> = sqlx::query_as(
                &format!("SELECT name, profession, birth_year FROM sheffield_people {} LIMIT 10", gender_filter)
            )
            .fetch_all(&pool)
            .await?;

            for (name, profession, birth_year) in samples {
                println!("  {} (born {}, {})",
                    name,
                    birth_year.map(|y| y.to_string()).unwrap_or("?".to_string()),
                    profession.unwrap_or("no profession".to_string())
                );
            }
        }
    } else {
        println!("No explicit gender/sex column found in sheffield_people table.");

        // Try to infer from names
        println!("\nAttempting to infer gender from first names...");

        // Common women's names
        let likely_women: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM sheffield_people
            WHERE first_name IN (
                'Elizabeth', 'Mary', 'Ann', 'Sarah', 'Jane', 'Hannah',
                'Margaret', 'Emma', 'Charlotte', 'Alice', 'Ellen', 'Martha',
                'Emily', 'Eliza', 'Annie', 'Catherine', 'Harriet', 'Patience',
                'Ruth', 'Rachel'
            )
            "#
        )
        .fetch_one(&pool)
        .await?;

        // Common men's names
        let likely_men: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) FROM sheffield_people
            WHERE first_name IN (
                'William', 'John', 'George', 'Thomas', 'James', 'Henry',
                'Joseph', 'Charles', 'Robert', 'Edward', 'Samuel', 'Benjamin',
                'Richard', 'Frederick', 'Alfred', 'David', 'Arthur', 'Francis'
            )
            "#
        )
        .fetch_one(&pool)
        .await?;

        let unknown = total.0 - likely_women.0 - likely_men.0;

        println!("\nInferred gender (based on common first names):");
        println!("  Likely women:  {} ({:.1}%)", likely_women.0, (likely_women.0 as f64 / total.0 as f64) * 100.0);
        println!("  Likely men:    {} ({:.1}%)", likely_men.0, (likely_men.0 as f64 / total.0 as f64) * 100.0);
        println!("  Unknown:       {} ({:.1}%)", unknown, (unknown as f64 / total.0 as f64) * 100.0);
    }

    // Check source breakdown
    println!("\n========================================");
    println!("BY SOURCE:");
    println!("========================================");
    let by_source: Vec<(Option<String>, i64)> = sqlx::query_as(
        "SELECT source, COUNT(*) FROM sheffield_people GROUP BY source"
    )
    .fetch_all(&pool)
    .await?;

    for (source, count) in by_source {
        let percentage = (count as f64 / total.0 as f64) * 100.0;
        println!("{:30} {:6} ({:.1}%)",
            source.unwrap_or("NULL".to_string()),
            count,
            percentage
        );
    }

    println!("========================================");

    Ok(())
}
