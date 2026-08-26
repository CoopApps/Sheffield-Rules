use rusqlite::Connection;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let conn = Connection::open(db_path)?;

    // Delete news items first (child table with foreign key)
    let deleted_news = conn.execute("DELETE FROM sheffield_news_items", [])?;
    println!("Deleted {} news items", deleted_news);

    // Then delete challenge invitations (parent table)
    let deleted_invitations = conn.execute("DELETE FROM sheffield_challenge_invitations", [])?;
    println!("Deleted {} challenge invitations", deleted_invitations);

    println!("✓ Successfully wiped all old challenge data from Sheffield1867.db");

    Ok(())
}
