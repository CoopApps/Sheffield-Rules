use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let db_path = "../Sheffield1867.db";
    let conn = Connection::open(db_path)?;

    println!("================================================================================");
    println!("FINDING UNASSIGNED CLUBS (372 clubs vs 332 league assignments)");
    println!("================================================================================\n");

    // Get total counts
    let total_clubs: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_clubs",
        [],
        |r| r.get(0),
    )?;

    let assigned_clubs: i64 = conn.query_row(
        "SELECT COUNT(*) FROM sheffield_league_clubs",
        [],
        |r| r.get(0),
    )?;

    let unassigned_count = total_clubs - assigned_clubs;

    println!("Total clubs in sheffield_clubs: {}", total_clubs);
    println!("Clubs assigned to league: {}", assigned_clubs);
    println!("Unassigned clubs: {}\n", unassigned_count);

    // Find which clubs are NOT in the league
    println!("================================================================================");
    println!("CLUBS NOT ASSIGNED TO ANY DIVISION:");
    println!("================================================================================\n");

    let mut stmt = conn.prepare(
        "SELECT sc.id, sc.name, sc.founded_year
         FROM sheffield_clubs sc
         WHERE sc.id NOT IN (SELECT club_id FROM sheffield_league_clubs)
         ORDER BY sc.founded_year, sc.name"
    )?;

    let mut rows = stmt.query([])?;
    let mut count = 0;

    while let Some(row) = rows.next()? {
        count += 1;
        let id: String = row.get(0)?;
        let name: String = row.get(1)?;
        let founded: i32 = row.get(2)?;

        let is_reserve = id.contains("-reserves");
        let reserve_flag = if is_reserve { " (RESERVE)" } else { "" };

        println!("{:3}. [{}] {}{}", count, founded, name, reserve_flag);
        println!("     ID: {}", id);
    }

    println!("\n================================================================================");
    println!("ANALYSIS OF UNASSIGNED CLUBS:");
    println!("================================================================================\n");

    // Analyze by type
    let reserve_count: i64 = conn.query_row(
        "SELECT COUNT(*)
         FROM sheffield_clubs sc
         WHERE sc.id NOT IN (SELECT club_id FROM sheffield_league_clubs)
         AND sc.id LIKE '%-reserves'",
        [],
        |r| r.get(0),
    )?;

    let main_count = unassigned_count - reserve_count;

    println!("Breakdown:");
    println!("  Main teams unassigned: {}", main_count);
    println!("  Reserve teams unassigned: {}\n", reserve_count);

    // Check if they're founded after 1867
    let founded_after_1867: i64 = conn.query_row(
        "SELECT COUNT(*)
         FROM sheffield_clubs sc
         WHERE sc.id NOT IN (SELECT club_id FROM sheffield_league_clubs)
         AND sc.founded_year > 1867",
        [],
        |r| r.get(0),
    )?;

    let founded_1867_or_before: i64 = conn.query_row(
        "SELECT COUNT(*)
         FROM sheffield_clubs sc
         WHERE sc.id NOT IN (SELECT club_id FROM sheffield_league_clubs)
         AND sc.founded_year <= 1867",
        [],
        |r| r.get(0),
    )?;

    println!("By founding year (relative to 1867 season):");
    println!("  Founded 1867 or earlier: {} (should probably be in league)", founded_1867_or_before);
    println!("  Founded after 1867: {} (makes sense to exclude)\n", founded_after_1867);

    // Show clubs founded 1867 or earlier that aren't in league
    if founded_1867_or_before > 0 {
        println!("Clubs founded by 1867 but NOT in league:");
        println!("--------------------------------------------------------------------------------");

        let mut stmt = conn.prepare(
            "SELECT sc.id, sc.name, sc.founded_year
             FROM sheffield_clubs sc
             WHERE sc.id NOT IN (SELECT club_id FROM sheffield_league_clubs)
             AND sc.founded_year <= 1867
             ORDER BY sc.founded_year, sc.name"
        )?;

        let mut rows = stmt.query([])?;
        while let Some(row) = rows.next()? {
            let id: String = row.get(0)?;
            let name: String = row.get(1)?;
            let founded: i32 = row.get(2)?;

            let is_reserve = id.contains("-reserves");
            let reserve_flag = if is_reserve { " (RESERVE)" } else { "" };

            println!("  [{}] {}{}", founded, name, reserve_flag);
        }
        println!();
    }

    // Check league capacity
    println!("================================================================================");
    println!("LEAGUE STRUCTURE CAPACITY:");
    println!("================================================================================\n");

    let mut stmt = conn.prepare(
        "SELECT division_id, COUNT(*) as count
         FROM sheffield_league_clubs
         GROUP BY division_id
         ORDER BY division_id"
    )?;

    let mut rows = stmt.query([])?;
    let mut total_capacity = 0;

    while let Some(row) = rows.next()? {
        let div_id: String = row.get(0)?;
        let count: i64 = row.get(1)?;
        total_capacity += count;
        println!("  {}: {} clubs", div_id, count);
    }

    println!("\nTotal clubs in all divisions: {}", total_capacity);
    println!("Database has {} clubs", total_clubs);
    println!("Difference: {} clubs not assigned\n", total_clubs - total_capacity);

    // Final analysis
    println!("================================================================================");
    println!("WHY ARE 40 CLUBS NOT IN THE LEAGUE?");
    println!("================================================================================\n");

    println!("Possible reasons:");
    println!("  1. ✓ Clubs founded after 1867 - wouldn't exist yet in the 1867 season");
    println!("  2. ? Clubs that disbanded before 1867 - already gone by 1867");
    println!("  3. ? League capacity constraints - only {} slots available", total_capacity);
    println!("  4. ? Historical accuracy - some clubs may not have participated in organized league play");
    println!("  5. ? Reserve team assignments - some reserves might be excluded\n");

    println!("RECOMMENDATION:");
    println!("  - Clubs founded AFTER 1867 should NOT be in the 1867 season league (correct to exclude)");
    println!("  - Clubs founded BY 1867 should probably be reviewed for league inclusion");
    println!("  - The {} unassigned clubs represents {} league slots that could be filled\n", unassigned_count, unassigned_count);

    println!("================================================================================\n");

    Ok(())
}
