use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("CREATING MISSING INSTITUTIONAL TABLES");
    println!("========================================\n");

    // Create asylum table
    println!("Creating sheffield_asylum table...");
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS sheffield_asylum (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            birth_year INTEGER,
            address TEXT,
            profession TEXT,
            source TEXT DEFAULT 'genealogy'
        )"
    )
    .execute(&pool)
    .await?;
    println!("✓ sheffield_asylum table created");

    // Create hospital table
    println!("Creating sheffield_hospital table...");
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS sheffield_hospital (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            birth_year INTEGER,
            address TEXT,
            profession TEXT,
            source TEXT DEFAULT 'genealogy'
        )"
    )
    .execute(&pool)
    .await?;
    println!("✓ sheffield_hospital table created");

    // Create prison table
    println!("Creating sheffield_prison table...");
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS sheffield_prison (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            birth_year INTEGER,
            address TEXT,
            profession TEXT,
            source TEXT DEFAULT 'genealogy'
        )"
    )
    .execute(&pool)
    .await?;
    println!("✓ sheffield_prison table created");

    // Create orphanage table
    println!("Creating sheffield_orphanage table...");
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS sheffield_orphanage (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            birth_year INTEGER,
            address TEXT,
            profession TEXT,
            source TEXT DEFAULT 'genealogy'
        )"
    )
    .execute(&pool)
    .await?;
    println!("✓ sheffield_orphanage table created");

    // Create poorhouse table
    println!("Creating sheffield_poorhouse table...");
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS sheffield_poorhouse (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            birth_year INTEGER,
            address TEXT,
            profession TEXT,
            source TEXT DEFAULT 'genealogy'
        )"
    )
    .execute(&pool)
    .await?;
    println!("✓ sheffield_poorhouse table created");

    // Create general institution table
    println!("Creating sheffield_institution table...");
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS sheffield_institution (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            birth_year INTEGER,
            address TEXT,
            profession TEXT,
            source TEXT DEFAULT 'genealogy'
        )"
    )
    .execute(&pool)
    .await?;
    println!("✓ sheffield_institution table created");

    println!("\n========================================");
    println!("MOVING PEOPLE TO INSTITUTIONAL TABLES");
    println!("========================================\n");

    // Move workhouse people (Note: table already exists with different schema - it has street_address, not address)
    println!("Moving workhouse residents...");
    let workhouse_moved = sqlx::query(
        "INSERT OR IGNORE INTO sheffield_workhouse (id, name, birth_year, street_address, profession, workhouse_name)
         SELECT id, name, birth_year, address, profession, address
         FROM unmatched_genealogy
         WHERE address LIKE '%workhouse%' OR address LIKE '%work house%'"
    )
    .execute(&pool)
    .await?;
    println!("  Added {} new workhouse residents", workhouse_moved.rows_affected());

    // Move asylum people (Note: table already exists with different schema - it has street_address and asylum_name)
    println!("Moving asylum residents...");
    let asylum_moved = sqlx::query(
        "INSERT OR IGNORE INTO sheffield_asylum (id, name, birth_year, street_address, profession, asylum_name)
         SELECT id, name, birth_year, address, profession, address
         FROM unmatched_genealogy
         WHERE address LIKE '%asylum%'"
    )
    .execute(&pool)
    .await?;
    println!("  Added {} asylum residents", asylum_moved.rows_affected());

    // Move hospital people
    println!("Moving hospital patients...");
    let hospital_moved = sqlx::query(
        "INSERT OR IGNORE INTO sheffield_hospital (id, name, birth_year, address, profession, source)
         SELECT id, name, birth_year, address, profession, 'genealogy'
         FROM unmatched_genealogy
         WHERE address LIKE '%hospital%' OR address LIKE '%infirmary%'"
    )
    .execute(&pool)
    .await?;
    println!("  Moved {} hospital patients", hospital_moved.rows_affected());

    // Move prison people
    println!("Moving prison inmates...");
    let prison_moved = sqlx::query(
        "INSERT OR IGNORE INTO sheffield_prison (id, name, birth_year, address, profession, source)
         SELECT id, name, birth_year, address, profession, 'genealogy'
         FROM unmatched_genealogy
         WHERE address LIKE '%prison%' OR address LIKE '%jail%' OR address LIKE '%gaol%'"
    )
    .execute(&pool)
    .await?;
    println!("  Moved {} prison inmates", prison_moved.rows_affected());

    // Move orphanage people
    println!("Moving orphanage residents...");
    let orphanage_moved = sqlx::query(
        "INSERT OR IGNORE INTO sheffield_orphanage (id, name, birth_year, address, profession, source)
         SELECT id, name, birth_year, address, profession, 'genealogy'
         FROM unmatched_genealogy
         WHERE address LIKE '%orphanage%' OR address LIKE '%children%home%' OR address LIKE '%orphan%'"
    )
    .execute(&pool)
    .await?;
    println!("  Moved {} orphanage residents", orphanage_moved.rows_affected());

    // Move barracks people to sheffield_people (sheffield_people doesn't have address column)
    println!("Moving barracks residents to sheffield_people...");
    let barracks_moved = sqlx::query(
        "INSERT OR IGNORE INTO sheffield_people (id, name, birth_year, profession, gender, source)
         SELECT id, name, birth_year, profession, NULL, 'genealogy'
         FROM unmatched_genealogy
         WHERE address LIKE '%barracks%' OR address LIKE '%barrick%'"
    )
    .execute(&pool)
    .await?;
    println!("  Moved {} barracks residents to sheffield_people", barracks_moved.rows_affected());

    // Move poorhouse people
    println!("Moving poorhouse residents...");
    let poorhouse_moved = sqlx::query(
        "INSERT OR IGNORE INTO sheffield_poorhouse (id, name, birth_year, address, profession, source)
         SELECT id, name, birth_year, address, profession, 'genealogy'
         FROM unmatched_genealogy
         WHERE address LIKE '%poor%house%' OR address LIKE '%union%workhouse%' OR address LIKE '%poor law%'"
    )
    .execute(&pool)
    .await?;
    println!("  Moved {} poorhouse residents", poorhouse_moved.rows_affected());

    // Move general institution people
    println!("Moving general institution residents...");
    let institution_moved = sqlx::query(
        "INSERT OR IGNORE INTO sheffield_institution (id, name, birth_year, address, profession, source)
         SELECT id, name, birth_year, address, profession, 'genealogy'
         FROM unmatched_genealogy
         WHERE address LIKE '%institution%' OR address LIKE '%institute%'"
    )
    .execute(&pool)
    .await?;
    println!("  Moved {} general institution residents", institution_moved.rows_affected());

    println!("\n========================================");
    println!("DELETING MOVED RECORDS FROM UNMATCHED_GENEALOGY");
    println!("========================================\n");

    // Delete all institutional records from unmatched_genealogy
    let deleted = sqlx::query(
        "DELETE FROM unmatched_genealogy
         WHERE address LIKE '%workhouse%'
            OR address LIKE '%work house%'
            OR address LIKE '%asylum%'
            OR address LIKE '%hospital%'
            OR address LIKE '%infirmary%'
            OR address LIKE '%prison%'
            OR address LIKE '%jail%'
            OR address LIKE '%gaol%'
            OR address LIKE '%orphanage%'
            OR address LIKE '%children%home%'
            OR address LIKE '%orphan%'
            OR address LIKE '%barracks%'
            OR address LIKE '%barrick%'
            OR address LIKE '%poor%house%'
            OR address LIKE '%union%workhouse%'
            OR address LIKE '%poor law%'
            OR address LIKE '%institution%'
            OR address LIKE '%institute%'"
    )
    .execute(&pool)
    .await?;
    println!("Deleted {} records from unmatched_genealogy", deleted.rows_affected());

    println!("\n========================================");
    println!("FINAL COUNTS:");
    println!("========================================");

    let workhouse_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_workhouse").fetch_one(&pool).await?;
    let asylum_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_asylum").fetch_one(&pool).await?;
    let hospital_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_hospital").fetch_one(&pool).await?;
    let prison_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_prison").fetch_one(&pool).await?;
    let orphanage_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_orphanage").fetch_one(&pool).await?;
    let poorhouse_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_poorhouse").fetch_one(&pool).await?;
    let institution_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_institution").fetch_one(&pool).await?;
    let sheffield_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_people").fetch_one(&pool).await?;
    let unmatched_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM unmatched_genealogy").fetch_one(&pool).await?;

    println!("sheffield_workhouse: {}", workhouse_count.0);
    println!("sheffield_asylum: {}", asylum_count.0);
    println!("sheffield_hospital: {}", hospital_count.0);
    println!("sheffield_prison: {}", prison_count.0);
    println!("sheffield_orphanage: {}", orphanage_count.0);
    println!("sheffield_poorhouse: {}", poorhouse_count.0);
    println!("sheffield_institution: {}", institution_count.0);
    println!("sheffield_people: {}", sheffield_count.0);
    println!("unmatched_genealogy: {}", unmatched_count.0);
    println!("========================================");

    Ok(())
}
