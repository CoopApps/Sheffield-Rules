use sqlx::sqlite::{SqlitePool, SqliteConnectOptions};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";

    println!("Opening database: {}", db_path);
    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?;
    let pool = SqlitePool::connect_with(connect_options).await?;

    // Count players per club
    println!("\n=== Players per Club (Top 20) ===");
    let club_sizes: Vec<(String, String, i64)> = sqlx::query_as(
        "SELECT 
            c.id,
            c.name,
            COUNT(f.id) as player_count
         FROM sheffield_clubs c
         LEFT JOIN sheffield_footballers f ON c.id = f.club_id
         GROUP BY c.id, c.name
         HAVING player_count > 0
         ORDER BY player_count DESC
         LIMIT 20"
    )
    .fetch_all(&pool)
    .await?;

    for (club_id, club_name, count) in club_sizes {
        println!("  {} ({}) - {} players", club_name, club_id, count);
    }

    // Count total assigned vs unassigned
    println!("\n=== Assignment Status ===");
    let (assigned,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_footballers WHERE club_id != 'UNASSIGNED'"
    )
    .fetch_one(&pool)
    .await?;

    let (unassigned,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_footballers WHERE club_id = 'UNASSIGNED'"
    )
    .fetch_one(&pool)
    .await?;

    let (total,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_footballers"
    )
    .fetch_one(&pool)
    .await?;

    println!("Total footballers: {}", total);
    println!("Assigned to clubs: {} ({:.1}%)", assigned, (assigned as f64 / total as f64) * 100.0);
    println!("Unassigned: {} ({:.1}%)", unassigned, (unassigned as f64 / total as f64) * 100.0);

    // Average players per club (for clubs that have players)
    let (avg_size,): (f64,) = sqlx::query_as(
        "SELECT AVG(player_count) FROM (
            SELECT COUNT(*) as player_count
            FROM sheffield_footballers
            WHERE club_id != 'UNASSIGNED'
            GROUP BY club_id
        )"
    )
    .fetch_one(&pool)
    .await?;

    println!("\nAverage players per club (clubs with players): {:.1}", avg_size);

    pool.close().await;
    Ok(())
}
