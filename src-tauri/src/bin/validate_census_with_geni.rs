use sqlx::sqlite::SqlitePool;
use csv::ReaderBuilder;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("VALIDATING CENSUS AGAINST GENI CSVs");
    println!("Using Piece + Folio + Age");
    println!("========================================\n");

    // Get all CSV files from geni folder
    let geni_folder = "D:/projects/Saturday at Three/geni";
    let paths = fs::read_dir(geni_folder)?;

    let mut csv_files = Vec::new();
    for path in paths {
        let path = path?.path();
        if path.extension().and_then(|s| s.to_str()) == Some("csv") {
            csv_files.push(path);
        }
    }

    println!("Found {} CSV files in /geni folder\n", csv_files.len());

    // Create a validation results table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS geni_census_validation (
            census_id TEXT,
            census_name TEXT,
            census_age INTEGER,
            census_piece TEXT,
            census_folio TEXT,
            geni_surname TEXT,
            geni_forename TEXT,
            geni_age INTEGER,
            geni_piece INTEGER,
            geni_folio INTEGER,
            geni_csv_file TEXT,
            match_type TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    // Clear existing validation data
    sqlx::query("DELETE FROM geni_census_validation")
        .execute(&pool)
        .await?;

    let mut total_csv_records = 0;
    let mut total_matches = 0;
    let mut perfect_matches = 0;
    let mut name_corrections = 0;

    // Process each CSV file
    for (idx, csv_path) in csv_files.iter().enumerate() {
        let file_name = csv_path.file_name().unwrap().to_str().unwrap();
        println!("[{}/{}] Processing: {}", idx + 1, csv_files.len(), file_name);

        let file = match fs::File::open(&csv_path) {
            Ok(f) => f,
            Err(e) => {
                println!("  Error opening file: {}", e);
                continue;
            }
        };

        let mut rdr = ReaderBuilder::new().from_reader(file);

        for result in rdr.records() {
            let record = match result {
                Ok(r) => r,
                Err(_) => continue,
            };

            total_csv_records += 1;

            // Parse CSV record
            let surname = record.get(0).unwrap_or("");
            let forename = record.get(1).unwrap_or("");
            let age_str = record.get(2).unwrap_or("");
            let piece_str = record.get(5).unwrap_or("");
            let folio_str = record.get(6).unwrap_or("");

            let age = age_str.trim().replace("?", "").parse::<i64>().ok();
            let piece = piece_str.trim().parse::<i64>().ok();
            let folio = folio_str.trim().parse::<i64>().ok();

            if let (Some(age), Some(piece), Some(folio)) = (age, piece, folio) {
                // Try to find match in unmatched_sheffieldcensus
                let census_matches: Vec<(String, String, i64, String, String)> = sqlx::query_as(
                    "SELECT id, name, age, piece, folio
                     FROM unmatched_sheffieldcensus
                     WHERE piece = ? AND folio = ? AND age = ?
                     LIMIT 1"
                )
                .bind(piece.to_string())
                .bind(folio.to_string())
                .bind(age)
                .fetch_all(&pool)
                .await?;

                if let Some((census_id, census_name, census_age, census_piece, census_folio)) = census_matches.first() {
                    total_matches += 1;

                    // Determine match type
                    let geni_full_name = format!("{} {}", forename, surname).trim().to_string();
                    let match_type = if census_name == &geni_full_name {
                        perfect_matches += 1;
                        "PERFECT_MATCH"
                    } else if surname.contains("?") || surname.contains("-") {
                        name_corrections += 1;
                        "NAME_CORRECTION"
                    } else {
                        "VARIANT"
                    };

                    // Save validation result
                    sqlx::query(
                        "INSERT INTO geni_census_validation
                         (census_id, census_name, census_age, census_piece, census_folio,
                          geni_surname, geni_forename, geni_age, geni_piece, geni_folio,
                          geni_csv_file, match_type)
                         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
                    )
                    .bind(census_id)
                    .bind(census_name)
                    .bind(census_age)
                    .bind(census_piece)
                    .bind(census_folio)
                    .bind(surname)
                    .bind(forename)
                    .bind(age)
                    .bind(piece)
                    .bind(folio)
                    .bind(file_name)
                    .bind(match_type)
                    .execute(&pool)
                    .await?;
                }
            }
        }
    }

    println!("\n========================================");
    println!("VALIDATION SUMMARY");
    println!("========================================");
    println!("Total geni CSV records processed: {}", total_csv_records);
    println!("Total matches found: {}", total_matches);
    println!("  Perfect matches (names identical): {}", perfect_matches);
    println!("  Name corrections (geni has damaged name): {}", name_corrections);
    println!("  Variants (different but valid names): {}", total_matches - perfect_matches - name_corrections);
    println!("\nMatch rate: {:.2}%", (total_matches as f64 / total_csv_records as f64) * 100.0);

    // Show some examples of name corrections
    println!("\n========================================");
    println!("EXAMPLE NAME CORRECTIONS");
    println!("========================================\n");

    let corrections: Vec<(String, String, String, String, String)> = sqlx::query_as(
        "SELECT census_name, geni_surname, geni_forename, census_piece, census_folio
         FROM geni_census_validation
         WHERE match_type = 'NAME_CORRECTION'
         LIMIT 20"
    )
    .fetch_all(&pool)
    .await?;

    for (census_name, geni_surname, geni_forename, piece, folio) in &corrections {
        println!("Piece: {}, Folio: {}", piece, folio);
        println!("  Database (correct): {}", census_name);
        println!("  Geni CSV (damaged): {} {}", geni_forename, geni_surname);
        println!();
    }

    // Also show some name variants
    println!("========================================");
    println!("EXAMPLE NAME VARIANTS");
    println!("(Same person, different name spelling)");
    println!("========================================\n");

    let variants: Vec<(String, String, String, String, String)> = sqlx::query_as(
        "SELECT census_name, geni_surname, geni_forename, census_piece, census_folio
         FROM geni_census_validation
         WHERE match_type = 'VARIANT'
         LIMIT 20"
    )
    .fetch_all(&pool)
    .await?;

    for (census_name, geni_surname, geni_forename, piece, folio) in &variants {
        let geni_name = format!("{} {}", geni_forename, geni_surname).trim().to_string();
        println!("Piece: {}, Folio: {}", piece, folio);
        println!("  Database: {}", census_name);
        println!("  Geni CSV: {}", geni_name);
        println!();
    }

    // Count how many unique census records were validated
    let unique_validated: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT census_id) FROM geni_census_validation"
    )
    .fetch_one(&pool)
    .await?;

    let total_census: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_sheffieldcensus"
    )
    .fetch_one(&pool)
    .await?;

    println!("\n========================================");
    println!("COVERAGE");
    println!("========================================");
    println!("Total census records in database: {}", total_census.0);
    println!("Census records validated by geni CSVs: {}", unique_validated.0);
    println!("Coverage: {:.2}%", (unique_validated.0 as f64 / total_census.0 as f64) * 100.0);

    println!("\nResults saved to table: geni_census_validation");

    Ok(())
}
