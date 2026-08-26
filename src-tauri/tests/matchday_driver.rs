//! End-to-end test of the engine matchday driver: due league matches are played
//! with real Sheffield-Rules results, a commentary ticker lands in
//! sheffield_match_events, a day round-up reaches the inbox — and the user's own
//! fixture is left untouched for interactive play. Runs on a copy of the real DB.

use saturday_at_three::fsim_bridge;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Row;

const REAL_DB: &str = "D:/projects/saturday at three/Sheffield1867.db";

#[tokio::test]
async fn due_matches_play_with_ticker_and_roundup_but_user_match_is_left() {
    let dst = std::env::temp_dir().join("fsim_matchday_test.db");
    std::fs::copy(REAL_DB, &dst).expect("copy db");
    let pool = SqlitePoolOptions::new()
        .connect(&format!("sqlite:{}", dst.display()))
        .await
        .expect("open db copy");

    // Four first-team clubs that have footballers.
    let clubs: Vec<(String,)> = sqlx::query_as(
        "SELECT lc.club_id FROM sheffield_league_clubs lc \
         JOIN sheffield_clubs c ON c.id = lc.club_id \
         WHERE c.is_reserve_team = 0 AND EXISTS \
           (SELECT 1 FROM sheffield_footballers f WHERE f.club_id = lc.club_id) \
         LIMIT 4",
    )
    .fetch_all(&pool).await.expect("clubs");
    assert!(clubs.len() >= 4, "need four clubs with footballers");
    let ids: Vec<&str> = clubs.iter().map(|c| c.0.as_str()).collect();

    let date = "1867-10-12";
    let due = vec![
        ("test-league-1".to_string(), ids[0].to_string(), ids[1].to_string(), 5i64),
        ("test-league-2".to_string(), ids[2].to_string(), ids[3].to_string(), 5i64),
    ];

    // The user manages ids[2] — their match must NOT be pre-played.
    let played = fsim_bridge::play_due_matches(&pool, date, &due, ids[2], 1867)
        .await.expect("driver");
    assert_eq!(played.len(), 1, "only the non-user match is played");
    assert_eq!(played[0].match_id, "test-league-1");

    // The result is persisted (insert path of the upsert).
    let m = sqlx::query("SELECT played, home_score, away_score FROM sheffield_matches WHERE id = 'test-league-1'")
        .fetch_one(&pool).await.expect("match row");
    assert_eq!(m.get::<i64, _>("played"), 1);

    // The user's match has no row and no result.
    let user_row = sqlx::query("SELECT id FROM sheffield_matches WHERE id = 'test-league-2'")
        .fetch_optional(&pool).await.expect("query");
    assert!(user_row.is_none(), "user fixture untouched");

    // The CM-style ticker: framed by a kickoff and a full-time verdict.
    let events: Vec<(String,)> = sqlx::query_as(
        "SELECT event_type FROM sheffield_match_events WHERE match_id = 'test-league-1' ORDER BY minute",
    )
    .fetch_all(&pool).await.expect("events");
    assert!(events.iter().any(|e| e.0 == "kickoff"), "ticker has a kickoff line");
    assert!(events.iter().any(|e| e.0 == "fulltime"), "ticker has a full-time line");

    // The day round-up item reached the inbox.
    let news = sqlx::query("SELECT body_text FROM sheffield_news_items WHERE article_type = 'match_report' AND publish_date = ?")
        .bind(date).fetch_one(&pool).await.expect("round-up");
    assert!(news.get::<String, _>("body_text").contains("–"), "round-up lists scorelines");

    // Determinism: reset and replay — identical score.
    let s1 = (played[0].home_score, played[0].away_score);
    sqlx::query("UPDATE sheffield_matches SET played = 0 WHERE id = 'test-league-1'")
        .execute(&pool).await.unwrap();
    let replay = fsim_bridge::play_due_matches(&pool, date, &due, ids[2], 1867)
        .await.expect("replay");
    assert_eq!((replay[0].home_score, replay[0].away_score), s1, "save-replay identical");
}
