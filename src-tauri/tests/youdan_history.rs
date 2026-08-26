//! The real 1867 Youdan Cup, written into the record books as history rather than
//! played — Hallam over Norfolk on rouges, 5 March 1867 at Bramall Lane.

use saturday_at_three::fsim_bridge;
use sqlx::sqlite::SqlitePoolOptions;

const REAL_DB: &str = "D:/projects/saturday at three/Sheffield1867.db";

#[tokio::test]
async fn the_1867_youdan_cup_is_seeded_as_history() {
    let dst = std::env::temp_dir().join("fsim_youdan_test.db");
    std::fs::copy(REAL_DB, &dst).expect("copy");
    let pool = SqlitePoolOptions::new().connect(&format!("sqlite:{}", dst.display())).await.unwrap();

    let seeded = fsim_bridge::seed_1867_youdan_cup_history(&pool).await.expect("seed");
    assert!(seeded, "the historical record is written");

    // Idempotent — seeding twice does not duplicate it.
    let again = fsim_bridge::seed_1867_youdan_cup_history(&pool).await.unwrap();
    assert!(!again, "already written; not repeated");
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_competition_winners WHERE competition_name = 'Youdan Cup' AND season = 1867")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(count.0, 1);

    // The winner is Hallam, over Norfolk, on rouges.
    let row: (String, String, String) = sqlx::query_as(
        "SELECT winner_club_id, runner_up_club_id, final_score FROM sheffield_competition_winners \
         WHERE competition_name = 'Youdan Cup' AND season = 1867")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(row.0, "hallam-fc");
    assert_eq!(row.1, "norfolk-fc");
    assert!(row.2.contains("rouges"));

    // The newspaper carries the story, dated the day after the final.
    let article: (String, String) = sqlx::query_as(
        "SELECT headline, edition_date FROM sheffield_newspaper_archive WHERE article_type = 'historical'")
        .fetch_one(&pool).await.unwrap();
    assert!(article.0.contains("Youdan"));
    assert_eq!(article.1, "1867-03-06");
}
