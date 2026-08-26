use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let db_path = "../Sheffield1867.db";

    println!("================================================================================");
    println!("SHEFFIELD1867.DB - SOURCE OF TRUTH CHECK");
    println!("================================================================================\n");

    let conn = Connection::open(db_path)?;

    // STEP 1: Check league assignments (MOST CRITICAL)
    println!("STEP 1: What clubs are in the league?");
    println!("Code queries: SELECT club_id FROM sheffield_league_clubs");
    println!("--------------------------------------------------------------------------------");

    let league_exists: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='sheffield_league_clubs'",
        [],
        |r| r.get(0),
    )?;

    if league_exists == 0 {
        println!("❌ Table does not exist!\n");
    } else {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sheffield_league_clubs",
            [],
            |r| r.get(0),
        )?;

        if count == 0 {
            println!("❌ EMPTY! No clubs assigned to league.");
            println!("Result: Game will initialize with 0 clubs in standings\n");
        } else {
            println!("✓ Found {} club assignments\n", count);

            // Show distribution
            println!("Sample from Division 1:");
            let mut stmt = conn.prepare(
                "SELECT club_id, division_id, position_in_division, is_reserve_team
                 FROM sheffield_league_clubs
                 WHERE division_id = '1'
                 ORDER BY position_in_division
                 LIMIT 10"
            )?;
            let mut rows = stmt.query([])?;
            while let Some(row) = rows.next()? {
                let club_id: String = row.get(0)?;
                let div_id: String = row.get(1)?;
                let pos: i32 = row.get(2)?;
                let is_reserve: i32 = row.get(3)?;
                let reserve = if is_reserve == 1 { " (RESERVE)" } else { "" };
                println!("  Pos {}: {} [div: {}]{}", pos, club_id, div_id, reserve);
            }
        }
    }

    // STEP 2: Check clubs
    println!("\n================================================================================");
    println!("STEP 2: What clubs exist in the database?");
    println!("Code queries: SELECT id FROM sheffield_clubs ORDER BY name");
    println!("--------------------------------------------------------------------------------");

    let clubs_exists: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='sheffield_clubs'",
        [],
        |r| r.get(0),
    )?;

    if clubs_exists == 0 {
        println!("❌ Table does not exist!\n");
    } else {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sheffield_clubs",
            [],
            |r| r.get(0),
        )?;
        println!("✓ Found {} total clubs\n", count);

        println!("First 10 clubs by founding:");
        let mut stmt = conn.prepare(
            "SELECT id, name, founded_year FROM sheffield_clubs ORDER BY founded_year LIMIT 10"
        )?;
        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let year: i32 = row.get(2)?;
            println!("  {}: {} ({})", year, name, id);
        }
    }

    // STEP 3: Check divisions
    println!("\n================================================================================");
    println!("STEP 3: What league structure is defined?");
    println!("--------------------------------------------------------------------------------");

    let div_exists: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='sheffield_league_divisions'",
        [],
        |r| r.get(0),
    )?;

    if div_exists == 0 {
        println!("❌ Table does not exist!\n");
    } else {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sheffield_league_divisions",
            [],
            |r| r.get(0),
        )?;
        println!("✓ Found {} divisions\n", count);

        println!("Division structure:");
        let mut stmt = conn.prepare(
            "SELECT id, name, level, region FROM sheffield_league_divisions ORDER BY level, id"
        )?;
        let mut rows = stmt.query([])?;
        let mut current_level = 0;
        while let Some(row) = rows.next()? {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let level: i32 = row.get(2)?;
            let region: Option<String> = row.get(3)?;

            if level != current_level {
                println!("\n  Level {}:", level);
                current_level = level;
            }
            let reg = region.map(|r| format!(" ({})", r)).unwrap_or_default();
            println!("    - {}: {}{}", id, name, reg);
        }
    }

    // STEP 4: Check game state tables
    println!("\n================================================================================");
    println!("STEP 4: Are game state tables clear?");
    println!("These should be EMPTY at initialization");
    println!("--------------------------------------------------------------------------------");

    let tables = vec![
        "sheffield_game_state",
        "sheffield_standings",
        "sheffield_fixtures",
        "sheffield_matches"
    ];

    for table in tables {
        let exists: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?",
            [table],
            |r| r.get(0),
        )?;

        if exists == 0 {
            println!("  {}: ✗ Does not exist", table);
        } else {
            let count: i64 = conn.query_row(
                &format!("SELECT COUNT(*) FROM {}", table),
                [],
                |r| r.get(0),
            )?;
            let status = if count == 0 {
                "✓ Empty (correct)"
            } else {
                "⚠ Has data (should be empty)"
            };
            println!("  {}: {}", table, status);
        }
    }

    // FINAL VERDICT
    println!("\n================================================================================");
    println!("DATABASE AS SOURCE OF TRUTH - FINAL ANALYSIS");
    println!("================================================================================\n");

    let league_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_league_clubs",
        [],
        |r| r.get(0),
    ).unwrap_or(0);

    let club_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_clubs",
        [],
        |r| r.get(0),
    ).unwrap_or(0);

    let div_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_league_divisions",
        [],
        |r| r.get(0),
    ).unwrap_or(0);

    if league_count > 0 && club_count > 0 && div_count > 0 {
        println!("✅ DATABASE IS VALID - Ready for Sheffield-Hallamshire League mode\n");
        println!("The game will use this database as the SOLE SOURCE OF TRUTH for:");
        println!("  - Which clubs exist: {} clubs in sheffield_clubs", club_count);
        println!("  - Which divisions exist: {} divisions in sheffield_league_divisions", div_count);
        println!("  - Which clubs are in which divisions: {} assignments in sheffield_league_clubs", league_count);
        println!("  - Initial standings will be generated from sheffield_league_clubs");
        println!("  - Fixtures will be generated based on divisional assignments\n");
    } else {
        println!("❌ DATABASE IS NOT READY\n");
        if league_count == 0 {
            println!("  ❌ CRITICAL: sheffield_league_clubs is EMPTY or missing");
        }
        if club_count == 0 {
            println!("  ❌ CRITICAL: sheffield_clubs is EMPTY or missing");
        }
        if div_count == 0 {
            println!("  ❌ CRITICAL: sheffield_league_divisions is EMPTY or missing");
        }
        println!("\nThe game REQUIRES a properly populated database to function.\n");
    }

    println!("================================================================================\n");

    Ok(())
}
