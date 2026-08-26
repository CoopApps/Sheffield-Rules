use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    println!("=== RESCUING SHEFFIELD_PEOPLE TABLE ===\n");

    // Open corrupted database
    let source_conn = Connection::open("Sheffield1867.db")?;

    // Create new database
    let dest_db_name = "Sheffield1867_rescued.db";
    let dest_conn = Connection::open(dest_db_name)?;

    println!("1. Creating new table with unique_id...");
    dest_conn.execute(
        "CREATE TABLE IF NOT EXISTS sheffield_people (
            unique_id INTEGER PRIMARY KEY AUTOINCREMENT,
            id INT,
            name TEXT,
            first_name TEXT,
            middle_name TEXT,
            surname TEXT,
            census_age INT,
            census_relation TEXT,
            census_gender TEXT,
            census_ed TEXT,
            census_household_schedule TEXT,
            census_piece TEXT,
            census_folio TEXT,
            census_page TEXT,
            civil_parish TEXT,
            ecclesiastical_parish TEXT,
            registration_district TEXT,
            sub_registration_district TEXT,
            street_address TEXT,
            house_number TEXT,
            sub_area TEXT,
            street_name TEXT,
            birth_year INT,
            birth_town TEXT,
            birth_county TEXT,
            birth_country TEXT,
            where_born TEXT,
            profession TEXT,
            occupation_expanded TEXT,
            genealogy_source TEXT,
            genealogy_id TEXT,
            business_name TEXT,
            business_type TEXT,
            postcode TEXT,
            postcode_area TEXT,
            postcode_district TEXT,
            postcode_sector TEXT,
            postcode_unit TEXT,
            latitude REAL,
            longitude REAL,
            created_at NUM,
            matched_to_ancestry_id INT,
            matched_to_business_id INT,
            match_confidence REAL,
            match_status TEXT,
            spouse_person_id INTEGER
        )",
        [],
    )?;
    println!("   ✓ New table created\n");

    // Count rows in source
    println!("2. Counting rows in source table...");
    let total_rows: i64 = source_conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people",
        [],
        |row| row.get(0)
    )?;
    println!("   Found {} rows\n", total_rows);

    // Copy data using ATTACH and INSERT SELECT
    println!("3. Copying data...");
    dest_conn.execute("ATTACH DATABASE 'Sheffield1867.db' AS source", [])?;

    let copied = dest_conn.execute(
        "INSERT INTO main.sheffield_people (
            id, name, first_name, middle_name, surname, census_age, census_relation,
            census_gender, census_ed, census_household_schedule, census_piece,
            census_folio, census_page, civil_parish, ecclesiastical_parish,
            registration_district, sub_registration_district, street_address,
            house_number, sub_area, street_name, birth_year, birth_town,
            birth_county, birth_country, where_born, profession, occupation_expanded,
            genealogy_source, genealogy_id, business_name, business_type,
            postcode, postcode_area, postcode_district, postcode_sector,
            postcode_unit, latitude, longitude, created_at, matched_to_ancestry_id,
            matched_to_business_id, match_confidence, match_status, spouse_person_id
        )
        SELECT
            id, name, first_name, middle_name, surname, census_age, census_relation,
            census_gender, census_ed, census_household_schedule, census_piece,
            census_folio, census_page, civil_parish, ecclesiastical_parish,
            registration_district, sub_registration_district, street_address,
            house_number, sub_area, street_name, birth_year, birth_town,
            birth_county, birth_country, where_born, profession, occupation_expanded,
            genealogy_source, genealogy_id, business_name, business_type,
            postcode, postcode_area, postcode_district, postcode_sector,
            postcode_unit, latitude, longitude, created_at, matched_to_ancestry_id,
            matched_to_business_id, match_confidence, match_status, spouse_person_id
        FROM source.sheffield_people",
        [],
    )?;

    dest_conn.execute("DETACH DATABASE source", [])?;

    println!("   ✓ Copied {} rows\n", copied);

    // Verify the new database
    println!("4. Verifying rescued data...");
    let new_count: i64 = dest_conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people",
        [],
        |row| row.get(0)
    )?;
    println!("   Rows in new database: {}", new_count);

    // Check unique_id integrity
    let min_id: Option<i64> = dest_conn.query_row(
        "SELECT MIN(unique_id) FROM sheffield_people",
        [],
        |row| row.get(0)
    )?;
    let max_id: Option<i64> = dest_conn.query_row(
        "SELECT MAX(unique_id) FROM sheffield_people",
        [],
        |row| row.get(0)
    )?;

    if let (Some(min), Some(max)) = (min_id, max_id) {
        println!("   Unique ID range: {} to {}", min, max);
    }

    // Check for any NULL unique_ids (shouldn't happen with AUTOINCREMENT)
    let null_ids: i64 = dest_conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people WHERE unique_id IS NULL",
        [],
        |row| row.get(0)
    )?;
    if null_ids == 0 {
        println!("   ✓ All rows have unique_id");
    } else {
        println!("   ✗ Warning: {} rows have NULL unique_id", null_ids);
    }

    println!("\n=== RESCUE COMPLETE ===");
    println!("New database created: {}", dest_db_name);
    println!("You can now use this database instead of the corrupted one.");

    Ok(())
}
