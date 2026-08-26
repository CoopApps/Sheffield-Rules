use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("D:/projects/Saturday at Three/Sheffield1867.db")?;

    println!("\n=== Finding players with household data ===\n");

    let mut stmt = conn.prepare(
        "SELECT id, name, surname, birth_year, ecclesiastical_parish,
                census_household_schedule, census_relation, census_gender,
                birth_town, birth_county
         FROM sheffield_players
         WHERE census_household_schedule IS NOT NULL
           AND census_household_schedule != ''
         LIMIT 20"
    )?;

    let players = stmt.query_map([], |row| {
        Ok((
            row.get::<_, String>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, Option<i32>>(3)?,
            row.get::<_, Option<String>>(4)?,
            row.get::<_, Option<String>>(5)?,
            row.get::<_, Option<String>>(6)?,
            row.get::<_, Option<String>>(7)?,
            row.get::<_, Option<String>>(8)?,
            row.get::<_, Option<String>>(9)?,
        ))
    })?;

    for (i, player) in players.enumerate() {
        let (id, name, surname, birth_year, parish, household, relation, gender, town, county) = player?;
        println!("{}. {} {} (b. {})", i + 1, name, surname,
                 birth_year.map(|y| y.to_string()).unwrap_or_else(|| "Unknown".to_string()));
        println!("   ID: {}", id);
        println!("   Parish: {}", parish.unwrap_or_else(|| "N/A".to_string()));
        println!("   Town: {}, County: {}",
                 town.unwrap_or_else(|| "N/A".to_string()),
                 county.unwrap_or_else(|| "N/A".to_string()));
        println!("   Household Schedule: {}", household.unwrap_or_else(|| "N/A".to_string()));
        println!("   Relation: {}, Gender: {}",
                 relation.unwrap_or_else(|| "N/A".to_string()),
                 gender.unwrap_or_else(|| "N/A".to_string()));
        println!();
    }

    Ok(())
}
