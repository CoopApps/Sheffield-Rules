use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("Sheffield1867_rescued.db")?;

    println!("=== CHECKING WELSH PERSON ===\n");

    // Get a Welsh person
    println!("1. A person from sheffield_welsh:");
    let mut stmt = conn.prepare("SELECT unique_id, name, first_name, surname, census_age, profession FROM sheffield_welsh LIMIT 1")?;
    let mut rows = stmt.query([])?;

    if let Some(row) = rows.next()? {
        let unique_id: i64 = row.get(0)?;
        let name: Option<String> = row.get(1)?;
        let first_name: Option<String> = row.get(2)?;
        let surname: Option<String> = row.get(3)?;
        let age: Option<i64> = row.get(4)?;
        let profession: Option<String> = row.get(5)?;

        println!("   unique_id: {}", unique_id);
        println!("   name: {:?}", name);
        println!("   first_name: {:?}", first_name);
        println!("   surname: {:?}", surname);
        println!("   age: {:?}", age);
        println!("   profession: {:?}", profession);

        // Now check if this person exists in sheffield_people with same unique_id
        println!("\n2. Same person in sheffield_people (by unique_id {}):", unique_id);

        let mut stmt2 = conn.prepare("SELECT unique_id, name, first_name, surname, census_age, profession FROM sheffield_people WHERE unique_id = ?")?;
        let mut rows2 = stmt2.query([unique_id])?;

        if let Some(row2) = rows2.next()? {
            let uid: i64 = row2.get(0)?;
            let n: Option<String> = row2.get(1)?;
            let fn_: Option<String> = row2.get(2)?;
            let sn: Option<String> = row2.get(3)?;
            let a: Option<i64> = row2.get(4)?;
            let p: Option<String> = row2.get(5)?;

            println!("   ✓ FOUND!");
            println!("   unique_id: {}", uid);
            println!("   name: {:?}", n);
            println!("   first_name: {:?}", fn_);
            println!("   surname: {:?}", sn);
            println!("   age: {:?}", a);
            println!("   profession: {:?}", p);
        } else {
            println!("   ✗ NOT FOUND in sheffield_people!");
        }
    }

    Ok(())
}
