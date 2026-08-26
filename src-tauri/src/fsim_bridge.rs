//! Bridge into the football simulation engine (`fsim_core`): converts database
//! footballers into engine squads, and runs the daily driver that plays scheduled
//! friendlies through the real match engine on the day they fall due — writing the
//! score to `sheffield_matches` and a result item into `sheffield_news_items`.
//!
//! Determinism: every roll is seeded from a stable database id, so replaying a
//! save reproduces identical results (no `thread_rng` anywhere).

use fsim_core::availability;
use fsim_core::commitment::{self, Factors};
use fsim_core::coop;
use fsim_core::governance::{self, Committee, Verdict};
use fsim_core::friendly::{self, FriendlyOutcome};
use fsim_core::negotiation::FootballCode;
use fsim_core::phrasebook::Phrasebook;
use fsim_core::sponsored_cup;
use fsim_core::{agm, commentary, news, season, Player, PlayerState, Position, Qualities, Rng, Ruleset, Squad};
use sqlx::{Row, SqlitePool};
use std::sync::OnceLock;

// ---------------------------------------------------------------------------
// The census layer: a footballer is a real Sheffield man. His willingness to
// turn out (reliability) and to stay (loyalty) are composed from his trade, his
// household, his neighbours and his temperament — all in the database.
// ---------------------------------------------------------------------------

/// How free a man's trade leaves him for football — a clerk or schoolmaster is at
/// liberty on a Tuesday; a furnaceman, grinder or miner is not; a shopkeeper is
/// tied to his trade hours. Keyed off the census `profession` string.
pub fn occupation_freedom(profession: &str) -> u8 {
    let p = profession.to_ascii_lowercase();
    let has = |ws: &[&str]| ws.iter().any(|w| p.contains(w));
    if has(&["clerk", "schoolmaster", "teacher", "clergy", "minister", "gentleman",
             "professional", "solicitor", "surgeon", "accountant", "agent", "manager",
             "merchant", "student", "scholar", "book"]) { 85 }
    else if has(&["shopkeeper", "grocer", "baker", "butcher", "tailor", "draper",
                  "publican", "innkeeper", "victualler", "shoemaker", "printer",
                  "clerk", "warehouse"]) { 55 }
    else if has(&["labour", "furnace", "grinder", "miner", "collier", "file", "forge",
                  "moulder", "smith", "cutler", "steel", "iron", "puddler", "founder",
                  "navvy", "mason", "bricklayer", "servant", "porter"]) { 30 }
    else { 50 }
}

/// Where a club is rooted: the parish most of its men live in, and their
/// registration district. A player from that parish lives by the ground; one from
/// another district is across the town. (The census has no coordinates, but its
/// parish and district data are near-complete — a truer period measure anyway.)
pub async fn club_locality(pool: &SqlitePool, club_id: &str) -> (Option<String>, Option<String>) {
    let parish: Option<(Option<String>,)> = sqlx::query_as(
        "SELECT p.civil_parish FROM sheffield_footballers f \
         JOIN sheffield_people p ON p.unique_id = f.person_id \
         WHERE f.club_id = ? AND p.civil_parish IS NOT NULL \
         GROUP BY p.civil_parish ORDER BY COUNT(*) DESC LIMIT 1")
        .bind(club_id).fetch_optional(pool).await.ok().flatten();
    let district: Option<(Option<String>,)> = sqlx::query_as(
        "SELECT p.registration_district FROM sheffield_footballers f \
         JOIN sheffield_people p ON p.unique_id = f.person_id \
         WHERE f.club_id = ? AND p.registration_district IS NOT NULL \
         GROUP BY p.registration_district ORDER BY COUNT(*) DESC LIMIT 1")
        .bind(club_id).fetch_optional(pool).await.ok().flatten();
    (parish.and_then(|p| p.0), district.and_then(|d| d.0))
}

/// Compose a footballer's [`Factors`] from the census. `row` is a footballer row
/// carrying `person_id`, `profession`, `street_address`, `club_id` and the
/// temperament attributes; `locality` is his club's parish/district (see
/// [`club_locality`]), against which his own is measured for proximity.
pub async fn derive_factors(
    pool: &SqlitePool,
    row: &sqlx::sqlite::SqliteRow,
    playing_time: u8,
    club_standing: u8,
    locality: (Option<&str>, Option<&str>),
) -> Factors {
    let person_id: Option<i64> = row.try_get("person_id").ok().flatten();
    let profession: String = row.try_get::<Option<String>, _>("profession").ok().flatten().unwrap_or_default();
    let street: String = row.try_get::<Option<String>, _>("street_address").ok().flatten().unwrap_or_default();
    let club_id: String = row.try_get::<Option<String>, _>("club_id").ok().flatten().unwrap_or_default();
    // Temperament attributes are on the 1–20 CM scale; ×5 → 0–100.
    let attr = |c: &str| -> i64 { row.try_get::<Option<i64>, _>(c).ok().flatten().unwrap_or(10) };
    let temperament = ((attr("loyalty") + attr("temperament") + attr("professionalism")
        + attr("determination")) * 5 / 4).clamp(0, 100) as u8;

    // Family ties: a household head with a wife to keep is less free than a young
    // single man. Read his census relation and whether a wife shares his household.
    let mut family_ties = 60u8;
    let mut proximity = 45u8; // unknown whereabouts — assume a fair walk
    if let Some(pid) = person_id {
        let rel: Option<(Option<String>, Option<i64>, Option<i64>, Option<String>, Option<String>)> =
            sqlx::query_as(
            "SELECT census_relation, census_household_schedule, census_piece, \
             civil_parish, registration_district \
             FROM sheffield_people WHERE unique_id = ? OR id = ? LIMIT 1")
            .bind(pid).bind(pid).fetch_optional(pool).await.ok().flatten();
        if let Some((_, _, _, ref parish, ref district)) = rel {
            // Proximity: the club's own parish is on the doorstep; its district a
            // walk; anywhere else is across the town.
            proximity = match (parish.as_deref(), locality.0) {
                (Some(p), Some(cp)) if p.eq_ignore_ascii_case(cp) => 90,
                _ => match (district.as_deref(), locality.1) {
                    (Some(d), Some(cd)) if d.eq_ignore_ascii_case(cd) => 62,
                    _ => 35,
                },
            };
        }
        if let Some((relation, sched, piece, _, _)) = rel {
            let r = relation.unwrap_or_default().to_ascii_lowercase();
            if r.contains("head") {
                // Is there a wife in the same household? A family to provide for.
                let wife: Option<(i64,)> = sqlx::query_as(
                    "SELECT COUNT(*) FROM sheffield_people WHERE LOWER(census_relation)='wife' \
                     AND census_household_schedule = ? AND census_piece = ?")
                    .bind(sched).bind(piece).fetch_optional(pool).await.ok().flatten();
                family_ties = if wife.map_or(false, |w| w.0 > 0) { 35 } else { 55 };
            } else if r.contains("son") || r.contains("lodger") || r.contains("boarder") || r.contains("visitor") {
                family_ties = 85; // a free young man
            }
        }
    }

    // Camaraderie: teammates who are his neighbours — men from the same club living
    // on his street turn out together and bind him to the side.
    let mut camaraderie = 35u8;
    if !street.is_empty() {
        let mates: Option<(i64,)> = sqlx::query_as(
            "SELECT COUNT(*) FROM sheffield_footballers WHERE club_id = ? AND street_address = ? AND person_id <> ?")
            .bind(&club_id).bind(&street).bind(person_id.unwrap_or(-1))
            .fetch_optional(pool).await.ok().flatten();
        camaraderie = (35 + mates.map_or(0, |m| m.0) as i64 * 20).clamp(0, 100) as u8;
    }

    Factors {
        occupation_freedom: occupation_freedom(&profession),
        proximity,
        family_ties,
        temperament,
        playing_time,
        camaraderie,
        club_standing,
        inducement: 0, // the shamateurism lever — not yet built
    }
}

/// Make sure the per-footballer live-state columns exist on the active game DB
/// (condition/morale/form and a games count). Idempotent — SQLite has no
/// ADD COLUMN IF NOT EXISTS, so a duplicate-column error is expected and ignored.
pub async fn ensure_state_columns(pool: &SqlitePool) {
    for (col, default) in [("condition", 100), ("morale", 0), ("form", 0), ("games_played", 0)] {
        let _ = sqlx::query(&format!(
            "ALTER TABLE sheffield_footballers ADD COLUMN {col} INTEGER DEFAULT {default}"))
            .execute(pool).await;
    }
}

/// The outcome of an arranged training session, for the day's report.
pub struct TrainingReport {
    pub present: Vec<String>,
    pub absent: Vec<String>,
}

/// Arrange a training session for a club and see who turns up. Each man's odds of
/// appearing are composed from the census (his trade, household, neighbours,
/// temperament); those who come sharpen their condition and lift their spirits.
/// Deterministic on `seed`. Persists the condition/morale changes.
pub async fn hold_training(pool: &SqlitePool, club_id: &str, seed: u64) -> Result<TrainingReport, sqlx::Error> {
    ensure_state_columns(pool).await;
    let rows = sqlx::query(
        "SELECT id, person_id, first_name, surname, profession, street_address, club_id, \
         loyalty, temperament, professionalism, determination, \
         COALESCE(morale, 0) AS morale, COALESCE(games_played, 0) AS games_played \
         FROM sheffield_footballers WHERE club_id = ?")
        .bind(club_id).fetch_all(pool).await?;

    let mut rng = Rng::seed(seed);
    let (parish, district) = club_locality(pool, club_id).await;
    let locality = (parish.as_deref(), district.as_deref());
    let mut report = TrainingReport { present: Vec::new(), absent: Vec::new() };
    for row in &rows {
        let id: i64 = row.try_get("id").unwrap_or(0);
        let morale: i64 = row.try_get("morale").unwrap_or(0);
        let games: i64 = row.try_get("games_played").unwrap_or(0);
        let name = format!("{} {}",
            row.try_get::<Option<String>, _>("first_name").ok().flatten().unwrap_or_default().trim(),
            row.try_get::<Option<String>, _>("surname").ok().flatten().unwrap_or_default().trim());
        let playing_time = (games * 8).clamp(0, 100) as u8;
        let factors = derive_factors(pool, row, playing_time, 50, locality).await;
        let commit = commitment::assess(&factors);

        if availability::attends_training(commit.reliability, morale.clamp(-100, 100) as i8, &mut rng) {
            sqlx::query(
                "UPDATE sheffield_footballers SET condition = MIN(100, COALESCE(condition,100) + 4), \
                 morale = MAX(-100, MIN(100, COALESCE(morale,0) + 1)) WHERE id = ?")
                .bind(id).execute(pool).await?;
            report.present.push(name.trim().to_string());
        } else {
            report.absent.push(name.trim().to_string());
        }
    }
    Ok(report)
}

