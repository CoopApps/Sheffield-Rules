use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("ANALYZING HEADS BY GENDER");
    println!("========================================\n");

    // Count all heads
    let total_heads: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy WHERE relation = 'Head'"
    )
    .fetch_one(&pool)
    .await?;

    println!("Total Heads: {}", total_heads.0);

    // Try to identify male heads by name patterns
    // Common male prefixes and patterns
    let male_heads: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy
         WHERE relation = 'Head'
         AND (
             name LIKE '% Mr %'
             OR name LIKE 'Mr %'
             OR name LIKE '% Esq%'
             OR name LIKE '% Jr%'
             OR name LIKE '% Sr%'
         )"
    )
    .fetch_one(&pool)
    .await?;

    println!("Heads with clear male indicators (Mr, Esq, etc.): {}", male_heads.0);

    // Female heads
    let female_heads: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_genealogy
         WHERE relation = 'Head'
         AND (
             name LIKE '% Mrs %'
             OR name LIKE 'Mrs %'
             OR name LIKE '% Miss %'
             OR name LIKE 'Miss %'
             OR name LIKE '% Ms %'
             OR name LIKE 'Ms %'
         )"
    )
    .fetch_one(&pool)
    .await?;

    println!("Heads with clear female indicators (Mrs, Miss, Ms): {}", female_heads.0);

    let unclear: i64 = total_heads.0 - male_heads.0 - female_heads.0;
    println!("Heads without clear gender indicator: {}", unclear);

    // Show some examples of each type
    println!("\n--- Sample Male Heads ---");
    let male_samples: Vec<(String, String)> = sqlx::query_as(
        "SELECT name, address FROM unmatched_genealogy
         WHERE relation = 'Head'
         AND (
             name LIKE '% Mr %'
             OR name LIKE 'Mr %'
             OR name LIKE '% Esq%'
         )
         LIMIT 5"
    )
    .fetch_all(&pool)
    .await?;

    for (name, address) in male_samples {
        println!("  {} at {}", name, address);
    }

    println!("\n--- Sample Female Heads ---");
    let female_samples: Vec<(String, String)> = sqlx::query_as(
        "SELECT name, address FROM unmatched_genealogy
         WHERE relation = 'Head'
         AND (
             name LIKE '% Mrs %'
             OR name LIKE 'Mrs %'
             OR name LIKE '% Miss %'
             OR name LIKE 'Miss %'
         )
         LIMIT 5"
    )
    .fetch_all(&pool)
    .await?;

    for (name, address) in female_samples {
        println!("  {} at {}", name, address);
    }

    println!("\n--- Sample Unclear Heads ---");
    let unclear_samples: Vec<(String, String)> = sqlx::query_as(
        "SELECT name, address FROM unmatched_genealogy
         WHERE relation = 'Head'
         AND NOT (
             name LIKE '% Mr %'
             OR name LIKE 'Mr %'
             OR name LIKE '% Esq%'
             OR name LIKE '% Jr%'
             OR name LIKE '% Sr%'
             OR name LIKE '% Mrs %'
             OR name LIKE 'Mrs %'
             OR name LIKE '% Miss %'
             OR name LIKE 'Miss %'
             OR name LIKE '% Ms %'
             OR name LIKE 'Ms %'
         )
         LIMIT 5"
    )
    .fetch_all(&pool)
    .await?;

    for (name, address) in unclear_samples {
        println!("  {} at {}", name, address);
    }

    // Count children that would match to male heads only
    println!("\n========================================");
    println!("CHILDREN MATCHING TO MALE HEADS ONLY");
    println!("========================================\n");

    let male_head_children: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT c.id)
         FROM unmatched_genealogy c
         WHERE c.relation IN ('Son', 'Daughter')
         AND EXISTS (
             SELECT 1 FROM unmatched_genealogy p
             WHERE p.relation = 'Head'
             AND p.address = c.address
             AND (
                 p.name LIKE '% Mr %'
                 OR p.name LIKE 'Mr %'
                 OR p.name LIKE '% Esq%'
                 OR p.name LIKE '% Jr%'
                 OR p.name LIKE '% Sr%'
             )
         )"
    )
    .fetch_one(&pool)
    .await?;

    println!("Children with male Head at same address: {}", male_head_children.0);

    Ok(())
}
