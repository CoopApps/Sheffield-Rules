use sqlx::sqlite::SqlitePool;
use csv::ReaderBuilder;
use std::fs;
use uuid::Uuid;
use chrono::Utc;

struct SheffieldCensusRecord {
    name: String,
    age: Option<i64>,
    birth_place: Option<String>,
    civil_parish: Option<String>,
    ecclesiastical_parish: Option<String>,
    registration_district: Option<String>,
    piece: Option<String>,
    folio: Option<String>,
    page: Option<String>,
    relation: Option<String>,
    gender: Option<String>,
    household_members: Option<String>,
    birth_year: Option<i64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("IMPORTING CENSUS DATA");
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
            println!("Processing file {}/77...", file_count);
        }

        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_path(&path)?;

        let headers = rdr.headers()?.clone();

        for result in rdr.records() {
            let record = result?;

            // Parse the CSV record
            let name = record.get(headers.iter().position(|h| h == "NAME" || h == "Name").unwrap_or(0))
                .unwrap_or("").to_string();

            if name.trim().is_empty() {
                continue;
            }

            let age_str = record.get(headers.iter().position(|h| h == "AGE" || h == "Age").unwrap_or(0))
                .unwrap_or("");
            let age = age_str.parse::<i64>().ok();

            let birth_year_str = record.get(headers.iter().position(|h| h == "ESTIMATED BIRTH YEAR").unwrap_or(0))
                .unwrap_or("");
            let birth_year = birth_year_str.parse::<i64>().ok();

            let census_record = SheffieldCensusRecord {
                name: name.trim().to_string(),
                age,
                birth_place: record.get(headers.iter().position(|h| h == "Birth Place" || h == "WHERE BORN").unwrap_or(0))
                    .map(|s| s.to_string()).filter(|s| !s.is_empty()),
                civil_parish: record.get(headers.iter().position(|h| h == "CIVIL PARISH").unwrap_or(0))
                    .map(|s| s.to_string()).filter(|s| !s.is_empty()),
                ecclesiastical_parish: record.get(headers.iter().position(|h| h == "ECCLESIASTICAL PARISH").unwrap_or(0))
                    .map(|s| s.to_string()).filter(|s| !s.is_empty()),
                registration_district: record.get(headers.iter().position(|h| h == "REGISTRATION DISTRICT").unwrap_or(0))
                    .map(|s| s.to_string()).filter(|s| !s.is_empty()),
                piece: record.get(headers.iter().position(|h| h == "PIECE").unwrap_or(0))
                    .map(|s| s.to_string()).filter(|s| !s.is_empty()),
                folio: record.get(headers.iter().position(|h| h == "FOLIO").unwrap_or(0))
                    .map(|s| s.to_string()).filter(|s| !s.is_empty()),
                page: record.get(headers.iter().position(|h| h == "PAGE NUMBER").unwrap_or(0))
                    .map(|s| s.to_string()).filter(|s| !s.is_empty()),
                relation: record.get(headers.iter().position(|h| h == "RELATION").unwrap_or(0))
                    .map(|s| s.to_string()).filter(|s| !s.is_empty()),
                gender: record.get(headers.iter().position(|h| h == "GENDER").unwrap_or(0))
                    .map(|s| s.to_string()).filter(|s| !s.is_empty()),
                household_members: record.get(headers.iter().position(|h| h == "HOUSEHOLD MEMBERS").unwrap_or(0))
                    .map(|s| s.to_string()).filter(|s| !s.is_empty()),
                birth_year,
            };

            // Insert into database
            let id = Uuid::new_v4().to_string();
            let created_at = Utc::now().to_rfc3339();

            sqlx::query(
                "INSERT INTO unmatched_sheffieldcensus (
                    id, name, age, birth_place, civil_parish, ecclesiastical_parish,
                    registration_district, piece, folio, page, relation, gender,
                    household_members, birth_year, year, created_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(&id)
            .bind(&census_record.name)
            .bind(census_record.age)
            .bind(&census_record.birth_place)
            .bind(&census_record.civil_parish)
            .bind(&census_record.ecclesiastical_parish)
            .bind(&census_record.registration_district)
            .bind(&census_record.piece)
            .bind(&census_record.folio)
            .bind(&census_record.page)
            .bind(&census_record.relation)
            .bind(&census_record.gender)
            .bind(&census_record.household_members)
            .bind(census_record.birth_year)
            .bind(1871) // Census year 1871
            .bind(&created_at)
            .execute(&pool)
            .await?;

            total_imported += 1;
        }
    }

    println!("\n✓ Imported {} records from Sheffield Census\n", total_imported);

    // Step 2: Import Geni CSVs (avoiding duplicates)
    println!("Step 2: Importing Geni CSVs (checking for duplicates)...\n");

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
    let mut geni_duplicates = 0;
    let mut geni_file_count = 0;

    for entry in geni_files {
        let path = entry.path();
        geni_file_count += 1;

        if geni_file_count % 20 == 0 {
            println!("Processing Geni file {}/{}... (Imported: {}, Duplicates: {})",
                geni_file_count, geni_total_files, geni_imported, geni_duplicates);
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

            let piece = record.get(4).unwrap_or("").trim().to_string();
            let folio = record.get(5).unwrap_or("").trim().to_string();

            // Check if this record already exists in the database
            // Match on: name + piece + folio
            let exists: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM unmatched_sheffieldcensus
                 WHERE LOWER(TRIM(name)) = LOWER(TRIM(?))
                 AND TRIM(COALESCE(piece, '')) = TRIM(?)
                 AND TRIM(COALESCE(folio, '')) = TRIM(?)"
            )
            .bind(&full_name)
            .bind(&piece)
            .bind(&folio)
            .fetch_one(&pool)
            .await?;

            if exists.0 > 0 {
                geni_duplicates += 1;
                continue; // Skip this record as it's a duplicate
            }

            // Insert the Geni record
            let id = Uuid::new_v4().to_string();
            let created_at = Utc::now().to_rfc3339();

            let registration_district = record.get(3).unwrap_or("").trim();
            let _sub_district = record.get(4).map(|s| s.trim().to_string()).filter(|s| !s.is_empty());

            // Calculate birth year from age (assuming census year 1871 for Geni data)
            let birth_year = age.map(|a| 1871 - a);

            sqlx::query(
                "INSERT INTO unmatched_sheffieldcensus (
                    id, name, age, registration_district, piece, folio, birth_year, year, created_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(&id)
            .bind(&full_name)
            .bind(age)
            .bind(if registration_district.is_empty() { None } else { Some(registration_district) })
            .bind(if piece.is_empty() { None } else { Some(piece) })
            .bind(if folio.is_empty() { None } else { Some(folio) })
            .bind(birth_year)
            .bind(1871) // Census year 1871
            .bind(&created_at)
            .execute(&pool)
            .await?;

            geni_imported += 1;
        }
    }

    println!("\n✓ Imported {} unique records from Geni", geni_imported);
    println!("✓ Skipped {} duplicate records\n", geni_duplicates);

    // Final count
    let final_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_sheffieldcensus"
    )
    .fetch_one(&pool)
    .await?;

    println!("========================================");
    println!("IMPORT COMPLETE");
    println!("========================================");
    println!("Total records in unmatched_sheffieldcensus: {}", final_count.0);
    println!("  - From Sheffield Census: {}", total_imported);
    println!("  - From Geni (unique): {}", geni_imported);
    println!("  - Geni duplicates skipped: {}", geni_duplicates);

    Ok(())
}
