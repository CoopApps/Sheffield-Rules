use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;

    // Common female names to check for
    let female_names = [
        "Mary", "Ann", "Elizabeth", "Sarah", "Jane", "Emma", "Hannah", "Ellen",
        "Martha", "Margaret", "Alice", "Emily", "Harriet", "Charlotte", "Eliza",
        "Frances", "Maria", "Catherine", "Rebecca", "Susanna", "Fanny", "Caroline",
        "Lucy", "Lydia", "Louisa", "Sophia", "Rachel", "Amelia", "Clara", "Rosa",
        "Annie", "Jessie", "Agnes", "Florence", "Edith", "Ada", "Beatrice",
    ];

    eprintln!("=== CHECKING FOR FEMALE NAMES IN FOOTBALLERS ===\n");

    let mut total_female = 0;
    for name in &female_names {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM sheffield_footballers WHERE first_name = ?1",
            [name],
            |r| r.get(0),
        )?;
        if count > 0 {
            eprintln!("{}: {}", name, count);
            total_female += count;
        }
    }

    eprintln!("\nTotal with female names: {}", total_female);

    // Show some examples
    eprintln!("\n=== SAMPLE WITH FEMALE NAMES ===\n");
    let mut stmt = conn.prepare(
        "SELECT person_id, first_name, surname, birth_year, profession
         FROM sheffield_footballers
         WHERE first_name IN ('Mary', 'Ann', 'Elizabeth', 'Sarah', 'Jane', 'Emma', 'Hannah', 'Ellen', 'Martha', 'Margaret')
         LIMIT 20"
    )?;
    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        let uid: i64 = row.get(0)?;
        let first: String = row.get::<_, Option<String>>(1)?.unwrap_or_default();
        let surname: String = row.get::<_, Option<String>>(2)?.unwrap_or_default();
        let year: i64 = row.get(3)?;
        let prof: String = row.get::<_, Option<String>>(4)?.unwrap_or_default();
        eprintln!("[{}] {} {} ({}) - {}", uid, first, surname, 1867 - year, prof);
    }

    Ok(())
}
