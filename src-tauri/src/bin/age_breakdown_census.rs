use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("AGE BREAKDOWN - UNMATCHED_SHEFFIELDCENSUS");
    println!("========================================\n");

    // Total count
    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_sheffieldcensus"
    )
    .fetch_one(&pool)
    .await?;

    println!("Total records: {}\n", total.0);

    // Age breakdown by ranges
    println!("Age Range Breakdown:");
    println!("-------------------");

    let age_ranges = vec![
        ("0-10", 0, 10),
        ("11-20", 11, 20),
        ("21-30", 21, 30),
        ("31-40", 31, 40),
        ("41-50", 41, 50),
        ("51-60", 51, 60),
        ("61-70", 61, 70),
        ("71-80", 71, 80),
        ("81-90", 81, 90),
        ("91-100", 91, 100),
        ("100+", 100, 999),
    ];

    for (label, min_age, max_age) in age_ranges {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM unmatched_sheffieldcensus
             WHERE age >= ? AND age <= ?"
        )
        .bind(min_age)
        .bind(max_age)
        .fetch_one(&pool)
        .await?;

        let percentage = (count.0 as f64 / total.0 as f64) * 100.0;
        println!("{:8}: {:5} ({:5.2}%)", label, count.0, percentage);
    }

    // NULL ages
    let null_ages: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_sheffieldcensus
         WHERE age IS NULL"
    )
    .fetch_one(&pool)
    .await?;

    if null_ages.0 > 0 {
        let percentage = (null_ages.0 as f64 / total.0 as f64) * 100.0;
        println!("{:8}: {:5} ({:5.2}%)", "NULL", null_ages.0, percentage);
    }

    // Most common ages
    println!("\n========================================");
    println!("TOP 20 MOST COMMON AGES");
    println!("========================================\n");

    let top_ages: Vec<(i64, i64)> = sqlx::query_as(
        "SELECT age, COUNT(*) as count
         FROM unmatched_sheffieldcensus
         WHERE age IS NOT NULL
         GROUP BY age
         ORDER BY count DESC
         LIMIT 20"
    )
    .fetch_all(&pool)
    .await?;

    for (age, count) in &top_ages {
        let percentage = (*count as f64 / total.0 as f64) * 100.0;
        println!("Age {:3}: {:5} people ({:5.2}%)", age, count, percentage);
    }

    // Average age
    let avg_age: (Option<f64>,) = sqlx::query_as(
        "SELECT AVG(age) FROM unmatched_sheffieldcensus WHERE age IS NOT NULL"
    )
    .fetch_one(&pool)
    .await?;

    println!("\n========================================");
    println!("STATISTICS");
    println!("========================================");
    if let Some(avg) = avg_age.0 {
        println!("Average age: {:.1}", avg);
    }

    // Median age
    let median: (i64,) = sqlx::query_as(
        "SELECT age FROM unmatched_sheffieldcensus
         WHERE age IS NOT NULL
         ORDER BY age
         LIMIT 1
         OFFSET (SELECT COUNT(*) FROM unmatched_sheffieldcensus WHERE age IS NOT NULL) / 2"
    )
    .fetch_one(&pool)
    .await?;

    println!("Median age: {}", median.0);

    // Min/Max
    let min_age: (Option<i64>,) = sqlx::query_as(
        "SELECT MIN(age) FROM unmatched_sheffieldcensus WHERE age IS NOT NULL"
    )
    .fetch_one(&pool)
    .await?;

    let max_age: (Option<i64>,) = sqlx::query_as(
        "SELECT MAX(age) FROM unmatched_sheffieldcensus WHERE age IS NOT NULL"
    )
    .fetch_one(&pool)
    .await?;

    if let Some(min) = min_age.0 {
        println!("Minimum age: {}", min);
    }
    if let Some(max) = max_age.0 {
        println!("Maximum age: {}", max);
    }

    Ok(())
}
