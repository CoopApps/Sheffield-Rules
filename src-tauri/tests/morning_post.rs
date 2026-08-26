//! The morning post: AI clubs issue challenges of their own accord —
//! deterministically per date, idempotently on replay, and a letter addressed to
//! the user's club lands in the inbox. Runs on a copy of the real database.

use saturday_at_three::fsim_bridge;
use sqlx::sqlite::SqlitePoolOptions;

const REAL_DB: &str = "D:/projects/saturday at three/Sheffield1867.db";

#[tokio::test]
async fn ai_clubs_post_challenges_deterministically() {
    let dst = std::env::temp_dir().join("fsim_post_test.db");
    std::fs::copy(REAL_DB, &dst).expect("copy db");
    let pool = SqlitePoolOptions::new()
        .connect(&format!("sqlite:{}", dst.display()))
        .await
        .expect("open db copy");

    // The user manages the first first-team club.
    let user: (String,) = sqlx::query_as(
        "SELECT lc.club_id FROM sheffield_league_clubs lc \
         JOIN sheffield_clubs c ON c.id = lc.club_id WHERE c.is_reserve_team = 0 LIMIT 1",
    )
    .fetch_one(&pool).await.expect("user club");

    // Two months of mornings — one in the close season (June/July), one in the
    // thick of the season (Sept/Oct). Letters must arrive, replays must post
    // nothing twice, and the close season must bring more correspondence.
    let mut posted_total = 0;
    let mut user_letters = 0;
    let mut distant_letters = 0;
    let mut off_season_count = 0;
    let mut in_season_count = 0;
    for day in 0..60 {
        let (date, off_season) = if day < 30 {
            (format!("1867-06-{:02}", day % 30 + 1), true)   // close season
        } else {
            (format!("1867-09-{:02}", day % 30 + 1), false)  // season underway
        };
        let first = fsim_bridge::generate_ai_challenges(&pool, &date, &user.0)
            .await.expect("post");
        let replay = fsim_bridge::generate_ai_challenges(&pool, &date, &user.0)
            .await.expect("replay");
        assert!(replay.is_empty(), "a replayed morning posts nothing twice");
        for p in &first {
            posted_total += 1;
            if off_season { off_season_count += 1; } else { in_season_count += 1; }
            if p.to_user { user_letters += 1; }
            if p.sender_club_id.starts_with("guest-") || p.recipient_club_id.starts_with("guest-") {
                distant_letters += 1;
                // A cross-country letter records the guest's code for the tie.
                let code: (String,) = sqlx::query_as(
                    "SELECT rules_type FROM sheffield_challenge_invitations WHERE id = ?")
                    .bind(&p.invitation_id).fetch_one(&pool).await.expect("code");
                assert!(["nottingham", "fa", "sheffield"].contains(&code.0.as_str()));
            }
            assert_ne!(p.sender_club_id, p.recipient_club_id);
        }
    }
    assert!(posted_total >= 2, "two months of post bring letters ({posted_total})");
    assert!(off_season_count > in_season_count,
            "the close season brings more letters ({off_season_count} v {in_season_count})");
    assert!(distant_letters >= 1,
            "the country writes: at least one inter-association letter ({distant_letters})");

    // Every posted letter is a real pending invitation the pipeline will answer.
    let pending: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_challenge_invitations WHERE id LIKE 'post-%' AND status = 'sent'",
    )
    .fetch_one(&pool).await.expect("pending");
    assert_eq!(pending.0 as usize, posted_total);

    // A letter to the user reached the inbox as an important item.
    if user_letters > 0 {
        let n: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM sheffield_news_items WHERE article_type = 'challenge_received' AND is_important = 1",
        )
        .fetch_one(&pool).await.expect("news");
        assert_eq!(n.0 as usize, user_letters, "each user-addressed letter filed once");
    }

    println!("month of post: {posted_total} letters, {user_letters} addressed to the user");
}
