use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867_rescued.db")?;

    println!("=== CREATING INSTITUTIONAL TABLES ===\n");

    // Create asylum table
    println!("1. Creating sheffield_asylum table...");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sheffield_asylum (
            unique_id INTEGER PRIMARY KEY,
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

    let asylum_count = conn.execute(
        "INSERT INTO sheffield_asylum
         SELECT * FROM sheffield_people
         WHERE street_address LIKE '%asylum%'
            OR street_address LIKE '%lunatic%'
            OR street_address LIKE '%lunstie%'",
        [],
    )?;
    println!("   ✓ Moved {} people to asylum table\n", asylum_count);

    // Create prison table
    println!("2. Creating sheffield_prison table...");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sheffield_prison (
            unique_id INTEGER PRIMARY KEY,
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

    let prison_count = conn.execute(
        "INSERT INTO sheffield_prison
         SELECT * FROM sheffield_people
         WHERE street_address LIKE '%prison%'
            OR street_address LIKE '%gaol%'
            OR street_address LIKE '%jail%'
            OR street_address LIKE '%refuge for discharged%'",
        [],
    )?;
    println!("   ✓ Moved {} people to prison table\n", prison_count);

    // Create workhouse table
    println!("3. Creating sheffield_workhouse table...");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sheffield_workhouse (
            unique_id INTEGER PRIMARY KEY,
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

    let workhouse_count = conn.execute(
        "INSERT INTO sheffield_workhouse
         SELECT * FROM sheffield_people
         WHERE street_address LIKE '%workhouse%'",
        [],
    )?;
    println!("   ✓ Moved {} people to workhouse table\n", workhouse_count);

    // Create clergy table (based on profession)
    println!("4. Creating sheffield_clergy table...");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sheffield_clergy (
            unique_id INTEGER PRIMARY KEY,
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

    let clergy_count = conn.execute(
        "INSERT INTO sheffield_clergy
         SELECT * FROM sheffield_people
         WHERE profession LIKE '%clergy%'
            OR profession LIKE '%minister%'
            OR profession LIKE '%vicar%'
            OR profession LIKE '%priest%'
            OR profession LIKE '%curate%'
            OR profession LIKE '%rector%'
            OR profession LIKE '%chaplain%'
            OR profession LIKE '%dean%'
            OR profession LIKE '%bishop%'",
        [],
    )?;
    println!("   ✓ Moved {} people to clergy table\n", clergy_count);

    // Now remove these people from sheffield_people
    println!("5. Removing institutional people from main table...");
    let deleted = conn.execute(
        "DELETE FROM sheffield_people
         WHERE unique_id IN (
            SELECT unique_id FROM sheffield_asylum
            UNION SELECT unique_id FROM sheffield_prison
            UNION SELECT unique_id FROM sheffield_workhouse
            UNION SELECT unique_id FROM sheffield_clergy
         )",
        [],
    )?;
    println!("   ✓ Removed {} people from sheffield_people\n", deleted);

    // Verify
    println!("6. Verification:");
    let remaining: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_people",
        [],
        |row| row.get(0)
    )?;
    println!("   Remaining in sheffield_people: {}", remaining);

    let total_institutional: i64 = conn.query_row(
        "SELECT
            (SELECT COUNT(*) FROM sheffield_asylum) +
            (SELECT COUNT(*) FROM sheffield_prison) +
            (SELECT COUNT(*) FROM sheffield_workhouse) +
            (SELECT COUNT(*) FROM sheffield_clergy)",
        [],
        |row| row.get(0)
    )?;
    println!("   Total in institutional tables: {}", total_institutional);
    println!("   Grand total: {}", remaining + total_institutional);

    println!("\n=== INSTITUTIONAL TABLES CREATED ===");

    Ok(())
}
