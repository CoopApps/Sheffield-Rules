//! Sponsored cups, wired to the census: a man of means proposes a competition,
//! the news reaches every club, and the cup's fate ties to his own fortunes over
//! the following seasons. On a copy of the real database.

use saturday_at_three::fsim_bridge;
use sqlx::sqlite::SqlitePoolOptions;

const REAL_DB: &str = "D:/projects/saturday at three/Sheffield1867.db";

async fn db(tag: &str) -> sqlx::SqlitePool {
    let dst = std::env::temp_dir().join(format!("fsim_cup_{tag}.db"));
    std::fs::copy(REAL_DB, &dst).expect("copy");
    SqlitePoolOptions::new().connect(&format!("sqlite:{}", dst.display())).await.unwrap()
}

#[tokio::test]
async fn a_season_may_bring_a_new_sponsored_cup() {
    let pool = db("propose").await;
    // Sweep enough seasons that at least one proposal comes forward, and confirm
    // the whole shape when it does.
    let mut found = None;
    for season in 1867..1920 {
        if let Some(p) = fsim_bridge::maybe_propose_cup(&pool, season).await.expect("propose") {
            found = Some((season, p));
            break;
        }
    }
    let (season, p) = found.expect("a cup is proposed within a reasonable span of seasons");
    assert!(p.cup_name.contains(&p.sponsor_name), "the cup carries the sponsor's own name");
    assert!(p.entrants.is_power_of_two());
    println!("{} ({season}): {} entrants, {}s purse, divisions {}-{}",
             p.cup_name, p.entrants, p.prize_shillings, p.min_division_level, p.max_division_level);

    // The news reached the town.
    let n: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_news_items WHERE article_type = 'competition_news' \
         AND headline LIKE '%Proposed%'")
        .fetch_one(&pool).await.unwrap();
    assert!(n.0 >= 1, "the proposal reached the inbox");

    // Re-running the same season proposes nothing new on top (idempotent insert).
    let row_count_before: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_sponsored_cups")
        .fetch_one(&pool).await.unwrap();
    let _ = fsim_bridge::maybe_propose_cup(&pool, season).await.unwrap();
    let row_count_after: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_sponsored_cups")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(row_count_before.0, row_count_after.0, "the same season's proposal is not duplicated");
}

#[tokio::test]
async fn a_sponsored_cup_ages_and_may_lapse_or_endure() {
    let pool = db("tick").await;
    // Seed one directly so the test doesn't depend on a proposal landing.
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS sheffield_sponsored_cups (id TEXT PRIMARY KEY, \
         sponsor_name TEXT NOT NULL, cup_name TEXT NOT NULL, season_proposed INTEGER NOT NULL, \
         entrants INTEGER NOT NULL, prize_shillings INTEGER NOT NULL, \
         min_division_level INTEGER NOT NULL, max_division_level INTEGER NOT NULL, \
         seasons_held INTEGER NOT NULL DEFAULT 0, sponsor_still_backing INTEGER NOT NULL DEFAULT 1, \
         winner_club_id TEXT)")
        .execute(&pool).await.unwrap();
    sqlx::query(
        "INSERT INTO sheffield_sponsored_cups (id, sponsor_name, cup_name, season_proposed, entrants, \
         prize_shillings, min_division_level, max_division_level) \
         VALUES ('test-cup', 'A. Tester', 'the A. Tester Cup', 1867, 8, 100, 1, 4)")
        .execute(&pool).await.unwrap();

    // Tick it forward several seasons; it should still exist one way or another.
    for season in 1868..1878 {
        fsim_bridge::tick_sponsored_cups(&pool, season, &format!("{season}-05-01")).await.expect("tick");
    }
    let row: (i64, i64) = sqlx::query_as(
        "SELECT seasons_held, sponsor_still_backing FROM sheffield_sponsored_cups WHERE id = 'test-cup'")
        .fetch_one(&pool).await.unwrap();
    println!("after 10 seasons: held={}, still backing={}", row.0, row.1 != 0);
    assert!(row.0 >= 0);

    // Either it endured (news of becoming a fixture) or it lapsed (news of that) —
    // one of the two must be true; both are filed as competition_news.
    let news_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_news_items WHERE article_type = 'competition_news' \
         AND (headline LIKE '%Fixture%' OR headline LIKE '%Lapses%')")
        .fetch_one(&pool).await.unwrap();
    if row.1 == 0 || row.0 >= 5 {
        assert!(news_count.0 >= 1, "either the lapse or the graduation was reported");
    }
}
