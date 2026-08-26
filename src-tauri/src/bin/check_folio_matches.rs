use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("CHECKING FOLIO MATCHES");
    println!("Examining if different people share same piece+folio+age");
    println!("========================================\n");

    // First, let's check if Richard Rotherham and Elizabeth Anderson exist
    println!("Looking for Rotherham records...");
    let rotherham: Vec<(String, String, i64, String, String)> = sqlx::query_as(
        "SELECT census_name, geni_forename || ' ' || geni_surname as geni_name,
                geni_age, census_piece, census_folio
         FROM geni_census_validation
         WHERE census_name LIKE '%Rotherham%' OR geni_surname LIKE '%Rotherham%'
         LIMIT 10"
    )
    .fetch_all(&pool)
    .await?;

    for (census_name, geni_name, age, piece, folio) in &rotherham {
        println!("  Census: {} | Geni: {} | Age: {} | Piece: {} | Folio: {}",
            census_name, geni_name, age, piece, folio);
    }

    println!("\nLooking for Anderson records...");
    let anderson: Vec<(String, String, i64, String, String)> = sqlx::query_as(
        "SELECT census_name, geni_forename || ' ' || geni_surname as geni_name,
                geni_age, census_piece, census_folio
         FROM geni_census_validation
         WHERE census_name LIKE '%Anderson%' OR geni_surname LIKE '%Anderson%'
         LIMIT 10"
    )
    .fetch_all(&pool)
    .await?;

    for (census_name, geni_name, age, piece, folio) in &anderson {
        println!("  Census: {} | Geni: {} | Age: {} | Piece: {} | Folio: {}",
            census_name, geni_name, age, piece, folio);
    }

    // Now let's find cases where same piece+folio+age has multiple different people
    println!("\n========================================");
    println!("DUPLICATE PIECE+FOLIO+AGE COMBINATIONS");
    println!("(Different people sharing same location/age)");
    println!("========================================\n");

    let duplicates: Vec<(String, String, i64, i64)> = sqlx::query_as(
        "SELECT census_piece, census_folio, census_age, COUNT(*) as count
         FROM geni_census_validation
         GROUP BY census_piece, census_folio, census_age
         HAVING COUNT(*) > 1
         ORDER BY count DESC
         LIMIT 20"
    )
    .fetch_all(&pool)
    .await?;

    println!("Found {} piece+folio+age combinations with multiple people\n", duplicates.len());

    for (piece, folio, age, count) in &duplicates {
        println!("Piece: {}, Folio: {}, Age: {} ({} people)", piece, folio, age, count);

        // Show the people at this location
        let people: Vec<(String, String)> = sqlx::query_as(
            "SELECT census_name, geni_forename || ' ' || geni_surname as geni_name
             FROM geni_census_validation
             WHERE census_piece = ? AND census_folio = ? AND census_age = ?"
        )
        .bind(piece)
        .bind(folio)
        .bind(age)
        .fetch_all(&pool)
        .await?;

        for (census_name, geni_name) in &people {
            println!("  Census: {} | Geni: {}", census_name, geni_name);
        }
        println!();
    }

    // Show some specific VARIANT matches with their piece/folio/age
    println!("========================================");
    println!("SAMPLE VARIANT MATCHES");
    println!("(Showing piece/folio/age details)");
    println!("========================================\n");

    let variants: Vec<(String, String, String, i64, String, String)> = sqlx::query_as(
        "SELECT census_name,
                geni_forename || ' ' || geni_surname as geni_name,
                match_type,
                geni_age,
                census_piece,
                census_folio
         FROM geni_census_validation
         WHERE match_type = 'VARIANT'
         LIMIT 30"
    )
    .fetch_all(&pool)
    .await?;

    for (census_name, geni_name, match_type, age, piece, folio) in &variants {
        println!("Piece: {}, Folio: {}, Age: {}", piece, folio, age);
        println!("  Census: {}", census_name);
        println!("  Geni:   {}", geni_name);
        println!("  Type:   {}", match_type);
        println!();
    }

    Ok(())
}
