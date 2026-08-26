use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let db_path = "../Sheffield1867.db";
    let conn = Connection::open(db_path)?;

    println!("================================================================================");
    println!("CHECKING CUP COMPETITIONS IN SHEFFIELD1867.DB");
    println!("================================================================================\n");

    // Check if table exists
    let table_exists: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='sheffield_cup_competitions'",
        [],
        |r| r.get(0),
    )?;

    if table_exists == 0 {
        println!("❌ sheffield_cup_competitions table does NOT exist\n");
        println!("This is likely causing the hang - the code is trying to query a non-existent table!\n");
        return Ok(());
    }

    println!("✓ sheffield_cup_competitions table exists\n");

    // Get all cup competitions
    let mut stmt = conn.prepare(
        "SELECT id, name, season, announcement_week, draw_week, eligible_teams
         FROM sheffield_cup_competitions
         ORDER BY season, announcement_week"
    )?;

    let competitions: Vec<(String, String, i32, i32, i32, Option<String>)> = stmt
        .query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    if competitions.is_empty() {
        println!("⚠️  No cup competitions found in database\n");
        println!("The initialize_season_cup_competitions function will return empty but should not hang.\n");
    } else {
        println!("Found {} cup competitions:\n", competitions.len());
        for (id, name, season, announce_week, draw_week, eligible) in &competitions {
            println!("  {} ({})", name, id);
            println!("    Season: {}", season);
            println!("    Announcement week: {}", announce_week);
            println!("    Draw week: {}", draw_week);
            println!("    Eligible teams: {}", eligible.as_deref().unwrap_or("All"));
            println!();
        }
    }

    // Check for 1867 specifically
    println!("================================================================================");
    println!("COMPETITIONS FOR 1867 SEASON:");
    println!("================================================================================\n");

    let count_1867: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_cup_competitions WHERE season = 1867",
        [],
        |r| r.get(0),
    )?;

    println!("Competitions for 1867: {}\n", count_1867);

    if count_1867 > 0 {
        let mut stmt = conn.prepare(
            "SELECT id, name, announcement_week, draw_week
             FROM sheffield_cup_competitions
             WHERE season = 1867"
        )?;

        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i32>(2)?,
                row.get::<_, i32>(3)?,
            ))
        })?;

        for row in rows {
            let (id, name, announce, draw) = row?;
            println!("  {}", name);
            println!("    ID: {}", id);
            println!("    Announces: Week {}", announce);
            println!("    Draw: Week {}", draw);
            println!();
        }
    }

    // Check sheffield_cup_draws table
    println!("================================================================================");
    println!("CHECKING sheffield_cup_draws TABLE:");
    println!("================================================================================\n");

    let draws_exist: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='sheffield_cup_draws'",
        [],
        |r| r.get(0),
    )?;

    if draws_exist == 0 {
        println!("❌ sheffield_cup_draws table does NOT exist\n");
        println!("This could cause the generate_cup_draw function to hang!\n");
    } else {
        println!("✓ sheffield_cup_draws table exists\n");

        let draw_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sheffield_cup_draws",
            [],
            |r| r.get(0),
        )?;

        println!("Existing cup draws: {}\n", draw_count);
    }

    println!("================================================================================\n");

    Ok(())
}
