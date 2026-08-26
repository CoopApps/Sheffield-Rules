//! End-to-end test of the close-season assembly: the member clubs vote on next
//! season's laws in reaction to the season just played, the new ruleset lands in
//! sheffield_rules_history, and law-change items reach the inbox. Idempotent and
//! deterministic. Runs on a copy of the real database.

use saturday_at_three::fsim_bridge::{self, LawGovernance};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::Row;

const REAL_DB: &str = "D:/projects/saturday at three/Sheffield1867.db";

async fn prepare(tag: &str) -> SqlitePool {
    let dst = std::env::temp_dir().join(format!("fsim_assembly_{tag}.db"));
    std::fs::copy(REAL_DB, &dst).expect("copy db");
    let pool = SqlitePoolOptions::new()
        .connect(&format!("sqlite:{}", dst.display()))
        .await
        .expect("open db copy");
    // Seed a played season: 40 matches with rouges in play (deterministic values).
    let clubs: Vec<(String,)> = sqlx::query_as(
        "SELECT lc.club_id FROM sheffield_league_clubs lc \
         JOIN sheffield_clubs c ON c.id = lc.club_id WHERE c.is_reserve_team = 0 LIMIT 8",
    )
    .fetch_all(&pool).await.expect("clubs");
    for i in 0..40u32 {
        let home = &clubs[(i % 8) as usize].0;
        let away = &clubs[((i + 3) % 8) as usize].0;
        let (hs, aws) = ((i % 3) as i64, ((i + 1) % 3) as i64);
        let (hr, ar) = ((i % 2) as i64, ((i / 2) % 2) as i64);
        sqlx::query(
            "INSERT INTO sheffield_matches (id, gameweek, season, rule_year, home_club_id, away_club_id, \
             home_score, away_score, home_rouges, away_rouges, played, match_date) \
             VALUES (?, ?, 1867, 1867, ?, ?, ?, ?, ?, ?, 1, '1867-11-01')",
        )
        .bind(format!("season-test-{i}")).bind((i / 4 + 1) as i64)
        .bind(home).bind(away).bind(hs).bind(aws).bind(hr).bind(ar)
        .execute(&pool).await.expect("seed match");
    }
    pool
}

#[tokio::test]
async fn assembly_votes_writes_laws_and_raises_news_idempotently() {
    let pool = prepare("main").await;

    let close = fsim_bridge::close_season(&pool, 1867, "1867-12-31", LawGovernance::Assembly)
        .await.expect("close").expect("first close runs");
    assert!(close.assembly_held, "the assembly convened");

    // Next season's laws are written, in the engine's shape.
    let row = sqlx::query("SELECT ruleset_json FROM sheffield_rules_history WHERE season_year = 1868")
        .fetch_one(&pool).await.expect("rules row");
    let json: String = row.get("ruleset_json");
    let rules: fsim_core::Ruleset = serde_json::from_str(&json).expect("parses as engine ruleset");
    assert!(rules.team_size > 0);

    // The inbox heard about it: carried motions, or the members holding firm.
    let n: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_news_items WHERE article_type = 'law_change' AND publish_date = '1867-12-31'",
    )
    .fetch_one(&pool).await.expect("news count");
    assert!(n.0 >= 1, "at least one law item filed");

    // Idempotent: closing again does nothing and files nothing new.
    let again = fsim_bridge::close_season(&pool, 1867, "1867-12-31", LawGovernance::Assembly)
        .await.expect("second close");
    assert!(again.is_none(), "already closed");
    let n2: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_news_items WHERE article_type = 'law_change'",
    )
    .fetch_one(&pool).await.expect("news recount");
    assert_eq!(n2.0, n.0, "no duplicate items");

    // Deterministic: an identical season on a fresh copy carries the same motions.
    let pool2 = prepare("replay").await;
    let close2 = fsim_bridge::close_season(&pool2, 1867, "1867-12-31", LawGovernance::Assembly)
        .await.expect("close2").expect("runs");
    assert_eq!(close.carried, close2.carried, "same season, same vote");
}

#[tokio::test]
async fn historical_governance_turns_the_calendar_without_a_vote() {
    let pool = prepare("hist").await;
    // 1867 (rouges, March code) → 1868 (rouge abolished): the calendar turns.
    let close = fsim_bridge::close_season(&pool, 1867, "1867-12-31", LawGovernance::Historical)
        .await.expect("close").expect("runs");
    assert!(!close.assembly_held, "no vote in a historical game");
    let row = sqlx::query("SELECT ruleset_json FROM sheffield_rules_history WHERE season_year = 1868")
        .fetch_one(&pool).await.expect("rules row");
    let rules: fsim_core::Ruleset = serde_json::from_str(&row.get::<String, _>("ruleset_json")).expect("parses");
    assert!(!rules.rouges(), "1868 historical laws have no rouge");
    // The abolition is news to every club.
    let n: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_news_items WHERE article_type = 'law_change' AND headline LIKE '%Abolished%'",
    )
    .fetch_one(&pool).await.expect("news");
    assert!(n.0 >= 1, "the rouge's abolition was reported");
}
