use sqlx::SqlitePool;
use std::path::Path;
use std::fs;
use csv::ReaderBuilder;
use uuid::Uuid;

#[derive(Debug)]
pub struct CensusPlayer {
    pub name: String,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub surname: Option<String>,
    pub age: Option<i32>,
    pub birth_year: i32,
    pub birth_date: Option<String>,
    pub birth_place: Option<String>,
    pub civil_parish: Option<String>,
    pub country: Option<String>,
    pub county: Option<String>,
    pub ecclesiastical_parish: Option<String>,
    pub ed_institution: Option<String>,
    pub folio: Option<String>,
    pub gender: Option<String>,
    pub household_schedule: Option<String>,
    pub household_members: Option<String>,
    pub page_number: Option<String>,
    pub piece: Option<String>,
    pub registration_district: Option<String>,
    pub relation: Option<String>,
    pub sub_registration_district: Option<String>,
    pub town: Option<String>,
    pub where_born: Option<String>,
}

/// Parse a census CSV file and return player records
pub fn parse_census_csv(file_path: &str) -> Result<Vec<CensusPlayer>, Box<dyn std::error::Error>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)?;

    let headers = reader.headers()?.clone();

    // Convert headers to Vec for easier iteration
    let headers_vec: Vec<&str> = headers.iter().collect();

    // Find column indices
    let idx = |name: &str| -> Option<usize> {
        headers_vec.iter().position(|h| h.eq_ignore_ascii_case(name))
    };

    let i_age = idx("AGE");
    let i_birth_date = idx("Birth Date");
    let i_birth_place = idx("Birth Place");
    let i_civil_parish = idx("CIVIL PARISH");
    let i_country = idx("COUNTRY");
    let i_county = idx("COUNTY/ISLAND");
    let i_eccl_parish = idx("ECCLESIASTICAL PARISH");
    let i_ed = idx("ED, INSTITUTION, OR VESSEL");
    let i_birth_year = idx("ESTIMATED BIRTH YEAR");
    let i_folio = idx("FOLIO");
    let i_gender = idx("GENDER");
    let i_household = idx("HOUSEHOLD SCHEDULE NUMBER");
    let i_household_members = idx("HOUSEHOLD MEMBERS");
    let i_name = headers_vec.iter().rposition(|h| h.eq_ignore_ascii_case("Name")); // Last "Name" column
    let i_page = idx("PAGE NUMBER");
    let i_piece = idx("PIECE");
    let i_reg_dist = idx("REGISTRATION DISTRICT");
    let i_relation = idx("RELATION");
    let i_sub_reg = idx("SUB-REGISTRATION DISTRICT");
    let i_town = idx("TOWN");
    let i_where_born = idx("WHERE BORN");

    let mut players = Vec::new();

    for result in reader.records() {
        let record = result?;

        let get = |idx: Option<usize>| -> Option<String> {
            idx.and_then(|i| record.get(i))
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().to_string())
        };

        // Parse birth year
        let birth_year_str = get(i_birth_year).or_else(|| get(i_birth_date));
        let birth_year = match birth_year_str.and_then(|s| s.parse::<i32>().ok()) {
            Some(year) if year >= 1700 && year <= 1900 => year,
            _ => continue, // Skip invalid birth years
        };

        // Get full name
        let full_name = get(i_name).unwrap_or_else(|| "Unknown".to_string());

        // Parse name into components
        let (first_name, middle_name, surname) = parse_name(&full_name);

        // Parse age
        let age = get(i_age).and_then(|s| s.parse::<i32>().ok());

        players.push(CensusPlayer {
            name: full_name,
            first_name: Some(first_name),
            middle_name,
            surname: Some(surname),
            age,
            birth_year,
            birth_date: get(i_birth_date),
            birth_place: get(i_birth_place),
            civil_parish: get(i_civil_parish),
            country: get(i_country),
            county: get(i_county),
            ecclesiastical_parish: get(i_eccl_parish),
            ed_institution: get(i_ed),
            folio: get(i_folio),
            gender: get(i_gender),
            household_schedule: get(i_household),
            household_members: get(i_household_members),
            page_number: get(i_page),
            piece: get(i_piece),
            registration_district: get(i_reg_dist),
            relation: get(i_relation),
            sub_registration_district: get(i_sub_reg),
            town: get(i_town),
            where_born: get(i_where_born),
        });
    }

    Ok(players)
}

/// Parse a full name into first, middle, and surname
fn parse_name(full_name: &str) -> (String, Option<String>, String) {
    let parts: Vec<&str> = full_name.split_whitespace().collect();

    match parts.len() {
        0 => ("Unknown".to_string(), None, "Unknown".to_string()),
        1 => (parts[0].to_string(), None, parts[0].to_string()),
        2 => (parts[0].to_string(), None, parts[1].to_string()),
        _ => {
            // First name, middle name(s), last name
            let first = parts[0].to_string();
            let surname = parts.last().unwrap().to_string();
            let middle = parts[1..parts.len()-1].join(" ");
            (first, Some(middle), surname)
        }
    }
}

