use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use sqlx::Row;
use std::str::FromStr;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";

    println!("Opening database: {}", db_path);
    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?;
    let pool = SqlitePool::connect_with(connect_options).await?;

    // Get unique parishes from sheffield_footballers/sheffield_people
    println!("\nExtracting unique parishes from player data...");
    let parishes: Vec<(String, i64)> = sqlx::query_as(
        "SELECT p.ecclesiastical_parish, COUNT(*) as count
         FROM sheffield_footballers f
         JOIN sheffield_people p ON f.person_id = p.unique_id
         WHERE p.ecclesiastical_parish IS NOT NULL
         AND p.ecclesiastical_parish != ''
         AND TRIM(p.ecclesiastical_parish) != ''
         GROUP BY p.ecclesiastical_parish
         ORDER BY p.ecclesiastical_parish"
    )
    .fetch_all(&pool)
    .await?;

    println!("Found {} unique parishes", parishes.len());

    // Parish to postcode mapping based on Sheffield church locations
    let mut parish_postcodes: HashMap<String, &str> = HashMap::new();

    // S1 - City Centre
    parish_postcodes.insert("St Peter".to_string(), "S1");
    parish_postcodes.insert("St Paul".to_string(), "S1");
    parish_postcodes.insert("Cathedral".to_string(), "S1");
    parish_postcodes.insert("St Marie".to_string(), "S1");

    // S2 - Heeley, Manor, Norfolk Park
    parish_postcodes.insert("St Bartholomew".to_string(), "S2");
    parish_postcodes.insert("Heeley".to_string(), "S2");
    parish_postcodes.insert("Manor".to_string(), "S2");

    // S3 - Broomhall, Netherthorpe, Pitsmoor
    parish_postcodes.insert("St Silas".to_string(), "S3");
    parish_postcodes.insert("St Philip".to_string(), "S3");
    parish_postcodes.insert("Netherthorpe".to_string(), "S3");
    parish_postcodes.insert("Broomhall".to_string(), "S3");

    // S4 - Brightside, Pitsmoor
    parish_postcodes.insert("St Matthew".to_string(), "S4");
    parish_postcodes.insert("Pitsmoor".to_string(), "S4");
    parish_postcodes.insert("Brightside".to_string(), "S4");

    // S5 - Ecclesfield
    parish_postcodes.insert("Ecclesfield".to_string(), "S5");
    parish_postcodes.insert("St Mary".to_string(), "S5");

    // S6 - Hillsborough, Walkley
    parish_postcodes.insert("Hillsborough".to_string(), "S6");
    parish_postcodes.insert("Wadsley".to_string(), "S6");
    parish_postcodes.insert("Walkley".to_string(), "S6");
    parish_postcodes.insert("Stannington".to_string(), "S6");

    // S7 - Nether Edge, Millhouses
    parish_postcodes.insert("Nether Edge".to_string(), "S7");
    parish_postcodes.insert("Millhouses".to_string(), "S7");

    // S8 - Norton, Woodseats
    parish_postcodes.insert("Norton".to_string(), "S8");
    parish_postcodes.insert("Woodseats".to_string(), "S8");
    parish_postcodes.insert("St James".to_string(), "S8");

    // S9 - Attercliffe
    parish_postcodes.insert("Attercliffe".to_string(), "S9");
    parish_postcodes.insert("St Lawrence".to_string(), "S9");

    // S10 - Crookes, Ranmoor, Broomhill
    parish_postcodes.insert("Crookes".to_string(), "S10");
    parish_postcodes.insert("Ranmoor".to_string(), "S10");
    parish_postcodes.insert("St Mark".to_string(), "S10");

    // S11 - Ecclesall, Sharrow
    parish_postcodes.insert("Ecclesall".to_string(), "S11");
    parish_postcodes.insert("Sharrow".to_string(), "S11");
    parish_postcodes.insert("St George".to_string(), "S11");

    // S12 - Gleadless, Intake
    parish_postcodes.insert("Gleadless".to_string(), "S12");
    parish_postcodes.insert("Intake".to_string(), "S12");

    // S17 - Dore, Totley
    parish_postcodes.insert("Dore".to_string(), "S17");
    parish_postcodes.insert("Totley".to_string(), "S17");

    // Additional common parishes
    parish_postcodes.insert("Wicker Trinity".to_string(), "S3");
    parish_postcodes.insert("St John".to_string(), "S3");
    parish_postcodes.insert("Dyers Hill".to_string(), "S3");
    parish_postcodes.insert("Neepsend St Michael".to_string(), "S3");
    parish_postcodes.insert("Christchurch".to_string(), "S11");
    parish_postcodes.insert("All Saints".to_string(), "S1");
    parish_postcodes.insert("Eldon Street".to_string(), "S1");
    parish_postcodes.insert("The Porter".to_string(), "S1");
    parish_postcodes.insert("St Thomas".to_string(), "S10");
    parish_postcodes.insert("Golcar".to_string(), "S5");

    // Clear existing parishes table
    println!("\nClearing existing sheffield_parishes table...");
    sqlx::query("DELETE FROM sheffield_parishes")
        .execute(&pool)
        .await?;

    // Insert parishes
    println!("Populating sheffield_parishes table...");
    let mut inserted = 0;
    for (parish_name, player_count) in &parishes {
        let parish_id = parish_name
            .to_lowercase()
            .replace(" ", "-")
            .replace("'", "")
            .replace(",", "");

        // Try to find postcode match
        let mut postcode = None;
        for (key, code) in &parish_postcodes {
            if parish_name.contains(key) || key.contains(parish_name) {
                postcode = Some(*code);
                break;
            }
        }

        // If no exact match, default to S1
        let final_postcode = postcode.unwrap_or("S1");

        sqlx::query(
            "INSERT OR REPLACE INTO sheffield_parishes (id, name, postcode, type) VALUES (?, ?, ?, 'ecclesiastical')"
        )
        .bind(&parish_id)
        .bind(parish_name)
        .bind(final_postcode)
        .execute(&pool)
        .await?;

        println!("  {} ({} players) -> {}", parish_name, player_count, final_postcode);
        inserted += 1;
    }

    println!("\n✓ Successfully populated {} parishes from player data!", inserted);

    // Show summary
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_parishes")
        .fetch_one(&pool)
        .await?;

    println!("Total parishes in table: {}", total.0);

    pool.close().await;
    Ok(())
}
