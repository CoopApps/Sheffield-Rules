use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867.db")?;

    eprintln!("=== TOP 50 PROFESSIONS IN SHEFFIELD_PEOPLE ===\n");

    let mut stmt = conn.prepare(
        "SELECT profession, COUNT(*) as cnt
         FROM sheffield_people
         WHERE profession IS NOT NULL AND profession != ''
         GROUP BY profession
         ORDER BY cnt DESC
         LIMIT 50"
    )?;

    let mut rows = stmt.query([])?;
    let mut rank = 1;
    while let Some(row) = rows.next()? {
        let prof: String = row.get(0)?;
        let cnt: i64 = row.get(1)?;
        println!("{:2}. {:50} {:>6}", rank, prof, cnt);
        rank += 1;
    }

    Ok(())
}
