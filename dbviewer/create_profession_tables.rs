use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;

    // Define profession categories and their search terms
    let categories: Vec<(&str, Vec<&str>)> = vec![
        ("sheffield_labourers", vec!["labourer", "laborer"]),
        ("sheffield_file_workers", vec!["file cutter", "file forger", "file smith", "file grinder", "file maker", "file hafter"]),
        ("sheffield_miners", vec!["coal miner", "miner", "collier", "pit"]),
        ("sheffield_servants", vec!["domestic servant", "general servant", "servant", "house keeper", "housekeeper", "char woman", "charwoman"]),
        ("sheffield_dressmakers", vec!["dress maker", "dressmaker", "milliner", "seamstress"]),
        ("sheffield_carters", vec!["carter", "carman", "carrier", "drayman", "waggoner", "wagoner"]),
        ("sheffield_cutlers", vec!["cutler", "spring knife cutler", "table knife cutler", "pen knife cutler", "pocket knife cutler"]),
        ("sheffield_knife_workers", vec!["knife cutter", "knife grinder", "knife hafter", "knife forger", "blade grinder", "blade forger"]),
        ("sheffield_soldiers", vec!["soldier", "private", "corporal", "sergeant", "army", "militia", "pensioner army"]),
        ("sheffield_joiners", vec!["joiner", "cabinet maker", "cabinetmaker", "carpenter"]),
        ("sheffield_tailors", vec!["tailor", "tailoress"]),
        ("sheffield_shoemakers", vec!["shoe maker", "shoemaker", "boot maker", "bootmaker", "cordwainer"]),
        ("sheffield_blacksmiths", vec!["blacksmith", "farrier"]),
        ("sheffield_silversmiths", vec!["silver smith", "silversmith", "silver plater", "silver burnisher", "electro plater"]),
        ("sheffield_butchers", vec!["butcher"]),
        ("sheffield_steel_workers", vec!["steel melter", "steel roller", "steel maker", "steel converter", "furnace man", "furnaceman"]),
        ("sheffield_grinders", vec!["grinder"]),
        ("sheffield_grocers", vec!["grocer", "provision dealer"]),
        ("sheffield_sawmakers", vec!["saw maker", "sawmaker", "saw grinder", "saw smith"]),
        ("sheffield_hawkers", vec!["hawker", "pedlar", "pedler", "peddler"]),
        ("sheffield_laundresses", vec!["laundress", "washerwoman", "washer woman", "laundry"]),
        ("sheffield_bricklayers", vec!["brick layer", "bricklayer", "brick maker", "brickmaker"]),
        ("sheffield_engine_workers", vec!["engine tenter", "engine fitter", "engine driver", "engineer", "engineman"]),
        ("sheffield_moulders", vec!["moulder", "molder", "iron moulder"]),
        ("sheffield_scholars", vec!["scholar"]),
        ("sheffield_bakers", vec!["baker", "confectioner"]),
        ("sheffield_painters", vec!["painter", "house painter", "decorator"]),
        ("sheffield_plumbers", vec!["plumber", "glazier", "gas fitter"]),
        ("sheffield_weavers", vec!["weaver", "warper", "spinner"]),
        ("sheffield_gardeners", vec!["gardener", "nurseryman"]),
        ("sheffield_teachers", vec!["teacher", "schoolmaster", "schoolmistress", "governess", "tutor"]),
        ("sheffield_nurses", vec!["nurse", "sick nurse", "monthly nurse", "midwife"]),
        ("sheffield_police", vec!["police", "constable", "policeman"]),
        ("sheffield_railway", vec!["railway"]),
        ("sheffield_brewers", vec!["brewer", "maltster", "brewery"]),
        ("sheffield_printers", vec!["printer", "compositor", "typesetter", "bookbinder"]),
        ("sheffield_coopers", vec!["cooper", "barrel maker"]),
        ("sheffield_wheelwrights", vec!["wheelwright", "coachmaker", "coach maker"]),
        ("sheffield_potters", vec!["potter", "earthenware"]),
        ("sheffield_rope_makers", vec!["rope maker", "ropemaker", "twine"]),
    ];

    for (table_name, terms) in &categories {
        eprintln!("Creating {}...", table_name);

        // Drop and create table
        conn.execute(&format!("DROP TABLE IF EXISTS {}", table_name), [])?;
        conn.execute(
            &format!(
                "CREATE TABLE {} (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    person_id INTEGER NOT NULL,
                    first_name TEXT,
                    surname TEXT,
                    profession TEXT,
                    street_address TEXT,
                    FOREIGN KEY (person_id) REFERENCES sheffield_people(unique_id)
                )",
                table_name
            ),
            [],
        )?;

        // Build WHERE clause
        let conditions: Vec<String> = terms
            .iter()
            .map(|t| format!("LOWER(profession) LIKE '%{}%'", t))
            .collect();
        let where_clause = conditions.join(" OR ");

        // Use INSERT...SELECT for fast bulk insert
        let insert_sql = format!(
            "INSERT INTO {} (person_id, first_name, surname, profession, street_address)
             SELECT unique_id, first_name, surname, profession, street_address
             FROM sheffield_people
             WHERE {}",
            table_name, where_clause
        );

        conn.execute(&insert_sql, [])?;

        let count: i64 = conn.query_row(
            &format!("SELECT COUNT(*) FROM {}", table_name),
            [],
            |r| r.get(0),
        )?;
        eprintln!("{}: {} rows", table_name, count);
    }

    // Print summary
    eprintln!("\n=== SUMMARY ===\n");
    for (table_name, _) in &categories {
        let count: i64 = conn.query_row(
            &format!("SELECT COUNT(*) FROM {}", table_name),
            [],
            |r| r.get(0),
        )?;
        if count > 0 {
            eprintln!("{:30} {:>6} rows", table_name, count);
        }
    }

    Ok(())
}