/// Determine nationality from country/county
fn get_nationality(country: Option<&str>, county: Option<&str>) -> String {
    if let Some(c) = country {
        let c_lower = c.to_lowercase();
        if c_lower.contains("scotland") {
            return "Scottish".to_string();
        } else if c_lower.contains("wales") {
            return "Welsh".to_string();
        } else if c_lower.contains("ireland") {
            return "Irish".to_string();
        }
    }

    if let Some(county) = county {
        let county_lower = county.to_lowercase();
        if county_lower.contains("yorkshire") {
            return "English".to_string();
        }
    }

    "English".to_string()
}

/// Import a single census CSV file into the database
pub async fn import_census_file(
    pool: &SqlitePool,
    file_path: &str,
) -> Result<usize, Box<dyn std::error::Error>> {
    let players = parse_census_csv(file_path)?;
    let mut count = 0;

    for player in players {
        let nationality = get_nationality(
            player.country.as_deref(),
            player.county.as_deref()
        );

        // Check if player already exists (by name and birth year)
        let existing: Option<(String,)> = sqlx::query_as(
            "SELECT id FROM sheffield_footballers WHERE name = ?1 AND birth_year = ?2 LIMIT 1"
        )
        .bind(&player.name)
        .bind(player.birth_year)
        .fetch_optional(pool)
        .await?;

        if let Some((existing_id,)) = existing {
            // Update existing player with new census data
            sqlx::query(
                r#"
                UPDATE sheffield_footballers SET
                    first_name = COALESCE(?1, first_name),
                    middle_name = COALESCE(?2, middle_name),
                    surname = COALESCE(?3, surname),
                    where_born = COALESCE(?4, where_born),
                    birth_town = COALESCE(?5, birth_town),
                    birth_county = COALESCE(?6, birth_county),
                    birth_country = COALESCE(?7, birth_country),
                    civil_parish = COALESCE(?8, civil_parish),
                    ecclesiastical_parish = COALESCE(?9, ecclesiastical_parish),
                    registration_district = COALESCE(?10, registration_district),
                    sub_registration_district = COALESCE(?11, sub_registration_district),
                    census_age = COALESCE(?12, census_age),
                    census_relation = COALESCE(?13, census_relation),
                    census_gender = COALESCE(?14, census_gender),
                    census_ed = COALESCE(?15, census_ed),
                    census_household_schedule = COALESCE(?16, census_household_schedule),
                    census_household_members = COALESCE(?17, census_household_members),
                    census_piece = COALESCE(?18, census_piece),
                    census_folio = COALESCE(?19, census_folio),
                    census_page = COALESCE(?20, census_page),
                    census_birth_date = COALESCE(?21, census_birth_date),
                    census_birth_place = COALESCE(?22, census_birth_place),
                    census_county = COALESCE(?23, census_county)
                WHERE id = ?24
                "#
            )
            .bind(&player.first_name)
            .bind(&player.middle_name)
            .bind(&player.surname)
            .bind(&player.where_born)
            .bind(&player.town)
            .bind(&player.county)
            .bind(&player.country)
            .bind(&player.civil_parish)
            .bind(&player.ecclesiastical_parish)
            .bind(&player.registration_district)
            .bind(&player.sub_registration_district)
            .bind(player.age)
            .bind(&player.relation)
            .bind(&player.gender)
            .bind(&player.ed_institution)
            .bind(&player.household_schedule)
            .bind(&player.household_members)
            .bind(&player.piece)
            .bind(&player.folio)
            .bind(&player.page_number)
            .bind(&player.birth_date)
            .bind(&player.birth_place)
            .bind(&player.county)
            .bind(&existing_id)
            .execute(pool)
            .await?;
            count += 1;
        } else {
            // Insert new player
            let id = Uuid::new_v4().to_string();
            sqlx::query(
                r#"
                INSERT INTO sheffield_footballers (
                id, name, first_name, middle_name, surname,
                club_id, position, birth_year, nationality,
                where_born, birth_town, birth_county, birth_country,
                civil_parish, ecclesiastical_parish,
                registration_district, sub_registration_district,
                census_age, census_relation, census_gender,
                census_ed, census_household_schedule, census_household_members,
                census_piece, census_folio, census_page,
                census_birth_date, census_birth_place, census_county,
                has_stats, is_real_player,
                current_ability, potential_ability, current_reputation
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5,
                NULL, 'FWD', ?6, ?7,
                ?8, ?9, ?10, ?11,
                ?12, ?13,
                ?14, ?15,
                ?16, ?17, ?18,
                ?19, ?20, ?21,
                ?22, ?23, ?24,
                ?25, ?26, ?27,
                0, 1,
                50, 100, 10
            )
            "#
        )
        .bind(&id)
        .bind(&player.name)
        .bind(&player.first_name)
        .bind(&player.middle_name)
        .bind(&player.surname)
        .bind(player.birth_year)
        .bind(&nationality)
        .bind(&player.where_born)
        .bind(&player.town)
        .bind(&player.county)
        .bind(&player.country)
        .bind(&player.civil_parish)
        .bind(&player.ecclesiastical_parish)
        .bind(&player.registration_district)
        .bind(&player.sub_registration_district)
        .bind(player.age)
        .bind(&player.relation)
        .bind(&player.gender)
        .bind(&player.ed_institution)
        .bind(&player.household_schedule)
        .bind(&player.household_members)
        .bind(&player.piece)
        .bind(&player.folio)
        .bind(&player.page_number)
        .bind(&player.birth_date)
        .bind(&player.birth_place)
        .bind(&player.county)
        .execute(pool)
        .await?;
            count += 1;
        }
    }

    Ok(count)
}

