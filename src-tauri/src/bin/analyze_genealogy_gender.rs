use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy"
    )
    .fetch_one(&pool)
    .await?;

    // Records WITH spouse (likely men - heads of household)
    let with_spouse: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy WHERE spouse IS NOT NULL AND spouse != ''"
    )
    .fetch_one(&pool)
    .await?;

    // Records WITHOUT spouse (likely women/children/unmarried)
    let without_spouse: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy WHERE spouse IS NULL OR spouse = ''"
    )
    .fetch_one(&pool)
    .await?;

    // Check if there's a gender column
    let gender_col: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM pragma_table_info('unmatched_genealogy') WHERE name = 'gender' OR name = 'sex'"
    )
    .fetch_all(&pool)
    .await?;

    println!("========================================");
    println!("UNMATCHED GENEALOGY GENDER BREAKDOWN");
    println!("========================================");
    println!("Total records: {}", total.0);
    println!();
    println!("WITH spouse data: {} ({:.1}%) - likely MEN (heads of household)",
        with_spouse.0,
        (with_spouse.0 as f64 / total.0 as f64) * 100.0
    );
    println!("WITHOUT spouse data: {} ({:.1}%) - likely WOMEN/children/unmarried",
        without_spouse.0,
        (without_spouse.0 as f64 / total.0 as f64) * 100.0
    );
    println!();

    if !gender_col.is_empty() {
        println!("Gender column found: {}", gender_col[0].0);

        let gender_breakdown: Vec<(Option<String>, i64)> = sqlx::query_as(
            &format!("SELECT {}, COUNT(*) FROM unmatched_genealogy GROUP BY {}",
                gender_col[0].0, gender_col[0].0)
        )
        .fetch_all(&pool)
        .await?;

        println!("\nActual gender breakdown:");
        for (gender, count) in gender_breakdown {
            println!("  {}: {}", gender.unwrap_or("NULL".to_string()), count);
        }
    } else {
        println!("No explicit gender/sex column found in table.");
    }

    // Sample records without spouse
    println!("\n========================================");
    println!("SAMPLE RECORDS WITHOUT SPOUSE (first 20):");
    println!("========================================");
    let samples: Vec<(String, Option<String>, Option<i64>)> = sqlx::query_as(
        "SELECT name, address, birth_year FROM unmatched_genealogy WHERE spouse IS NULL OR spouse = '' LIMIT 20"
    )
    .fetch_all(&pool)
    .await?;

    for (name, address, birth_year) in samples {
        println!("{} (born {}, {})",
            name,
            birth_year.map(|y| y.to_string()).unwrap_or("?".to_string()),
            address.unwrap_or("no address".to_string())
        );
    }

    println!("\n========================================");
    println!("RECOMMENDATION:");
    println!("========================================");
    println!("If records WITHOUT spouse are women/dependents, we could:");
    println!("1. Match them using household members from census");
    println!("2. Filter them out if they're duplicates of heads of household");
    println!("3. Keep them as separate records if they're distinct people");
    println!("========================================");

    Ok(())
}
