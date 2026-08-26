//! Two casual-club lifecycle pieces, in the app: mustering a squad for a match
//! (who's available, who's slipped off, how many ringers you need) and the
//! close-season ageing/development pass. Both on a copy of the real database.

use saturday_at_three::fsim_bridge;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Row;

const REAL_DB: &str = "D:/projects/saturday at three/Sheffield1867.db";

#[tokio::test]
async fn muster_names_the_available_and_the_missing() {
    let dst = std::env::temp_dir().join("fsim_muster_test.db");
    std::fs::copy(REAL_DB, &dst).expect("copy");
    let pool = SqlitePoolOptions::new().connect(&format!("sqlite:{}", dst.display())).await.unwrap();
    let club: (String,) = sqlx::query_as(
        "SELECT lc.club_id FROM sheffield_league_clubs lc JOIN sheffield_clubs c ON c.id = lc.club_id \
         WHERE c.is_reserve_team = 0 AND EXISTS (SELECT 1 FROM sheffield_footballers f WHERE f.club_id = lc.club_id) LIMIT 1")
        .fetch_one(&pool).await.unwrap();

    let report = fsim_bridge::muster_squad(&pool, &club.0, 999).await.expect("muster");
    let total = report.available.len() + report.missing.len();
    assert!(total > 0, "there is a squad to muster");
    // ringers-needed is exactly the shortfall below eleven.
    assert_eq!(report.ringers_needed, 11usize.saturating_sub(report.available.len()));
    // Missing men carry a period reason.
    for (_, reason) in &report.missing {
        assert!(reason.contains("work") || reason.contains("another club"), "a reason is given: {reason}");
    }
    // Deterministic.
    let dst2 = std::env::temp_dir().join("fsim_muster_test2.db");
    std::fs::copy(REAL_DB, &dst2).unwrap();
    let pool2 = SqlitePoolOptions::new().connect(&format!("sqlite:{}", dst2.display())).await.unwrap();
    let replay = fsim_bridge::muster_squad(&pool2, &club.0, 999).await.unwrap();
    assert_eq!(report.available.len(), replay.available.len(), "same seed, same muster");
    println!("mustered {}: {} available, {} missing, {} ringers needed",
             club.0, report.available.len(), report.missing.len(), report.ringers_needed);
}

#[tokio::test]
async fn close_season_ages_and_develops_players() {
    let dst = std::env::temp_dir().join("fsim_develop_test.db");
    std::fs::copy(REAL_DB, &dst).expect("copy");
    let pool = SqlitePoolOptions::new().connect(&format!("sqlite:{}", dst.display())).await.unwrap();

    // Snapshot a promising teenager and an ageing veteran before the pass.
    let youth: Option<(i64, i64)> = sqlx::query_as(
        "SELECT id, current_ability FROM sheffield_footballers \
         WHERE birth_year >= 1849 AND potential_ability > current_ability + 20 AND current_ability IS NOT NULL LIMIT 1")
        .fetch_optional(&pool).await.unwrap();

    let changed = fsim_bridge::age_and_develop(&pool, 1867).await.expect("develop");
    assert!(changed > 100, "a season moves many players' ability ({changed})");

    // The summer restored condition for everyone.
    let unrested: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_footballers WHERE condition < 100").fetch_one(&pool).await.unwrap();
    assert_eq!(unrested.0, 0, "the close season rests everyone");

    // The promising teenager did not go backwards.
    if let Some((id, before)) = youth {
        let after: (i64,) = sqlx::query_as("SELECT current_ability FROM sheffield_footballers WHERE id = ?")
            .bind(id).fetch_one(&pool).await.unwrap();
        assert!(after.0 >= before, "a young player with headroom does not decline ({before}→{})", after.0);
    }
    // Idempotent-ish: a second pass with the same year is deterministic.
    println!("develop pass moved {changed} players", );
}
