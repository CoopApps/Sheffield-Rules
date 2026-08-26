use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("MATCHING CHILDREN TO PARENTS");
    println!("========================================\n");

    // Count children and servants
    let children: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy
         WHERE relation IN ('Son', 'Daughter')"
    )
    .fetch_one(&pool)
    .await?;

    let servants: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy
         WHERE relation = 'Servant'"
    )
    .fetch_one(&pool)
    .await?;

    println!("Total children (Son/Daughter): {}", children.0);
    println!("Total servants: {}", servants.0);
    println!("Note: Including all household members\n");

    // Find children with heads at same address
    let matchable: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT c.id)
         FROM unmatched_genealogy c
         WHERE c.relation IN ('Son', 'Daughter')
         AND EXISTS (
             SELECT 1 FROM unmatched_genealogy p
             WHERE p.relation = 'Head'
             AND p.address = c.address
         )"
    )
    .fetch_one(&pool)
    .await?;

    println!("Children with Head at same address: {}", matchable.0);

    // Show example household
    println!("\nExample household:\n");

    let example: Vec<(String, String, Option<i64>)> = sqlx::query_as(
        "SELECT name, relation, age
         FROM unmatched_genealogy
         WHERE address = (
             SELECT address FROM unmatched_genealogy
             WHERE relation IN ('Son', 'Daughter')
             AND EXISTS (
                 SELECT 1 FROM unmatched_genealogy p
                 WHERE p.relation = 'Head'
                 AND p.address = unmatched_genealogy.address
             )
             LIMIT 1
         )
         ORDER BY
             CASE relation
                 WHEN 'Head' THEN 1
                 WHEN 'Wife' THEN 2
                 WHEN 'Son' THEN 3
                 WHEN 'Daughter' THEN 4
                 ELSE 5
             END,
             age DESC"
    )
    .fetch_all(&pool)
    .await?;

    for (name, relation, age) in &example {
        println!("  {} - {} (age: {})",
            name,
            relation,
            age.map(|a| a.to_string()).unwrap_or("?".to_string())
        );
    }

    println!("\n========================================");
    println!("Creating child_parent_matches table...");
    println!("========================================\n");

    // Create table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS child_parent_matches (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            child_id TEXT NOT NULL,
            parent_id TEXT NOT NULL,
            child_name TEXT NOT NULL,
            parent_name TEXT NOT NULL,
            address TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    // Insert matches
    let inserted = sqlx::query(
        "INSERT INTO child_parent_matches (child_id, parent_id, child_name, parent_name, address)
         SELECT c.id, p.id, c.name, p.name, c.address
         FROM unmatched_genealogy c
         JOIN unmatched_genealogy p ON p.address = c.address
         WHERE c.relation IN ('Son', 'Daughter')
         AND p.relation = 'Head'"
    )
    .execute(&pool)
    .await?;

    println!("Successfully created {} child-parent matches", inserted.rows_affected());

    println!("\n========================================");
    println!("SUMMARY");
    println!("========================================");
    println!("Matched {} children to their parents", inserted.rows_affected());
    println!("Servants ({}) also present in households", servants.0);
    println!("Data saved to: child_parent_matches table");
    println!("========================================\n");

    Ok(())
}
