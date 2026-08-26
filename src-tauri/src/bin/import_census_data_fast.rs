use sqlx::sqlite::SqlitePool;
use csv::ReaderBuilder;
use std::fs;
use uuid::Uuid;
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("FAST IMPORT: NO DUPLICATE CHECKING");
    println!("========================================\n");

    // Step 1: Import Sheffield Census CSVs
    println!("Step 1: Importing Sheffield Census CSVs...\n");

    let census_dir = "D:/projects/Saturday at Three/sheffield census";
    let census_files: Vec<_> = fs::read_dir(census_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().extension()
                .and_then(|s| s.to_str())
                .map(|s| s.eq_ignore_ascii_case("csv"))
                .unwrap_or(false)
        })
        .collect();

    println!("Found {} Sheffield census CSV files\n", census_files.len());

    let mut total_imported = 0;
    let mut file_count = 0;

    for entry in census_files {
        let path = entry.path();
        file_count += 1;

        if file_count % 10 == 0 {
            println!("Processing file {}/{}...", file_count, 77);
        }

        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_path(&path)?;

        let headers = rdr.headers()?.clone();

        for result in rdr.records() {
            let record = result?;

            let name = record.get(headers.iter().position(|h| h == "NAME" || h == "Name").unwrap_or(0))
                .unwrap_or("").trim();

            if name.is_empty() {
                continue;
            }

            let age_str = record.get(headers.iter().position(|h| h == "AGE" || h == "Age").unwrap_or(0))
                .unwrap_or("");
            let age = age_str.parse::<i64>().ok();

            let birth_year_str = record.get(headers.iter().position(|h| h == "ESTIMATED BIRTH YEAR").unwrap_or(0))
                .unwrap_or("");
            let birth_year = birth_year_str.parse::<i64>().ok();

            let birth_place = record.get(headers.iter().position(|h| h == "Birth Place" || h == "WHERE BORN").unwrap_or(0))
                .filter(|s| !s.trim().is_empty());
            let civil_parish = record.get(headers.iter().position(|h| h == "CIVIL PARISH").unwrap_or(0))
                .filter(|s| !s.trim().is_empty());
            let ecclesiastical_parish = record.get(headers.iter().position(|h| h == "ECCLESIASTICAL PARISH").unwrap_or(0))
                .filter(|s| !s.trim().is_empty());
            let registration_district = record.get(headers.iter().position(|h| h == "REGISTRATION DISTRICT").unwrap_or(0))
                .filter(|s| !s.trim().is_empty());
            let piece = record.get(headers.iter().position(|h| h == "PIECE").unwrap_or(0))
                .filter(|s| !s.trim().is_empty());
            let folio = record.get(headers.iter().position(|h| h == "FOLIO").unwrap_or(0))
                .filter(|s| !s.trim().is_empty());
            let page = record.get(headers.iter().position(|h| h == "PAGE NUMBER").unwrap_or(0))
                .filter(|s| !s.trim().is_empty());
            let relation = record.get(headers.iter().position(|h| h == "RELATION").unwrap_or(0))
                .filter(|s| !s.trim().is_empty());
            let gender = record.get(headers.iter().position(|h| h == "GENDER").unwrap_or(0))
                .filter(|s| !s.trim().is_empty());
            let household_members = record.get(headers.iter().position(|h| h == "HOUSEHOLD MEMBERS").unwrap_or(0))
                .filter(|s| !s.trim().is_empty());

            let id = Uuid::new_v4().to_string();
            let created_at = Utc::now().to_rfc3339();

            sqlx::query(
                "INSERT INTO unmatched_sheffieldcensus (
                    id, name, age, birth_place, civil_parish, ecclesiastical_parish,
                    registration_district, piece, folio, page, relation, gender,
                    household_members, birth_year, year, source, created_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(&id)
            .bind(name)
            .bind(age)
            .bind(birth_place)
            .bind(civil_parish)
            .bind(ecclesiastical_parish)
            .bind(registration_district)
            .bind(piece)
            .bind(folio)
            .bind(page)
            .bind(relation)
            .bind(gender)
            .bind(household_members)
            .bind(birth_year)
            .bind(1871)
            .bind("sheffield")
            .bind(&created_at)
            .execute(&pool)
            .await?;

            total_imported += 1;
        }
    }

    println!("\n✓ Imported {} records from Sheffield Census\n", total_imported);

    // Step 2: Import Geni CSVs (NO duplicate checking - much faster!)
    println!("Step 2: Importing ALL Geni CSVs (no duplicate checking)...\n");

    let geni_dir = "D:/projects/Saturday at Three/Geni";
    let geni_files: Vec<_> = fs::read_dir(geni_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().extension()
                .and_then(|s| s.to_str())
                .map(|s| s.eq_ignore_ascii_case("csv"))
                .unwrap_or(false)
        })
        .collect();

    let geni_total_files = geni_files.len();
    println!("Found {} Geni CSV files\n", geni_total_files);

    let mut geni_imported = 0;
    let mut geni_file_count = 0;

    for entry in geni_files {
        let path = entry.path();
        geni_file_count += 1;

        if geni_file_count % 10 == 0 {
            println!("Processing Geni file {}/{}... (Imported: {})",
                geni_file_count, geni_total_files, geni_imported);
        }

        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_path(&path)?;

        for result in rdr.records() {
            let record = result?;

            let surname = record.get(0).unwrap_or("").trim();
            let fore_name = record.get(1).unwrap_or("").trim();

            if surname.is_empty() && fore_name.is_empty() {
                continue;
            }

            // Combine surname and fore name
            let full_name = if !surname.is_empty() && !fore_name.is_empty() {
                format!("{} {}", fore_name, surname)
            } else if !fore_name.is_empty() {
                fore_name.to_string()
            } else {
                surname.to_string()
            };

            let age_str = record.get(2).unwrap_or("");
            let age = age_str.parse::<i64>().ok();

            let registration_district = record.get(3).unwrap_or("").trim();
            let piece = record.get(4).unwrap_or("").trim();
            let folio = record.get(5).unwrap_or("").trim();

            // Calculate birth year from age (assuming census year 1871)
            let birth_year = age.map(|a| 1871 - a);

            let id = Uuid::new_v4().to_string();
            let created_at = Utc::now().to_rfc3339();

            sqlx::query(
                "INSERT INTO unmatched_sheffieldcensus (
                    id, name, age, registration_district, piece, folio, birth_year, year, source, created_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(&id)
            .bind(&full_name)
            .bind(age)
            .bind(if registration_district.is_empty() { None } else { Some(registration_district) })
            .bind(if piece.is_empty() { None } else { Some(piece) })
            .bind(if folio.is_empty() { None } else { Some(folio) })
            .bind(birth_year)
            .bind(1871)
            .bind("geni")
            .bind(&created_at)
            .execute(&pool)
            .await?;

            geni_imported += 1;
        }
    }

    println!("\n✓ Imported {} records from Geni\n", geni_imported);

    // Final count
    let final_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_sheffieldcensus"
    )
    .fetch_one(&pool)
    .await?;

    let sheffield_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_sheffieldcensus WHERE source = 'sheffield'"
    )
    .fetch_one(&pool)
    .await?;

    let geni_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_sheffieldcensus WHERE source = 'geni'"
    )
    .fetch_one(&pool)
    .await?;

    println!("========================================");
    println!("IMPORT COMPLETE");
    println!("========================================");
    println!("Total records in unmatched_sheffieldcensus: {}", final_count.0);
    println!("  - From Sheffield Census: {}", sheffield_count.0);
    println!("  - From Geni: {}", geni_count.0);
    println!("\nNote: Use SQL to identify duplicates later by matching name+piece+folio");

    Ok(())
}
