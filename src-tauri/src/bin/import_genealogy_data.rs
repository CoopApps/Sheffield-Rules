use sqlx::sqlite::SqlitePool;
use csv::ReaderBuilder;
use std::fs;

#[derive(Debug, serde::Deserialize)]
struct GenealogyRecord {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Spouse")]
    spouse: String,
    #[serde(rename = "Address")]
    address: String,
    #[serde(rename = "Parish")]
    parish: String,
    #[serde(rename = "Area")]
    area: String,
    #[serde(rename = "Age")]
    age: String,
    #[serde(rename = "Born Approx")]
    born_approx: String,
    #[serde(rename = "Birth Place")]
    birth_place: String,
    #[serde(rename = "Relation")]
    relation: String,
    #[serde(rename = "Profession")]
    profession: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    let genealogy_dir = "D:/projects/Saturday at Three/genealogy";

    println!("========================================");
    println!("IMPORTING GENEALOGY DATA");
    println!("========================================\n");

    // Get all CSV files
    let mut csv_files: Vec<_> = fs::read_dir(genealogy_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "csv" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    csv_files.sort();

    println!("Found {} CSV files\n", csv_files.len());

    // Clear existing data
    println!("Clearing existing genealogy data...");
    sqlx::query("DELETE FROM unmatched_genealogy")
        .execute(&pool)
        .await?;
    println!("✓ Cleared existing data\n");

    let mut total_imported = 0;
    let mut file_count = 0;
    let total_files = csv_files.len();

    for csv_path in csv_files {
        file_count += 1;
        let filename = csv_path.file_name().unwrap().to_string_lossy();

        println!("Processing file {}/{}: {}", file_count, total_files, filename);

        let mut reader = match ReaderBuilder::new()
            .has_headers(true)
            .from_path(&csv_path) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("ERROR: Failed to open file {}: {}", filename, e);
                    continue;
                }
            };

        let mut records_in_file = 0;

        for result in reader.deserialize::<GenealogyRecord>() {
            match result {
                Ok(record) => {
                    // Parse age
                    let age: Option<i64> = record.age.trim().parse().ok();

                    // Parse birth year
                    let birth_year: Option<i64> = record.born_approx.trim().parse().ok();

                    // Insert into database (only fields that exist in table)
                    match sqlx::query(
                        "INSERT INTO unmatched_genealogy
                         (name, address, birth_year, age, profession, relation, spouse)
                         VALUES (?, ?, ?, ?, ?, ?, ?)"
                    )
                    .bind(&record.name)
                    .bind(if record.address.is_empty() { None } else { Some(&record.address) })
                    .bind(birth_year)
                    .bind(age)
                    .bind(if record.profession.is_empty() { None } else { Some(&record.profession) })
                    .bind(if record.relation.is_empty() { None } else { Some(&record.relation) })
                    .bind(if record.spouse.is_empty() { None } else { Some(&record.spouse) })
                    .execute(&pool)
                    .await {
                        Ok(_) => {
                            records_in_file += 1;
                        }
                        Err(e) => {
                            eprintln!("ERROR: Failed to insert record from {}: {}", filename, e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Failed to parse record in {}: {}", filename, e);
                }
            }
        }

        println!("  ✓ Imported {} records from {}", records_in_file, filename);
        total_imported += records_in_file;
    }

    println!("\n========================================");
    println!("IMPORT COMPLETE");
    println!("========================================");
    println!("Files processed: {}", file_count);
    println!("Total records imported: {}", total_imported);

    // Verify
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unmatched_genealogy")
        .fetch_one(&pool)
        .await?;

    println!("Records in database: {}", count.0);

    Ok(())
}