/// The real 1867 Youdan Cup — the world's first football knockout tournament,
/// sponsored by Thomas Youdan of the Alexandra Music Hall. Twelve Sheffield clubs
/// entered; the final, at Bramall Lane on 5 March 1867, ended goalless and was
/// decided on rouges — Hallam over Norfolk. Seeded once as a historical record: it
/// happened before the game begins, so it is written into the record books rather
/// than played. Idempotent.
pub async fn seed_1867_youdan_cup_history(pool: &SqlitePool) -> Result<bool, sqlx::Error> {
    let already: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_competition_winners WHERE competition_name = 'Youdan Cup' AND season = 1867")
        .fetch_one(pool).await?;
    if already.0 > 0 { return Ok(false); }

    let winner: Option<(String,)> = sqlx::query_as("SELECT id FROM sheffield_clubs WHERE id = 'hallam-fc'")
        .fetch_optional(pool).await?;
    let runner_up: Option<(String,)> = sqlx::query_as("SELECT id FROM sheffield_clubs WHERE id = 'norfolk-fc'")
        .fetch_optional(pool).await?;
    let (winner_id, runner_id) = match (winner, runner_up) {
        (Some(w), Some(r)) => (w.0, r.0),
        _ => return Ok(false), // the clubs don't exist in this database — nothing to record
    };

    sqlx::query(
        "INSERT INTO sheffield_competition_winners \
         (competition_name, season, winner_club_id, runner_up_club_id, final_score) \
         VALUES ('Youdan Cup', 1867, ?, ?, '0-0 (won on rouges)')")
        .bind(&winner_id).bind(&runner_id).execute(pool).await?;

    sqlx::query(
        "INSERT INTO sheffield_newspaper_archive \
         (publication_name, edition_date, article_type, headline, body, clubs_mentioned, season) \
         VALUES ('The Sheffield Independent', '1867-03-06', 'historical', ?, ?, ?, 1867)")
        .bind("Hallam Carry Off the Youdan Cup")
        .bind(
            "The final of Mr. Youdan's Cup, the first tournament of its kind ever contested, was \
             played on Tuesday at Bramall Lane before a large concourse of spectators. Hallam and \
             Norfolk could not be separated by a goal in ninety minutes of determined and, at times, \
             ill-tempered play, and the destination of the trophy was left to the rouges — Hallam's \
             greater number of touch-downs behind the flags carrying the day. Mr. Youdan, proprietor \
             of the Alexandra Music Hall, presented the Cup to the Hallam captain amid warm applause, \
             remarking that he trusted the competition would become an annual fixture of the Sheffield \
             football calendar.")
        .bind(format!("[\"{winner_id}\",\"{runner_id}\"]"))
        .execute(pool).await?;
    Ok(true)
}

/// Generate a season's league fixtures — a proper double round-robin by the circle
/// method, so every club plays exactly once each matchday, played out on
/// consecutive Saturdays from the season's opening. Deterministic (the draw is
/// seeded from the division and season, so a replayed save gets the same card).
/// Idempotent: a season already drawn is left alone. Returns fixtures created.
pub async fn generate_league_fixtures(
    pool: &SqlitePool, season: i64, season_start: &str,
) -> Result<u32, sqlx::Error> {
    // Already drawn? Leave it be.
    let existing: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_matches WHERE season = ? AND gameweek > 0")
        .bind(season).fetch_one(pool).await?;
    if existing.0 > 0 { return Ok(0); }

    let start = chrono::NaiveDate::parse_from_str(season_start, "%Y-%m-%d")
        .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
    let divisions: Vec<(String,)> = sqlx::query_as(
        "SELECT DISTINCT lc.division_id FROM sheffield_league_clubs lc \
         JOIN sheffield_clubs c ON c.id = lc.club_id WHERE c.is_reserve_team = 0")
        .fetch_all(pool).await?;

    let mut created = 0u32;
    let mut tx = pool.begin().await?;
    for (division,) in &divisions {
        let clubs: Vec<(String,)> = sqlx::query_as(
            "SELECT lc.club_id FROM sheffield_league_clubs lc \
             JOIN sheffield_clubs c ON c.id = lc.club_id \
             WHERE lc.division_id = ? AND c.is_reserve_team = 0 ORDER BY lc.club_id")
            .bind(division).fetch_all(&mut *tx).await?;
        let mut ids: Vec<String> = clubs.into_iter().map(|c| c.0).collect();
        if ids.len() < 2 { continue; }

        // A deterministic shuffle of the draw, then the circle method.
        let mut rng = Rng::seed(seed_from(&format!("draw-{division}-{season}")));
        for i in (1..ids.len()).rev() {
            let j = rng.roll((i + 1) as u32) as usize;
            ids.swap(i, j);
        }
        let bye = ids.len() % 2 == 1;
        if bye { ids.push(String::new()); } // an odd division gives one club a rest
        let n = ids.len();
        let rounds = n - 1;

        for round in 0..rounds {
            for i in 0..n / 2 {
                let (a, b) = (ids[i].clone(), ids[n - 1 - i].clone());
                if a.is_empty() || b.is_empty() { continue; } // the bye
                // First half of the season, then the return fixture with the
                // ground reversed — matchday r and matchday r + rounds.
                for (leg, (home, away)) in [(0usize, (&a, &b)), (1, (&b, &a))] {
                    let matchday = round + leg * rounds;
                    let date = start + chrono::Duration::weeks(matchday as i64);
                    let id = format!("lg-{season}-{division}-{matchday}-{home}");
                    sqlx::query(
                        "INSERT OR IGNORE INTO sheffield_matches (id, gameweek, season, rule_year, \
                         home_club_id, away_club_id, home_score, away_score, home_rouges, away_rouges, \
                         played, match_date) VALUES (?, ?, ?, ?, ?, ?, 0, 0, 0, 0, 0, ?)")
                        .bind(&id).bind((matchday + 1) as i64).bind(season).bind(season)
                        .bind(home).bind(away).bind(date.format("%Y-%m-%d").to_string())
                        .execute(&mut *tx).await?;
                    created += 1;
                }
            }
            // Rotate all but the first club — the circle method.
            ids[1..].rotate_right(1);
        }
    }
    tx.commit().await?;
    Ok(created)
}

/// Muster the user's club for a match: who is available, who is not (and why),
/// and how many ringers must be found to make up an eleven. Each man's odds are
/// census-composed, as in training, but a real fixture pulls harder. Deterministic.
pub struct MusterReport {
    pub available: Vec<String>,
    pub missing: Vec<(String, String)>, // (name, reason)
    pub ringers_needed: usize,
    /// Men called up to fill the gaps — the reserve side first, then whoever can
    /// be found. Named so the manager knows who he is fielding.
    pub ringers: Vec<String>,
}

pub async fn muster_squad(pool: &SqlitePool, club_id: &str, seed: u64) -> Result<MusterReport, sqlx::Error> {
    ensure_state_columns(pool).await;
    let rows = sqlx::query(
        "SELECT id, person_id, first_name, surname, profession, street_address, club_id, \
         loyalty, temperament, professionalism, determination, \
         COALESCE(morale, 0) AS morale, COALESCE(games_played, 0) AS games_played \
         FROM sheffield_footballers WHERE club_id = ? ORDER BY current_ability DESC LIMIT 16")
        .bind(club_id).fetch_all(pool).await?;

    let mut rng = Rng::seed(seed);
    let (parish, district) = club_locality(pool, club_id).await;
    let locality = (parish.as_deref(), district.as_deref());
    let mut report = MusterReport { available: Vec::new(), missing: Vec::new(), ringers_needed: 0, ringers: Vec::new() };
    for row in &rows {
        let morale: i64 = row.try_get("morale").unwrap_or(0);
        let games: i64 = row.try_get("games_played").unwrap_or(0);
        let name = format!("{} {}",
            row.try_get::<Option<String>, _>("first_name").ok().flatten().unwrap_or_default().trim(),
            row.try_get::<Option<String>, _>("surname").ok().flatten().unwrap_or_default().trim())
            .trim().to_string();
        let factors = derive_factors(pool, row, (games * 8).clamp(0, 100) as u8, 50, locality).await;
        let commit = commitment::assess(&factors);
        match availability::match_availability(commit.reliability, commit.loyalty, morale.clamp(-100, 100) as i8, &mut rng) {
            availability::MatchAvailability::Available => report.available.push(name),
            availability::MatchAvailability::Busy =>
                report.missing.push((name, "detained by work or family".into())),
            availability::MatchAvailability::PlayingElsewhere =>
                report.missing.push((name, "gone to turn out for another club".into())),
        }
    }
    report.ringers_needed = 11usize.saturating_sub(report.available.len());

    // Short of an eleven? Call up the reserve side — and if they cannot fill it,
    // any willing man of the parish. This is how a short club took the field.
    if report.ringers_needed > 0 {
        let reserves = sqlx::query(
            "SELECT f.first_name, f.surname FROM sheffield_footballers f \
             JOIN sheffield_clubs c ON c.id = f.club_id \
             WHERE c.parent_club_id = ? ORDER BY f.current_ability DESC LIMIT ?")
            .bind(club_id).bind(report.ringers_needed as i64)
            .fetch_all(pool).await.unwrap_or_default();
        for r in &reserves {
            let n = format!("{} {}",
                r.try_get::<Option<String>, _>("first_name").ok().flatten().unwrap_or_default().trim(),
                r.try_get::<Option<String>, _>("surname").ok().flatten().unwrap_or_default().trim());
            report.ringers.push(format!("{} (reserves)", n.trim()));
        }
        // Still short — a ringer from the parish, as clubs did when desperate.
        let still = report.ringers_needed.saturating_sub(report.ringers.len());
        for i in 0..still {
            let outsider = sqlx::query(
                "SELECT first_name, surname FROM sheffield_footballers \
                 WHERE club_id IS NULL OR club_id = '' LIMIT 1 OFFSET ?")
                .bind((seed_from(club_id) as i64 + i as i64) % 500)
                .fetch_optional(pool).await.ok().flatten();
            let name = outsider.map(|r| format!("{} {}",
                r.try_get::<Option<String>, _>("first_name").ok().flatten().unwrap_or_default().trim(),
                r.try_get::<Option<String>, _>("surname").ok().flatten().unwrap_or_default().trim()))
                .unwrap_or_else(|| "a man of the parish".into());
            report.ringers.push(format!("{} (ringer)", name.trim()));
        }
    }
    Ok(report)
}

