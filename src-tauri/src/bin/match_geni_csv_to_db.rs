use sqlx::sqlite::SqlitePool;
use csv::ReaderBuilder;
use std::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("MATCHING GENI CSV TO DATABASE");
    println!("Using Age, Piece, Folio Numbers");
    println!("========================================\n");

    // Read one of the Geni CSVs as a test
    let csv_path = "D:/projects/Saturday at Three/geni/Sheffield_1871_A-Alk.csv";
    println!("Reading CSV: {}\n", csv_path);

    let file = File::open(csv_path)?;
    let mut rdr = ReaderBuilder::new()
        .from_reader(file);

    println!("CSV Headers:");
    if let Some(headers) = rdr.headers().ok() {
        for (i, header) in headers.iter().enumerate() {
            println!("  {}: {}", i, header);
        }
    }
    println!();

    // Process first 20 records from CSV
    println!("--- Processing Sample CSV Records ---\n");

    let mut csv_records = Vec::new();
    for result in rdr.records().take(20) {
        let record = result?;

        // CSV format: Surname, Fore Name, Age, Registration District, Sub District, Piece Number RG10, Folio Number
        let surname = record.get(0).unwrap_or("");
        let forename = record.get(1).unwrap_or("");
        let age_str = record.get(2).unwrap_or("");
        let district = record.get(3).unwrap_or("");
        let subdistrict = record.get(4).unwrap_or("");
        let piece_str = record.get(5).unwrap_or("");
        let folio_str = record.get(6).unwrap_or("");

        // Parse age, piece, folio
        let age = age_str.trim().replace("?", "").parse::<i64>().ok();
        let piece = piece_str.trim().parse::<i64>().ok();
        let folio = folio_str.trim().parse::<i64>().ok();

        println!("CSV Record:");
        println!("  Surname: '{}'", surname);
        println!("  Forename: '{}'", forename);
        println!("  Age: {} (raw: '{}')", age.map(|a| a.to_string()).unwrap_or("?".to_string()), age_str);
        println!("  District: {}", district);
        println!("  Subdistrict: {}", subdistrict);
        println!("  Piece: {} (raw: '{}')", piece.map(|p| p.to_string()).unwrap_or("?".to_string()), piece_str);
        println!("  Folio: {} (raw: '{}')", folio.map(|f| f.to_string()).unwrap_or("?".to_string()), folio_str);

        csv_records.push((surname.to_string(), forename.to_string(), age, district.to_string(), subdistrict.to_string(), piece, folio));
        println!();
    }

    // Now let's check what columns exist in the database tables
    println!("\n--- Checking Database Schema ---\n");

    // Check unmatched_genealogy schema
    println!("unmatched_genealogy columns:");
    let gen_columns: Vec<(i64, String, String, i64, Option<String>, i64)> = sqlx::query_as(
        "PRAGMA table_info(unmatched_genealogy)"
    )
    .fetch_all(&pool)
    .await?;

    for (_, col_name, col_type, _, _, _) in &gen_columns {
        println!("  {} ({})", col_name, col_type);
    }
    println!();

    // Check unmatched_sheffieldcensus schema
    println!("unmatched_sheffieldcensus columns:");
    let census_columns: Vec<(i64, String, String, i64, Option<String>, i64)> = sqlx::query_as(
        "PRAGMA table_info(unmatched_sheffieldcensus)"
    )
    .fetch_all(&pool)
    .await?;

    for (_, col_name, col_type, _, _, _) in &census_columns {
        println!("  {} ({})", col_name, col_type);
    }
    println!();

    // Try to find matches in the database using age, piece, folio
    println!("\n--- Attempting to Match CSV Records to Database ---\n");

    for (surname, forename, age_opt, _district, _subdistrict, piece_opt, folio_opt) in &csv_records {
        if let (Some(age), Some(piece), Some(folio)) = (age_opt, piece_opt, folio_opt) {
            println!("Looking for matches for '{}  {}' (age: {}, piece: {}, folio: {})",
                forename, surname, age, piece, folio);

            // Try to find in unmatched_genealogy
            // Note: We need to check what columns actually exist - piece and folio might not be there
            // For now, let's try matching on age only
            let gen_matches: Vec<(String, Option<i64>, String)> = sqlx::query_as(
                "SELECT name, age, address
                 FROM unmatched_genealogy
                 WHERE age = ?
                 LIMIT 5"
            )
            .bind(age)
            .fetch_all(&pool)
            .await?;

            if !gen_matches.is_empty() {
                println!("  Found {} potential matches in unmatched_genealogy (by age):", gen_matches.len());
                for (name, db_age, address) in &gen_matches {
                    println!("    {} (age: {}, address: {})",
                        name,
                        db_age.map(|a| a.to_string()).unwrap_or("?".to_string()),
                        address);
                }
            } else {
                println!("  No matches found in unmatched_genealogy");
            }
            println!();
        }
    }

    Ok(())
}
