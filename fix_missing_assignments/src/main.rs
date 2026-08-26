use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = SqlitePool::connect(&format!("sqlite:{}", db_path)).await?;

    println!("Checking 105th Regiment Reserves...");
    let regiment: Option<(String, String, i32)> = sqlx::query_as(
        "SELECT club_id, division_id, is_reserve_team FROM sheffield_league_clubs WHERE club_id LIKE '%105th-regiment%reserve%'"
    )
    .fetch_optional(&pool)
    .await?;

    if let Some((club_id, div_id, is_reserve)) = regiment {
        println!("Found: {} in {} (reserve: {})", club_id, div_id, is_reserve);
    } else {
        println!("NOT FOUND in sheffield_league_clubs - need to add it");
        // Add it to Reserve Division 7C (where it should be)
        sqlx::query(
            "INSERT INTO sheffield_league_clubs (club_id, division_id, is_reserve_team, reserve_of_club_id)
             VALUES (?, 'res-div-7c', 1, '105th-regiment-fc')"
        )
        .bind("105th-regiment-fc-reserve")
        .execute(&pool)
        .await?;
        println!("✓ Added 105th Regiment Reserves to Reserve Division 7C");
    }

    println!("\nChecking Albion FC B...");
    let albion: Option<(String, String, i32)> = sqlx::query_as(
        "SELECT club_id, division_id, is_reserve_team FROM sheffield_league_clubs WHERE club_id LIKE '%albion-fc%reserve%' OR club_id LIKE '%albion-fc-b%'"
    )
    .fetch_optional(&pool)
    .await?;

    if let Some((club_id, div_id, is_reserve)) = albion {
        println!("Found: {} in {} (reserve: {})", club_id, div_id, is_reserve);
    } else {
        println!("NOT FOUND in sheffield_league_clubs - need to add it");
        // Add it to Reserve Division 7C (where it should be)
        sqlx::query(
            "INSERT INTO sheffield_league_clubs (club_id, division_id, is_reserve_team, reserve_of_club_id)
             VALUES (?, 'res-div-7c', 1, 'albion-fc')"
        )
        .bind("albion-fc-b")
        .execute(&pool)
        .await?;
        println!("✓ Added Albion FC B to Reserve Division 7C");
    }

    println!("\n✓ Done!");
    Ok(())
}
