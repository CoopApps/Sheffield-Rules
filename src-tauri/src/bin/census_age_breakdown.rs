use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    // Total count
    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_sheffieldcensus"
    )
    .fetch_one(&pool)
    .await?;

    // Count with NULL birth year
    let null_birth: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_sheffieldcensus WHERE birth_year IS NULL"
    )
    .fetch_one(&pool)
    .await?;

    // Age breakdown (calculated from birth year, assuming census is 1871)
    let age_ranges: Vec<(String, i64)> = sqlx::query_as(
        r#"
        SELECT
            CASE
                WHEN birth_year IS NULL THEN 'Unknown'
                WHEN (1871 - birth_year) < 18 THEN 'Under 18'
                WHEN (1871 - birth_year) BETWEEN 18 AND 25 THEN '18-25'
                WHEN (1871 - birth_year) BETWEEN 26 AND 35 THEN '26-35'
                WHEN (1871 - birth_year) BETWEEN 36 AND 45 THEN '36-45'
                WHEN (1871 - birth_year) BETWEEN 46 AND 55 THEN '46-55'
                WHEN (1871 - birth_year) BETWEEN 56 AND 65 THEN '56-65'
                WHEN (1871 - birth_year) >= 66 THEN '66+'
                ELSE 'Invalid'
            END as age_range,
            COUNT(*) as count
        FROM unmatched_sheffieldcensus
        GROUP BY age_range
        ORDER BY
            CASE
                WHEN age_range = 'Unknown' THEN 0
                WHEN age_range = 'Under 18' THEN 1
                WHEN age_range = '18-25' THEN 2
                WHEN age_range = '26-35' THEN 3
                WHEN age_range = '36-45' THEN 4
                WHEN age_range = '46-55' THEN 5
                WHEN age_range = '56-65' THEN 6
                WHEN age_range = '66+' THEN 7
                ELSE 8
            END
        "#
    )
    .fetch_all(&pool)
    .await?;

    // Birth year distribution
    let birth_year_ranges: Vec<(String, i64)> = sqlx::query_as(
        r#"
        SELECT
            CASE
                WHEN birth_year IS NULL THEN 'Unknown'
                WHEN birth_year < 1800 THEN 'Before 1800'
                WHEN birth_year BETWEEN 1800 AND 1809 THEN '1800-1809'
                WHEN birth_year BETWEEN 1810 AND 1819 THEN '1810-1819'
                WHEN birth_year BETWEEN 1820 AND 1829 THEN '1820-1829'
                WHEN birth_year BETWEEN 1830 AND 1839 THEN '1830-1839'
                WHEN birth_year BETWEEN 1840 AND 1849 THEN '1840-1849'
                WHEN birth_year BETWEEN 1850 AND 1859 THEN '1850-1859'
                WHEN birth_year BETWEEN 1860 AND 1869 THEN '1860-1869'
                WHEN birth_year >= 1870 THEN '1870+'
                ELSE 'Invalid'
            END as birth_decade,
            COUNT(*) as count
        FROM unmatched_sheffieldcensus
        GROUP BY birth_decade
        ORDER BY
            CASE
                WHEN birth_decade = 'Unknown' THEN 0
                WHEN birth_decade = 'Before 1800' THEN 1
                WHEN birth_decade = '1800-1809' THEN 2
                WHEN birth_decade = '1810-1819' THEN 3
                WHEN birth_decade = '1820-1829' THEN 4
                WHEN birth_decade = '1830-1839' THEN 5
                WHEN birth_decade = '1840-1849' THEN 6
                WHEN birth_decade = '1850-1859' THEN 7
                WHEN birth_decade = '1860-1869' THEN 8
                WHEN birth_decade = '1870+' THEN 9
                ELSE 10
            END
        "#
    )
    .fetch_all(&pool)
    .await?;

    println!("========================================");
    println!("UNMATCHED CENSUS AGE BREAKDOWN");
    println!("========================================");
    println!("Total unmatched census records: {}", total.0);
    println!("Records with unknown birth year: {}", null_birth.0);
    println!();

    println!("AGE DISTRIBUTION (as of 1871 census):");
    println!("----------------------------------------");
    for (range, count) in &age_ranges {
        let percentage = ((*count as f64) / (total.0 as f64)) * 100.0;
        println!("{:12} {:6} ({:5.1}%)", range, count, percentage);
    }

    println!();
    println!("BIRTH YEAR DISTRIBUTION:");
    println!("----------------------------------------");
    for (decade, count) in &birth_year_ranges {
        let percentage = ((*count as f64) / (total.0 as f64)) * 100.0;
        println!("{:15} {:6} ({:5.1}%)", decade, count, percentage);
    }

    println!("========================================");

    Ok(())
}
