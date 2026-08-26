use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let conn = rusqlite::Connection::open("D:/projects/Saturday at Three/Sheffield1867.db")?;

    // Delete news first (child table with foreign key)
    conn.execute("DELETE FROM sheffield_news_items", [])?;

    // Then delete invitations (parent table)
    conn.execute("DELETE FROM sheffield_challenge_invitations", [])?;

    println!("✓ Wiped old challenge letters and news from Sheffield1867.db");

    Ok(())
}