/// The close-season lifecycle: every footballer ages a year — the young rise
/// toward their potential, the peak hold, the veterans decline — and the summer's
/// rest restores condition. One pass over the club game's players, once a season.
/// (Adapts CM's development curves; ability is on the 1–200 scale.)
pub async fn age_and_develop(pool: &SqlitePool, season_year: i64) -> Result<u32, sqlx::Error> {
    ensure_state_columns(pool).await;
    let rows = sqlx::query(
        "SELECT id, current_ability, potential_ability, birth_year \
         FROM sheffield_footballers WHERE current_ability IS NOT NULL")
        .fetch_all(pool).await?;
    let mut rng = Rng::seed(seed_from(&format!("develop-{season_year}")));
    let mut changed = 0u32;
    let mut tx = pool.begin().await?;
    for row in &rows {
        let id: i64 = row.try_get("id").unwrap_or(0);
        let ca: i64 = row.try_get::<Option<i64>, _>("current_ability").ok().flatten().unwrap_or(0);
        let pa: i64 = row.try_get::<Option<i64>, _>("potential_ability").ok().flatten().unwrap_or(ca);
        let birth: i64 = row.try_get::<Option<i64>, _>("birth_year").ok().flatten().unwrap_or(season_year - 25);
        let age = (season_year - birth).clamp(15, 45) as u8;
        let new_ca = fsim_core::develop_aged(ca.clamp(0, 200) as u8, pa.clamp(0, 200) as u8, age, &mut rng) as i64;
        if new_ca != ca {
            sqlx::query("UPDATE sheffield_footballers SET current_ability = ? WHERE id = ?")
                .bind(new_ca).bind(id).execute(&mut *tx).await?;
            changed += 1;
        }
    }
    // The summer restores everyone; form fades toward level.
    sqlx::query("UPDATE sheffield_footballers SET condition = 100, form = form / 2").execute(&mut *tx).await?;
    tx.commit().await?;
    Ok(changed)
}

// ---------------------------------------------------------------------------
// Club identity: what institution stands behind each side. Read from its name
// (Christ Church, Atkin Brothers, the Royal Oak) and, failing that, from the
// trades its men actually follow.
// ---------------------------------------------------------------------------

use fsim_core::institution::{self, Patronage};
use fsim_core::rivalry::{self, Rivalry};
use fsim_core::secretary::Secretary;

async fn ensure_club_meta(pool: &SqlitePool) {
    let _ = sqlx::query(
        "CREATE TABLE IF NOT EXISTS sheffield_club_meta (club_id TEXT PRIMARY KEY, \
         patronage TEXT NOT NULL DEFAULT 'Independent', patron_trade INTEGER NOT NULL DEFAULT 0, \
         seasons INTEGER NOT NULL DEFAULT 0)")
        .execute(pool).await;
}

fn patronage_from_name(name: &str) -> Option<Patronage> {
    let n = name.to_ascii_lowercase();
    let has = |ws: &[&str]| ws.iter().any(|w| n.contains(w));
    if has(&["church", "chapel", "zion", "st ", "saints", "trinity", "christ", "wesley",
             "methodist", "bethel", "ebenezer", "sunday school"]) { return Some(Patronage::Church); }
    if has(&["arms", " inn", "hotel", "tavern", "royal oak", "crown", "red lion"]) { return Some(Patronage::PublicHouse); }
    if has(&["works", "mill", "brothers", "bros", "& sons", "foundry", "forge", "atlas", "baltic"])
        || n.contains("'s ") { return Some(Patronage::Works); }
    if has(&["wanderers", "albion", "college", "collegiate", "grammar", "regiment", "artillery",
             "hallamshire", "athenaeum"]) { return Some(Patronage::Gentlemen); }
    None
}

/// Classify a club by what stands behind it: its name first, then the trades its
/// men follow — a side of farm labourers is a village side whatever it calls
/// itself; a side that is three-quarters one industrial trade is a works side.
pub async fn classify_patronage(pool: &SqlitePool, club_id: &str, club_name: &str) -> Patronage {
    if let Some(p) = patronage_from_name(club_name) { return p; }
    // Fall back on the census: what do these men do for a living?
    let rows: Vec<(Option<String>,)> = sqlx::query_as(
        "SELECT profession FROM sheffield_footballers WHERE club_id = ? LIMIT 40")
        .bind(club_id).fetch_all(pool).await.unwrap_or_default();
    if rows.is_empty() { return Patronage::Independent; }
    let total = rows.len() as u32;
    let mut agricultural = 0u32;
    let mut industrial = 0u32;
    let mut genteel = 0u32;
    for (p,) in &rows {
        let p = p.clone().unwrap_or_default().to_ascii_lowercase();
        if p.contains("farm") || p.contains("agricultur") || p.contains("shepherd") || p.contains("dairy") {
            agricultural += 1;
        } else if p.contains("cutler") || p.contains("grinder") || p.contains("file") || p.contains("steel")
            || p.contains("forge") || p.contains("furnace") || p.contains("moulder") || p.contains("smith") {
            industrial += 1;
        } else if p.contains("gentleman") || p.contains("solicitor") || p.contains("surgeon")
            || p.contains("merchant") || p.contains("clerk") || p.contains("teacher") {
            genteel += 1;
        }
    }
    if agricultural * 100 / total >= 30 { Patronage::Rural }
    else if industrial * 100 / total >= 45 { Patronage::Works }
    else if genteel * 100 / total >= 50 { Patronage::Gentlemen }
    else { Patronage::Independent }
}

fn patronage_name(p: Patronage) -> &'static str {
    match p {
        Patronage::Works => "Works", Patronage::Church => "Church",
        Patronage::PublicHouse => "PublicHouse", Patronage::Gentlemen => "Gentlemen",
        Patronage::Rural => "Rural", Patronage::Independent => "Independent",
    }
}
fn patronage_of(s: &str) -> Patronage {
    match s {
        "Works" => Patronage::Works, "Church" => Patronage::Church,
        "PublicHouse" => Patronage::PublicHouse, "Gentlemen" => Patronage::Gentlemen,
        "Rural" => Patronage::Rural, _ => Patronage::Independent,
    }
}

/// The club's patronage, classifying and storing it the first time it is asked for.
pub async fn patronage_for(pool: &SqlitePool, club_id: &str) -> Patronage {
    ensure_club_meta(pool).await;
    if let Some((p,)) = sqlx::query_as::<_, (String,)>(
        "SELECT patronage FROM sheffield_club_meta WHERE club_id = ?")
        .bind(club_id).fetch_optional(pool).await.ok().flatten() {
        return patronage_of(&p);
    }
    let name: String = sqlx::query_as::<_, (String,)>("SELECT name FROM sheffield_clubs WHERE id = ?")
        .bind(club_id).fetch_optional(pool).await.ok().flatten().map(|r| r.0).unwrap_or_default();
    let p = classify_patronage(pool, club_id, &name).await;
    let _ = sqlx::query("INSERT OR REPLACE INTO sheffield_club_meta (club_id, patronage) VALUES (?, ?)")
        .bind(club_id).bind(patronage_name(p)).execute(pool).await;
    p
}

/// The close season may change what a club is — the works shuts, a magnate takes
/// a rising side up. Files the news and stores the new identity.
pub async fn season_patronage_event(
    pool: &SqlitePool, club_id: &str, standing: i32, date: &str, season: i64,
) -> Result<Option<institution::PatronageChange>, sqlx::Error> {
    let current = patronage_for(pool, club_id).await;
    let meta: Option<(i64, i64)> = sqlx::query_as(
        "SELECT patron_trade, seasons FROM sheffield_club_meta WHERE club_id = ?")
        .bind(club_id).fetch_optional(pool).await.ok().flatten();
    let (trade, seasons) = meta.unwrap_or((0, 0));
    let fortunes = institution::Fortunes {
        standing, patron_trade: trade.clamp(-100, 100) as i8, seasons: seasons.clamp(0, 255) as u8,
    };
    let mut rng = Rng::seed(seed_from(&format!("patronage-{club_id}-{season}")));
    let outcome = institution::patronage_event(current, &fortunes, &mut rng);
    // Another season under the present arrangement, whatever happens.
    let _ = sqlx::query("UPDATE sheffield_club_meta SET seasons = seasons + 1 WHERE club_id = ?")
        .bind(club_id).execute(pool).await;
    if let Some((new_p, change)) = outcome {
        let _ = sqlx::query(
            "UPDATE sheffield_club_meta SET patronage = ?, seasons = 0 WHERE club_id = ?")
            .bind(patronage_name(new_p)).bind(club_id).execute(pool).await;
        file_committee_news(pool, club_id, date, change.headline(), change.body()).await?;
        return Ok(Some(change));
    }
    Ok(None)
}

// ---------------------------------------------------------------------------
// Rivalries — read from the census, then earned on the field.
// ---------------------------------------------------------------------------

async fn ensure_rivalry_table(pool: &SqlitePool) {
    let _ = sqlx::query(
        "CREATE TABLE IF NOT EXISTS sheffield_rivalries (club_a TEXT NOT NULL, club_b TEXT NOT NULL, \
         kind TEXT NOT NULL, intensity INTEGER NOT NULL, meetings INTEGER NOT NULL DEFAULT 0, \
         PRIMARY KEY (club_a, club_b))").execute(pool).await;
}

/// Find a club's natural rivals — sides rooted in the same parish or drawn from
/// the same trade — and record them. Returns how many were found.
pub async fn discover_rivalries(pool: &SqlitePool, club_id: &str) -> Result<u32, sqlx::Error> {
    ensure_rivalry_table(pool).await;
    let (parish, district) = club_locality(pool, club_id).await;
    let mine = patronage_for(pool, club_id).await;
    let others: Vec<(String,)> = sqlx::query_as(
        "SELECT lc.club_id FROM sheffield_league_clubs lc JOIN sheffield_clubs c ON c.id = lc.club_id \
         WHERE c.is_reserve_team = 0 AND lc.club_id <> ? LIMIT 60")
        .bind(club_id).fetch_all(pool).await?;
    let mut found = 0;
    for (other,) in &others {
        let (op, od) = club_locality(pool, other).await;
        let same_parish = matches!((&parish, &op), (Some(a), Some(b)) if a.eq_ignore_ascii_case(b));
        let same_district = matches!((&district, &od), (Some(a), Some(b)) if a.eq_ignore_ascii_case(b));
        let theirs = patronage_for(pool, other).await;
        let shared_trade = mine == theirs && matches!(mine, Patronage::Works | Patronage::Rural);
        if let Some(kind) = rivalry::natural_rivalry(same_parish, same_district, shared_trade, false) {
            let (a, b) = if club_id < other.as_str() { (club_id, other.as_str()) } else { (other.as_str(), club_id) };
            let r = Rivalry::new(a, b, kind);
            let inserted = sqlx::query(
                "INSERT OR IGNORE INTO sheffield_rivalries (club_a, club_b, kind, intensity, meetings) \
                 VALUES (?, ?, ?, ?, 0)")
                .bind(a).bind(b).bind(format!("{:?}", kind)).bind(r.intensity as i64)
                .execute(pool).await?;
            found += inserted.rows_affected() as u32;
        }
    }
    Ok(found)
}

