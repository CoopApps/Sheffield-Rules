use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let db_path = "../Sheffield1867.db";
    let conn = Connection::open(db_path)?;

    println!("================================================================================");
    println!("CHECKING sheffield_league_clubs TABLE STRUCTURE");
    println!("================================================================================\n");

    // Get table structure
    println!("Columns in sheffield_league_clubs:\n");
    let mut stmt = conn.prepare("PRAGMA table_info(sheffield_league_clubs)")?;
    let mut rows = stmt.query([])?;

    let mut columns = Vec::new();
    while let Some(row) = rows.next()? {
        let name: String = row.get(1)?;
        let col_type: String = row.get(2)?;
        let not_null: i32 = row.get(3)?;

        println!("  {:<25} | {:<10} | {}",
            name,
            col_type,
            if not_null == 1 { "NOT NULL" } else { "NULL" }
        );

        columns.push(name);
    }

    println!("\n================================================================================");
    println!("SAMPLE DATA FROM sheffield_league_clubs:");
    println!("================================================================================\n");

    let mut stmt = conn.prepare("SELECT * FROM sheffield_league_clubs LIMIT 5")?;
    let mut rows = stmt.query([])?;
    let mut count = 0;

    while let Some(row) = rows.next()? {
        count += 1;
        println!("Row {}:", count);

        let id: String = row.get(0)?;
        let division_id: String = row.get(1)?;
        let club_id: String = row.get(2)?;
        let position: Option<i32> = row.get(3)?;
        let is_reserve: i32 = row.get(4)?;
        let reserve_of: Option<String> = row.get(5)?;

        println!("  id:                      {}", id);
        println!("  division_id:             {}", division_id);
        println!("  club_id:                 {}", club_id);
        println!("  position_in_division:    {:?}", position);
        println!("  is_reserve_team:         {}", is_reserve);
        println!("  reserve_of_club_id:      {:?}", reserve_of);
        println!();
    }

    println!("================================================================================");
    println!("CHECKING sheffield_league_divisions TABLE:");
    println!("================================================================================\n");

    let mut stmt = conn.prepare(
        "SELECT id, name, level, region FROM sheffield_league_divisions ORDER BY level, id"
    )?;
    let mut rows = stmt.query([])?;

    println!("All divisions:\n");
    let mut current_level = 0;

    while let Some(row) = rows.next()? {
        let id: String = row.get(0)?;
        let name: String = row.get(1)?;
        let level: i32 = row.get(2)?;
        let region: Option<String> = row.get(3)?;

        if level != current_level {
            if current_level > 0 {
                println!();
            }
            current_level = level;
        }

        let region_str = region
            .map(|r| format!(" ({})", r))
            .unwrap_or_default();

        println!("  Level {}: {:<15} - {}{}", level, id, name, region_str);
    }

    println!("\n================================================================================");
    println!("ANSWER: Is there a league identifier column?");
    println!("================================================================================\n");

    let has_division_id = columns.contains(&"division_id".to_string());
    let has_league_id = columns.contains(&"league_id".to_string());
    let has_competition_id = columns.contains(&"competition_id".to_string());

    println!("division_id column:      {}", if has_division_id { "✓ YES" } else { "✗ NO" });
    println!("league_id column:        {}", if has_league_id { "✓ YES" } else { "✗ NO" });
    println!("competition_id column:   {}", if has_competition_id { "✓ YES" } else { "✗ NO" });

    println!("\n{}", "=".repeat(80));
    println!("CONCLUSION:");
    println!("{}\n", "=".repeat(80));

    if has_division_id {
        println!("✓ The \"division_id\" column identifies which division (and therefore which league)");
        println!("  a club is in.\n");
        println!("How it works:");
        println!("  - division_id references sheffield_league_divisions.id");
        println!("  - Each division belongs to a specific level (1-7)");
        println!("  - Main divisions: div-1, div-2, div-3, etc.");
        println!("  - Reserve divisions: res-div-1, res-div-2, etc.");
        println!("  - Regional divisions at lower levels: div-6a (West), div-6b (East), etc.\n");

        println!("Example:");
        println!("  club_id = \"sheffield-fc\"");
        println!("  division_id = \"div-1\"");
        println!("  → Sheffield FC is in Division 1 (Level 1, Main Pyramid)\n");

        println!("  club_id = \"sheffield-fc-reserves\"");
        println!("  division_id = \"res-div-1\"");
        println!("  → Sheffield FC Reserves is in Reserve Division 1 (Level 1, Reserve Pyramid)\n");

        println!("Main vs Reserve Pyramids:");
        println!("  - Main pyramid: div-1, div-2, div-3, div-4, div-5a/b, div-6a/b/c/d, div-7a/b");
        println!("  - Reserve pyramid: res-div-1, res-div-2, etc. (same structure)\n");
    } else {
        println!("✗ No league identifier column found.");
        println!("  Clubs are not assigned to divisions in this table.\n");
    }

    println!("{}\n", "=".repeat(80));

    Ok(())
}