/// Import all census CSV files from a directory
pub async fn import_all_census_files(
    pool: &SqlitePool,
    census_dir: &str,
) -> Result<ImportStats, Box<dyn std::error::Error>> {
    let mut stats = ImportStats::default();

    // Create backup before import
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    if let Ok(backup_path) = crate::database::backup::create_backup(db_path) {
        println!("Created backup: {}", backup_path);
    }

    // Add missing columns to sheffield_footballers table if they don't exist
    println!("Checking database schema...");
    sqlx::query("ALTER TABLE sheffield_footballers ADD COLUMN first_name TEXT")
        .execute(pool)
        .await
        .ok(); // Ignore error if column already exists

    sqlx::query("ALTER TABLE sheffield_footballers ADD COLUMN middle_name TEXT")
        .execute(pool)
        .await
        .ok(); // Ignore error if column already exists

    sqlx::query("ALTER TABLE sheffield_footballers ADD COLUMN surname TEXT")
        .execute(pool)
        .await
        .ok(); // Ignore error if column already exists

    sqlx::query("ALTER TABLE sheffield_footballers ADD COLUMN census_household_members TEXT")
        .execute(pool)
        .await
        .ok(); // Ignore error if column already exists

    sqlx::query("ALTER TABLE sheffield_footballers ADD COLUMN census_birth_date TEXT")
        .execute(pool)
        .await
        .ok(); // Ignore error if column already exists

    sqlx::query("ALTER TABLE sheffield_footballers ADD COLUMN census_birth_place TEXT")
        .execute(pool)
        .await
        .ok(); // Ignore error if column already exists

    sqlx::query("ALTER TABLE sheffield_footballers ADD COLUMN census_county TEXT")
        .execute(pool)
        .await
        .ok(); // Ignore error if column already exists

    sqlx::query("ALTER TABLE sheffield_footballers ADD COLUMN street_address TEXT")
        .execute(pool)
        .await
        .ok(); // Ignore error if column already exists

    sqlx::query("ALTER TABLE sheffield_footballers ADD COLUMN profession TEXT")
        .execute(pool)
        .await
        .ok(); // Ignore error if column already exists

    println!("Schema check complete.");

    let census_path = Path::new(census_dir);
    if !census_path.exists() {
        return Err("Census directory not found".into());
    }

    // Get all CSV files
    let mut csv_files: Vec<_> = fs::read_dir(census_path)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension()
                .and_then(|s| s.to_str())
                .map(|s| s.eq_ignore_ascii_case("csv"))
                .unwrap_or(false)
        })
        .collect();

    // Sort by filename (which should be birth year)
    csv_files.sort_by(|a, b| {
        a.file_name().cmp(&b.file_name())
    });

    stats.total_files = csv_files.len();

    for entry in csv_files {
        let file_path = entry.path();
        let file_name = file_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        println!("Importing {}...", file_name);

        match import_census_file(pool, file_path.to_str().unwrap()).await {
            Ok(count) => {
                stats.successful_files += 1;
                stats.total_players += count;
                println!("  ✓ Imported {} players from {}", count, file_name);
            }
            Err(e) => {
                stats.failed_files += 1;
                eprintln!("  ✗ Failed to import {}: {}", file_name, e);
                stats.errors.push(format!("{}: {}", file_name, e));
            }
        }
    }

    Ok(stats)
}

#[derive(Debug, Default, Clone)]
pub struct ImportStats {
    pub total_files: usize,
    pub successful_files: usize,
    pub failed_files: usize,
    pub total_players: usize,
    pub errors: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_name() {
        assert_eq!(
            parse_name("John Smith"),
            ("John".to_string(), None, "Smith".to_string())
        );

        assert_eq!(
            parse_name("John William Smith"),
            ("John".to_string(), Some("William".to_string()), "Smith".to_string())
        );

        assert_eq!(
            parse_name("John Henry William Smith"),
            ("John".to_string(), Some("Henry William".to_string()), "Smith".to_string())
        );
    }

    #[test]
    fn test_get_nationality() {
        assert_eq!(get_nationality(Some("Scotland"), None), "Scottish");
        assert_eq!(get_nationality(Some("England"), Some("Yorkshire")), "English");
        assert_eq!(get_nationality(Some("Wales"), None), "Welsh");
        assert_eq!(get_nationality(Some("Ireland"), None), "Irish");
    }
}
