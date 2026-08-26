use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("MATCHING UNIQUE GENEALOGY NAMES");
    println!("TO UNASSIGNED CENSUS HOUSEHOLDS");
    println!("========================================\n");

    // Find unique names in unmatched_genealogy (names appearing only once)
    println!("Finding unique names in unmatched_genealogy...\n");

    let unique_genealogy_names: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM (
            SELECT name
            FROM unmatched_genealogy
            GROUP BY name
            HAVING COUNT(*) = 1
        )"
    )
    .fetch_one(&pool)
    .await?;

    println!("Unique names in unmatched_genealogy: {}", unique_genealogy_names.0);

    // Find how many of these unique names appear in unmatched_sheffieldcensus
    let matches: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT uc.name)
         FROM unmatched_sheffieldcensus uc
         WHERE uc.name IN (
             SELECT name
             FROM unmatched_genealogy
             GROUP BY name
             HAVING COUNT(*) = 1
         )"
    )
    .fetch_one(&pool)
    .await?;

    println!("Unique genealogy names found in unmatched_sheffieldcensus: {}\n", matches.0);

    // Count total potential match records
    let match_records: (i64,) = sqlx::query_as(
        "SELECT COUNT(*)
         FROM unmatched_sheffieldcensus uc
         WHERE uc.name IN (
             SELECT name
             FROM unmatched_genealogy
             GROUP BY name
             HAVING COUNT(*) = 1
         )"
    )
    .fetch_one(&pool)
    .await?;

    println!("Total census records with these unique names: {}\n", match_records.0);

    // Show examples of matches with household context
    println!("--- Example Matches with Household Context ---\n");

    let examples: Vec<(String, String, Option<i64>, Option<String>)> = sqlx::query_as(
        "SELECT uc.name, uc.relation, uc.age, uc.civil_parish
         FROM unmatched_sheffieldcensus uc
         WHERE uc.name IN (
             SELECT name
             FROM unmatched_genealogy
             GROUP BY name
             HAVING COUNT(*) = 1
         )
         ORDER BY uc.civil_parish
         LIMIT 20"
    )
    .fetch_all(&pool)
    .await?;

    for (name, relation, age, parish) in &examples {
        println!("Name: {}", name);
        println!("  Census Relation: {}", relation);
        println!("  Census Age: {}", age.map(|a| a.to_string()).unwrap_or("?".to_string()));
        println!("  Census Parish: {}", parish.as_ref().unwrap_or(&"?".to_string()));

        // Get the genealogy record for this name
        let genealogy: Option<(String, String, Option<i64>)> = sqlx::query_as(
            "SELECT relation, address, age
             FROM unmatched_genealogy
             WHERE name = ?"
        )
        .bind(name)
        .fetch_optional(&pool)
        .await?;

        if let Some((g_relation, g_address, g_age)) = genealogy {
            println!("  Genealogy Relation: {}", g_relation);
            println!("  Genealogy Address: {}", g_address);
            println!("  Genealogy Age: {}", g_age.map(|a| a.to_string()).unwrap_or("?".to_string()));
        }
        println!();
    }

    // Breakdown by relation match
    println!("--- Relation Breakdown of Matches ---\n");

    let by_relation: Vec<(String, String, i64)> = sqlx::query_as(
        "SELECT uc.relation as census_relation,
                ug.relation as genealogy_relation,
                COUNT(*) as count
         FROM unmatched_sheffieldcensus uc
         JOIN unmatched_genealogy ug ON uc.name = ug.name
         WHERE ug.name IN (
             SELECT name
             FROM unmatched_genealogy
             GROUP BY name
             HAVING COUNT(*) = 1
         )
         GROUP BY uc.relation, ug.relation
         ORDER BY count DESC
         LIMIT 20"
    )
    .fetch_all(&pool)
    .await?;

    println!("Census Relation -> Genealogy Relation: Count");
    for (census_rel, gen_rel, count) in by_relation {
        println!("{} -> {}: {}", census_rel, gen_rel, count);
    }

    Ok(())
}
