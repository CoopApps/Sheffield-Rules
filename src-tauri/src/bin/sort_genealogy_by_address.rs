use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
struct GenealogyRecord {
    name: String,
    address: Option<String>,
    birth_year: Option<i64>,
    age: Option<i64>,
    profession: Option<String>,
    relation: Option<String>,
    spouse: Option<String>,
}

fn extract_street_and_number(address: &str) -> (String, i32) {
    let trimmed = address.trim();

    // Try to extract leading number
    let mut chars = trimmed.chars();
    let mut number_str = String::new();

    while let Some(c) = chars.next() {
        if c.is_numeric() {
            number_str.push(c);
        } else if !number_str.is_empty() {
            break;
        }
    }

    // Extract street name (everything after the number)
    let street = if !number_str.is_empty() {
        trimmed[number_str.len()..].trim().to_lowercase()
    } else {
        trimmed.to_lowercase()
    };

    let number: i32 = number_str.parse().unwrap_or(0);

    (street, number)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("SORTING GENEALOGY BY ADDRESS");
    println!("========================================\n");

    // Count records
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unmatched_genealogy")
        .fetch_one(&pool)
        .await?;

    println!("Total records: {}\n", count.0);

    println!("Loading all records from database...");

    // Load all records
    let records: Vec<GenealogyRecord> = sqlx::query_as(
        "SELECT name, address, birth_year, age, profession, relation, spouse
         FROM unmatched_genealogy"
    )
    .fetch_all(&pool)
    .await?;

    println!("✓ Loaded {} records\n", records.len());
    println!("Sorting by street name alphabetically...");

    // Sort records by street name, then by house number
    let mut sorted_records = records;
    sorted_records.sort_by(|a, b| {
        let addr_a = a.address.as_deref().unwrap_or("");
        let addr_b = b.address.as_deref().unwrap_or("");

        let (street_a, num_a) = extract_street_and_number(addr_a);
        let (street_b, num_b) = extract_street_and_number(addr_b);

        // First compare streets alphabetically
        match street_a.cmp(&street_b) {
            std::cmp::Ordering::Equal => {
                // If streets are the same, compare house numbers
                num_a.cmp(&num_b)
            }
            other => other,
        }
    });

    println!("✓ Sorted {} records\n", sorted_records.len());

    // Create a new table with the same structure
    println!("Creating genealogy_sorted table...");

    sqlx::query("DROP TABLE IF EXISTS genealogy_sorted")
        .execute(&pool)
        .await?;

    sqlx::query(
        "CREATE TABLE genealogy_sorted (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            address TEXT,
            birth_year INT,
            age INT,
            profession TEXT,
            relation TEXT,
            spouse TEXT
        )"
    )
    .execute(&pool)
    .await?;

    println!("✓ Created table\n");
    println!("Inserting sorted records...");

    // Insert sorted records
    let mut inserted = 0;
    for (i, record) in sorted_records.iter().enumerate() {
        sqlx::query(
            "INSERT INTO genealogy_sorted (name, address, birth_year, age, profession, relation, spouse)
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&record.name)
        .bind(&record.address)
        .bind(record.birth_year)
        .bind(record.age)
        .bind(&record.profession)
        .bind(&record.relation)
        .bind(&record.spouse)
        .execute(&pool)
        .await?;

        inserted += 1;

        if (i + 1) % 10000 == 0 {
            println!("  Inserted {}/{} records...", i + 1, sorted_records.len());
        }
    }

    println!("✓ Inserted {} records\n", inserted);

    // Show sample of sorted addresses
    println!("Sample of sorted addresses (first 50):\n");

    let samples: Vec<(String, String)> = sqlx::query_as(
        "SELECT name, address FROM genealogy_sorted WHERE address IS NOT NULL AND address != '' LIMIT 50"
    )
    .fetch_all(&pool)
    .await?;

    for (i, (name, address)) in samples.iter().enumerate() {
        println!("{}. {} - {}", i + 1, name, address);
    }

    println!("\n========================================");
    println!("SORTING COMPLETE");
    println!("========================================");
    println!("Original table: unmatched_genealogy");
    println!("Sorted table: genealogy_sorted");
    println!("Records: {}", inserted);

    Ok(())
}