/// The standing rivalry between two clubs, if any.
pub async fn rivalry_between(pool: &SqlitePool, a: &str, b: &str) -> Option<Rivalry> {
    ensure_rivalry_table(pool).await;
    let (x, y) = if a < b { (a, b) } else { (b, a) };
    let row: Option<(String, String, i64, i64)> = sqlx::query_as(
        "SELECT club_a, club_b, intensity, meetings FROM sheffield_rivalries WHERE club_a = ? AND club_b = ?")
        .bind(x).bind(y).fetch_optional(pool).await.ok().flatten();
    row.map(|(ca, cb, intensity, meetings)| Rivalry {
        club_a: ca, club_b: cb, kind: rivalry::Kind::Parish,
        intensity: intensity.clamp(0, 100) as u8, meetings: meetings.clamp(0, 65535) as u16,
    })
}

/// Record that two rivals have met — a close, consequential match stokes it most.
pub async fn record_rivalry_meeting(
    pool: &SqlitePool, a: &str, b: &str, margin: u8, mattered: bool,
) -> Result<(), sqlx::Error> {
    if let Some(mut r) = rivalry_between(pool, a, b).await {
        r.after_meeting(margin, mattered);
        let (x, y) = if a < b { (a, b) } else { (b, a) };
        sqlx::query("UPDATE sheffield_rivalries SET intensity = ?, meetings = ? WHERE club_a = ? AND club_b = ?")
            .bind(r.intensity as i64).bind(r.meetings as i64).bind(x).bind(y)
            .execute(pool).await?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// The secretary — your own name in the game.
// ---------------------------------------------------------------------------

async fn ensure_secretary_table(pool: &SqlitePool) {
    let _ = sqlx::query(
        "CREATE TABLE IF NOT EXISTS sheffield_secretary (id INTEGER PRIMARY KEY CHECK (id = 1), \
         name TEXT NOT NULL, reputation INTEGER NOT NULL, seasons INTEGER NOT NULL, honours INTEGER NOT NULL)")
        .execute(pool).await;
}

pub async fn secretary_for(pool: &SqlitePool) -> Secretary {
    ensure_secretary_table(pool).await;
    let row: Option<(String, i64, i64, i64)> = sqlx::query_as(
        "SELECT name, reputation, seasons, honours FROM sheffield_secretary WHERE id = 1")
        .fetch_optional(pool).await.ok().flatten();
    match row {
        Some((name, rep, seasons, honours)) => Secretary {
            name, reputation: rep as i32,
            seasons_served: seasons.clamp(0, 65535) as u16,
            honours: honours.clamp(0, 65535) as u16,
        },
        None => Secretary::new("The Hon. Secretary"),
    }
}

async fn save_secretary(pool: &SqlitePool, s: &Secretary) -> Result<(), sqlx::Error> {
    ensure_secretary_table(pool).await;
    sqlx::query(
        "INSERT INTO sheffield_secretary (id, name, reputation, seasons, honours) VALUES (1, ?, ?, ?, ?) \
         ON CONFLICT(id) DO UPDATE SET name = excluded.name, reputation = excluded.reputation, \
         seasons = excluded.seasons, honours = excluded.honours")
        .bind(&s.name).bind(s.reputation as i64).bind(s.seasons_served as i64).bind(s.honours as i64)
        .execute(pool).await?;
    Ok(())
}

/// A season's work on your name — and whether a better club comes asking.
pub async fn secretary_after_season(
    pool: &SqlitePool, club_id: &str, finished_pos: u32, clubs: u32, won_honour: bool,
    committee_standing: i32, date: &str, season: i64,
) -> Result<Secretary, sqlx::Error> {
    let mut s = secretary_for(pool).await;
    s.after_season(finished_pos, clubs, won_honour, committee_standing);
    save_secretary(pool, &s).await?;

    // Does a better-placed club come asking? They look only at a name that has
    // outgrown its position.
    let mut rng = Rng::seed(seed_from(&format!("approach-{season}")));
    let suitor: Option<(String, String)> = sqlx::query_as(
        "SELECT c.id, c.name FROM sheffield_league_clubs lc JOIN sheffield_clubs c ON c.id = lc.club_id \
         JOIN sheffield_league_divisions d ON d.id = lc.division_id \
         WHERE c.is_reserve_team = 0 AND lc.club_id <> ? ORDER BY d.level ASC LIMIT 1")
        .bind(club_id).fetch_optional(pool).await.ok().flatten();
    if let Some((suitor_id, suitor_name)) = suitor {
        let their_standing = committee_for(pool, &suitor_id).await.standing;
        if s.approached_by(their_standing, &mut rng) {
            file_committee_news(pool, club_id, date,
                &format!("An Approach from {suitor_name}"),
                &format!("{} — {} — has been sounded out by {suitor_name}, who are in want of a secretary.",
                    s.name, s.standing_words())).await?;
        }
    }
    Ok(s)
}

// ---------------------------------------------------------------------------
// Sponsored cups: a real man of means proposes a competition sized to his own
// wealth and standing. Runs on the existing knockout bracket engine; its survival
// is tied to its sponsor's fortunes.
// ---------------------------------------------------------------------------

async fn ensure_sponsored_cup_table(pool: &SqlitePool) {
    let _ = sqlx::query(
        "CREATE TABLE IF NOT EXISTS sheffield_sponsored_cups (id TEXT PRIMARY KEY, \
         sponsor_name TEXT NOT NULL, cup_name TEXT NOT NULL, season_proposed INTEGER NOT NULL, \
         entrants INTEGER NOT NULL, prize_shillings INTEGER NOT NULL, \
         min_division_level INTEGER NOT NULL, max_division_level INTEGER NOT NULL, \
         seasons_held INTEGER NOT NULL DEFAULT 0, sponsor_still_backing INTEGER NOT NULL DEFAULT 1, \
         winner_club_id TEXT)")
        .execute(pool).await;
}

/// Some seasons a man of substance in the town proposes a cup of his own. Chosen
/// from the census businessmen, weighted loosely by how their trade is faring
/// (the same prosperity signal patronage fortunes use). Deterministic per season;
/// at most one new proposal a season, and never certain — most years bring none.
pub async fn maybe_propose_cup(pool: &SqlitePool, season: i64) -> Result<Option<sponsored_cup::Proposal>, sqlx::Error> {
    ensure_sponsored_cup_table(pool).await;
    let mut rng = Rng::seed(seed_from(&format!("sponsor-propose-{season}")));
    // Most seasons, nobody comes forward.
    if rng.roll(100) >= 20 { return Ok(None); }

    let candidate: Option<(String, String, String)> = sqlx::query_as(
        "SELECT surname, forename, business_type FROM sheffield_businessmen \
         ORDER BY id LIMIT 1 OFFSET ?")
        .bind((seed_from(&format!("sponsor-who-{season}")) % 200) as i64)
        .fetch_optional(pool).await?;
    let Some((surname, forename, trade)) = candidate else { return Ok(None); };
    let name = format!("{forename} {surname}");

    // Wealth from the trade's general standing (cutlery/steel magnates rank high);
    // standing from a stable hash of his own name, so the same man is always the
    // same man.
    let trade_l = trade.to_ascii_lowercase();
    let wealth = if trade_l.contains("manufacturer") || trade_l.contains("master") { 70 }
                 else if trade_l.contains("merchant") || trade_l.contains("agent") { 55 }
                 else { 35 };
    let standing = (30 + seed_from(&name) % 60) as u8;
    let means = sponsored_cup::Means { wealth, standing };
    let proposal = sponsored_cup::propose(&name, &means, &mut rng);

    let id = format!("cup-{season}-{}", seed_from(&name));
    sqlx::query(
        "INSERT OR IGNORE INTO sheffield_sponsored_cups (id, sponsor_name, cup_name, season_proposed, \
         entrants, prize_shillings, min_division_level, max_division_level) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(&id).bind(&proposal.sponsor_name).bind(&proposal.cup_name).bind(season)
        .bind(proposal.entrants as i64).bind(proposal.prize_shillings as i64)
        .bind(proposal.min_division_level as i64).bind(proposal.max_division_level as i64)
        .execute(pool).await?;

    // The news of it — a broadcast item every club in range can read.
    let date: (Option<String>,) = sqlx::query_as(
        "SELECT MAX(match_date) FROM sheffield_matches WHERE season = ?")
        .bind(season).fetch_optional(pool).await?.unwrap_or((None,));
    let date = date.0.unwrap_or_else(|| format!("{season}-09-07"));
    file_broadcast_news(pool, &date, "competition_news",
        &format!("{} Proposed", proposal.cup_name),
        &format!("{} has come forward to sponsor {}, open to clubs of the {} to {} divisions, \
                 with a purse of {} shillings for the winners.",
                proposal.sponsor_name, proposal.cup_name, proposal.min_division_level,
                proposal.max_division_level, proposal.prize_shillings)).await?;

    Ok(Some(proposal))
}

/// The close-season tick for every sponsored cup still running: another year
/// under its belt, or — if its sponsor's trade has failed — the cup may lapse.
/// Files news either way. Returns how many cups are still running.
pub async fn tick_sponsored_cups(pool: &SqlitePool, season: i64, date: &str) -> Result<u32, sqlx::Error> {
    ensure_sponsored_cup_table(pool).await;
    let cups: Vec<(String, String, String, i64, i64)> = sqlx::query_as(
        "SELECT id, sponsor_name, cup_name, seasons_held, sponsor_still_backing \
         FROM sheffield_sponsored_cups WHERE sponsor_still_backing = 1")
        .fetch_all(pool).await?;
    let mut running = 0u32;
    for (id, sponsor_name, cup_name, seasons_held, _) in &cups {
        let mut standing = sponsored_cup::Standing {
            seasons_held: (*seasons_held).clamp(0, 65535) as u16, sponsor_still_backing: true,
        };
        // A modest, stable trade signal — the same one patronage fortunes uses,
        // sampled fresh each season so a run of bad luck can end a cup.
        let mut rng = Rng::seed(seed_from(&format!("sponsor-trade-{id}-{season}")));
        let trade = (rng.roll(200) as i32 - 100) as i8;
        standing.after_season(trade, &mut rng);
        sqlx::query("UPDATE sheffield_sponsored_cups SET seasons_held = ?, sponsor_still_backing = ? WHERE id = ?")
            .bind(standing.seasons_held as i64).bind(standing.sponsor_still_backing as i64).bind(id)
            .execute(pool).await?;
        if standing.sponsor_still_backing {
            running += 1;
            if standing.is_established() && standing.seasons_held == 5 {
                let _ = file_broadcast_news(pool, date, "competition_news",
                    &format!("{cup_name} Now a Fixture"),
                    &format!("Five seasons unbroken, {cup_name} is now spoken of as a settled part \
                             of the football calendar, not the whim of one man.")).await;
            }
        } else {
            let _ = file_broadcast_news(pool, date, "competition_news",
                &format!("{cup_name} Lapses"),
                &format!("{sponsor_name}'s affairs no longer permit it, and {cup_name} will not \
                         be contested this year.")).await;
        }
    }
    Ok(running)
}

// ---------------------------------------------------------------------------
// Committee governance: a club's standing with its members, drawn from CM's
// board-confidence model. Persisted per club; reviewed each season; a great
// club may attract a benefactor from the businessmen of the census.
// ---------------------------------------------------------------------------

async fn ensure_committee_table(pool: &SqlitePool) {
    let _ = sqlx::query(
        "CREATE TABLE IF NOT EXISTS sheffield_committee (club_id TEXT PRIMARY KEY, \
         standing INTEGER NOT NULL DEFAULT 5250, has_benefactor INTEGER NOT NULL DEFAULT 0, \
         benefactor_name TEXT)")
        .execute(pool).await;
}

/// Load a club's committee (or the default 5250 mid-band if it has none yet).
pub async fn committee_for(pool: &SqlitePool, club_id: &str) -> Committee {
    ensure_committee_table(pool).await;
    let row: Option<(i64, i64)> = sqlx::query_as(
        "SELECT standing, has_benefactor FROM sheffield_committee WHERE club_id = ?")
        .bind(club_id).fetch_optional(pool).await.ok().flatten();
    match row {
        Some((s, b)) => Committee { standing: s as i32, has_benefactor: b != 0 },
        None => Committee::default(),
    }
}

async fn save_committee(pool: &SqlitePool, club_id: &str, c: &Committee) -> Result<(), sqlx::Error> {
    ensure_committee_table(pool).await;
    sqlx::query(
        "INSERT INTO sheffield_committee (club_id, standing, has_benefactor) VALUES (?, ?, ?) \
         ON CONFLICT(club_id) DO UPDATE SET standing = excluded.standing, has_benefactor = excluded.has_benefactor")
        .bind(club_id).bind(c.standing as i64).bind(c.has_benefactor as i64)
        .execute(pool).await?;
    Ok(())
}

fn verdict_words(v: Verdict) -> &'static str {
    match v {
        Verdict::Delighted => "The committee are delighted with the club's affairs.",
        Verdict::Content => "The committee are well content with how the club is run.",
        Verdict::Watchful => "The committee watch the season's progress closely.",
        Verdict::Restless => "The committee grow restless; the members mutter at the meeting.",
        Verdict::NoConfidence => "The committee's confidence has all but gone.",
    }
}

/// Review a club's season: standing moves on how they fared against expectation
/// (their league finish vs where their standing suggested), a benefactor may be
/// won or a no-confidence motion moved. Files a committee news item on `date`.
pub async fn review_committee_season(
    pool: &SqlitePool, club_id: &str, finished_pos: u32, clubs_in_division: u32,
    season: i64, date: &str,
) -> Result<Verdict, sqlx::Error> {
    let mut c = committee_for(pool, club_id).await;
    // Expectation: a top-third finish is a success, a bottom-third a failure.
    let third = (clubs_in_division / 3).max(1);
    let (won, drawn, expected) = if finished_pos <= third { (true, false, false) }
        else if finished_pos > clubs_in_division - third { (false, false, true) }
        else { (false, true, false) };
    // Apply as a season-weight (a season is many results).
    for _ in 0..5 { c.after_result(won, drawn, expected); }

    // A rising club attracts a benefactor from the businessmen of the town.
    if !c.has_benefactor && c.standing > 7000 {
        if let Some((name,)) = sqlx::query_as::<_, (String,)>(
            "SELECT forename || ' ' || surname FROM sheffield_businessmen ORDER BY id LIMIT 1 OFFSET ?")
            .bind((seed_from(club_id) % 200) as i64).fetch_optional(pool).await.ok().flatten() {
            c.has_benefactor = true;
            let _ = sqlx::query("UPDATE sheffield_committee SET benefactor_name = ? WHERE club_id = ?")
                .bind(&name).bind(club_id).execute(pool).await;
            file_committee_news(pool, club_id, date,
                &format!("A Benefactor for the Club"),
                &format!("{name}, a man of means, has consented to lend the club his patronage.")).await?;
        }
    }

    let verdict = c.verdict();
    let mut rng = Rng::seed(seed_from(&format!("noconf-{club_id}-{season}")));
    if c.faces_no_confidence(&mut rng) {
        file_committee_news(pool, club_id, date, "Motion of No Confidence",
            "At the annual meeting a motion of no confidence in the running of the club was moved.").await?;
    } else {
        file_committee_news(pool, club_id, date, "The Committee's Review", verdict_words(verdict)).await?;
    }
    save_committee(pool, club_id, &c).await?;
    Ok(verdict)
}

/// A broadcast item every club's inbox shows — no particular club addressed.
async fn file_broadcast_news(pool: &SqlitePool, date: &str, article_type: &str, headline: &str, body: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO sheffield_news_items (id, headline, article_type, publish_date, body_text, \
         is_read, is_important, requires_action, related_club_ids, related_player_ids, related_match_id, \
         related_invitation_id, has_action_button, action_button_text, action_type, action_data) \
         VALUES (?, ?, ?, ?, ?, 0, 1, 0, NULL, NULL, NULL, NULL, 0, NULL, NULL, NULL)")
        .bind(uuid::Uuid::new_v4().to_string()).bind(headline).bind(article_type).bind(date).bind(body)
        .execute(pool).await?;
    Ok(())
}

async fn file_committee_news(pool: &SqlitePool, club_id: &str, date: &str, headline: &str, body: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO sheffield_news_items (id, headline, article_type, publish_date, body_text, \
         is_read, is_important, requires_action, related_club_ids, related_player_ids, related_match_id, \
         related_invitation_id, has_action_button, action_button_text, action_type, action_data) \
         VALUES (?, ?, 'board_confidence', ?, ?, 0, 1, 0, ?, NULL, NULL, NULL, 0, NULL, NULL, NULL)")
        .bind(uuid::Uuid::new_v4().to_string()).bind(headline).bind(date).bind(body)
        .bind(format!("[\"{club_id}\"]")).execute(pool).await?;
    Ok(())
}

// ---------------------------------------------------------------------------
// The Co-operative Society — a club joins, buys its goods cheaply, and earns the
// divi each season. Uses the existing sheffield_cooperative_* tables.
// ---------------------------------------------------------------------------

/// The society's state this year (founds itself in 1868); persists to the system row.
pub async fn coop_status(pool: &SqlitePool, year: u16) -> Option<coop::Society> {
    let s = coop::Society::in_year(year)?;
    let _ = sqlx::query(
        "UPDATE sheffield_cooperative_system SET is_founded = 1, membership_count = ?, \
         dividend_rate = ?")
        .bind(s.membership as i64).bind(s.dividend_rate_x10 as f64 / 10.0)
        .execute(pool).await;
    Some(s)
}

/// A club joins the society (idempotent).
pub async fn coop_join(pool: &SqlitePool, club_id: &str, club_name: &str, date: &str) -> Result<(), sqlx::Error> {
    let already: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM sheffield_cooperative_memberships WHERE club_id = ?")
        .bind(club_id).fetch_optional(pool).await?;
    if already.is_some() { return Ok(()); }
    let member_no = (seed_from(club_id) % 5000) as i64 + 1;
    sqlx::query(
        "INSERT INTO sheffield_cooperative_memberships \
         (club_id, club_name, joined_date, member_number, share_capital, total_purchases, dividend_earned, is_active) \
         VALUES (?, ?, ?, ?, 0, 0, 0, 1)")
        .bind(club_id).bind(club_name).bind(date).bind(member_no)
        .execute(pool).await?;
    Ok(())
}

/// Record a club's purchase through the co-op (cost in shillings; divi-eligible flagged).
pub async fn coop_purchase(pool: &SqlitePool, club_id: &str, item: coop::Purchase, date: &str) -> Result<u32, sqlx::Error> {
    let cost = item.cost_shillings();
    sqlx::query(
        "INSERT INTO sheffield_cooperative_transactions \
         (transaction_id, club_id, transaction_date, transaction_type, item_description, cost_shillings, dividend_eligible, dividend_earned) \
         VALUES (?, ?, ?, 'purchase', ?, ?, ?, 0)")
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(club_id).bind(date).bind(item.label()).bind(cost as i64).bind(item.dividend_eligible() as i64)
        .execute(pool).await?;
    let _ = sqlx::query("UPDATE sheffield_cooperative_memberships SET total_purchases = total_purchases + ? WHERE club_id = ?")
        .bind(cost as i64).bind(club_id).execute(pool).await;
    Ok(cost)
}

/// Pay the season's dividend to a club on its eligible spend. Returns the shillings.
pub async fn coop_pay_dividend(pool: &SqlitePool, club_id: &str, year: u16) -> Result<u32, sqlx::Error> {
    let rate = match coop::Society::in_year(year) { Some(s) => s.dividend_rate_x10, None => return Ok(0) };
    let spend: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(cost_shillings), 0) FROM sheffield_cooperative_transactions \
         WHERE club_id = ? AND dividend_eligible = 1")
        .bind(club_id).fetch_one(pool).await?;
    let divi = coop::dividend_shillings(spend.0 as u32, rate);
    if divi > 0 {
        let _ = sqlx::query("UPDATE sheffield_cooperative_memberships SET dividend_earned = dividend_earned + ? WHERE club_id = ?")
            .bind(divi as i64).bind(club_id).execute(pool).await;
    }
    Ok(divi)
}

/// The engine's shipped phrase book, parsed once — the law-gated commentary
/// vocabulary every ticker draws from.
fn phrasebook() -> &'static Phrasebook {
    static BOOK: OnceLock<Phrasebook> = OnceLock::new();
    BOOK.get_or_init(|| Phrasebook::from_json(fsim_core::assets::PHRASEBOOK_1867)
        .unwrap_or_default())
}

