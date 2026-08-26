use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;
    conn.execute("PRAGMA foreign_keys = OFF", [])?;

    println!("Linking remaining tables using exact name matches...\n");

    // Link sheffield_asylum
    println!("Processing sheffield_asylum...");
    let asylum = conn.execute(
        "UPDATE sheffield_asylum
         SET matched_to_ancestry_id = (
           SELECT id FROM sheffield_people
           WHERE sheffield_people.name = sheffield_asylum.name
           LIMIT 1
         )
         WHERE name IN (SELECT name FROM sheffield_people)",
        [],
    )?;
    let asylum_matched: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_asylum WHERE matched_to_ancestry_id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;
    let asylum_total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_asylum",
        [],
        |row| row.get(0),
    )?;
    println!("  ✓ Linked {}/{} asylum records\n", asylum_matched, asylum_total);

    // Link sheffield_prison
    println!("Processing sheffield_prison...");
    let prison = conn.execute(
        "UPDATE sheffield_prison
         SET matched_to_ancestry_id = (
           SELECT id FROM sheffield_people
           WHERE sheffield_people.name = sheffield_prison.name
           LIMIT 1
         )
         WHERE name IN (SELECT name FROM sheffield_people)",
        [],
    )?;
    let prison_matched: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_prison WHERE matched_to_ancestry_id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;
    let prison_total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_prison",
        [],
        |row| row.get(0),
    )?;
    println!("  ✓ Linked {}/{} prison records\n", prison_matched, prison_total);

    // Link sheffield_workhouse
    println!("Processing sheffield_workhouse...");
    let workhouse = conn.execute(
        "UPDATE sheffield_workhouse
         SET matched_to_ancestry_id = (
           SELECT id FROM sheffield_people
           WHERE sheffield_people.name = sheffield_workhouse.name
           LIMIT 1
         )
         WHERE name IN (SELECT name FROM sheffield_people)",
        [],
    )?;
    let workhouse_matched: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_workhouse WHERE matched_to_ancestry_id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;
    let workhouse_total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_workhouse",
        [],
        |row| row.get(0),
    )?;
    println!("  ✓ Linked {}/{} workhouse records\n", workhouse_matched, workhouse_total);

    // Link sheffield_patron
    println!("Processing sheffield_patron...");
    let patron = conn.execute(
        "UPDATE sheffield_patron
         SET matched_to_ancestry_id = (
           SELECT id FROM sheffield_people
           WHERE sheffield_people.name = sheffield_patron.name
           LIMIT 1
         )
         WHERE name IN (SELECT name FROM sheffield_people)",
        [],
    )?;
    let patron_matched: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_patron WHERE matched_to_ancestry_id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;
    let patron_total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_patron",
        [],
        |row| row.get(0),
    )?;
    println!("  ✓ Linked {}/{} patron records\n", patron_matched, patron_total);

    // Link sheffield_family_members
    println!("Processing sheffield_family_members...");
    let family = conn.execute(
        "UPDATE sheffield_family_members
         SET sheffield_person_id = (
           SELECT id FROM sheffield_people
           WHERE sheffield_people.name = sheffield_family_members.name
           LIMIT 1
         )
         WHERE name IN (SELECT name FROM sheffield_people)",
        [],
    )?;
    let family_matched: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_family_members WHERE sheffield_person_id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;
    let family_total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_family_members",
        [],
        |row| row.get(0),
    )?;
    println!("  ✓ Linked {}/{} family members\n", family_matched, family_total);

    // Link sheffield_household_members
    println!("Processing sheffield_household_members...");
    let household = conn.execute(
        "UPDATE sheffield_household_members
         SET sheffield_person_id = (
           SELECT id FROM sheffield_people
           WHERE sheffield_people.name = sheffield_household_members.name
           LIMIT 1
         )
         WHERE name IN (SELECT name FROM sheffield_people)",
        [],
    )?;
    let household_matched: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_household_members WHERE sheffield_person_id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;
    let household_total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_household_members",
        [],
        |row| row.get(0),
    )?;
    println!("  ✓ Linked {}/{} household members\n", household_matched, household_total);

    // Link sheffield_households head_person_id
    println!("Processing sheffield_households (head_person_id)...");
    let households = conn.execute(
        "UPDATE sheffield_households
         SET head_person_id = (
           SELECT id FROM sheffield_people
           WHERE sheffield_people.name = sheffield_households.head_name
           LIMIT 1
         )
         WHERE head_name IN (SELECT name FROM sheffield_people)",
        [],
    )?;
    let households_matched: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_households WHERE head_person_id IS NOT NULL",
        [],
        |row| row.get(0),
    )?;
    let households_total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_households",
        [],
        |row| row.get(0),
    )?;
    println!("  ✓ Linked {}/{} household heads\n", households_matched, households_total);

    println!("\n=== SUMMARY ===");
    println!("sheffield_asylum: {}/{} linked", asylum_matched, asylum_total);
    println!("sheffield_prison: {}/{} linked", prison_matched, prison_total);
    println!("sheffield_workhouse: {}/{} linked", workhouse_matched, workhouse_total);
    println!("sheffield_patron: {}/{} linked", patron_matched, patron_total);
    println!("sheffield_family_members: {}/{} linked", family_matched, family_total);
    println!("sheffield_household_members: {}/{} linked", household_matched, household_total);
    println!("sheffield_households (heads): {}/{} linked", households_matched, households_total);

    println!("\nAll remaining tables linked!");

    Ok(())
}
