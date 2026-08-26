use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";

    println!("================================================================================");
    println!("SHEFFIELD1867.DB - SOURCE OF TRUTH CHECK");
    println!("================================================================================\n");

    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?
        .create_if_missing(false);

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(connect_options)
        .await?;

    // Check 1: League club assignments (MOST CRITICAL)
    println!("STEP 1: What clubs are in the league?");
    println!("Code queries: SELECT club_id FROM sheffield_league_clubs");
    println!("--------------------------------------------------------------------------------");

    let league_clubs = sqlx::query("SELECT COUNT(*) as count FROM sheffield_league_clubs")
        .fetch_one(&pool)
        .await?;
    let count: i64 = league_clubs.get("count");

    if count == 0 {
        println!("❌ EMPTY! No clubs assigned to league.");
        println!("Result: Game will initialize with 0 clubs in standings\n");
    } else {
        println!("✓ Found {} club assignments\n", count);

        // Sample some assignments
        let sample: Vec<(String, String, i32)> = sqlx::query_as(
            "SELECT club_id, division_id, position_in_division
             FROM sheffield_league_clubs
             WHERE division_id = '1'
             ORDER BY position_in_division
             LIMIT 10"
        )
        .fetch_all(&pool)
        .await?;

        println!("Sample from Division 1:");
        for (club_id, div_id, pos) in sample {
            println!("  Pos {}: {} (div: {})", pos, club_id, div_id);
        }
    }

    // Check 2: Total clubs
    println!("\n================================================================================");
    println!("STEP 2: What clubs exist in the database?");
    println!("Code queries: SELECT id FROM sheffield_clubs ORDER BY name");
    println!("--------------------------------------------------------------------------------");

    let total_clubs = sqlx::query("SELECT COUNT(*) as count FROM sheffield_clubs")
        .fetch_one(&pool)
        .await?;
    let club_count: i64 = total_clubs.get("count");
    println!("✓ Found {} total clubs\n", club_count);

    let sample_clubs: Vec<(String, String, i32)> = sqlx::query_as(
        "SELECT id, name, founded_year FROM sheffield_clubs ORDER BY founded_year LIMIT 10"
    )
    .fetch_all(&pool)
    .await?;

    println!("First 10 clubs by founding:");
    for (id, name, year) in sample_clubs {
        println!("  {}: {} ({})", year, name, id);
    }

    // Check 3: Divisions
    println!("\n================================================================================");
    println!("STEP 3: What league structure is defined?");
    println!("--------------------------------------------------------------------------------");

    let division_count = sqlx::query("SELECT COUNT(*) as count FROM sheffield_league_divisions")
        .fetch_one(&pool)
        .await?;
    let div_count: i64 = division_count.get("count");
    println!("✓ Found {} divisions\n", div_count);

    let divisions: Vec<(String, String, i32)> = sqlx::query_as(
        "SELECT id, name, level FROM sheffield_league_divisions ORDER BY level, id"
    )
    .fetch_all(&pool)
    .await?;

    let mut current_level = 0;
    for (id, name, level) in divisions {
        if level != current_level {
            println!("\n  Level {}:", level);
            current_level = level;
        }
        println!("    - {}: {}", id, name);
    }

    // Check 4: Game state tables (should be empty)
    println!("\n================================================================================");
    println!("STEP 4: Are game state tables clear?");
    println!("--------------------------------------------------------------------------------");

    let game_state = sqlx::query("SELECT COUNT(*) as count FROM sheffield_game_state")
        .fetch_one(&pool)
        .await?;
    let gs_count: i64 = game_state.get("count");
    let gs_status = if gs_count == 0 { "✓ Empty (correct)" } else { "⚠ Has data (should be empty)" };
    println!("  sheffield_game_state: {}", gs_status);

    let standings = sqlx::query("SELECT COUNT(*) as count FROM sheffield_standings")
        .fetch_one(&pool)
        .await?;
    let st_count: i64 = standings.get("count");
    let st_status = if st_count == 0 { "✓ Empty (correct)" } else { "⚠ Has data (should be empty)" };
    println!("  sheffield_standings: {}", st_status);

    // FINAL VERDICT
    println!("\n================================================================================");
    println!("DATABASE AS SOURCE OF TRUTH - FINAL ANALYSIS");
    println!("================================================================================\n");

    if count > 0 && club_count > 0 && div_count > 0 {
        println!("✅ DATABASE IS VALID - Ready for Sheffield-Hallamshire League mode\n");
        println!("The game will use this database as the SOLE SOURCE OF TRUTH for:");
        println!("  - Which clubs exist: {} clubs in sheffield_clubs", club_count);
        println!("  - Which divisions exist: {} divisions in sheffield_league_divisions", div_count);
        println!("  - Which clubs are in which divisions: {} assignments in sheffield_league_clubs", count);
        println!("  - Initial standings will be generated from sheffield_league_clubs");
        println!("  - Fixtures will be generated based on divisional assignments\n");
    } else {
        println!("❌ DATABASE IS NOT READY\n");
        if count == 0 {
            println!("  ❌ CRITICAL: sheffield_league_clubs is EMPTY");
        }
        if club_count == 0 {
            println!("  ❌ CRITICAL: sheffield_clubs is EMPTY");
        }
        if div_count == 0 {
            println!("  ❌ CRITICAL: sheffield_league_divisions is EMPTY");
        }
        println!("\nThe game REQUIRES a properly populated database to function.\n");
    }

    println!("================================================================================\n");

    Ok(())
}
