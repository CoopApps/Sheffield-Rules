use sqlx::sqlite::SqlitePool;
use sqlx::Row;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite://{}", db_path)).await?;

    println!("========================================");
    println!("COMPARING TABLE SCHEMAS");
    println!("========================================\n");

    // Get sheffield_people columns
    let people_cols: Vec<(i64, String, String, i64, Option<String>, i64)> = sqlx::query_as(
        "PRAGMA table_info(sheffield_people)"
    )
    .fetch_all(&pool)
    .await?;

    // Get unmatched_sheffieldcensus columns
    let census_cols: Vec<(i64, String, String, i64, Option<String>, i64)> = sqlx::query_as(
        "PRAGMA table_info(unmatched_sheffieldcensus)"
    )
    .fetch_all(&pool)
    .await?;

    println!("SHEFFIELD_PEOPLE ({} columns):", people_cols.len());
    println!("========================================");
    for (idx, col_name, col_type, not_null, default_val, pk) in &people_cols {
        println!("{:2}. {} ({}) NOT NULL={} DEFAULT={:?} PK={}",
            idx, col_name, col_type, not_null, default_val, pk);
    }

    println!("\n\nUNMATCHED_SHEFFIELDCENSUS ({} columns):", census_cols.len());
    println!("========================================");
    for (idx, col_name, col_type, not_null, default_val, pk) in &census_cols {
        println!("{:2}. {} ({}) NOT NULL={} DEFAULT={:?} PK={}",
            idx, col_name, col_type, not_null, default_val, pk);
    }

    // Check which census columns exist in people table
    println!("\n\nCOMPATIBILITY CHECK:");
    println!("========================================");

    let people_col_names: Vec<String> = people_cols.iter()
        .map(|(_, name, _, _, _, _)| name.clone())
        .collect();

    let census_col_names: Vec<String> = census_cols.iter()
        .map(|(_, name, _, _, _, _)| name.clone())
        .collect();

    println!("\nColumns in CENSUS that exist in PEOPLE:");
    let mut matching_cols = Vec::new();
    for census_col in &census_col_names {
        if people_col_names.contains(census_col) {
            matching_cols.push(census_col.clone());
            println!("  ✓ {}", census_col);
        }
    }

    println!("\nColumns in CENSUS that DON'T exist in PEOPLE:");
    let mut non_matching_cols = Vec::new();
    for census_col in &census_col_names {
        if !people_col_names.contains(census_col) {
            non_matching_cols.push(census_col.clone());
            println!("  ✗ {}", census_col);
        }
    }

    println!("\nColumns in PEOPLE that DON'T exist in CENSUS:");
    for people_col in &people_col_names {
        if !census_col_names.contains(people_col) {
            println!("  ✗ {}", people_col);
        }
    }

    println!("\n\nSUMMARY:");
    println!("========================================");
    println!("Census columns that match: {}/{}", matching_cols.len(), census_col_names.len());
    println!("Census columns that DON'T match: {}/{}", non_matching_cols.len(), census_col_names.len());

    if non_matching_cols.is_empty() {
        println!("\n✓ ALL census columns exist in sheffield_people!");
        println!("✓ We can safely transfer data using explicit column mapping.");

        println!("\n\nPROPOSED INSERT STATEMENT:");
        println!("========================================");
        println!("INSERT INTO sheffield_people ({})", matching_cols.join(", "));
        println!("SELECT {} FROM unmatched_sheffieldcensus WHERE id = ?", matching_cols.join(", "));
    } else {
        println!("\n✗ Some census columns don't exist in sheffield_people!");
        println!("✗ Need to handle these columns carefully.");
    }

    Ok(())
}
