use rusqlite::{Connection, Result};
use std::collections::HashSet;

fn main() -> Result<()> {
    let db_path = "../Sheffield1867.db";
    let conn = Connection::open(db_path)?;

    println!("================================================================================");
    println!("COMPARING SHEFFIELD1867.DB WITH EXPECTED SCHEMA");
    println!("================================================================================\n");

    // Expected tables from sheffield_schema.sql
    let expected_tables = vec![
        "sheffield_clubs",
        "sheffield_players",
        "sheffield_matches",
        "sheffield_match_incidents",
        "sheffield_standings",
        "sheffield_game_state",
        "sheffield_rules_history",
        "sheffield_fixtures",
        "sheffield_league_divisions",
        "sheffield_league_clubs",
        "sheffield_league_promotion_rules",
        "sheffield_league_movement_history",
        "sheffield_competitions",
        "sheffield_cup_ties",
        "sheffield_competition_participants",
        "sheffield_competition_winners",
        "sheffield_match_lineups",
        "sheffield_match_timeline",
        "sheffield_match_weather",
        "sheffield_newspaper_archive",
        "sheffield_season_archives",
        "sheffield_player_season_stats",
        "sheffield_club_season_stats",
        "sheffield_cooperative_system",
        "sheffield_cooperative_memberships",
        "sheffield_cooperative_transactions",
        "sheffield_cooperative_events",
        "sheffield_match_visual_states",
        "sheffield_match_events",
        "sheffield_formations",
        "sheffield_league_config",
    ];

    // Get actual tables from database
    let mut stmt = conn.prepare(
        "SELECT name FROM sqlite_master WHERE type='table' AND name LIKE 'sheffield_%' ORDER BY name"
    )?;

    let actual_tables: Vec<String> = stmt
        .query_map([], |row| row.get(0))?
        .collect::<Result<Vec<_>, _>>()?;

    let expected_set: HashSet<&str> = expected_tables.iter().cloned().collect();
    let actual_set: HashSet<String> = actual_tables.iter().cloned().collect();

    println!("EXPECTED TABLES (from schema): {}", expected_tables.len());
    println!("ACTUAL TABLES (in database): {}\n", actual_tables.len());

    // Check for missing tables
    println!("================================================================================");
    println!("MISSING TABLES (Expected but not in database):");
    println!("================================================================================\n");

    let mut missing_count = 0;
    for expected in &expected_tables {
        if !actual_set.contains(*expected) {
            missing_count += 1;
            println!("  ❌ {}", expected);
        }
    }

    if missing_count == 0 {
        println!("  ✓ No missing tables - all expected tables exist!\n");
    } else {
        println!("\nTotal missing: {}\n", missing_count);
    }

    // Check for extra tables
    println!("================================================================================");
    println!("EXTRA TABLES (In database but not in schema):");
    println!("================================================================================\n");

    let mut extra_count = 0;
    for actual in &actual_tables {
        if !expected_set.contains(actual.as_str()) {
            extra_count += 1;

            // Get row count
            let count: i64 = conn.query_row(
                &format!("SELECT COUNT(*) FROM {}", actual),
                [],
                |r| r.get(0),
            ).unwrap_or(0);

            println!("  ℹ️  {} ({} rows)", actual, count);
        }
    }

    if extra_count == 0 {
        println!("  No extra tables\n");
    } else {
        println!("\nTotal extra: {} (probably profession/occupation tables)\n", extra_count);
    }

    // Check critical tables with data
    println!("================================================================================");
    println!("CRITICAL TABLES STATUS:");
    println!("================================================================================\n");

    let critical_tables = vec![
        ("sheffield_clubs", 372, "Must have 372 clubs"),
        ("sheffield_league_clubs", 372, "Must have 372 league assignments"),
        ("sheffield_league_divisions", 28, "Must have 28 divisions"),
        ("sheffield_competitions", 2, "Should have 2 cups (Youdan + Cromwell)"),
        ("sheffield_footballers", 45000, "Should have ~45k historical players"),
        ("sheffield_players", 0, "Empty until game starts"),
        ("sheffield_standings", 0, "Empty until game starts"),
        ("sheffield_fixtures", 0, "Empty until game starts"),
        ("sheffield_matches", 0, "Empty until game starts"),
        ("sheffield_game_state", 0, "Empty until game starts"),
    ];

    for (table, expected_count, description) in &critical_tables {
        if actual_set.contains(&table.to_string()) {
            let count: i64 = conn.query_row(
                &format!("SELECT COUNT(*) FROM {}", table),
                [],
                |r| r.get(0),
            ).unwrap_or(0);

            let status = if count >= *expected_count || *expected_count == 0 {
                "✓"
            } else if count == 0 && *expected_count > 0 {
                "⚠️"
            } else {
                "❌"
            };

            println!("  {} {:<35} {} rows ({})", status, table, count, description);
        } else {
            println!("  ❌ {:<35} MISSING!", table);
        }
    }

    println!("\n================================================================================");
    println!("SUMMARY:");
    println!("================================================================================\n");

    if missing_count == 0 {
        println!("✅ All {} expected game tables exist", expected_tables.len());
    } else {
        println!("⚠️  {} tables missing from schema", missing_count);
    }

    if extra_count > 0 {
        println!("ℹ️  {} extra tables (profession/occupation data)", extra_count);
    }

    // Overall assessment
    println!("\nOVERALL ASSESSMENT:\n");

    let clubs_ok = actual_set.contains(&"sheffield_clubs".to_string());
    let league_ok = actual_set.contains(&"sheffield_league_clubs".to_string());
    let divisions_ok = actual_set.contains(&"sheffield_league_divisions".to_string());
    let competitions_ok = actual_set.contains(&"sheffield_competitions".to_string());
    let game_state_ok = actual_set.contains(&"sheffield_game_state".to_string());

    if clubs_ok && league_ok && divisions_ok && game_state_ok {
        println!("✅ CORE GAME TABLES: All present");

        if competitions_ok {
            println!("✅ CUP COMPETITIONS: Table exists");
        } else {
            println!("❌ CUP COMPETITIONS: Missing table");
        }

        println!("\nThe database should work with both the game and editor!");
    } else {
        println!("❌ CRITICAL TABLES MISSING - Database may not work properly");
        if !clubs_ok { println!("   - sheffield_clubs missing"); }
        if !league_ok { println!("   - sheffield_league_clubs missing"); }
        if !divisions_ok { println!("   - sheffield_league_divisions missing"); }
        if !game_state_ok { println!("   - sheffield_game_state missing"); }
    }

    println!("\n================================================================================\n");

    Ok(())
}
