use sqlx::sqlite::SqlitePool;
use csv::ReaderBuilder;
use std::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("MATCHING GENI CSV USING PIECE+FOLIO+AGE");
    println!("========================================\n");

    // Read one of the Geni CSVs as a test
    let csv_path = "D:/projects/Saturday at Three/geni/Sheffield_1871_A-Alk.csv";
    println!("Reading CSV: {}\n", csv_path);

    let file = File::open(csv_path)?;
    let mut rdr = ReaderBuilder::new()
        .from_reader(file);

    // Process first 50 records from CSV
    println!("--- Matching CSV Records to Database ---\n");

    let mut total_records = 0;
    let mut found_in_census = 0;
    let mut found_in_genealogy = 0;

    for result in rdr.records().take(50) {
        let record = result?;

        // CSV format: Surname, Fore Name, Age, Registration District, Sub District, Piece Number RG10, Folio Number
        let surname = record.get(0).unwrap_or("");
        let forename = record.get(1).unwrap_or("");
        let age_str = record.get(2).unwrap_or("");
        let piece_str = record.get(5).unwrap_or("");
        let folio_str = record.get(6).unwrap_or("");

        // Parse age, piece, folio
        let age = age_str.trim().replace("?", "").parse::<i64>().ok();
        let piece = piece_str.trim().parse::<i64>().ok();
        let folio = folio_str.trim().parse::<i64>().ok();

        total_records += 1;

        if let (Some(age), Some(piece), Some(folio)) = (age, piece, folio) {
            // Try to find in unmatched_sheffieldcensus using piece, folio, AND age
            let census_matches: Vec<(String, i64, String, String, String)> = sqlx::query_as(
                "SELECT name, age, piece, folio, civil_parish
                 FROM unmatched_sheffieldcensus
                 WHERE piece = ? AND folio = ? AND age = ?
                 LIMIT 5"
            )
            .bind(piece.to_string())
            .bind(folio.to_string())
            .bind(age)
            .fetch_all(&pool)
            .await?;

            if !census_matches.is_empty() {
                found_in_census += 1;
                println!("CSV: '{}' '{}' (age: {}, piece: {}, folio: {})",
                    forename, surname, age, piece, folio);
                println!("  MATCHES IN CENSUS ({} found):", census_matches.len());
                for (name, db_age, db_piece, db_folio, parish) in &census_matches {
                    println!("    {} (age: {}, piece: {}, folio: {}, parish: {})",
                        name, db_age, db_piece, db_folio, parish);
                }
                println!();
            }
        }
    }

    println!("\n========================================");
    println!("SUMMARY");
    println!("========================================");
    println!("Total CSV records processed: {}", total_records);
    println!("Records found in unmatched_sheffieldcensus: {}", found_in_census);
    println!("Match rate: {:.1}%", (found_in_census as f64 / total_records as f64) * 100.0);

    // Now let's see how many records we have in unmatched_sheffieldcensus
    let total_census: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_sheffieldcensus"
    )
    .fetch_one(&pool)
    .await?;

    println!("\nTotal records in unmatched_sheffieldcensus: {}", total_census.0);

    // Check how many have piece and folio
    let with_piece_folio: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_sheffieldcensus
         WHERE piece IS NOT NULL AND piece != '' AND folio IS NOT NULL AND folio != ''"
    )
    .fetch_one(&pool)
    .await?;

    println!("Records with piece+folio data: {} ({:.1}%)",
        with_piece_folio.0,
        (with_piece_folio.0 as f64 / total_census.0 as f64) * 100.0);

    Ok(())
}
