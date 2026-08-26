use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    println!("=== POPULATING CATEGORIZATION TABLES ===\n");

    let dest_conn = Connection::open("Sheffield1867_rescued.db")?;
    dest_conn.busy_timeout(std::time::Duration::from_secs(30))?;

    // Attach source database
    dest_conn.execute("ATTACH DATABASE 'Sheffield1867.db' AS source", [])?;

    // Drop existing tables
    println!("1. Cleaning up existing tables...");
    dest_conn.execute("DROP TABLE IF EXISTS sheffield_employers", [])?;
    dest_conn.execute("DROP TABLE IF EXISTS sheffield_professionals", [])?;
    dest_conn.execute("DROP TABLE IF EXISTS sheffield_tradesmen", [])?;
    dest_conn.execute("DROP TABLE IF EXISTS sheffield_publicans", [])?;
    dest_conn.execute("DROP TABLE IF EXISTS sheffield_lodgers", [])?;
    dest_conn.execute("DROP TABLE IF EXISTS sheffield_german", [])?;
    dest_conn.execute("DROP TABLE IF EXISTS sheffield_irish", [])?;
    dest_conn.execute("DROP TABLE IF EXISTS sheffield_scottish", [])?;
    dest_conn.execute("DROP TABLE IF EXISTS sheffield_welsh", [])?;
    println!("   ✓ Cleaned\n");

    // Create and populate each table using JOIN
    println!("2. Populating categorization tables...\n");

    // Employers
    let count = dest_conn.execute(
        "CREATE TABLE sheffield_employers AS
         SELECT p.* FROM main.sheffield_people p
         INNER JOIN source.sheffield_employers s ON p.unique_id = s.sheffield_person_id",
        [],
    )?;
    let employers: i64 = dest_conn.query_row("SELECT COUNT(*) FROM sheffield_employers", [], |r| r.get(0))?;
    println!("   Employers: {}", employers);

    // Professionals
    dest_conn.execute(
        "CREATE TABLE sheffield_professionals AS
         SELECT p.* FROM main.sheffield_people p
         INNER JOIN source.sheffield_professionals s ON p.unique_id = s.sheffield_person_id",
        [],
    )?;
    let professionals: i64 = dest_conn.query_row("SELECT COUNT(*) FROM sheffield_professionals", [], |r| r.get(0))?;
    println!("   Professionals: {}", professionals);

    // Tradesmen
    dest_conn.execute(
        "CREATE TABLE sheffield_tradesmen AS
         SELECT p.* FROM main.sheffield_people p
         INNER JOIN source.sheffield_tradesmen s ON p.unique_id = s.sheffield_person_id",
        [],
    )?;
    let tradesmen: i64 = dest_conn.query_row("SELECT COUNT(*) FROM sheffield_tradesmen", [], |r| r.get(0))?;
    println!("   Tradesmen: {}", tradesmen);

    // Publicans
    dest_conn.execute(
        "CREATE TABLE sheffield_publicans AS
         SELECT p.* FROM main.sheffield_people p
         INNER JOIN source.sheffield_publicans s ON p.unique_id = s.sheffield_person_id",
        [],
    )?;
    let publicans: i64 = dest_conn.query_row("SELECT COUNT(*) FROM sheffield_publicans", [], |r| r.get(0))?;
    println!("   Publicans: {}", publicans);

    // Lodgers
    dest_conn.execute(
        "CREATE TABLE sheffield_lodgers AS
         SELECT p.* FROM main.sheffield_people p
         INNER JOIN source.sheffield_lodgers s ON p.unique_id = s.sheffield_person_id",
        [],
    )?;
    let lodgers: i64 = dest_conn.query_row("SELECT COUNT(*) FROM sheffield_lodgers", [], |r| r.get(0))?;
    println!("   Lodgers: {}", lodgers);

    // German
    dest_conn.execute(
        "CREATE TABLE sheffield_german AS
         SELECT p.* FROM main.sheffield_people p
         INNER JOIN source.sheffield_german s ON p.unique_id = s.sheffield_person_id",
        [],
    )?;
    let german: i64 = dest_conn.query_row("SELECT COUNT(*) FROM sheffield_german", [], |r| r.get(0))?;
    println!("   German: {}", german);

    // Irish
    dest_conn.execute(
        "CREATE TABLE sheffield_irish AS
         SELECT p.* FROM main.sheffield_people p
         INNER JOIN source.sheffield_irish s ON p.unique_id = s.sheffield_person_id",
        [],
    )?;
    let irish: i64 = dest_conn.query_row("SELECT COUNT(*) FROM sheffield_irish", [], |r| r.get(0))?;
    println!("   Irish: {}", irish);

    // Scottish
    dest_conn.execute(
        "CREATE TABLE sheffield_scottish AS
         SELECT p.* FROM main.sheffield_people p
         INNER JOIN source.sheffield_scottish s ON p.unique_id = s.sheffield_person_id",
        [],
    )?;
    let scottish: i64 = dest_conn.query_row("SELECT COUNT(*) FROM sheffield_scottish", [], |r| r.get(0))?;
    println!("   Scottish: {}", scottish);

    // Welsh
    dest_conn.execute(
        "CREATE TABLE sheffield_welsh AS
         SELECT p.* FROM main.sheffield_people p
         INNER JOIN source.sheffield_welsh s ON p.unique_id = s.sheffield_person_id",
        [],
    )?;
    let welsh: i64 = dest_conn.query_row("SELECT COUNT(*) FROM sheffield_welsh", [], |r| r.get(0))?;
    println!("   Welsh: {}", welsh);

    // Copy sheffield_clubs from Sheffield1867.db (source is still attached)
    println!("\n3. Copying sheffield_clubs...");
    dest_conn.execute("DROP TABLE IF EXISTS sheffield_clubs", [])?;
    dest_conn.execute("CREATE TABLE sheffield_clubs AS SELECT * FROM source.sheffield_clubs", [])?;
    let clubs: i64 = dest_conn.query_row("SELECT COUNT(*) FROM sheffield_clubs", [], |r| r.get(0))?;
    println!("   Clubs: {}", clubs);

    dest_conn.execute("DETACH DATABASE source", [])?;

    println!("\n=== COMPLETE ===");
    let total = employers + professionals + tradesmen + publicans + lodgers + german + irish + scottish + welsh;
    println!("Total categorized: {}", total);

    Ok(())
}
