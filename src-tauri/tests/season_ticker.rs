//! SIMULATION MODE, headless: a whole 1867 season ticks through `advance_day`
//! day by day — matchdays resolving through the real engine, the inbox filling
//! with round-ups, and the member clubs' assembly voting on the laws at the
//! close. Uses the app's own new-game flow (master DB copied to the temp DB), so
//! the real database is never touched. This loop is exactly what a "Continue /
//! Holiday" button in the UI would drive.

use chrono::Utc;
use saturday_at_three::game::{EventType, GameEvent, GameState, Match};
use saturday_at_three::{commands, database};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::Row;

const TEMP_DB: &str = "D:/projects/Saturday at Three/sheffield1867_temp.db";
const SEASON_END: &str = "1867-10-26";

#[tokio::test]
async fn a_season_ticks_through_matchdays_inbox_and_assembly() {
    // The app's own new-game step: copy the master DB to the temp game DB.
    database::init_new_game().await.expect("init temp db");
    let pool = SqlitePoolOptions::new()
        .connect(&format!("sqlite:{}", TEMP_DB))
        .await
        .expect("open temp db");

    // Eight first-team clubs that have footballers.
    let clubs: Vec<(String,)> = sqlx::query_as(
        "SELECT lc.club_id FROM sheffield_league_clubs lc \
         JOIN sheffield_clubs c ON c.id = lc.club_id \
         WHERE c.is_reserve_team = 0 AND EXISTS \
           (SELECT 1 FROM sheffield_footballers f WHERE f.club_id = lc.club_id) \
         LIMIT 8",
    )
    .fetch_all(&pool).await.expect("clubs");
    assert_eq!(clubs.len(), 8);
    let ids: Vec<String> = clubs.into_iter().map(|c| c.0).collect();

    // The season calendar: league config so the assembly convenes on Oct 26.
    // (Clear any pre-existing 1867 config so ours is authoritative.)
    sqlx::query("DELETE FROM sheffield_league_config WHERE season_year = 1867")
        .execute(&pool).await.expect("clear config");
    sqlx::query(
        "INSERT OR REPLACE INTO sheffield_league_config (id, season_year, season_start_date, season_end_date) \
         VALUES ('ticker-config', 1867, '1867-09-07', ?)",
    )
    .bind(SEASON_END).execute(&pool).await.expect("config");

    // Fixtures: 8 clubs, single round-robin over 7 Saturdays (circle method).
    let saturdays = ["1867-09-07", "1867-09-14", "1867-09-21", "1867-09-28",
                     "1867-10-05", "1867-10-12", "1867-10-19"];
    let mut matches = Vec::new();
    let mut events = Vec::new();
    for (r, date) in saturdays.iter().enumerate() {
        let mut rot: Vec<usize> = (1..8).collect();
        rot.rotate_left(r % 7);
        let pairs = [(0usize, rot[0]), (rot[1], rot[6]), (rot[2], rot[5]), (rot[3], rot[4])];
        for (mi, (h, a)) in pairs.iter().enumerate() {
            let mid = format!("tick-r{r}-m{mi}");
            matches.push(Match {
                id: mid.clone(),
                gameweek: (r + 1) as u8,
                home_team_id: ids[*h].clone(),
                away_team_id: ids[*a].clone(),
                home_score: None,
                away_score: None,
                date: date.to_string(),
                played: false,
            });
            events.push(GameEvent {
                id: format!("ev-{mid}"),
                date: date.to_string(),
                event_type: EventType::Match { match_id: mid, is_user_team: false },
                requires_user_action: false,
                processed: false,
                result: None,
            });
        }
    }

    // A spectator save: the "manager" runs no club, so every day auto-resolves —
    // pure simulation mode.
    let mut game = GameState {
        id: "season-ticker-test".into(),
        season: 1867,
        current_gameweek: 1,
        current_date: "1867-09-06".into(),
        user_club_id: "spectator".into(),
        game_mode: Some("sheffield_and_hallamshire_league".into()),
        clubs: vec![],
        matches,
        players: vec![],
        standings: vec![],
        pending_events: events,
        processed_events: vec![],
        created_at: Utc::now(),
        updated_at: Utc::now(),
        loaded_from_file: None,
    };

    // Tick, day by day, to the season's close.
    for _ in 0..60 {
        let before = game.current_date.clone();
        commands::advance_day(game.clone()).await.expect("advance_day");
        game = commands::get_current_game().await.expect("refetch");
        assert_ne!(game.current_date, before, "the calendar advanced");
        if game.current_date.as_str() >= SEASON_END {
            break;
        }
    }
    assert!(game.current_date.as_str() >= SEASON_END, "reached the season's end");

    // 1. Every fixture resolved, in the save itself.
    let tick: Vec<&Match> = game.matches.iter().filter(|m| m.id.starts_with("tick-")).collect();
    assert_eq!(tick.len(), 28);
    assert!(tick.iter().all(|m| m.played && m.home_score.is_some()), "all matchdays resolved");

    // 2. The engine really played them: persisted rows, rouges in evidence.
    let n: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_matches WHERE id LIKE 'tick-%' AND played = 1")
        .fetch_one(&pool).await.expect("rows");
    assert_eq!(n.0, 28, "all matches persisted as played");
    let rouges: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(home_rouges + away_rouges), 0) FROM sheffield_matches WHERE id LIKE 'tick-%'")
        .fetch_one(&pool).await.expect("rouges");
    assert!(rouges.0 > 0, "a season under 1867 laws produces rouges (engine, not fallback)");

    // 3. Every match has its ticker, framed kickoff to full-time.
    let ft: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_match_events WHERE match_id LIKE 'tick-%' AND event_type = 'fulltime'")
        .fetch_one(&pool).await.expect("fulltimes");
    assert_eq!(ft.0, 28, "a full-time line per match");

    // 4. The inbox filled week by week: a round-up per matchday.
    let roundups: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_news_items WHERE article_type = 'match_report' \
         AND publish_date BETWEEN '1867-09-07' AND '1867-10-19'")
        .fetch_one(&pool).await.expect("roundups");
    assert!(roundups.0 >= 7, "a results round-up for each of the seven matchdays");

    // 5. The assembly convened at the close and next season's laws exist.
    let rules: (String,) = sqlx::query_as(
        "SELECT ruleset_json FROM sheffield_rules_history WHERE season_year = 1868")
        .fetch_one(&pool).await.expect("1868 laws written");
    let _parsed: fsim_core::Ruleset = serde_json::from_str(&rules.0).expect("engine-shaped laws");
    let laws: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_news_items WHERE article_type = 'law_change' AND publish_date = ?")
        .bind(SEASON_END).fetch_one(&pool).await.expect("law items");
    assert!(laws.0 >= 1, "the assembly's outcome reached the inbox");

    // The story, for the log: the final week of the inbox and one match's ticker.
    println!("\n──── season complete: {} matches, {} rouges, laws of 1868 written ────",
             n.0, rouges.0);
    let inbox: Vec<(String, String, String)> = sqlx::query(
        "SELECT publish_date, article_type, headline FROM sheffield_news_items \
         ORDER BY publish_date DESC LIMIT 6")
        .fetch_all(&pool).await.expect("inbox")
        .iter().map(|r| (r.get(0), r.get(1), r.get(2))).collect();
    for (d, t, h) in inbox.iter().rev() {
        println!("  {d}  {t:<14} {h}");
    }
    println!("──── one afternoon's ticker (tick-r0-m0) ────");
    let ticker: Vec<(String,)> = sqlx::query_as(
        "SELECT description FROM sheffield_match_events WHERE match_id = 'tick-r0-m0' ORDER BY minute")
        .fetch_all(&pool).await.expect("ticker");
    for (line,) in &ticker {
        println!("  {line}");
    }
}
