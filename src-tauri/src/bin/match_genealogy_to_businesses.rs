use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("MATCHING GENEALOGY TO BUSINESSES");
    println!("========================================\n");

    // First, let's find exact name and address matches
    println!("Finding exact name + address matches...");

    let exact_matches: Vec<(String, String, String, String, String, String)> = sqlx::query_as(
        "SELECT
            g.id as gen_id,
            g.name as gen_name,
            g.address as gen_address,
            b.id as bus_id,
            b.full_name as bus_name,
            b.address as bus_address
         FROM unmatched_genealogy g
         INNER JOIN sheffield_businesses b
         ON LOWER(TRIM(g.name)) = LOWER(TRIM(b.full_name))
         AND LOWER(TRIM(g.address)) = LOWER(TRIM(b.address))
         LIMIT 20"
    )
    .fetch_all(&pool)
    .await?;

    println!("Found {} exact name+address matches (showing first 20):\n", exact_matches.len());
    for (gen_id, gen_name, gen_addr, bus_id, bus_name, bus_addr) in &exact_matches {
        println!("Genealogy: {} - {}", gen_name, gen_addr);
        println!("Business:  {} - {}", bus_name, bus_addr);
        println!();
    }

    // Now find exact name matches (where address might differ slightly)
    println!("\n========================================");
    println!("Finding exact name matches (any address)...");

    let name_matches_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*)
         FROM unmatched_genealogy g
         INNER JOIN sheffield_businesses b
         ON LOWER(TRIM(g.name)) = LOWER(TRIM(b.full_name))"
    )
    .fetch_one(&pool)
    .await?;

    println!("Found {} exact name matches total", name_matches_count.0);

    // Show some examples where names match but addresses differ
    let name_diff_addr: Vec<(String, String, String, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT
            g.name,
            g.address as gen_address,
            b.address as bus_address,
            g.profession,
            b.occupation
         FROM unmatched_genealogy g
         INNER JOIN sheffield_businesses b
         ON LOWER(TRIM(g.name)) = LOWER(TRIM(b.full_name))
         WHERE LOWER(TRIM(g.address)) != LOWER(TRIM(b.address))
         LIMIT 20"
    )
    .fetch_all(&pool)
    .await?;

    println!("\nExamples of name matches with different addresses:");
    for (name, gen_addr, bus_addr, profession, occupation) in &name_diff_addr {
        println!("\nName: {}", name);
        println!("  Genealogy address: {}", gen_addr);
        println!("  Business address:  {}", bus_addr);
        println!("  Genealogy profession: {}", profession.as_deref().unwrap_or("N/A"));
        println!("  Business occupation:  {}", occupation.as_deref().unwrap_or("N/A"));
    }

    // Find surname + address matches (where forename might differ)
    println!("\n========================================");
    println!("Finding surname + address matches...");

    let surname_addr_matches: (i64,) = sqlx::query_as(
        "SELECT COUNT(*)
         FROM unmatched_genealogy g
         INNER JOIN sheffield_businesses b
         ON LOWER(TRIM(g.address)) = LOWER(TRIM(b.address))
         WHERE g.name LIKE '% ' || b.surname || '%'
         OR b.surname LIKE '% ' || SUBSTR(g.name, INSTR(g.name, ' ') + 1) || '%'"
    )
    .fetch_one(&pool)
    .await?;

    println!("Found {} potential surname+address matches", surname_addr_matches.0);

    Ok(())
}