/// Write a match's commentary ticker into sheffield_match_events (replacing any
/// previous lines), narrated from the phrase book under the given laws.
async fn write_ticker(
    pool: &SqlitePool, match_id: &str, res: &fsim_core::MatchResult,
    home: &Squad, away: &Squad, rules: &Ruleset,
) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM sheffield_match_events WHERE match_id = ?")
        .bind(match_id).execute(pool).await?;
    let salt = seed_from(match_id);
    for line in commentary::commentate_with(phrasebook(), res, home, away, rules, &[], salt) {
        sqlx::query(
            "INSERT INTO sheffield_match_events (match_id, minute, second, event_type, description, team_side) \
             VALUES (?, ?, 0, ?, ?, 'neutral')",
        )
        .bind(match_id)
        .bind(line.minute as i64)
        .bind(format!("{:?}", line.emphasis).to_lowercase())
        .bind(&line.text)
        .execute(pool).await?;
    }
    Ok(())
}

/// Deterministic seed from a stable id (FNV-1a) — the same match id always plays
/// the same match.
pub fn seed_from(id: &str) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in id.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn eng_position(pos: &str) -> Position {
    let p = pos.to_ascii_uppercase();
    if p.contains("GK") {
        Position::Goalkeeper
    } else if p.starts_with("FB") || p.starts_with("CB") || p.starts_with("DEF")
        || p.starts_with("RB") || p.starts_with("LB")
    {
        Position::Defender
    } else if p.starts_with("FW") || p.starts_with("ST") || p.starts_with("CF")
        || p.starts_with("WG") || p.starts_with("WI")
    {
        Position::Forward
    } else {
        Position::Midfielder
    }
}

