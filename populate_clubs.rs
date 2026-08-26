use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = SqlitePool::connect("sqlite:sheffield_save.db").await?;

    // Get all clubs
    let clubs = sqlx::query!("SELECT id, name FROM sheffield_clubs")
        .fetch_all(&pool)
        .await?;

    println!("Found {} clubs in database", clubs.len());

    // Division 1 assignments
    let assignments = vec![
        ("sheffield", "div-1", 1),
        ("hallam", "div-1", 2),
        ("norfolk", "div-1", 3),
        ("cemetery", "div-1", 4),
        ("york", "div-1", 5),
        ("norton", "div-1", 6),
        ("pitsmoor", "div-1", 7),
        ("fir vale", "div-1", 8),
        ("newhall", "div-1", 9),
        ("attercliffe", "div-1", 10),
        ("sheaf house", "div-1", 11),
        ("exchange", "div-1", 12),
        ("mechanics", "div-1", 13),
        ("broomhall", "div-1", 14),
        ("brightside", "div-1", 15),
        ("heeley", "div-1", 16),
    ];

    let mut success = 0;
    let mut not_found = Vec::new();

    for (name_hint, division_id, position) in assignments {
        // Find club by name
        let club = clubs.iter().find(|c| {
            c.name.to_lowercase().contains(name_hint)
        });

        if let Some(club) = club {
            sqlx::query!(
                "INSERT OR REPLACE INTO sheffield_league_clubs
                 (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
                 VALUES (?, ?, ?, ?, ?, ?)",
                format!("{}-{}", club.id, division_id),
                division_id,
                club.id,
                position,
                false,
                None::<String>
            )
            .execute(&pool)
            .await?;

            println!("✓ Assigned {} to {} at position {}", club.name, division_id, position);
            success += 1;
        } else {
            not_found.push(name_hint);
            println!("✗ Could not find club: {}", name_hint);
        }
    }

    println!("\n✓ Successfully assigned {} clubs", success);
    if !not_found.is_empty() {
        println!("✗ Could not find {} clubs: {:?}", not_found.len(), not_found);
    }

    Ok(())
}
