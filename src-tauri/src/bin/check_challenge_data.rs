use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "sqlite:../Sheffield1867.db";

    println!("Connecting to database: {}", database_url);
    let pool = SqlitePool::connect(database_url).await?;

    // Check invitations
    let invitation_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_challenge_invitations"
    )
    .fetch_one(&pool)
    .await?;
    println!("\n📨 Challenge Invitations: {}", invitation_count.0);

    if invitation_count.0 > 0 {
        let invitations: Vec<(String, String, String, String, String)> = sqlx::query_as(
            "SELECT id, sender_club_id, recipient_club_id, sent_date, status FROM sheffield_challenge_invitations ORDER BY sent_date DESC LIMIT 5"
        )
        .fetch_all(&pool)
        .await?;

        for (id, sender, recipient, date, status) in invitations {
            println!("  - {} → {} on {} ({})", sender, recipient, date, status);
        }
    }

    // Check news items
    let news_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_news_items"
    )
    .fetch_one(&pool)
    .await?;
    println!("\n📰 News Items: {}", news_count.0);

    if news_count.0 > 0 {
        let news: Vec<(String, String, String, String)> = sqlx::query_as(
            "SELECT id, headline, article_type, publish_date FROM sheffield_news_items ORDER BY publish_date DESC LIMIT 5"
        )
        .fetch_all(&pool)
        .await?;

        for (id, headline, article_type, date) in news {
            println!("  - [{}] {} ({})", date, headline, article_type);
        }
    }

    Ok(())
}