fn clamp100(v: f64) -> u8 {
    v.round().clamp(0.0, 100.0) as u8
}

async fn club_name(pool: &SqlitePool, club_id: &str) -> Result<String, sqlx::Error> {
    let row: Option<(String,)> = sqlx::query_as("SELECT name FROM sheffield_clubs WHERE id = ?")
        .bind(club_id)
        .fetch_optional(pool)
        .await?;
    Ok(row.map(|r| r.0).unwrap_or_else(|| club_id.to_string()))
}

/// Load a club's strongest sixteen into an engine squad. Attributes (1–20) map to
/// the engine's 0–100 qualities the same way the offline tools do.
pub async fn load_squad(pool: &SqlitePool, club_id: &str) -> Result<Squad, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT id, first_name, surname, position, pace, strength, stamina, passing, dribbling, \
         technique, vision, decision_making, tackling, marking, handling, reflexes, one_on_ones, finishing \
         FROM sheffield_footballers WHERE club_id = ? AND current_ability IS NOT NULL \
         ORDER BY current_ability DESC LIMIT 16",
    )
    .bind(club_id)
    .fetch_all(pool)
    .await?;

    let name = club_name(pool, club_id).await?;
    let get = |row: &sqlx::sqlite::SqliteRow, col: &str| -> f64 {
        row.try_get::<Option<i64>, _>(col).ok().flatten().unwrap_or(0) as f64
    };

    let mut players = Vec::new();
    for row in &rows {
        let id: i64 = row.try_get("id").unwrap_or(0);
        let first: String = row.try_get::<Option<String>, _>("first_name").ok().flatten().unwrap_or_default();
        let sur: String = row.try_get::<Option<String>, _>("surname").ok().flatten().unwrap_or_default();
        let pos: String = row.try_get::<Option<String>, _>("position").ok().flatten().unwrap_or_default();
        let avg = |cols: &[&str]| cols.iter().map(|c| get(row, c)).sum::<f64>() / cols.len() as f64;
        let qualities = Qualities {
            finishing: clamp100(avg(&["finishing", "technique", "one_on_ones"]) * 5.0),
            creativity: clamp100(avg(&["passing", "vision", "dribbling", "decision_making"]) * 5.0),
            defending: clamp100(avg(&["tackling", "marking", "strength"]) * 5.0),
            goalkeeping: clamp100(avg(&["handling", "reflexes"]) * 5.0),
            pace: clamp100(get(row, "pace") * 5.0),
            physical: clamp100(avg(&["strength", "stamina"]) * 5.0),
        };
        let full = format!("{} {}", first.trim(), sur.trim());
        players.push(Player {
            id: id.to_string(),
            name: if full.trim().is_empty() { "Unknown".into() } else { full.trim().to_string() },
            position: eng_position(&pos),
            qualities,
            state: Default::default(),
        });
    }
    // A distant club has no census footballers — field its deterministic guest XI.
    if players.is_empty() {
        if let Some((id, gname, _, _, strength)) =
            DISTANT_CLUBS.iter().find(|c| c.0 == club_id)
        {
            return Ok(synthetic_squad(id, gname, *strength));
        }
    }

    // Appoint the captain: the strongest outfield head — his command kit drawn
    // from his own qualities (a general's eye from creativity, a voice from his
    // physical presence, steadiness from his defending).
    let captaincy = players.iter()
        .filter(|p| !matches!(p.position, Position::Goalkeeper))
        .max_by_key(|p| p.qualities.creativity as u32 + p.qualities.defending as u32)
        .map(|p| fsim_core::Captaincy {
            player_id: p.id.clone(),
            tactical_sense: p.qualities.creativity,
            voice: p.qualities.physical,
            discipline: p.qualities.defending,
        });
    Ok(Squad { club_id: club_id.to_string(), name, players, captaincy })
}

/// One league/cup match played by the engine matchday driver.
pub struct PlayedMatch {
    pub match_id: String,
    pub home_club_id: String,
    pub away_club_id: String,
    pub home_name: String,
    pub away_name: String,
    pub home_score: u8,
    pub away_score: u8,
    pub home_rouges: u8,
    pub away_rouges: u8,
}

