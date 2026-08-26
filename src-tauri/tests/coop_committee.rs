//! Two period institutions, in the app: the committee's standing with a club
//! (reviewed each season, a benefactor won or no-confidence moved) and the
//! Co-operative Society (join, buy goods, earn the divi). On a copy of the DB.

use saturday_at_three::fsim_bridge;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Row;

const REAL_DB: &str = "D:/projects/saturday at three/Sheffield1867.db";

async fn db(tag: &str) -> sqlx::SqlitePool {
    let dst = std::env::temp_dir().join(format!("fsim_inst_{tag}.db"));
    std::fs::copy(REAL_DB, &dst).expect("copy");
    SqlitePoolOptions::new().connect(&format!("sqlite:{}", dst.display())).await.unwrap()
}

#[tokio::test]
async fn committee_review_moves_standing_and_files_news() {
    let pool = db("committee").await;
    let club = "cemetery-road-church-fc";

    // A champion's season lifts the committee; a wooden-spoon season sinks it.
    let good = fsim_bridge::review_committee_season(&pool, club, 1, 16, 1867, "1867-10-26").await.unwrap();
    let after_good = fsim_bridge::committee_for(&pool, club).await.standing;
    assert!(after_good > 5250, "topping the table pleases the members ({after_good})");
    let _ = good;

    let pool2 = db("committee2").await;
    let _ = fsim_bridge::review_committee_season(&pool2, club, 16, 16, 1867, "1867-10-26").await.unwrap();
    let after_bad = fsim_bridge::committee_for(&pool2, club).await.standing;
    assert!(after_bad < 5250, "finishing bottom sours them ({after_bad})");

    // A committee news item was filed either way.
    let n: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_news_items WHERE article_type = 'board_confidence' AND publish_date = '1867-10-26'")
        .fetch_one(&pool).await.unwrap();
    assert!(n.0 >= 1, "the committee's review reached the inbox");
}

#[tokio::test]
async fn coop_society_join_buy_and_dividend() {
    let pool = db("coop").await;
    let club = "cemetery-road-church-fc";

    // No society before 1868.
    assert!(fsim_bridge::coop_status(&pool, 1867).await.is_none());
    let s = fsim_bridge::coop_status(&pool, 1875).await.expect("founded by 1875");
    assert!(s.membership > 50, "the society has grown by 1875");

    // Join, buy kit and a ball (divi-eligible) and goalposts (not).
    fsim_bridge::coop_join(&pool, club, "Cemetery Road Church FC", "1875-06-01").await.unwrap();
    fsim_bridge::coop_purchase(&pool, club, fsim_core::coop::Purchase::Kit, "1875-06-01").await.unwrap();
    fsim_bridge::coop_purchase(&pool, club, fsim_core::coop::Purchase::Ball, "1875-06-01").await.unwrap();
    fsim_bridge::coop_purchase(&pool, club, fsim_core::coop::Purchase::Goalposts, "1875-06-01").await.unwrap();

    // The divi is paid on the eligible spend (kit + ball = 37s), not the goalposts.
    let divi = fsim_bridge::coop_pay_dividend(&pool, club, 1875).await.unwrap();
    assert!(divi > 0, "a dividend is earned ({divi}s)");
    let earned: (i64,) = sqlx::query_as(
        "SELECT dividend_earned FROM sheffield_cooperative_memberships WHERE club_id = ?")
        .bind(club).fetch_one(&pool).await.unwrap();
    assert_eq!(earned.0 as u32, divi, "the divi is credited to the membership");
    println!("co-op 1875: {} members, {:.1}% divi → {}s back", s.membership, s.dividend_rate_x10 as f64 / 10.0, divi);
}
