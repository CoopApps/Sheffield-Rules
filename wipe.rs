use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite:D:/projects/Saturday at Three/Sheffield1867.db")
        .await?;

    // Delete news first (child table with foreign key)
    sqlx::query("DELETE FROM sheffield_news_items")
        .execute(&pool)
        .await?;

    // Then delete invitations (parent table)
    sqlx::query("DELETE FROM sheffield_challenge_invitations")
        .execute(&pool)
        .await?;

    println!("✓ Successfully wiped all challenge letters and news from Sheffield1867.db");

    Ok(())
}