/// Pre-simulate a day's due matches through the real engine — genuine
/// Sheffield-Rules results (rouges and all) plus a text-commentary ticker per
/// match. Matches involving `user_club_id` are skipped (the user plays theirs
/// interactively). Persists everything: upserts `sheffield_matches`, replaces
/// `sheffield_match_events` with the commentary lines, and raises one day
/// round-up item into `sheffield_news_items`.
///
/// `due` entries are `(match_id, home_club_id, away_club_id, gameweek)`.
/// Deterministic: each match is seeded from its id, so replays are identical.
pub async fn play_due_matches(
    pool: &SqlitePool,
    date: &str,
    due: &[(String, String, String, i64)],
    user_club_id: &str,
    season: i64,
) -> Result<Vec<PlayedMatch>, sqlx::Error> {
    use std::collections::HashMap;
    let rules = Ruleset::historical(season as u16);
    let cfg = rules.match_config();
    let mut squads: HashMap<String, Squad> = HashMap::new();
    let mut played: Vec<PlayedMatch> = Vec::new();

    for (match_id, home_id, away_id, gameweek) in due {
        if home_id == user_club_id || away_id == user_club_id {
            continue; // the user's own fixture stays interactive
        }
        if !squads.contains_key(home_id) {
            squads.insert(home_id.clone(), load_squad(pool, home_id).await?);
        }
        if !squads.contains_key(away_id) {
            squads.insert(away_id.clone(), load_squad(pool, away_id).await?);
        }
        let home = squads.get(home_id).unwrap();
        let away = squads.get(away_id).unwrap();
        if home.players.is_empty() || away.players.is_empty() {
            continue; // no footballers registered — leave for the fallback roll
        }

        let mut rng = Rng::seed(seed_from(match_id));
        let result = fsim_core::simulate_match(home, away, &mut rng, &cfg);

        // Persist the result (insert or update — fixtures may not have rows yet).
        sqlx::query(
            "INSERT INTO sheffield_matches (id, gameweek, season, rule_year, home_club_id, away_club_id, \
             home_score, away_score, home_rouges, away_rouges, played, match_date) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 1, ?) \
             ON CONFLICT(id) DO UPDATE SET played = 1, home_score = excluded.home_score, \
             away_score = excluded.away_score, home_rouges = excluded.home_rouges, \
             away_rouges = excluded.away_rouges",
        )
        .bind(match_id).bind(gameweek).bind(season).bind(season)
        .bind(home_id).bind(away_id)
        .bind(result.home_score as i64).bind(result.away_score as i64)
        .bind(result.home_rouges as i64).bind(result.away_rouges as i64)
        .bind(date)
        .execute(pool).await?;

        // The ticker: phrase-book prose, gated by the laws in force.
        write_ticker(pool, match_id, &result, home, away, &rules).await?;

        // A result settles or sours a side (the CM ±3), and a match is a game played.
        let outcome = result.outcome();
        for (club, won) in [
            (home_id, outcome == fsim_core::Outcome::HomeWin),
            (away_id, outcome == fsim_core::Outcome::AwayWin),
        ] {
            let delta = if outcome == fsim_core::Outcome::Draw { 0 } else if won { 3 } else { -3 };
            let _ = sqlx::query(
                "UPDATE sheffield_footballers SET \
                 morale = MAX(-100, MIN(100, COALESCE(morale, 0) + ?)), \
                 games_played = COALESCE(games_played, 0) + 1, \
                 condition = MAX(0, COALESCE(condition, 100) - 25) WHERE club_id = ?")
                .bind(delta).bind(club).execute(pool).await;
        }

        played.push(PlayedMatch {
            match_id: match_id.clone(),
            home_club_id: home_id.clone(),
            away_club_id: away_id.clone(),
            home_name: home.name.clone(),
            away_name: away.name.clone(),
            home_score: result.home_score,
            away_score: result.away_score,
            home_rouges: result.home_rouges,
            away_rouges: result.away_rouges,
        });
    }

    // One round-up item for the day's results — the living inbox, day by day.
    if !played.is_empty() {
        let body = played.iter()
            .map(|p| {
                let rouges = if p.home_rouges + p.away_rouges > 0 {
                    format!(" (rouges {}–{})", p.home_rouges, p.away_rouges)
                } else { String::new() };
                format!("{} {}–{} {}{}", p.home_name, p.home_score, p.away_score, p.away_name, rouges)
            })
            .collect::<Vec<_>>().join("\n");
        sqlx::query(
            "INSERT INTO sheffield_news_items (id, headline, article_type, publish_date, body_text, \
             is_read, is_important, requires_action, related_club_ids, related_player_ids, \
             related_match_id, related_invitation_id, has_action_button, action_button_text, \
             action_type, action_data) VALUES (?, ?, 'match_report', ?, ?, 0, 0, 0, NULL, NULL, NULL, NULL, 0, NULL, NULL, NULL)",
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(format!("Results of {}", date))
        .bind(date)
        .bind(&body)
        .execute(pool).await?;
    }
    Ok(played)
}

// ---------------------------------------------------------------------------
// The close-season assembly: at season's end the member clubs vote on the laws
// (or, in a historical game, the calendar turns them). Writes next season's
// ruleset to sheffield_rules_history and raises law-change items into the inbox.
// ---------------------------------------------------------------------------

/// How this save governs its laws at the close of a season.
pub enum LawGovernance {
    /// The member clubs hold their annual assembly and vote (the fantasy league).
    Assembly,
    /// The laws follow the recorded timeline — no vote.
    Historical,
    /// The laws are frozen — carried forward unchanged.
    Fixed,
}

/// Map the save's `game_mode` string onto a governance. The fantasy league (and
/// any unknown/absent mode) holds the assembly; historical modes follow the
/// calendar; a single-ruleset game freezes.
pub fn governance_from_mode(mode: &Option<String>) -> LawGovernance {
    match mode.as_deref().map(|s| s.to_ascii_lowercase()) {
        Some(m) if m.contains("ahistorical") || m.contains("single") => LawGovernance::Fixed,
        Some(m) if m.contains("historical") => LawGovernance::Historical,
        _ => LawGovernance::Assembly,
    }
}

/// The outcome of closing a season, for the day's event feed.
pub struct SeasonClose {
    pub assembly_held: bool,
    pub carried: Vec<String>,
    pub next_rules_year: u16,
}

/// Close the season: decide next year's laws (vote, calendar, or freeze), write
/// them to `sheffield_rules_history`, and raise law-change news dated `agm_date`.
/// Idempotent — returns `Ok(None)` if next season's laws already exist.
/// Deterministic — the vote is seeded from the season year.
pub async fn close_season(
    pool: &SqlitePool,
    season_year: i64,
    agm_date: &str,
    governance: LawGovernance,
) -> Result<Option<SeasonClose>, sqlx::Error> {
    let next_year = season_year + 1;
    let existing: Option<(i64,)> =
        sqlx::query_as("SELECT season_year FROM sheffield_rules_history WHERE season_year = ?")
            .bind(next_year)
            .fetch_optional(pool)
            .await?;
    if existing.is_some() {
        return Ok(None); // already closed
    }

    // The laws the season was played under: the latest stored ruleset that parses
    // in the engine's shape, else the recorded laws of the season's year.
    let mut current: Ruleset = Ruleset::historical(season_year as u16);
    let rows: Vec<(String,)> =
        sqlx::query_as("SELECT ruleset_json FROM sheffield_rules_history ORDER BY season_year DESC")
            .fetch_all(pool)
            .await?;
    for (rj,) in &rows {
        if let Ok(r) = serde_json::from_str::<Ruleset>(rj) {
            current = r;
            break;
        }
    }

    let mut items: Vec<news::NewsItem> = Vec::new();
    let (assembly_held, carried, new_rules) = match governance {
        LawGovernance::Fixed => (false, Vec::new(), current.clone()),
        LawGovernance::Historical => {
            let next = Ruleset::historical(next_year as u16);
            if next.rouges() != current.rouges() {
                let restored = next.rouges();
                items.push(news::law_change(0,
                    if restored { "The Rouge Restored" } else { "The Rouge Abolished" },
                    format!("Under the laws adopted for {next_year}.")));
            }
            (false, Vec::new(), next)
        }
        LawGovernance::Assembly => {
            // The season's football, aggregated — what the members react to.
            let s: (i64, i64, i64, i64, i64) = sqlx::query_as(
                "SELECT COUNT(*), COALESCE(SUM(home_score + away_score), 0), \
                 COALESCE(SUM(home_rouges + away_rouges), 0), \
                 COALESCE(SUM(CASE WHEN home_score = away_score AND home_rouges = away_rouges THEN 1 ELSE 0 END), 0), \
                 COALESCE(SUM(CASE WHEN home_score = away_score AND home_rouges <> away_rouges THEN 1 ELSE 0 END), 0) \
                 FROM sheffield_matches WHERE season = ? AND played = 1",
            )
            .bind(season_year)
            .fetch_one(pool)
            .await?;
            let agg = season::SeasonReport {
                table: vec![], results: vec![],
                matches: s.0 as u32, total_goals: s.1 as u32, total_rouges: s.2 as u32,
                draws: s.3 as u32, rouge_decided: s.4 as u32,
            };

            // The member clubs, each with its temperament (stable per club name).
            let names: Vec<(String,)> = sqlx::query_as(
                "SELECT DISTINCT c.name FROM sheffield_league_clubs lc \
                 JOIN sheffield_clubs c ON c.id = lc.club_id WHERE c.is_reserve_team = 0",
            )
            .fetch_all(pool)
            .await?;
            let voters: Vec<agm::Voter> = names.iter()
                .map(|(n,)| agm::Voter {
                    club: n.clone(),
                    progressiveness: (25 + n.bytes().map(|b| b as u32).sum::<u32>() * 7 % 45) as u8,
                })
                .collect();

            let mut rng = Rng::seed(seed_from(&format!("assembly-{season_year}")));
            let sentiment = agm::SeasonSentiment::from_report(&agg, &mut rng);
            let outcome = agm::hold(&current, next_year as u16, &voters, &sentiment, &mut rng);

            let carried: Vec<String> = outcome.results.iter()
                .filter(|r| r.carried)
                .map(|r| r.title.clone())
                .collect();
            for r in outcome.results.iter().filter(|r| r.carried) {
                items.push(news::law_change(0,
                    format!("Assembly carries: {}", r.title.trim_start_matches("That the ")),
                    format!("Carried by {} votes to {}.", r.ayes, r.noes)));
            }
            if carried.is_empty() {
                items.push(news::law_change(0,
                    "Laws Unchanged as the Members Hold Firm",
                    format!("Every motion at the {season_year} assembly was rebuffed; the code stands as it was.")));
            }
            (true, carried, outcome.new_rules.clone())
        }
    };

    // Next season's laws, in the engine's shape (the season loop reads this back).
    sqlx::query(
        "INSERT OR REPLACE INTO sheffield_rules_history (season_year, rule_year, ruleset_json) VALUES (?, ?, ?)",
    )
    .bind(next_year)
    .bind(new_rules.year as i64)
    .bind(serde_json::to_string(&new_rules).unwrap_or_default())
    .execute(pool)
    .await?;

    for item in &items {
        sqlx::query(
            "INSERT INTO sheffield_news_items (id, headline, article_type, publish_date, body_text, \
             is_read, is_important, requires_action, related_club_ids, related_player_ids, \
             related_match_id, related_invitation_id, has_action_button, action_button_text, \
             action_type, action_data) VALUES (?, ?, ?, ?, ?, 0, 1, 0, NULL, NULL, NULL, NULL, 0, NULL, NULL, NULL)",
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(&item.headline)
        .bind(item.kind.article_type())
        .bind(agm_date)
        .bind(&item.body)
        .execute(pool)
        .await?;
    }

    Ok(Some(SeasonClose {
        assembly_held,
        carried,
        next_rules_year: new_rules.year,
    }))
}

/// Clubs and associations from around the country — the inter-association ties
/// (Sheffield first met Nottingham in 1865). Each brings its own code to the
/// negotiating table; none plays in the Sheffield pyramid. `(id, name, region,
/// code, strength)`.
const DISTANT_CLUBS: &[(&str, &str, &str, &str, u8)] = &[
    ("guest-notts-county",      "Notts County",      "Nottinghamshire", "nottingham", 58),
    ("guest-nottingham-forest", "Nottingham Forest", "Nottinghamshire", "nottingham", 55),
    ("guest-lincoln",           "Lincoln FC",        "Lincolnshire",    "sheffield",  50),
    ("guest-barnes",            "Barnes FC",         "London",          "fa",         56),
    ("guest-wanderers",         "Wanderers FC",      "London",          "fa",         62),
    ("guest-leeds-athletic",    "Leeds Athletic",    "Yorkshire",       "sheffield",  52),
];

/// The code a club plays: distant guests bring their own; everyone local plays
/// the Sheffield code. Feeds the pre-match rules negotiation.
pub fn club_code(club_id: &str) -> &'static str {
    DISTANT_CLUBS.iter().find(|c| c.0 == club_id).map(|c| c.3).unwrap_or("sheffield")
}

/// Make sure the distant clubs exist as club rows (outside the league pyramid),
/// so letters, matches and news can reference them.
async fn ensure_guest_clubs(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    for (id, name, region, _, _) in DISTANT_CLUBS {
        sqlx::query(
            "INSERT OR IGNORE INTO sheffield_clubs (id, name, short_name, city, region, origin, is_reserve_team) \
             VALUES (?, ?, ?, ?, ?, 'guest', 0)",
        )
        .bind(id).bind(name).bind(name).bind(region).bind(region)
        .execute(pool).await?;
    }
    Ok(())
}

/// A deterministic squad for a distant club (no census footballers of their own):
/// sixteen period men seeded from the club id, strength from the roster.
fn synthetic_squad(club_id: &str, name: &str, strength: u8) -> Squad {
    const SURNAMES: [&str; 16] = [
        "Fletcher", "Hardcastle", "Whitworth", "Prescott", "Ashworth", "Broadhead",
        "Sutcliffe", "Gladwin", "Marsden", "Ollerton", "Pemberton", "Rawnsley",
        "Stainton", "Thackeray", "Underwood", "Wolstenholme",
    ];
    let mut rng = Rng::seed(seed_from(club_id));
    let jitter = |rng: &mut Rng, base: u8| -> u8 {
        (base as i32 + rng.roll(11) as i32 - 5).clamp(20, 95) as u8
    };
    let mut players = Vec::new();
    for (i, surname) in SURNAMES.iter().enumerate() {
        let position = match i { 0 => Position::Goalkeeper, 1..=5 => Position::Defender,
                                 6..=10 => Position::Midfielder, _ => Position::Forward };
        let s = strength;
        players.push(Player {
            id: format!("{club_id}-{i}"),
            name: format!("{}. {surname}", (b'A' + (rng.roll(26)) as u8) as char),
            position,
            qualities: Qualities {
                finishing: jitter(&mut rng, s), creativity: jitter(&mut rng, s),
                defending: jitter(&mut rng, s), goalkeeping: jitter(&mut rng, s),
                pace: jitter(&mut rng, s), physical: jitter(&mut rng, s),
            },
            state: Default::default(),
        });
    }
    let captaincy = players.get(7).map(|p| fsim_core::Captaincy {
        player_id: p.id.clone(),
        tactical_sense: jitter(&mut rng, strength),
        voice: jitter(&mut rng, strength),
        discipline: jitter(&mut rng, strength),
    });
    Squad { club_id: club_id.to_string(), name: name.to_string(), players, captaincy }
}

/// A letter posted by an AI club this morning.
pub struct PostedChallenge {
    pub invitation_id: String,
    pub sender_club_id: String,
    pub recipient_club_id: String,
    pub to_user: bool,
}

/// The morning post: some mornings an AI club takes it upon itself to issue a
/// challenge — sometimes to the user's club (a letter lands in the inbox), more
/// often to a rival. Deterministic per date and idempotent (a replayed day posts
/// the same letter once). AI↔AI letters resolve entirely through the existing
/// pipeline: the reply on the response date, the match played by the engine on
/// match day, the result in the feed. A letter to the user is answered by the
/// Hon. Secretary on the response date if the user leaves it unanswered.
pub async fn generate_ai_challenges(
    pool: &SqlitePool,
    current_date: &str,
    user_club_id: &str,
) -> Result<Vec<PostedChallenge>, sqlx::Error> {
    let mut rng = Rng::seed(seed_from(&format!("morning-post-{current_date}")));
    let sent = chrono::NaiveDate::parse_from_str(current_date, "%Y-%m-%d")
        .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;

    // Most mornings bring no letters at all — and during the season the fixture
    // card leaves little room for arranging friendlies; the close season is when
    // the correspondence flows.
    {
        use chrono::Datelike;
        let in_season = sent.month() >= 9 || sent.month() <= 4;
        let threshold = if in_season { 8 } else { 30 };
        if rng.roll(100) >= threshold {
            return Ok(Vec::new());
        }
    }

    ensure_guest_clubs(pool).await?;
    let clubs: Vec<(String, String)> = sqlx::query_as(
        "SELECT lc.club_id, c.name FROM sheffield_league_clubs lc \
         JOIN sheffield_clubs c ON c.id = lc.club_id WHERE c.is_reserve_team = 0",
    )
    .fetch_all(pool).await?;
    if clubs.len() < 2 {
        return Ok(Vec::new());
    }

    // Every so often the letter crosses association lines — a club from around
    // the country (Nottinghamshire, London…) writes, or is written to; whose
    // rules govern the tie is settled by negotiation before kick-off.
    let distant = rng.roll(4) == 0;
    let (sender, recipient) = if distant {
        let g = DISTANT_CLUBS[rng.roll(DISTANT_CLUBS.len() as u32) as usize];
        let guest = (g.0.to_string(), g.1.to_string());
        let local = if rng.roll(3) == 0 {
            clubs.iter().find(|c| c.0 == user_club_id).cloned()
                .unwrap_or_else(|| clubs[rng.roll(clubs.len() as u32) as usize].clone())
        } else {
            clubs[rng.roll(clubs.len() as u32) as usize].clone()
        };
        // The letter travels either direction across the country.
        if rng.roll(2) == 0 { (guest, local) } else { (local, guest) }
    } else {
        let sender = clubs[rng.roll(clubs.len() as u32) as usize].clone();
        // A third of the local post is addressed to the user; the rest to rivals.
        let recipient = if rng.roll(3) == 0 {
            clubs.iter().find(|c| c.0 == user_club_id).cloned()
                .unwrap_or_else(|| clubs[rng.roll(clubs.len() as u32) as usize].clone())
        } else {
            clubs[rng.roll(clubs.len() as u32) as usize].clone()
        };
        (sender, recipient)
    };
    if sender.0 == user_club_id {
        return Ok(Vec::new()); // the user writes their own letters
    }
    if recipient.0 == sender.0 {
        return Ok(Vec::new());
    }
    let response_date = (sent + chrono::Duration::days(3)).format("%Y-%m-%d").to_string();
    let match_date = (sent + chrono::Duration::days(14)).format("%Y-%m-%d").to_string();

    // The recipient's circumstances, weighed the same way replies are.
    let level = |pool: &SqlitePool, id: &str| {
        let id = id.to_string();
        let pool = pool.clone();
        async move {
            let r: Option<(i64,)> = sqlx::query_as(
                "SELECT d.level FROM sheffield_league_clubs lc \
                 JOIN sheffield_league_divisions d ON lc.division_id = d.id WHERE lc.club_id = ?",
            ).bind(&id).fetch_optional(&pool).await.ok().flatten();
            r.map(|x| x.0).unwrap_or(5)
        }
    };
    let s_level = level(pool, &sender.0).await;
    let r_level = level(pool, &recipient.0).await;
    let regions: Vec<(Option<String>,)> = sqlx::query_as(
        "SELECT region FROM sheffield_clubs WHERE id IN (?, ?)",
    ).bind(&sender.0).bind(&recipient.0).fetch_all(pool).await?;
    let same_region = regions.len() == 2 && regions[0].0.is_some() && regions[0].0 == regions[1].0;
    let clash: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_matches WHERE (home_club_id = ? OR away_club_id = ?) AND match_date = ?",
    ).bind(&recipient.0).bind(&recipient.0).bind(&match_date).fetch_one(pool).await?;
    let match_day = sent + chrono::Duration::days(14);
    use chrono::Datelike;
    let stakes_pool = ["honor", "small", "trophy", "dinner", "charity"];
    let stakes_str = stakes_pool[rng.roll(stakes_pool.len() as u32) as usize];
    let stakes = match stakes_str {
        "small" => friendly::Stakes::SmallWager,
        "trophy" => friendly::Stakes::Trophy,
        "dinner" => friendly::Stakes::Dinner,
        "charity" => friendly::Stakes::Charity,
        _ => friendly::Stakes::Honour,
    };
    let circ = friendly::Circumstances {
        division_gap: (s_level - r_level).unsigned_abs() as i32,
        same_region,
        stakes,
        fixture_clash: clash.0 > 0,
        off_season: !(match_day.month() >= 9 || match_day.month() <= 4),
        weekend: match_day.weekday().number_from_monday() >= 6,
    };
    let likelihood = friendly::acceptance_likelihood(&circ);

    // The code the letter proposes: a cross-country tie carries the guest's own
    // code — whose rules actually govern is settled by negotiation at the ground.
    let proposed_code = if club_code(&sender.0) != "sheffield" { club_code(&sender.0) }
                        else { club_code(&recipient.0) };

    // Deterministic id ⇒ idempotent post (INSERT OR IGNORE).
    let invitation_id = format!("post-{current_date}-{}", sender.0);
    let inserted = sqlx::query(
        "INSERT OR IGNORE INTO sheffield_challenge_invitations (id, sender_club_id, recipient_club_id, \
         sent_date, response_date, proposed_match_date, match_type, venue, stakes, tone, rules_type, \
         match_duration, status, acceptance_likelihood) \
         VALUES (?, ?, ?, ?, ?, ?, 'friendly', 'home', ?, 'cordial', ?, 90, 'sent', ?)",
    )
    .bind(&invitation_id).bind(&sender.0).bind(&recipient.0)
    .bind(current_date).bind(&response_date).bind(&match_date)
    .bind(stakes_str).bind(proposed_code).bind(likelihood as i64)
    .execute(pool).await?;
    if inserted.rows_affected() == 0 {
        return Ok(Vec::new()); // already posted (replayed day)
    }

    let to_user = recipient.0 == user_club_id;
    if to_user {
        // The letter lands in the user's inbox. (Answer it or the Hon. Secretary
        // will reply on your behalf on the response date.)
        let item = friendly::news_challenge_received(0, &sender.0, &sender.1, &recipient.0, stakes);
        sqlx::query(
            "INSERT INTO sheffield_news_items (id, headline, article_type, publish_date, body_text, \
             is_read, is_important, requires_action, related_club_ids, related_player_ids, \
             related_match_id, related_invitation_id, has_action_button, action_button_text, \
             action_type, action_data) VALUES (?, ?, ?, ?, ?, 0, 1, 0, ?, NULL, NULL, ?, 0, NULL, NULL, NULL)",
        )
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(&item.headline)
        .bind(item.kind.article_type())
        .bind(current_date)
        .bind({
            let code_note = match proposed_code {
                "sheffield" => String::new(),
                c => format!(" They write under the {} code — whose rules govern would be \
                              settled before kick-off.",
                             match c { "nottingham" => "Nottingham", "fa" => "Association",
                                       "cambridge" => "Cambridge", other => other }),
            };
            format!("{} A match is proposed for {}.{code_note} If no answer is given, the \
                     Hon. Secretary will reply by {}.", item.body, match_date, response_date)
        })
        .bind(format!("[\"{}\",\"{}\"]", sender.0, recipient.0))
        .bind(&invitation_id)
        .execute(pool).await?;
    }

    Ok(vec![PostedChallenge {
        invitation_id,
        sender_club_id: sender.0,
        recipient_club_id: recipient.0,
        to_user,
    }])
}

