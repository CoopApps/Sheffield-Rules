use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let db_path = "../Sheffield1867.db";
    let conn = Connection::open(db_path)?;

    println!("================================================================================");
    println!("REDISTRIBUTING ALL 372 CLUBS - 16 PER DIVISION FROM TOP DOWN");
    println!("================================================================================\n");

    println!("⚠️  WARNING: This will DELETE all current league assignments and rebuild them!\n");
    println!("Starting redistribution NOW...\n");

    // Get current state
    let before_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_league_clubs",
        [],
        |r| r.get(0),
    )?;
    println!("Current assignments: {}\n", before_count);

    // Get all clubs
    let main_clubs: Vec<(String, String, i32)> = conn
        .prepare("SELECT id, name, founded_year FROM sheffield_clubs WHERE id NOT LIKE '%-reserves' ORDER BY founded_year, name")?
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?
        .collect::<Result<Vec<_>, _>>()?;

    let reserve_clubs: Vec<(String, String, i32)> = conn
        .prepare("SELECT id, name, founded_year FROM sheffield_clubs WHERE id LIKE '%-reserves' ORDER BY founded_year, name")?
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?
        .collect::<Result<Vec<_>, _>>()?;

    println!("Main clubs: {}", main_clubs.len());
    println!("Reserve clubs: {}", reserve_clubs.len());
    println!("Total: {}\n", main_clubs.len() + reserve_clubs.len());

    // Division structure
    let divisions = vec![
        // Level 1
        ("div-1", "main", 16), ("res-div-1", "reserve", 16),
        // Level 2
        ("div-2", "main", 16), ("res-div-2", "reserve", 16),
        // Level 3
        ("div-3", "main", 16), ("res-div-3", "reserve", 16),
        // Level 4
        ("div-4", "main", 16), ("res-div-4", "reserve", 16),
        // Level 5
        ("div-5a", "main", 13), ("div-5b", "main", 13),
        ("res-div-5a", "reserve", 13), ("res-div-5b", "reserve", 13),
        // Level 6
        ("div-6a", "main", 12), ("div-6b", "main", 12),
        ("div-6c", "main", 12), ("div-6d", "main", 12),
        ("res-div-6a", "reserve", 12), ("res-div-6b", "reserve", 12),
        ("res-div-6c", "reserve", 12), ("res-div-6d", "reserve", 12),
        // Level 7
        ("div-7a", "main", 12), ("div-7b", "main", 12),
        ("div-7c", "main", 12), ("div-7d", "main", 12),
        ("res-div-7a", "reserve", 12), ("res-div-7b", "reserve", 12),
        ("res-div-7c", "reserve", 12), ("res-div-7d", "reserve", 12),
    ];

    println!("Target distribution:");
    println!("  Levels 1-4: 16 clubs per division");
    println!("  Level 5: 13 clubs per division");
    println!("  Levels 6-7: 12 clubs per division\n");

    // Start transaction
    conn.execute("BEGIN TRANSACTION", [])?;

    // Clear existing
    println!("Clearing existing assignments...");
    conn.execute("DELETE FROM sheffield_league_clubs", [])?;
    println!("✓ Cleared\n");

    // Distribute main clubs
    println!("Distributing MAIN clubs:\n");
    let mut main_index = 0;

    for (div_id, div_type, capacity) in &divisions {
        if *div_type != "main" {
            continue;
        }

        println!("  {}:", div_id);

        for pos in 1..=*capacity {
            if main_index >= main_clubs.len() {
                break;
            }

            let (club_id, club_name, _) = &main_clubs[main_index];
            let league_id = format!("{}-{}", club_id, div_id);

            conn.execute(
                "INSERT INTO sheffield_league_clubs (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
                 VALUES (?, ?, ?, ?, 0, NULL)",
                [&league_id, &div_id.to_string(), club_id, &pos.to_string()],
            )?;

            if pos <= 3 || pos == *capacity {
                println!("    {}. {}", pos, club_name);
            } else if pos == 4 {
                println!("    ... ({} more clubs)", capacity - 3);
            }

            main_index += 1;
        }
        println!();
    }

    // Distribute reserve clubs
    println!("Distributing RESERVE clubs:\n");
    let mut reserve_index = 0;

    for (div_id, div_type, capacity) in &divisions {
        if *div_type != "reserve" {
            continue;
        }

        println!("  {}:", div_id);

        for pos in 1..=*capacity {
            if reserve_index >= reserve_clubs.len() {
                break;
            }

            let (club_id, club_name, _) = &reserve_clubs[reserve_index];
            let parent_club = club_id.replace("-reserves", "");
            let league_id = format!("{}-{}", club_id, div_id);

            conn.execute(
                "INSERT INTO sheffield_league_clubs (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
                 VALUES (?, ?, ?, ?, 1, ?)",
                [&league_id, &div_id.to_string(), club_id, &pos.to_string(), &parent_club],
            )?;

            if pos <= 3 || pos == *capacity {
                println!("    {}. {}", pos, club_name);
            } else if pos == 4 {
                println!("    ... ({} more clubs)", capacity - 3);
            }

            reserve_index += 1;
        }
        println!();
    }

    // Commit
    conn.execute("COMMIT", [])?;
    println!("✅ Transaction committed\n");

    // Verify
    println!("================================================================================");
    println!("VERIFICATION:");
    println!("================================================================================\n");

    let after_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_league_clubs",
        [],
        |r| r.get(0),
    )?;

    let unassigned: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_clubs WHERE id NOT IN (SELECT club_id FROM sheffield_league_clubs)",
        [],
        |r| r.get(0),
    )?;

    println!("Clubs assigned: {}", after_count);
    println!("Clubs unassigned: {}", unassigned);
    println!("Total clubs in database: {}\n", main_clubs.len() + reserve_clubs.len());

    if after_count as usize == main_clubs.len() + reserve_clubs.len() {
        println!("✅ SUCCESS! All 372 clubs are now assigned to divisions\n");
    } else {
        println!("⚠️  WARNING: Expected 372 assignments, got {}\n", after_count);
    }

    // Show division sizes
    println!("Final distribution by division:\n");

    let mut stmt = conn.prepare(
        "SELECT slc.division_id, sld.level, COUNT(*) as count
         FROM sheffield_league_clubs slc
         JOIN sheffield_league_divisions sld ON slc.division_id = sld.id
         GROUP BY slc.division_id, sld.level
         ORDER BY sld.level, slc.division_id"
    )?;

    let mut rows = stmt.query([])?;
    let mut current_level = 0;

    while let Some(row) = rows.next()? {
        let div_id: String = row.get(0)?;
        let level: i32 = row.get(1)?;
        let count: i64 = row.get(2)?;

        if level != current_level {
            if current_level > 0 {
                println!();
            }
            println!("Level {}:", level);
            current_level = level;
        }

        println!("  {}: {} clubs", div_id, count);
    }

    println!("\n================================================================================\n");

    Ok(())
}
