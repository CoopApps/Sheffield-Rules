use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;
    conn.execute("PRAGMA foreign_keys = OFF", [])?;

    println!("Repopulating grouping tables from sheffield_people...\n");

    // Clear and repopulate sheffield_family_groups and sheffield_family_members
    println!("Processing sheffield_family_groups and sheffield_family_members...");
    conn.execute("DELETE FROM sheffield_family_members", [])?;
    conn.execute("DELETE FROM sheffield_family_groups", [])?;

    // Create family groups (same surname + same street_address)
    let family_groups = conn.execute(
        "INSERT INTO sheffield_family_groups (surname, street_address, member_count, males_14_40, average_age, is_large_family)
         SELECT
           surname,
           street_address,
           COUNT(*) as member_count,
           SUM(CASE WHEN census_gender = 'Male' AND census_age BETWEEN 14 AND 40 THEN 1 ELSE 0 END) as males_14_40,
           AVG(CAST(census_age AS REAL)) as average_age,
           CASE WHEN COUNT(*) >= 5 THEN 1 ELSE 0 END as is_large_family
         FROM sheffield_people
         WHERE surname IS NOT NULL AND street_address IS NOT NULL
         GROUP BY surname, street_address
         HAVING COUNT(*) > 1",
        [],
    )?;
    println!("  ✓ Created {} family groups", family_groups);

    // Populate family members
    let family_members = conn.execute(
        "INSERT INTO sheffield_family_members (
           family_group_id, sheffield_person_id, name, first_name, census_age,
           census_gender, census_relation, profession, is_footballer
         )
         SELECT
           fg.family_group_id,
           p.id,
           p.name,
           p.first_name,
           p.census_age,
           p.census_gender,
           p.census_relation,
           p.profession,
           0
         FROM sheffield_people p
         INNER JOIN sheffield_family_groups fg
           ON p.surname = fg.surname AND p.street_address = fg.street_address",
        [],
    )?;
    println!("  ✓ Created {} family member records\n", family_members);

    // Clear and repopulate sheffield_households and sheffield_household_members
    println!("Processing sheffield_households and sheffield_household_members...");
    conn.execute("DELETE FROM sheffield_household_members", [])?;
    conn.execute("DELETE FROM sheffield_households", [])?;

    // Create households (same census_household_schedule)
    let households = conn.execute(
        "INSERT INTO sheffield_households (
           census_household_schedule, street_address, postcode, household_size,
           males_total, females_total, footballers_count, has_head, head_person_id,
           head_name, head_profession, avg_age
         )
         SELECT
           census_household_schedule,
           MAX(street_address) as street_address,
           MAX(postcode) as postcode,
           COUNT(*) as household_size,
           SUM(CASE WHEN census_gender = 'Male' THEN 1 ELSE 0 END) as males_total,
           SUM(CASE WHEN census_gender = 'Female' THEN 1 ELSE 0 END) as females_total,
           0 as footballers_count,
           MAX(CASE WHEN census_relation LIKE '%Head%' THEN 1 ELSE 0 END) as has_head,
           (SELECT id FROM sheffield_people p2
            WHERE p2.census_household_schedule = p.census_household_schedule
              AND p2.census_relation LIKE '%Head%' LIMIT 1) as head_person_id,
           (SELECT name FROM sheffield_people p2
            WHERE p2.census_household_schedule = p.census_household_schedule
              AND p2.census_relation LIKE '%Head%' LIMIT 1) as head_name,
           (SELECT profession FROM sheffield_people p2
            WHERE p2.census_household_schedule = p.census_household_schedule
              AND p2.census_relation LIKE '%Head%' LIMIT 1) as head_profession,
           AVG(CAST(census_age AS REAL)) as avg_age
         FROM sheffield_people p
         WHERE census_household_schedule IS NOT NULL AND census_household_schedule != ''
         GROUP BY census_household_schedule",
        [],
    )?;
    println!("  ✓ Created {} households", households);

    // Populate household members
    let household_members = conn.execute(
        "INSERT INTO sheffield_household_members (
           household_id, sheffield_person_id, census_household_schedule,
           name, census_age, census_gender, census_relation, profession,
           is_footballer, is_head
         )
         SELECT
           h.household_id,
           p.id,
           p.census_household_schedule,
           p.name,
           p.census_age,
           p.census_gender,
           p.census_relation,
           p.profession,
           0,
           CASE WHEN p.census_relation LIKE '%Head%' THEN 1 ELSE 0 END
         FROM sheffield_people p
         INNER JOIN sheffield_households h
           ON p.census_household_schedule = h.census_household_schedule",
        [],
    )?;
    println!("  ✓ Created {} household member records\n", household_members);

    println!("\n=== COMPLETE ===");
    println!("Family groups: {} groups, {} members", family_groups, family_members);
    println!("Households: {} households, {} members", households, household_members);

    Ok(())
}
