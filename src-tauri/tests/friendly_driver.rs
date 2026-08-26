//! End-to-end test of the friendly daily driver: an accepted challenge whose
//! match day has arrived gets PLAYED by the engine (the step the old flow never
//! reached), the score lands in sheffield_matches, and a result item appears in
//! sheffield_news_items. Runs against a copy of the real database.

use saturday_at_three::fsim_bridge;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Row;

const REAL_DB: &str = "D:/projects/saturday at three/Sheffield1867.db";

#[tokio::test]
async fn due_friendly_is_played_and_raises_news() {
    // Work on a copy — the real DB is never touched.
    let dst = std::env::temp_dir().join("fsim_driver_test.db");
    std::fs::copy(REAL_DB, &dst).expect("copy db");
    let pool = SqlitePoolOptions::new()
        .connect(&format!("sqlite:{}", dst.display()))
        .await
        .expect("open db copy");

    // Two first-team clubs that actually have footballers.
    let clubs: Vec<(String,)> = sqlx::query_as(
        "SELECT lc.club_id FROM sheffield_league_clubs lc \
         JOIN sheffield_clubs c ON c.id = lc.club_id \
         WHERE c.is_reserve_team = 0 AND EXISTS \
           (SELECT 1 FROM sheffield_footballers f WHERE f.club_id = lc.club_id) \
         LIMIT 2",
    )
    .fetch_all(&pool)
    .await
    .expect("clubs");
    let (home, away) = (&clubs[0].0, &clubs[1].0);

    // An accepted challenge with its match scheduled for today, unplayed.
    let today = "1867-10-05";
    let match_id = "test-friendly-match";
    let inv_id = "test-friendly-invitation";
    sqlx::query(
        "INSERT INTO sheffield_matches (id, gameweek, season, rule_year, home_club_id, away_club_id, \
         home_score, away_score, home_rouges, away_rouges, played, match_date) \
         VALUES (?, 0, 1867, 1867, ?, ?, 0, 0, 0, 0, 0, ?)",
    )
    .bind(match_id).bind(home).bind(away).bind(today)
    .execute(&pool).await.expect("insert match");
    sqlx::query(
        "INSERT INTO sheffield_challenge_invitations (id, sender_club_id, recipient_club_id, sent_date, \
         response_date, proposed_match_date, match_type, venue, stakes, tone, rules_type, match_duration, \
         status, scheduled_match_id) VALUES (?, ?, ?, '1867-09-21', '1867-09-24', ?, 'friendly', 'home', \
         'honor', 'cordial', 'sheffield', 90, 'accepted', ?)",
    )
    .bind(inv_id).bind(home).bind(away).bind(today).bind(match_id)
    .execute(&pool).await.expect("insert invitation");

    // THE daily driver — the in-app code path advance_day now calls.
    let played = fsim_bridge::play_due_friendlies(&pool, today).await.expect("driver");
    assert_eq!(played.len(), 1, "exactly the one due friendly is played");

    // The match is really played, by the engine.
    let m = sqlx::query("SELECT played, home_score, away_score FROM sheffield_matches WHERE id = ?")
        .bind(match_id).fetch_one(&pool).await.expect("match row");
    assert_eq!(m.get::<i64, _>("played"), 1, "match marked played");

    // A result item reached the inbox, dated today, linked to the match.
    let n = sqlx::query(
        "SELECT article_type, headline FROM sheffield_news_items \
         WHERE related_match_id = ? AND publish_date = ?",
    )
    .bind(match_id).bind(today)
    .fetch_one(&pool).await.expect("news row");
    assert_eq!(n.get::<String, _>("article_type"), "friendly_result");
    assert!(n.get::<String, _>("headline").starts_with("Friendly:"));

    // The friendly gets its commentary ticker too, framed kickoff to full-time.
    let ticker: Vec<(String,)> = sqlx::query_as(
        "SELECT event_type FROM sheffield_match_events WHERE match_id = ? ORDER BY minute",
    )
    .bind(match_id).fetch_all(&pool).await.expect("ticker");
    assert!(ticker.iter().any(|e| e.0 == "kickoff") && ticker.iter().any(|e| e.0 == "fulltime"),
            "friendly ticker framed kickoff to full-time");

    // Determinism: the same match id replays to the same score.
    let s1 = (played[0].home_score, played[0].away_score);
    sqlx::query("UPDATE sheffield_matches SET played = 0 WHERE id = ?")
        .bind(match_id).execute(&pool).await.unwrap();
    let replay = fsim_bridge::play_due_friendlies(&pool, today).await.expect("replay");
    assert_eq!((replay[0].home_score, replay[0].away_score), s1, "save-replay identical");
}
