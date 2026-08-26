use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;
    conn.execute("PRAGMA foreign_keys = OFF", [])?;

    println!("Repopulating profession tables from sheffield_people...\n");

    // Clear and repopulate sheffield_employers
    println!("Processing sheffield_employers...");
    conn.execute("DELETE FROM sheffield_employers", [])?;
    let employers = conn.execute(
        "INSERT INTO sheffield_employers (
            sheffield_person_id, name, first_name, middle_name, surname,
            census_gender, census_age, birth_year, profession, occupation_expanded,
            street_address, house_number, sub_area, street_name, postcode,
            civil_parish, ecclesiastical_parish, census_folio, census_piece
        )
        SELECT id, name, first_name, middle_name, surname,
            census_gender, census_age, birth_year, profession, occupation_expanded,
            street_address, house_number, sub_area, street_name, postcode,
            civil_parish, ecclesiastical_parish, census_folio, census_piece
        FROM sheffield_people
        WHERE profession LIKE '%employ%'",
        [],
    )?;
    println!("  ✓ Inserted {} employers\n", employers);

    // Clear and repopulate sheffield_professionals
    println!("Processing sheffield_professionals...");
    conn.execute("DELETE FROM sheffield_professionals", [])?;
    let professionals = conn.execute(
        "INSERT INTO sheffield_professionals (
            sheffield_person_id, name, first_name, middle_name, surname,
            census_gender, census_age, birth_year, profession, occupation_expanded,
            professional_type, street_address, house_number, sub_area, street_name,
            postcode, civil_parish, ecclesiastical_parish, census_folio, census_piece
        )
        SELECT id, name, first_name, middle_name, surname,
            census_gender, census_age, birth_year, profession, occupation_expanded,
            CASE
                WHEN profession LIKE '%surgeon%' OR profession LIKE '%doctor%' OR profession LIKE '%nurse%' THEN 'medical'
                WHEN profession LIKE '%solicitor%' OR profession LIKE '%barrister%' OR profession LIKE '%attorney%' THEN 'legal'
                WHEN profession LIKE '%clergy%' OR profession LIKE '%minister%' OR profession LIKE '%priest%' THEN 'clergy'
                ELSE 'other'
            END,
            street_address, house_number, sub_area, street_name,
            postcode, civil_parish, ecclesiastical_parish, census_folio, census_piece
        FROM sheffield_people
        WHERE profession LIKE '%surgeon%' OR profession LIKE '%doctor%' OR profession LIKE '%nurse%'
           OR profession LIKE '%solicitor%' OR profession LIKE '%barrister%' OR profession LIKE '%attorney%'
           OR profession LIKE '%clergy%' OR profession LIKE '%minister%' OR profession LIKE '%priest%'",
        [],
    )?;
    println!("  ✓ Inserted {} professionals\n", professionals);

    // Clear and repopulate sheffield_tradesmen
    println!("Processing sheffield_tradesmen...");
    conn.execute("DELETE FROM sheffield_tradesmen", [])?;
    let tradesmen = conn.execute(
        "INSERT INTO sheffield_tradesmen (
            sheffield_person_id, name, first_name, middle_name, surname,
            census_gender, census_age, birth_year, profession, occupation_expanded,
            trade_type, street_address, house_number, sub_area, street_name,
            postcode, civil_parish, ecclesiastical_parish, census_folio, census_piece
        )
        SELECT id, name, first_name, middle_name, surname,
            census_gender, census_age, birth_year, profession, occupation_expanded,
            CASE
                WHEN profession LIKE '%cutler%' THEN 'cutler'
                ELSE 'other'
            END,
            street_address, house_number, sub_area, street_name,
            postcode, civil_parish, ecclesiastical_parish, census_folio, census_piece
        FROM sheffield_people
        WHERE profession LIKE '%cutler%' OR profession LIKE '%grinder%' OR profession LIKE '%file%'
           OR profession LIKE '%knife%' OR profession LIKE '%scissor%' OR profession LIKE '%blade%'",
        [],
    )?;
    println!("  ✓ Inserted {} tradesmen\n", tradesmen);

    // Clear and repopulate sheffield_publicans
    println!("Processing sheffield_publicans...");
    conn.execute("DELETE FROM sheffield_publicans", [])?;
    let publicans = conn.execute(
        "INSERT INTO sheffield_publicans (
            sheffield_person_id, name, first_name, middle_name, surname,
            census_gender, census_age, birth_year, profession, occupation_expanded,
            street_address, house_number, sub_area, street_name, postcode,
            civil_parish, ecclesiastical_parish, census_folio, census_piece
        )
        SELECT id, name, first_name, middle_name, surname,
            census_gender, census_age, birth_year, profession, occupation_expanded,
            street_address, house_number, sub_area, street_name, postcode,
            civil_parish, ecclesiastical_parish, census_folio, census_piece
        FROM sheffield_people
        WHERE profession LIKE '%publican%' OR profession LIKE '%beerseller%' OR profession LIKE '%innkeeper%'",
        [],
    )?;
    println!("  ✓ Inserted {} publicans\n", publicans);

    // Clear and repopulate sheffield_lodgers
    println!("Processing sheffield_lodgers...");
    conn.execute("DELETE FROM sheffield_lodgers", [])?;
    let lodgers = conn.execute(
        "INSERT INTO sheffield_lodgers (
            sheffield_person_id, name, first_name, middle_name, surname,
            census_gender, census_age, birth_year, profession, occupation_expanded,
            census_relation, street_address, house_number, sub_area, street_name,
            postcode, civil_parish, ecclesiastical_parish, census_folio, census_piece
        )
        SELECT id, name, first_name, middle_name, surname,
            census_gender, census_age, birth_year, profession, occupation_expanded,
            census_relation, street_address, house_number, sub_area, street_name,
            postcode, civil_parish, ecclesiastical_parish, census_folio, census_piece
        FROM sheffield_people
        WHERE census_relation LIKE '%lodger%'",
        [],
    )?;
    println!("  ✓ Inserted {} lodgers\n", lodgers);

    // Clear and repopulate sheffield_german
    println!("Processing sheffield_german...");
    conn.execute("DELETE FROM sheffield_german", [])?;
    let german = conn.execute(
        "INSERT INTO sheffield_german (
            sheffield_person_id, name, first_name, middle_name, surname,
            census_gender, census_age, birth_year, profession, occupation_expanded,
            street_address, house_number, sub_area, street_name, postcode,
            civil_parish, ecclesiastical_parish, census_folio, census_piece,
            birth_town, birth_county, where_born
        )
        SELECT id, name, first_name, middle_name, surname,
            census_gender, census_age, birth_year, profession, occupation_expanded,
            street_address, house_number, sub_area, street_name, postcode,
            civil_parish, ecclesiastical_parish, census_folio, census_piece,
            birth_town, birth_county, where_born
        FROM sheffield_people
        WHERE where_born LIKE '%German%' OR birth_country LIKE '%German%'",
        [],
    )?;
    println!("  ✓ Inserted {} German\n", german);

    // Clear and repopulate sheffield_irish
    println!("Processing sheffield_irish...");
    conn.execute("DELETE FROM sheffield_irish", [])?;
    let irish = conn.execute(
        "INSERT INTO sheffield_irish (
            sheffield_person_id, name, first_name, middle_name, surname,
            census_gender, census_age, birth_year, profession, occupation_expanded,
            street_address, house_number, sub_area, street_name, postcode,
            civil_parish, ecclesiastical_parish, census_folio, census_piece,
            birth_town, birth_county, where_born
        )
        SELECT id, name, first_name, middle_name, surname,
            census_gender, census_age, birth_year, profession, occupation_expanded,
            street_address, house_number, sub_area, street_name, postcode,
            civil_parish, ecclesiastical_parish, census_folio, census_piece,
            birth_town, birth_county, where_born
        FROM sheffield_people
        WHERE where_born LIKE '%Ireland%' OR birth_country LIKE '%Ireland%'",
        [],
    )?;
    println!("  ✓ Inserted {} Irish\n", irish);

    // Clear and repopulate sheffield_scottish
    println!("Processing sheffield_scottish...");
    conn.execute("DELETE FROM sheffield_scottish", [])?;
    let scottish = conn.execute(
        "INSERT INTO sheffield_scottish (
            sheffield_person_id, name, first_name, middle_name, surname,
            census_gender, census_age, birth_year, profession, occupation_expanded,
            street_address, house_number, sub_area, street_name, postcode,
            civil_parish, ecclesiastical_parish, census_folio, census_piece,
            birth_town, birth_county, where_born
        )
        SELECT id, name, first_name, middle_name, surname,
            census_gender, census_age, birth_year, profession, occupation_expanded,
            street_address, house_number, sub_area, street_name, postcode,
            civil_parish, ecclesiastical_parish, census_folio, census_piece,
            birth_town, birth_county, where_born
        FROM sheffield_people
        WHERE where_born LIKE '%Scotland%' OR birth_country LIKE '%Scotland%'",
        [],
    )?;
    println!("  ✓ Inserted {} Scottish\n", scottish);

    // Clear and repopulate sheffield_welsh
    println!("Processing sheffield_welsh...");
    conn.execute("DELETE FROM sheffield_welsh", [])?;
    let welsh = conn.execute(
        "INSERT INTO sheffield_welsh (
            sheffield_person_id, name, first_name, middle_name, surname,
            census_gender, census_age, birth_year, profession, occupation_expanded,
            street_address, house_number, sub_area, street_name, postcode,
            civil_parish, ecclesiastical_parish, census_folio, census_piece,
            birth_town, birth_county, where_born
        )
        SELECT id, name, first_name, middle_name, surname,
            census_gender, census_age, birth_year, profession, occupation_expanded,
            street_address, house_number, sub_area, street_name, postcode,
            civil_parish, ecclesiastical_parish, census_folio, census_piece,
            birth_town, birth_county, where_born
        FROM sheffield_people
        WHERE where_born LIKE '%Wales%' OR birth_country LIKE '%Wales%'",
        [],
    )?;
    println!("  ✓ Inserted {} Welsh\n", welsh);

    println!("\n=== COMPLETE ===");
    println!("All profession tables repopulated with sheffield_person_id set correctly!");

    Ok(())
}
