//! Club identity, rivalries and your own name — wired to the census. Classifies
//! what stands behind each side, finds its natural rivals, and tracks the
//! secretary's reputation across seasons. On a copy of the real database.

use saturday_at_three::fsim_bridge;
use sqlx::sqlite::SqlitePoolOptions;

const REAL_DB: &str = "D:/projects/saturday at three/Sheffield1867.db";

async fn db(tag: &str) -> sqlx::SqlitePool {
    let dst = std::env::temp_dir().join(format!("fsim_ident_{tag}.db"));
    std::fs::copy(REAL_DB, &dst).expect("copy");
    SqlitePoolOptions::new().connect(&format!("sqlite:{}", dst.display())).await.unwrap()
}

#[tokio::test]
async fn clubs_are_classified_by_name_and_by_trade() {
    let pool = db("class").await;
    // Names carry the institution: a church side, a works side, a gentlemen's club.
    let church = fsim_bridge::classify_patronage(&pool, "x", "Attercliffe (Christ Church)").await;
    let works = fsim_bridge::classify_patronage(&pool, "x", "Atkin Brothers FC").await;
    let gents = fsim_bridge::classify_patronage(&pool, "x", "Artillery & Hallamshire FC").await;
    let pub_side = fsim_bridge::classify_patronage(&pool, "x", "Royal Oak FC").await;
    assert_eq!(format!("{church:?}"), "Church");
    assert_eq!(format!("{works:?}"), "Works");
    assert_eq!(format!("{gents:?}"), "Gentlemen");
    assert_eq!(format!("{pub_side:?}"), "PublicHouse");

    // A real club classifies and is remembered.
    let p1 = fsim_bridge::patronage_for(&pool, "cemetery-road-church-fc").await;
    let p2 = fsim_bridge::patronage_for(&pool, "cemetery-road-church-fc").await;
    assert_eq!(format!("{p1:?}"), format!("{p2:?}"), "patronage is stored, not re-guessed");
    println!("Cemetery Road Church FC is {}", p1.label());

    // Every patronage carries a constraint — none is a free lunch.
    assert!(!p1.constraint().is_empty());
}

#[tokio::test]
async fn rivalries_are_discovered_and_stoked() {
    let pool = db("rival").await;
    let club = "cemetery-road-church-fc";
    let found = fsim_bridge::discover_rivalries(&pool, club).await.expect("discover");
    println!("found {found} natural rivals for {club}");

    // Discovering twice does not duplicate them.
    let again = fsim_bridge::discover_rivalries(&pool, club).await.expect("again");
    assert_eq!(again, 0, "rivalries are not duplicated");

    if found > 0 {
        let row: (String, String) = sqlx::query_as(
            "SELECT club_a, club_b FROM sheffield_rivalries LIMIT 1").fetch_one(&pool).await.unwrap();
        let before = fsim_bridge::rivalry_between(&pool, &row.0, &row.1).await.unwrap();
        // A close match that mattered stokes the quarrel.
        fsim_bridge::record_rivalry_meeting(&pool, &row.0, &row.1, 1, true).await.unwrap();
        let after = fsim_bridge::rivalry_between(&pool, &row.0, &row.1).await.unwrap();
        assert!(after.intensity > before.intensity, "a derby stokes the rivalry");
        assert_eq!(after.meetings, before.meetings + 1);
        assert!(after.crowd_pct() > 100, "a derby swells the gate");
    }
}

#[tokio::test]
async fn a_secretary_builds_a_name_over_seasons() {
    let pool = db("sec").await;
    let club = "cemetery-road-church-fc";
    let start = fsim_bridge::secretary_for(&pool).await.reputation;

    // Four championship seasons build a considerable name.
    for season in 1867..1871 {
        fsim_bridge::secretary_after_season(&pool, club, 1, 16, true, 7000, "1867-10-26", season)
            .await.expect("season");
    }
    let s = fsim_bridge::secretary_for(&pool).await;
    assert!(s.reputation > start, "winning builds a name ({start} → {})", s.reputation);
    assert_eq!(s.honours, 4);
    assert_eq!(s.seasons_served, 4);
    println!("the secretary is now {} ({} rep, {} honours)", s.standing_words(), s.reputation, s.honours);
}
