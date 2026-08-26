use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("FINDING DUPLICATE RECORDS");
    println!("========================================\n");

    // Find duplicates by matching name + piece + folio + age
    println!("Searching for duplicates based on: name + piece + folio + age\n");

    let duplicates: Vec<(String, String, String, Option<i64>, i64)> = sqlx::query_as(
        "SELECT
            LOWER(TRIM(name)) as name,
            TRIM(COALESCE(piece, '')) as piece,
            TRIM(COALESCE(folio, '')) as folio,
            age,
            COUNT(*) as count
         FROM unmatched_sheffieldcensus
         WHERE name IS NOT NULL AND TRIM(name) != ''
         GROUP BY LOWER(TRIM(name)), TRIM(COALESCE(piece, '')), TRIM(COALESCE(folio, '')), age
         HAVING COUNT(*) > 1
         ORDER BY count DESC"
    )
    .fetch_all(&pool)
    .await?;

    println!("Found {} unique name+piece+folio+age combinations with duplicates\n", duplicates.len());

    if duplicates.is_empty() {
        println!("✓ No duplicates found!");
        return Ok(());
    }

    // Calculate total duplicate records
    let total_duplicate_records: i64 = duplicates.iter().map(|(_, _, _, _, count)| count - 1).sum();
    let total_records_involved: i64 = duplicates.iter().map(|(_, _, _, _, count)| *count).sum();

    println!("Summary:");
    println!("  - Total records involved in duplicates: {}", total_records_involved);
    println!("  - Total excess duplicate records: {}", total_duplicate_records);
    println!();

    // Show top 20 duplicates
    println!("Top 20 duplicate groups (by count):\n");
    for (i, (name, piece, folio, age, count)) in duplicates.iter().take(20).enumerate() {
        let age_str = age.map(|a| a.to_string()).unwrap_or_else(|| "NULL".to_string());
        println!("{}. \"{}\" | Piece: \"{}\" | Folio: \"{}\" | Age: {} | Count: {}",
            i + 1, name, piece, folio, age_str, count);
    }

    println!("\n========================================");
    println!("Analyzing duplicate sources...");
    println!("========================================\n");

    // Analyze where duplicates come from
    for (i, (name, piece, folio, age, _count)) in duplicates.iter().take(5).enumerate() {
        let sources: Vec<(String, i64)> = sqlx::query_as(
            "SELECT source, COUNT(*) as count
             FROM unmatched_sheffieldcensus
             WHERE LOWER(TRIM(name)) = LOWER(TRIM(?))
             AND TRIM(COALESCE(piece, '')) = TRIM(?)
             AND TRIM(COALESCE(folio, '')) = TRIM(?)
             AND (age = ? OR (age IS NULL AND ? IS NULL))
             GROUP BY source"
        )
        .bind(name)
        .bind(piece)
        .bind(folio)
        .bind(age)
        .bind(age)
        .fetch_all(&pool)
        .await?;

        let age_str = age.map(|a| a.to_string()).unwrap_or_else(|| "NULL".to_string());
        println!("Example {}. \"{}\" | Piece: \"{}\" | Folio: \"{}\" | Age: {}", i + 1, name, piece, folio, age_str);
        for (source, count) in sources {
            println!("  - {}: {} records", source, count);
        }
        println!();
    }

    // Count by source combination (with age included)
    let sheffield_only: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT LOWER(TRIM(name)) || '|' || TRIM(COALESCE(piece, '')) || '|' || TRIM(COALESCE(folio, '')) || '|' || COALESCE(CAST(age AS TEXT), 'NULL'))
         FROM unmatched_sheffieldcensus a
         WHERE EXISTS (
             SELECT 1 FROM unmatched_sheffieldcensus b
             WHERE LOWER(TRIM(a.name)) = LOWER(TRIM(b.name))
             AND TRIM(COALESCE(a.piece, '')) = TRIM(COALESCE(b.piece, ''))
             AND TRIM(COALESCE(a.folio, '')) = TRIM(COALESCE(b.folio, ''))
             AND (a.age = b.age OR (a.age IS NULL AND b.age IS NULL))
             AND a.id != b.id
         )
         AND source = 'sheffield'
         AND NOT EXISTS (
             SELECT 1 FROM unmatched_sheffieldcensus c
             WHERE LOWER(TRIM(a.name)) = LOWER(TRIM(c.name))
             AND TRIM(COALESCE(a.piece, '')) = TRIM(COALESCE(c.piece, ''))
             AND TRIM(COALESCE(a.folio, '')) = TRIM(COALESCE(c.folio, ''))
             AND (a.age = c.age OR (a.age IS NULL AND c.age IS NULL))
             AND c.source = 'geni'
         )"
    )
    .fetch_one(&pool)
    .await?;

    let geni_only: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT LOWER(TRIM(name)) || '|' || TRIM(COALESCE(piece, '')) || '|' || TRIM(COALESCE(folio, '')) || '|' || COALESCE(CAST(age AS TEXT), 'NULL'))
         FROM unmatched_sheffieldcensus a
         WHERE EXISTS (
             SELECT 1 FROM unmatched_sheffieldcensus b
             WHERE LOWER(TRIM(a.name)) = LOWER(TRIM(b.name))
             AND TRIM(COALESCE(a.piece, '')) = TRIM(COALESCE(b.piece, ''))
             AND TRIM(COALESCE(a.folio, '')) = TRIM(COALESCE(b.folio, ''))
             AND (a.age = b.age OR (a.age IS NULL AND b.age IS NULL))
             AND a.id != b.id
         )
         AND source = 'geni'
         AND NOT EXISTS (
             SELECT 1 FROM unmatched_sheffieldcensus c
             WHERE LOWER(TRIM(a.name)) = LOWER(TRIM(c.name))
             AND TRIM(COALESCE(a.piece, '')) = TRIM(COALESCE(c.piece, ''))
             AND TRIM(COALESCE(a.folio, '')) = TRIM(COALESCE(c.folio, ''))
             AND (a.age = c.age OR (a.age IS NULL AND c.age IS NULL))
             AND c.source = 'sheffield'
         )"
    )
    .fetch_one(&pool)
    .await?;

    let cross_source: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT LOWER(TRIM(name)) || '|' || TRIM(COALESCE(piece, '')) || '|' || TRIM(COALESCE(folio, '')) || '|' || COALESCE(CAST(age AS TEXT), 'NULL'))
         FROM unmatched_sheffieldcensus a
         WHERE source = 'sheffield'
         AND EXISTS (
             SELECT 1 FROM unmatched_sheffieldcensus b
             WHERE LOWER(TRIM(a.name)) = LOWER(TRIM(b.name))
             AND TRIM(COALESCE(a.piece, '')) = TRIM(COALESCE(b.piece, ''))
             AND TRIM(COALESCE(a.folio, '')) = TRIM(COALESCE(b.folio, ''))
             AND (a.age = b.age OR (a.age IS NULL AND b.age IS NULL))
             AND b.source = 'geni'
         )"
    )
    .fetch_one(&pool)
    .await?;

    println!("========================================");
    println!("DUPLICATE SOURCE BREAKDOWN");
    println!("========================================");
    println!("Duplicates within Sheffield only: {}", sheffield_only.0);
    println!("Duplicates within Geni only: {}", geni_only.0);
    println!("Duplicates across both sources: {}", cross_source.0);
    println!();

    Ok(())
}
