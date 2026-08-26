use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("Creating institutional tables...\n");

    // Create workhouse table
    println!("Creating workhouse_residents table...");
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS workhouse_residents (
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
    println!("✓ workhouse_residents table created");

    // Create asylum table
    println!("Creating asylum_residents table...");
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS asylum_residents (
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
    println!("✓ asylum_residents table created");

    // Create hospital table
    println!("Creating hospital_patients table...");
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS hospital_patients (
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
    println!("✓ hospital_patients table created");

    // Create prison table
    println!("Creating prison_inmates table...");
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS prison_inmates (
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
    println!("✓ prison_inmates table created");

    // Create orphanage table
    println!("Creating orphanage_residents table...");
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS orphanage_residents (
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
    println!("✓ orphanage_residents table created");

    // Create poorhouse table
    println!("Creating poorhouse_residents table...");
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS poorhouse_residents (
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
    println!("✓ poorhouse_residents table created");

    // Create general institution table
    println!("Creating institution_residents table...");
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS institution_residents (
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
    println!("✓ institution_residents table created");

    println!("\n========================================");
    println!("All institutional tables created successfully!");
    println!("========================================");

    Ok(())
}
