use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("MATCHING UNMATCHED CENSUS TO BUSINESS ADDRESSES");
    println!("========================================\n");

    // First, let's check what business-related tables exist
    println!("Checking available tables...\n");
    let tables: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM sqlite_master
         WHERE type='table' AND (name LIKE '%business%' OR name LIKE '%sheffield%')
         ORDER BY name"
    )
    .fetch_all(&pool)
    .await?;

    println!("Found {} relevant tables:\n", tables.len());
    for (table_name,) in &tables {
        println!("  - {}", table_name);

        // Get column info
        let columns: Vec<(i64, String, String, i64, Option<String>, i64)> = sqlx::query_as(
            &format!("PRAGMA table_info({})", table_name)
        )
        .fetch_all(&pool)
        .await?;

        for (_, col_name, col_type, _, _, _) in &columns {
            println!("      {} ({})", col_name, col_type);
        }
        println!();
    }

    // Get sample business addresses
    println!("\n========================================");
    println!("SAMPLE BUSINESS ADDRESSES");
    println!("========================================\n");

    let business_samples: Vec<(String, String)> = sqlx::query_as(
        "SELECT full_name, address FROM sheffield_businesses LIMIT 10"
    )
    .fetch_all(&pool)
    .await?;

    for (name, address) in &business_samples {
        println!("Business: {}", name);
        println!("Address:  {}", address);
        println!();
    }

    // Get sample census records (checking for civil_parish, registration_district, etc)
    println!("\n========================================");
    println!("SAMPLE CENSUS RECORDS");
    println!("========================================\n");

    let census_samples: Vec<(String, Option<String>, Option<String>, i64)> = sqlx::query_as(
        "SELECT name, civil_parish, registration_district, age
         FROM unmatched_sheffieldcensus
         WHERE civil_parish IS NOT NULL OR registration_district IS NOT NULL
         LIMIT 10"
    )
    .fetch_all(&pool)
    .await?;

    for (name, parish, district, age) in &census_samples {
        println!("Census: {} (age: {})", name, age);
        if let Some(p) = parish {
            println!("  Parish: {}", p);
        }
        if let Some(d) = district {
            println!("  District: {}", d);
        }
        println!();
    }

    // Try to find street name matches
    println!("\n========================================");
    println!("FINDING ADDRESS MATCHES");
    println!("(Matching census records to business streets)");
    println!("========================================\n");

    let total_census_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM unmatched_sheffieldcensus"
    )
    .fetch_one(&pool)
    .await?;

    println!("Total census records in unmatched_sheffieldcensus: {}\n", total_census_count.0);

    let mut matches_found = 0;

    // Get all census records
    let census_records: Vec<(String, String, i64, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT id, name, age, civil_parish, registration_district
         FROM unmatched_sheffieldcensus"
    )
    .fetch_all(&pool)
    .await?;

    println!("Processing first 100 census records for business address matches...\n");

    for (_census_id, census_name, census_age, parish, district) in census_records.iter().take(100) {
        // Try to find businesses in the same parish or district
        let mut location_found = false;

        if let Some(p) = parish {
            if !p.is_empty() {
                let business_matches: Vec<(String, String)> = sqlx::query_as(
                    "SELECT full_name, address
                     FROM sheffield_businesses
                     WHERE address LIKE ?
                     LIMIT 3"
                )
                .bind(format!("%{}%", p))
                .fetch_all(&pool)
                .await?;

                if !business_matches.is_empty() {
                    matches_found += 1;
                    location_found = true;
                    println!("MATCH FOUND (Parish):");
                    println!("  Census: {} (age: {})", census_name, census_age);
                    println!("  Parish: {}", p);
                    println!("  Matching businesses:");
                    for (biz_name, biz_address) in &business_matches {
                        println!("    - {} at {}", biz_name, biz_address);
                    }
                    println!();
                }
            }
        }

        if !location_found {
            if let Some(d) = district {
                if !d.is_empty() {
                    let business_matches: Vec<(String, String)> = sqlx::query_as(
                        "SELECT full_name, address
                         FROM sheffield_businesses
                         WHERE address LIKE ?
                         LIMIT 3"
                    )
                    .bind(format!("%{}%", d))
                    .fetch_all(&pool)
                    .await?;

                    if !business_matches.is_empty() {
                        matches_found += 1;
                        println!("MATCH FOUND (District):");
                        println!("  Census: {} (age: {})", census_name, census_age);
                        println!("  District: {}", d);
                        println!("  Matching businesses:");
                        for (biz_name, biz_address) in &business_matches {
                            println!("    - {} at {}", biz_name, biz_address);
                        }
                        println!();
                    }
                }
            }
        }
    }

    println!("\n========================================");
    println!("SUMMARY");
    println!("========================================");
    println!("Total census records: {}", total_census_count.0);
    println!("Census records with business location matches (first 100): {}", matches_found);
    println!("Match rate: {:.1}%", (matches_found as f64 / 100.0) * 100.0);

    Ok(())
}
