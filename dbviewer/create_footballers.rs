use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;

    eprintln!("=== CREATING SHEFFIELD_FOOTBALLERS TABLE ===\n");

    // Drop and create table
    conn.execute("DROP TABLE IF EXISTS sheffield_footballers", [])?;
    conn.execute(
        "CREATE TABLE sheffield_footballers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            person_id INTEGER NOT NULL,
            first_name TEXT,
            surname TEXT,
            birth_year INTEGER,
            profession TEXT,
            street_address TEXT,
            FOREIGN KEY (person_id) REFERENCES sheffield_people(unique_id)
        )",
        [],
    )?;

    // Exclusion terms - people who wouldn't play football
    let exclude_professions = [
        // Employers/manufacturers
        "employ", "manufacturer", "proprietor", "master", "owner",
        // Clergy
        "vicar", "curate", "minister", "rector", "reverend", "clergyman",
        "clergy", "pastor", "chaplain", "preacher", "priest", "bishop",
        "incumbent", "missionary",
        // Professionals who wouldn't play
        "surgeon", "physician", "doctor", "solicitor", "barrister",
        "magistrate", "alderman", "mayor",
        // Disabled/infirm
        "blind", "deaf", "lame", "cripple", "invalid", "idiot", "lunatic",
        "pauper", "inmate",
        // Other unsuitable
        "prisoner", "convict",
        // Female professions
        "wife", "daughter", "widow", "seamstress", "dress maker", "dressmaker",
        "milliner", "laundress", "char woman", "charwoman", "housewife",
        "domestic servant", "servant girl", "nurse", "midwife", "governess",
        "needlewoman", "bonnet maker",
    ];

    // Female names to exclude
    let female_names = [
        "Mary", "Ann", "Elizabeth", "Sarah", "Jane", "Emma", "Hannah", "Ellen",
        "Martha", "Margaret", "Alice", "Emily", "Harriet", "Charlotte", "Eliza",
        "Frances", "Maria", "Catherine", "Rebecca", "Susanna", "Fanny", "Caroline",
        "Lucy", "Lydia", "Louisa", "Sophia", "Rachel", "Amelia", "Clara", "Rosa",
        "Annie", "Jessie", "Agnes", "Florence", "Edith", "Ada", "Beatrice",
        "Esther", "Nancy", "Phoebe", "Ruth", "Betsy", "Matilda", "Isabella",
        "Selina", "Rosanna", "Lavinia", "Priscilla", "Dinah", "Leah", "Naomi",
        "Miriam", "Deborah", "Jemima", "Kezia", "Rhoda", "Dorcas", "Patience",
        "Prudence", "Temperance", "Mercy", "Grace", "Faith", "Hope", "Charity",
        "Virtue", "Comfort", "Lettice", "Mildred", "Winifred", "Gertrude",
        "Dorothy", "Helen", "Julia", "Laura", "Susan", "Ellen", "Kate",
        "Bridget", "Norah", "Teresa", "Johanna", "Honora",
    ];

    // Build exclusion clause
    let exclude_conditions: Vec<String> = exclude_professions
        .iter()
        .map(|t| format!("LOWER(profession) LIKE '%{}%'", t))
        .collect();
    let exclude_clause = exclude_conditions.join(" OR ");

    // Build female name exclusion clause
    let name_conditions: Vec<String> = female_names
        .iter()
        .map(|n| format!("first_name = '{}'", n))
        .collect();
    let name_exclude_clause = name_conditions.join(" OR ");

    // Select men born 1827-1853 (aged 14-40 in 1867)
    let insert_sql = format!(
        "INSERT INTO sheffield_footballers (person_id, first_name, surname, birth_year, profession, street_address)
         SELECT unique_id, first_name, surname, birth_year, profession, street_address
         FROM sheffield_people
         WHERE birth_year >= 1827 AND birth_year <= 1853
         AND NOT ({})
         AND NOT ({})
         AND unique_id NOT IN (SELECT person_id FROM sheffield_clergy WHERE person_id IS NOT NULL)
         AND unique_id NOT IN (SELECT person_id FROM sheffield_businessmen WHERE person_id IS NOT NULL)",
        exclude_clause, name_exclude_clause
    );

    conn.execute(&insert_sql, [])?;

    let total: i64 = conn.query_row("SELECT COUNT(*) FROM sheffield_footballers", [], |r| r.get(0))?;
    eprintln!("Total potential footballers: {}", total);

    // Show birth year breakdown
    eprintln!("\n=== BIRTH YEAR BREAKDOWN ===\n");
    let mut stmt = conn.prepare(
        "SELECT birth_year, COUNT(*) as cnt FROM sheffield_footballers
         GROUP BY birth_year ORDER BY birth_year"
    )?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let year: i64 = row.get(0)?;
        let cnt: i64 = row.get(1)?;
        eprintln!("{} (age {}): {}", year, 1867 - year, cnt);
    }

    // Show profession breakdown (top 20)
    eprintln!("\n=== TOP PROFESSIONS ===\n");
    let mut stmt = conn.prepare(
        "SELECT profession, COUNT(*) as cnt FROM sheffield_footballers
         WHERE profession IS NOT NULL AND profession != ''
         GROUP BY profession ORDER BY cnt DESC LIMIT 20"
    )?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let prof: String = row.get(0)?;
        let cnt: i64 = row.get(1)?;
        eprintln!("{}: {}", prof, cnt);
    }

    // Show sample
    eprintln!("\n=== SAMPLE FOOTBALLERS ===\n");
    let mut stmt = conn.prepare(
        "SELECT person_id, first_name, surname, birth_year, profession, street_address
         FROM sheffield_footballers
         ORDER BY RANDOM() LIMIT 20"
    )?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let uid: i64 = row.get(0)?;
        let first: String = row.get::<_, Option<String>>(1)?.unwrap_or_default();
        let surname: String = row.get::<_, Option<String>>(2)?.unwrap_or_default();
        let year: i64 = row.get(3)?;
        let prof: String = row.get::<_, Option<String>>(4)?.unwrap_or_default();
        let addr: String = row.get::<_, Option<String>>(5)?.unwrap_or_default();
        eprintln!("[{}] {} {}, born {} (age {}) - {} @ {}", uid, first, surname, year, 1867 - year, prof, addr);
    }

    Ok(())
}
