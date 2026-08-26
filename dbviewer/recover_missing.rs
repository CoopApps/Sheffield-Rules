use rusqlite::{Connection, Result};
use std::env;

fn main() -> Result<()> {
    let missing_ids: Vec<i64> = vec![
        282, 1480, 3197, 4113, 4685, 5709, 7608, 9463, 9782, 10054,
        11566, 12072, 14767, 15628, 15741, 16644, 16868, 17350, 17906, 18189,
        18486, 18612, 19174, 19200, 21237, 21239, 22053, 22683, 23765, 23795,
        25374, 26124, 26787, 27137, 29317, 32495, 33896, 34813, 35539, 35795,
        35883, 38885, 39455, 39547, 40440, 41085, 42426, 42617, 43349, 46468,
        47842, 48305, 49644, 49834, 50814, 51221, 51375, 51559, 51682, 53187,
        53268, 53727, 53755, 54461, 54462, 55591, 56775, 57558, 58445, 58457,
        59068, 59130, 60607, 60773, 61886, 63312, 63958, 64108, 65383, 67022,
        67239, 67354, 69010, 71298, 75647, 77048, 77426, 77729, 78302, 79251,
        80869, 81919, 84346, 84966, 84991, 86306, 87429, 88059, 88370, 90364,
        90400, 90557, 91322, 91511, 92233, 97428, 97688, 97738, 98995, 100416,
        100973, 102601, 103057, 108963, 111930, 111963, 116411, 117500, 123141, 123547,
    ];

    let commit = env::args().any(|arg| arg == "--commit");

    eprintln!("=== RECOVERING MISSING PEOPLE FROM BACKUP (by unique_id) ===\n");

    let backup = Connection::open("Sheffield1867b.db")?;
    let rescued = Connection::open("Sheffield1867_rescued.db")?;

    let mut found = 0;
    let mut duplicates = 0;
    let mut not_found = 0;
    let mut inserted = 0;

    for &id in &missing_ids {
        // Get person from backup by unique_id (not rowid)
        let result = backup.query_row(
            "SELECT first_name, surname, street_address, COALESCE(birth_year, ''),
                    COALESCE(birth_place, ''), COALESCE(parish, ''), COALESCE(profession, ''),
                    COALESCE(age, 0), COALESCE(sex, '')
             FROM sheffield_people WHERE unique_id = ?1",
            [id],
            |row| Ok((
                row.get::<_, Option<String>>(0)?.unwrap_or_default(),
                row.get::<_, Option<String>>(1)?.unwrap_or_default(),
                row.get::<_, Option<String>>(2)?.unwrap_or_default(),
                row.get::<_, Option<String>>(3)?.unwrap_or_default(),
                row.get::<_, Option<String>>(4)?.unwrap_or_default(),
                row.get::<_, Option<String>>(5)?.unwrap_or_default(),
                row.get::<_, Option<String>>(6)?.unwrap_or_default(),
                row.get::<_, Option<i32>>(7)?.unwrap_or(0),
                row.get::<_, Option<String>>(8)?.unwrap_or_default(),
            ))
        );

        match result {
            Ok((first, surname, address, birth_year, birth_place, parish, profession, age, sex)) => {
                // Check if this person already exists in rescued (same name + address)
                let exists: i64 = rescued.query_row(
                    "SELECT COUNT(*) FROM sheffield_people
                     WHERE LOWER(first_name) = LOWER(?1)
                     AND LOWER(surname) = LOWER(?2)
                     AND LOWER(street_address) = LOWER(?3)",
                    rusqlite::params![&first, &surname, &address],
                    |row| row.get(0)
                )?;

                if exists > 0 {
                    eprintln!("DUPLICATE: {} - {} {} @ {} (born {})", id, first, surname, address, birth_year);
                    duplicates += 1;
                } else {
                    eprintln!("OK: {} - {} {} @ {} (born {}) [{}]", id, first, surname, address, birth_year, profession);
                    found += 1;

                    if commit {
                        rescued.execute(
                            "INSERT INTO sheffield_people (unique_id, first_name, surname, street_address,
                             birth_year, birth_place, parish, profession, age, sex)
                             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                            rusqlite::params![id, first, surname, address, birth_year, birth_place, parish, profession, age, sex]
                        )?;
                        inserted += 1;
                    }
                }
            }
            Err(_) => {
                eprintln!("NOT FOUND IN BACKUP: {}", id);
                not_found += 1;
            }
        }
    }

    eprintln!("\n=== SUMMARY ===");
    eprintln!("Unique (safe to add): {}", found);
    eprintln!("Duplicates (skip): {}", duplicates);
    eprintln!("Not found in backup: {}", not_found);
    if commit {
        eprintln!("Inserted: {}", inserted);
    } else {
        eprintln!("\nRun with --commit to insert missing people");
    }

    Ok(())
}
