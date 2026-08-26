use rusqlite::{Connection, Result};
use std::collections::HashMap;

fn main() -> Result<()> {
    let db_path = "../Sheffield1867.db";
    let conn = Connection::open(db_path)?;

    println!("================================================================================");
    println!("CHECKING FOR DUPLICATE CLUBS IN sheffield_clubs");
    println!("================================================================================\n");

    // Check 1: Duplicate IDs
    println!("CHECK 1: Duplicate club IDs");
    println!("--------------------------------------------------------------------------------");

    let mut stmt = conn.prepare(
        "SELECT id, COUNT(*) as count
         FROM sheffield_clubs
         GROUP BY id
         HAVING COUNT(*) > 1"
    )?;

    let mut rows = stmt.query([])?;
    let mut has_dup_ids = false;

    while let Some(row) = rows.next()? {
        has_dup_ids = true;
        let id: String = row.get(0)?;
        let count: i32 = row.get(1)?;
        println!("  ❌ ID '{}' appears {} times", id, count);
    }

    if !has_dup_ids {
        println!("  ✓ No duplicate IDs found\n");
    } else {
        println!();
    }

    // Check 2: Duplicate Names
    println!("CHECK 2: Duplicate club names");
    println!("--------------------------------------------------------------------------------");

    let mut stmt = conn.prepare(
        "SELECT name, COUNT(*) as count
         FROM sheffield_clubs
         GROUP BY name
         HAVING COUNT(*) > 1"
    )?;

    let mut rows = stmt.query([])?;
    let mut has_dup_names = false;

    while let Some(row) = rows.next()? {
        has_dup_names = true;
        let name: String = row.get(0)?;
        let count: i32 = row.get(1)?;
        println!("  ⚠ Name '{}' appears {} times", name, count);

        // Show which clubs have this name
        let mut detail_stmt = conn.prepare(
            "SELECT id, name, founded_year FROM sheffield_clubs WHERE name = ? ORDER BY id"
        )?;
        let mut detail_rows = detail_stmt.query([&name])?;

        while let Some(detail_row) = detail_rows.next()? {
            let id: String = detail_row.get(0)?;
            let year: i32 = detail_row.get(2)?;
            println!("      - ID: {}, Founded: {}", id, year);
        }
        println!();
    }

    if !has_dup_names {
        println!("  ✓ No duplicate names found\n");
    }

    // Check 3: Similar names (potential duplicates)
    println!("CHECK 3: Similar names (potential duplicates)");
    println!("--------------------------------------------------------------------------------");

    let mut all_clubs: Vec<(String, String, i32)> = Vec::new();
    let mut stmt = conn.prepare(
        "SELECT id, name, founded_year FROM sheffield_clubs ORDER BY name"
    )?;
    let mut rows = stmt.query([])?;

    while let Some(row) = rows.next()? {
        let id: String = row.get(0)?;
        let name: String = row.get(1)?;
        let year: i32 = row.get(2)?;
        all_clubs.push((id, name, year));
    }

    // Group by normalized name (removing "Reserves", "Second XI", etc.)
    let mut base_names: HashMap<String, Vec<(String, String, i32)>> = HashMap::new();

    for (id, name, year) in &all_clubs {
        // Normalize: remove reserve team indicators
        let normalized = name
            .replace(" Reserves", "")
            .replace(" Second XI", "")
            .replace(" Juniors", "")
            .replace(" B Team", "")
            .replace(" Colts", "")
            .replace(" Reserve XI", "")
            .replace(" Second Team", "")
            .replace(" Junior XI", "")
            .replace(" Second Eleven", "")
            .trim()
            .to_string();

        base_names
            .entry(normalized)
            .or_insert_with(Vec::new)
            .push((id.clone(), name.clone(), *year));
    }

    let mut found_similar = false;

    // Check for clubs with same base name but different reserve team naming
    for (base_name, clubs) in &base_names {
        // Filter to only main teams (exclude ones with reserve indicators)
        let main_teams: Vec<_> = clubs.iter()
            .filter(|(id, name, _)| {
                !id.contains("-reserves") &&
                !name.contains("Reserves") &&
                !name.contains("Second") &&
                !name.contains("Junior") &&
                !name.contains("Colts") &&
                !name.contains("B Team")
            })
            .collect();

        if main_teams.len() > 1 {
            found_similar = true;
            println!("  ⚠ Multiple main teams with base name '{}':", base_name);
            for (id, name, year) in main_teams {
                println!("      - {} ({}) - Founded {}", name, id, year);
            }
            println!();
        }
    }

    if !found_similar {
        println!("  ✓ No similar names found that could be duplicates\n");
    }

    // Check 4: Reserve teams without main teams
    println!("CHECK 4: Reserve teams without corresponding main team");
    println!("--------------------------------------------------------------------------------");

    let mut orphaned_reserves = false;

    for (id, name, year) in &all_clubs {
        if id.contains("-reserves") {
            // Extract main club ID
            let main_id = id.replace("-reserves", "");

            // Check if main club exists
            let exists: i64 = conn.query_row(
                "SELECT COUNT(*) FROM sheffield_clubs WHERE id = ?",
                [&main_id],
                |r| r.get(0),
            )?;

            if exists == 0 {
                orphaned_reserves = true;
                println!("  ⚠ Reserve team '{}' ({}) has no main team with id '{}'",
                    name, id, main_id);
            }
        }
    }

    if !orphaned_reserves {
        println!("  ✓ All reserve teams have corresponding main teams\n");
    } else {
        println!();
    }

    // Check 5: Multiple clubs with same founding year and similar names
    println!("CHECK 5: Clubs with identical founding years and similar names");
    println!("--------------------------------------------------------------------------------");

    let mut year_groups: HashMap<i32, Vec<(String, String)>> = HashMap::new();

    for (id, name, year) in &all_clubs {
        // Skip reserves for this check
        if !id.contains("-reserves") {
            year_groups
                .entry(*year)
                .or_insert_with(Vec::new)
                .push((id.clone(), name.clone()));
        }
    }

    let mut found_same_year = false;

    for (year, clubs) in &year_groups {
        if clubs.len() > 1 {
            // Check if any names are very similar
            for i in 0..clubs.len() {
                for j in (i + 1)..clubs.len() {
                    let name1 = &clubs[i].1;
                    let name2 = &clubs[j].1;

                    // Very simple similarity check
                    if name1.contains(&name2.split_whitespace().next().unwrap_or("")) ||
                       name2.contains(&name1.split_whitespace().next().unwrap_or("")) {
                        found_same_year = true;
                        println!("  ⚠ Similar clubs founded in {}:", year);
                        println!("      - {} ({})", clubs[i].1, clubs[i].0);
                        println!("      - {} ({})", clubs[j].1, clubs[j].0);
                        println!();
                    }
                }
            }
        }
    }

    if !found_same_year {
        println!("  ✓ No suspicious duplicates with same founding year\n");
    }

    // Summary
    println!("================================================================================");
    println!("SUMMARY");
    println!("================================================================================\n");

    let total_clubs: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_clubs",
        [],
        |r| r.get(0),
    )?;

    println!("Total clubs in database: {}", total_clubs);

    if !has_dup_ids && !has_dup_names && !orphaned_reserves {
        println!("\n✅ No critical duplicates or issues found!");
        println!("Database appears clean.\n");
    } else {
        println!("\n⚠ Issues found:");
        if has_dup_ids {
            println!("  - Duplicate IDs detected");
        }
        if has_dup_names {
            println!("  - Duplicate names detected");
        }
        if orphaned_reserves {
            println!("  - Orphaned reserve teams detected");
        }
        println!();
    }

    println!("================================================================================\n");

    Ok(())
}
