use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867_temp.db";

    println!("🔍 Checking Sheffield1867_temp.db...\n");

    let pool = SqlitePool::connect(&format!("sqlite:{}", db_path)).await?;

    // Check if database file exists and is accessible
    println!("✓ Database connection successful\n");

    // Count sheffield_league_clubs
    let league_clubs: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_league_clubs")
        .fetch_one(&pool)
        .await?;
    println!("📊 sheffield_league_clubs: {} entries", league_clubs.0);

    // Count sheffield_standings
    let standings: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_standings")
        .fetch_one(&pool)
        .await?;
    println!("📊 sheffield_standings: {} entries", standings.0);

    // Count clubs
    let clubs: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_clubs")
        .fetch_one(&pool)
        .await?;
    println!("📊 sheffield_clubs: {} entries\n", clubs.0);

    // Sample data from division 1
    println!("🔍 Querying First Division (div-1)...\n");

    let div1_query = "
        SELECT
            slc.club_id,
            sc.name as club_name,
            COALESCE(ss.played, 0) as played,
            COALESCE(ss.won, 0) as won,
            COALESCE(ss.drawn, 0) as drawn,
            COALESCE(ss.lost, 0) as lost,
            COALESCE(ss.goals_for, 0) as goals_for,
            COALESCE(ss.goals_against, 0) as goals_against,
            COALESCE(ss.points, 0) as points
        FROM sheffield_league_clubs slc
        JOIN sheffield_clubs sc ON slc.club_id = sc.id
        LEFT JOIN sheffield_standings ss ON slc.club_id = ss.club_id
        WHERE slc.division_id = 'div-1'
        ORDER BY COALESCE(ss.points, 0) DESC
        LIMIT 5
    ";

    let rows: Vec<(String, String, i32, i32, i32, i32, i32, i32, i32)> =
        sqlx::query_as(div1_query)
        .fetch_all(&pool)
        .await?;

    if rows.is_empty() {
        println!("❌ No clubs found in div-1!");
        println!("\nLet's check what divisions exist:");

        let divisions: Vec<(String,)> = sqlx::query_as(
            "SELECT DISTINCT division_id FROM sheffield_league_clubs ORDER BY division_id"
        )
        .fetch_all(&pool)
        .await?;

        println!("Found {} divisions:", divisions.len());
        for (div_id,) in divisions {
            let count: (i64,) = sqlx::query_as(
                "SELECT COUNT(*) FROM sheffield_league_clubs WHERE division_id = ?"
            )
            .bind(&div_id)
            .fetch_one(&pool)
            .await?;
            println!("  - {}: {} clubs", div_id, count.0);
        }
    } else {
        println!("✓ Found {} clubs in First Division:\n", rows.len());
        println!("{:<15} {:<30} P  W  D  L  GF GA Pts", "Club ID", "Club Name");
        println!("{}", "-".repeat(80));

        for (club_id, name, p, w, d, l, gf, ga, pts) in rows {
            println!("{:<15} {:<30} {:2} {:2} {:2} {:2} {:2} {:2} {:3}",
                club_id, name, p, w, d, l, gf, ga, pts);
        }
    }

    println!("\n✓ Database check complete");

    Ok(())
}
