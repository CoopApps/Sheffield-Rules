use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867_temp.db";

    println!("🔍 Checking divisions in Sheffield1867_temp.db...\n");

    let pool = SqlitePool::connect(&format!("sqlite:{}", db_path)).await?;

    // Check club distribution across divisions
    let division_counts: Vec<(String, i64)> = sqlx::query_as(
        "SELECT division_id, COUNT(*) as club_count
         FROM sheffield_league_clubs
         GROUP BY division_id
         ORDER BY division_id"
    )
    .fetch_all(&pool)
    .await?;

    println!("📊 Division Distribution:");
    println!("{}", "=".repeat(40));
    for (div_id, count) in &division_counts {
        println!("{:<20} {:>3} clubs", div_id, count);
    }
    println!("{}", "=".repeat(40));
    let total: i64 = division_counts.iter().map(|(_, count)| count).sum();
    println!("{:<20} {:>3} clubs\n", "TOTAL", total);

    // Check which division Attercliffe Zion is in
    let attercliffe: Option<(String, String)> = sqlx::query_as(
        "SELECT sc.name, slc.division_id
         FROM sheffield_clubs sc
         JOIN sheffield_league_clubs slc ON sc.id = slc.club_id
         WHERE sc.name LIKE '%Attercliffe%Zion%'"
    )
    .fetch_optional(&pool)
    .await?;

    if let Some((club_name, div_id)) = attercliffe {
        println!("🎯 Found: {} in division {}\n", club_name, div_id);

        // Check how many clubs are in that division
        let div_clubs: Vec<(String,)> = sqlx::query_as(
            "SELECT sc.name
             FROM sheffield_league_clubs slc
             JOIN sheffield_clubs sc ON slc.club_id = sc.id
             WHERE slc.division_id = ?
             ORDER BY sc.name"
        )
        .bind(&div_id)
        .fetch_all(&pool)
        .await?;

        println!("📋 Clubs in {} ({} clubs):", div_id, div_clubs.len());
        println!("{}", "-".repeat(40));
        for (i, (name,)) in div_clubs.iter().enumerate() {
            println!("{:2}. {}", i + 1, name);
        }
    } else {
        println!("❌ Attercliffe Zion not found!");
    }

    println!("\n✓ Division check complete");

    Ok(())
}