/// One friendly played by the daily driver (for logging / the day summary).
pub struct PlayedFriendly {
    pub match_id: String,
    pub home_club_id: String,
    pub away_club_id: String,
    pub home_score: u8,
    pub away_score: u8,
    pub headline: String,
}

/// The daily driver: play every accepted friendly whose match day has arrived and
/// which has not yet been played — through the real engine — writing the score to
/// `sheffield_matches` and a result item into `sheffield_news_items`. This is the
/// step the old flow never reached (a scheduled friendly stayed `played = 0`).
pub async fn play_due_friendlies(
    pool: &SqlitePool,
    current_date: &str,
) -> Result<Vec<PlayedFriendly>, sqlx::Error> {
    let due = sqlx::query(
        "SELECT i.rules_type AS rules_type, m.id AS match_id, m.home_club_id AS home_club_id, \
         m.away_club_id AS away_club_id, m.rule_year AS rule_year \
         FROM sheffield_challenge_invitations i \
         JOIN sheffield_matches m ON m.id = i.scheduled_match_id \
         WHERE i.status = 'accepted' AND m.played = 0 AND m.match_date <= ?",
    )
    .bind(current_date)
    .fetch_all(pool)
    .await?;

    let mut played = Vec::new();
    for row in &due {
        let match_id: String = row.try_get("match_id").unwrap_or_default();
        let home_id: String = row.try_get("home_club_id").unwrap_or_default();
        let away_id: String = row.try_get("away_club_id").unwrap_or_default();
        let rule_year: i64 = row.try_get::<Option<i64>, _>("rule_year").ok().flatten().unwrap_or(1867);

        let home = load_squad(pool, &home_id).await?;
        let away = load_squad(pool, &away_id).await?;
        // Each side brings its own code and laws to the table; the pre-match
        // negotiation settles whose rules govern the tie.
        let rules_for = |code: &str| -> Ruleset {
            if code == "fa" { Ruleset::modern() } else { Ruleset::historical(rule_year as u16) }
        };
        let (home_code, away_code) = (club_code(&home_id), club_code(&away_id));
        let (home_rules, away_rules) = (rules_for(home_code), rules_for(away_code));
        let mut rng = Rng::seed(seed_from(&match_id));
        let outcome: FriendlyOutcome = friendly::play(
            &home, FootballCode::parse(home_code), &home_rules, 50,
            &away, FootballCode::parse(away_code), &away_rules, 50,
            &mut rng,
        );
        let r = &outcome.result;

        sqlx::query(
            "UPDATE sheffield_matches SET played = 1, home_score = ?, away_score = ?, \
             home_rouges = ?, away_rouges = ? WHERE id = ?",
        )
        .bind(r.home_score as i64)
        .bind(r.away_score as i64)
        .bind(r.home_rouges as i64)
        .bind(r.away_rouges as i64)
        .bind(&match_id)
        .execute(pool)
        .await?;

        // The friendly gets its ticker too — narrated under the negotiated laws.
        write_ticker(pool, &match_id, r, &home, &away, &outcome.ruleset).await?;

        // Raise the result item — same prose as the engine's own news module.
        let item = friendly::news_result(0, &home_id, &home.name, &away_id, &away.name, &outcome);
        let news_id = uuid::Uuid::new_v4().to_string();
        let related = format!("[\"{}\",\"{}\"]", home_id, away_id);
        sqlx::query(
            "INSERT INTO sheffield_news_items (id, headline, article_type, publish_date, body_text, \
             is_read, is_important, requires_action, related_club_ids, related_player_ids, \
             related_match_id, related_invitation_id, has_action_button, action_button_text, \
             action_type, action_data) VALUES (?, ?, ?, ?, ?, 0, 0, 0, ?, NULL, ?, NULL, 0, NULL, NULL, NULL)",
        )
        .bind(&news_id)
        .bind(&item.headline)
        .bind(item.kind.article_type())
        .bind(current_date)
        .bind(&item.body)
        .bind(&related)
        .bind(&match_id)
        .execute(pool)
        .await?;

        played.push(PlayedFriendly {
            match_id,
            home_club_id: home_id,
            away_club_id: away_id,
            home_score: r.home_score,
            away_score: r.away_score,
            headline: item.headline.clone(),
        });
    }
    Ok(played)
}
