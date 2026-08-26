use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;

    println!("Adding stat columns to sheffield_footballers table...\n");

    let columns = vec![
        "position TEXT",
        "nationality TEXT",
        // Physical
        "pace INTEGER",
        "acceleration INTEGER",
        "strength INTEGER",
        "stamina INTEGER",
        "balance INTEGER",
        "jumping INTEGER",
        "agility INTEGER",
        "natural_fitness INTEGER",
        // Technical
        "passing INTEGER",
        "dribbling INTEGER",
        "first_touch INTEGER",
        "technique INTEGER",
        "heading INTEGER",
        "long_passing INTEGER",
        "crossing INTEGER",
        "long_shots INTEGER",
        "tackling INTEGER",
        "handling INTEGER",
        "reflexes INTEGER",
        "corners INTEGER",
        "free_kicks INTEGER",
        "throw_ins INTEGER",
        "vision INTEGER",
        "left_foot INTEGER",
        "right_foot INTEGER",
        "one_on_ones INTEGER",
        // Mental
        "courage INTEGER",
        "bravery INTEGER",
        "concentration INTEGER",
        "decision_making INTEGER",
        "leadership INTEGER",
        "aggression INTEGER",
        "anticipation INTEGER",
        "determination INTEGER",
        "flair INTEGER",
        "influence INTEGER",
        "adaptability INTEGER",
        "ambition INTEGER",
        "loyalty INTEGER",
        "pressure INTEGER",
        "professionalism INTEGER",
        "sportsmanship INTEGER",
        "temperament INTEGER",
        // Positioning
        "awareness INTEGER",
        "marking INTEGER",
        "positioning INTEGER",
        "work_rate INTEGER",
        "off_the_ball INTEGER",
        "movement INTEGER",
        "teamwork INTEGER",
        // Specialization
        "finishing INTEGER",
        "penalties INTEGER",
        "set_pieces INTEGER",
        // Hidden
        "consistency INTEGER",
        "dirtiness INTEGER",
        "versatility INTEGER",
        "injury_proneness INTEGER",
        "important_matches INTEGER",
        // Ability & Reputation
        "current_ability INTEGER",
        "potential_ability INTEGER",
        "current_reputation INTEGER",
    ];

    let mut completed = 0;
    let mut errors = 0;

    for column in columns {
        let column_name = column.split(' ').next().unwrap();
        let sql = format!("ALTER TABLE sheffield_footballers ADD COLUMN {}", column);

        match conn.execute(&sql, []) {
            Ok(_) => {
                println!("✓ {}", column_name);
                completed += 1;
            }
            Err(e) => {
                if e.to_string().contains("duplicate column name") {
                    println!("⊘ {} - already exists", column_name);
                } else {
                    println!("✗ {} - Error: {}", column_name, e);
                }
                errors += 1;
            }
        }
    }

    println!("\n✓ Completed! Added {} columns", completed);
    if errors > 0 {
        println!("⊘ {} columns already existed or had errors", errors);
    }

    Ok(())
}
