use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let db_path = "../Sheffield1867.db";
    let conn = Connection::open(db_path)?;

    println!("================================================================================");
    println!("ADDING YOUDAN CUP (1867) AND CROMWELL CUP (1868) TO SHEFFIELD1867.DB");
    println!("================================================================================\n");

    // Check if tables exist
    let competitions_exist: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='sheffield_competitions'",
        [],
        |r| r.get(0),
    )?;

    if competitions_exist == 0 {
        println!("❌ sheffield_competitions table does NOT exist!");
        println!("   Need to create schema first.\n");
        return Ok(());
    }

    println!("✓ sheffield_competitions table exists\n");

    // Start transaction
    conn.execute("BEGIN TRANSACTION", [])?;

    println!("================================================================================");
    println!("YOUDAN CUP 1867");
    println!("================================================================================\n");

    println!("Inserting Youdan Cup 1867...");

    conn.execute(
        "INSERT OR REPLACE INTO sheffield_competitions (
            id, name, competition_type, season,
            min_division_level, max_division_level,
            current_round, total_rounds, is_active,
            rules_type, prestige_level,
            start_week, announcement_week, draw_week
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        [
            "youdan-cup-1867",      // id
            "Youdan Cup",           // name
            "knockout_cup",         // competition_type
            "1867",                 // season
            "1",                    // min_division_level (Divisions 1-3)
            "3",                    // max_division_level
            "0",                    // current_round (not started)
            "4",                    // total_rounds (12 teams = R1, R2, SF, F)
            "1",                    // is_active
            "sheffield_rules",      // rules_type
            "high",                 // prestige_level
            "6",                    // start_week (early February)
            "2",                    // announcement_week (late January)
            "4",                    // draw_week
        ],
    )?;

    println!("✓ Youdan Cup 1867 inserted");
    println!("  - Open to: Divisions 1-3 (top 48 clubs)");
    println!("  - Announcement: Week 2 (late January)");
    println!("  - Draw: Week 4");
    println!("  - Start: Week 6 (early February)");
    println!("  - Format: 12-team knockout (4 rounds)");
    println!("  - Prestige: HIGH (world's first football cup!)");
    println!("  - Historical winner: Hallam FC");
    println!("  - Historical runner-up: Norfolk FC");

    println!("\n================================================================================");
    println!("CROMWELL CUP 1868");
    println!("================================================================================\n");

    println!("Inserting Cromwell Cup 1868...");

    conn.execute(
        "INSERT OR REPLACE INTO sheffield_competitions (
            id, name, competition_type, season,
            min_division_level, max_division_level,
            current_round, total_rounds, is_active,
            rules_type, prestige_level,
            start_week, announcement_week, draw_week
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        [
            "cromwell-cup-1868",    // id
            "Cromwell Cup",         // name
            "knockout_cup",         // competition_type
            "1867",                 // season (starts in 1867 season)
            "4",                    // min_division_level (Divisions 4-7)
            "7",                    // max_division_level
            "0",                    // current_round (not started)
            "2",                    // total_rounds (4 teams = SF, F)
            "1",                    // is_active
            "sheffield_rules",      // rules_type
            "standard",             // prestige_level
            "30",                   // start_week (February 1868)
            "26",                   // announcement_week (January 1868)
            "28",                   // draw_week
        ],
    )?;

    println!("✓ Cromwell Cup 1868 inserted");
    println!("  - Open to: Divisions 4-7 (lower 324 clubs)");
    println!("  - Announcement: Week 26 (January 1868)");
    println!("  - Draw: Week 28");
    println!("  - Start: Week 30 (February 1868)");
    println!("  - Format: 4-team knockout (2 rounds: SF, F)");
    println!("  - Prestige: STANDARD");
    println!("  - Historical winner: Sheffield Wednesday FC");
    println!("  - Historical runner-up: Garrick FC");

    // Commit transaction
    conn.execute("COMMIT", [])?;
    println!("\n✅ Transaction committed\n");

    // Verify results
    println!("================================================================================");
    println!("VERIFICATION:");
    println!("================================================================================\n");

    let mut stmt = conn.prepare(
        "SELECT id, name, season, min_division_level, max_division_level,
                prestige_level, start_week, announcement_week
         FROM sheffield_competitions
         ORDER BY season, start_week"
    )?;

    let competitions: Vec<(String, String, i32, i32, i32, String, i32, i32)> = stmt
        .query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
                row.get(7)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    println!("Total competitions in database: {}\n", competitions.len());

    for (id, name, season, min_div, max_div, prestige, start_week, announce_week) in &competitions {
        println!("{} ({})", name, season);
        println!("  ID: {}", id);
        println!("  Divisions: {}-{}", min_div, max_div);
        println!("  Prestige: {}", prestige);
        println!("  Announces: Week {}, Starts: Week {}", announce_week, start_week);
        println!();
    }

    println!("================================================================================");
    println!("SUMMARY:");
    println!("================================================================================\n");

    println!("✓ Youdan Cup 1867 added for top divisions (1-3)");
    println!("✓ Cromwell Cup 1868 added for lower divisions (4-7)");
    println!("\nBoth competitions will now be loaded when you start a Sheffield & Hallamshire");
    println!("Fantasy League game in 1867.\n");

    println!("WHAT HAPPENS IN GAME:");
    println!("  Week 2:  📰 Youdan Cup announced");
    println!("  Week 4:  🎲 Youdan Cup draw made");
    println!("  Week 6:  ⚽ Youdan Cup matches begin");
    println!("  Week 26: 📰 Cromwell Cup announced");
    println!("  Week 28: 🎲 Cromwell Cup draw made");
    println!("  Week 30: ⚽ Cromwell Cup matches begin\n");

    println!("Historical note:");
    println!("  The Youdan Cup (1867) preceded the FA Cup by 4 years and is the world's");
    println!("  oldest football trophy. The actual trophy is now valued at £350,000 and");
    println!("  held by Hallam FC, who won the original competition.\n");

    println!("================================================================================\n");

    Ok(())
}
