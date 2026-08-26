use crate::game::{GameState, Match, DayProcessingResult, Club, Standing};
use crate::database;
use crate::database::cup_competitions::{Competition, CupTie, EligibleClub};
use crate::database::{LeagueConfig, LeagueMetadata};
use crate::sheffield_rules;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use uuid;
use tauri::{Manager, Emitter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerDetail {
    pub id: String,
    pub name: String,
    pub club_id: String,
    pub position: Option<String>,
    pub birth_year: Option<i32>,
    pub age: i32,
    pub nationality: Option<String>,
    // Census/Historical Data
    pub profession: Option<String>,
    pub parish: Option<String>,
    pub address: Option<String>,
    pub birthplace: Option<String>,
    // Physical
    pub pace: Option<i32>,
    pub acceleration: Option<i32>,
    pub strength: Option<i32>,
    pub stamina: Option<i32>,
    pub balance: Option<i32>,
    pub jumping: Option<i32>,
    pub agility: Option<i32>,
    pub natural_fitness: Option<i32>,
    // Technical
    pub passing: Option<i32>,
    pub dribbling: Option<i32>,
    pub first_touch: Option<i32>,
    pub technique: Option<i32>,
    pub heading: Option<i32>,
    pub long_passing: Option<i32>,
    pub crossing: Option<i32>,
    pub long_shots: Option<i32>,
    pub tackling: Option<i32>,
    pub handling: Option<i32>,
    pub reflexes: Option<i32>,
    pub corners: Option<i32>,
    pub free_kicks: Option<i32>,
    pub throw_ins: Option<i32>,
    pub vision: Option<i32>,
    pub left_foot: Option<i32>,
    pub right_foot: Option<i32>,
    pub one_on_ones: Option<i32>,
    // Mental
    pub courage: Option<i32>,
    pub bravery: Option<i32>,
    pub concentration: Option<i32>,
    pub decision_making: Option<i32>,
    pub leadership: Option<i32>,
    pub aggression: Option<i32>,
    pub anticipation: Option<i32>,
    pub determination: Option<i32>,
    pub flair: Option<i32>,
    pub influence: Option<i32>,
    pub adaptability: Option<i32>,
    pub ambition: Option<i32>,
    pub loyalty: Option<i32>,
    pub pressure: Option<i32>,
    pub professionalism: Option<i32>,
    pub sportsmanship: Option<i32>,
    pub temperament: Option<i32>,
    // Positioning
    pub awareness: Option<i32>,
    pub marking: Option<i32>,
    pub positioning: Option<i32>,
    pub work_rate: Option<i32>,
    pub off_the_ball: Option<i32>,
    pub movement: Option<i32>,
    pub teamwork: Option<i32>,
    // Specialization
    pub finishing: Option<i32>,
    pub penalties: Option<i32>,
    pub set_pieces: Option<i32>,
    // Hidden
    pub consistency: Option<i32>,
    pub dirtiness: Option<i32>,
    pub versatility: Option<i32>,
    pub injury_proneness: Option<i32>,
    pub important_matches: Option<i32>,
    // Ability & Reputation
    pub current_ability: Option<i32>,
    pub potential_ability: Option<i32>,
    pub current_reputation: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClubInfo {
    pub id: String,
    pub name: String,
    pub short_name: String,
    pub founded_year: i32,
    pub ground_name: String,
    pub ground_capacity: i32,
    pub city: String,
    pub region: String,
    pub origin: String,
    pub primary_color: String,
    pub secondary_color: String,
    pub reserve_of_club_id: Option<String>,
    pub parent_club_name: Option<String>,
    pub where_from: Option<String>,
    pub additional_postcode: Option<String>,
}

#[tauri::command]
pub async fn new_game(club_id: String) -> Result<GameState, String> {
    // Copy master database to temp
    database::init_new_game().await.map_err(|e| e.to_string())?;

    let mut game = GameState::new(club_id);

    // Mark as using temp database (not yet saved)
    game.loaded_from_file = None;

    Ok(game)
}

#[tauri::command]
pub async fn load_game(game_id: String) -> Result<GameState, String> {
    database::load_game(&game_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_current_game() -> Result<GameState, String> {
    database::get_latest_game().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_game(game: GameState) -> Result<(), String> {
    // If the game was loaded from a file, save to that file
    // Otherwise, we can't auto-save (temp database is temporary)
    if let Some(save_file) = &game.loaded_from_file {
        let sanitized_name = save_file.trim_end_matches(".sav")
            .trim_end_matches(".json")
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
            .collect::<String>();
        database::save_game_named(&game, &sanitized_name).await.map_err(|e| e.to_string())
    } else {
        // Game hasn't been saved yet - can't auto-save to temp
        Err("Game must be saved with a name first (use Save As)".to_string())
    }
}

#[tauri::command]
pub async fn save_game_as(game: GameState, save_name: String) -> Result<GameState, String> {
    let mut updated_game = game.clone();
    updated_game.loaded_from_file = Some(format!("{}.sav", save_name));
    database::save_game_named(&updated_game, &save_name).await.map_err(|e| e.to_string())?;
    Ok(updated_game)
}

#[tauri::command]
pub async fn load_game_by_name(save_name: String) -> Result<GameState, String> {
    database::load_game_named(&save_name).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_save_list() -> Result<Vec<String>, String> {
    database::get_save_list().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_save_file(save_name: String) -> Result<(), String> {
    database::delete_save_file(&save_name).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn advance_gameweek(mut game: GameState) -> Result<GameState, String> {
    if game.current_gameweek < 38 {
        game.current_gameweek += 1;
    }
    Ok(game)
}

#[tauri::command]
pub async fn advance_day(mut game: GameState) -> Result<DayProcessingResult, String> {
    // Pre-simulate tomorrow's due matches through the real engine (excluding the
    // user's own fixture, which is played interactively). Match events then
    // resolve with genuine Sheffield-Rules results — score, rouges and a
    // text-commentary ticker — instead of placeholder randoms.
    if let Ok(pool) = database::get_active_game_pool(&game).await.map_err(|e| e.to_string()) {
        if let Ok(today) = chrono::NaiveDate::parse_from_str(&game.current_date, "%Y-%m-%d") {
            let next_date = (today + chrono::Duration::days(1)).format("%Y-%m-%d").to_string();
            let due: Vec<(String, String, String, i64)> = game.matches.iter()
                .filter(|m| m.date == next_date && !m.played)
                .map(|m| (m.id.clone(), m.home_team_id.clone(), m.away_team_id.clone(), m.gameweek as i64))
                .collect();
            if !due.is_empty() {
                match crate::fsim_bridge::play_due_matches(&pool, &next_date, &due, &game.user_club_id, game.season as i64).await {
                    Ok(played) => {
                        eprintln!("[MATCHDAY] engine played {}/{} due matches for {}", played.len(), due.len(), next_date);
                        for p in &played {
                            if let Some(m) = game.matches.iter_mut().find(|m| m.id == p.match_id) {
                                m.home_score = Some(p.home_score);
                                m.away_score = Some(p.away_score);
                                m.played = true; // execute_event keeps these scores
                            }
                        }
                    }
                    Err(e) => eprintln!("[MATCHDAY] engine error: {}", e),
                }
            }

            // Season's end? The member clubs convene: the close-season assembly
            // votes on next year's laws (or the historical calendar turns them),
            // writes them to the rules history, and files law-change news.
            let season_end: Option<(String,)> = sqlx::query_as(
                "SELECT season_end_date FROM sheffield_league_config WHERE season_year = ? \
                 ORDER BY rowid DESC LIMIT 1")
                .bind(game.season as i64)
                .fetch_optional(&pool).await.ok().flatten();
            let end_date = season_end.map(|r| r.0)
                .unwrap_or_else(|| format!("{}-12-31", game.season));
            if next_date == end_date {
                let governance = crate::fsim_bridge::governance_from_mode(&game.game_mode);
                match crate::fsim_bridge::close_season(&pool, game.season as i64, &next_date, governance).await {
                    Ok(Some(close)) => {
                        eprintln!("[ASSEMBLY] season {} closed: assembly={}, {} motions carried, laws of {}",
                            game.season, close.assembly_held, close.carried.len(), close.next_rules_year);
                        let description = if close.carried.is_empty() {
                            "The laws stand unchanged for the coming season.".to_string()
                        } else {
                            format!("Carried: {}", close.carried.join("; "))
                        };
                        game.pending_events.push(crate::game::GameEvent {
                            id: uuid::Uuid::new_v4().to_string(),
                            date: next_date.clone(),
                            event_type: crate::game::EventType::HistoricalAnnouncement {
                                title: "The Annual General Meeting".to_string(),
                                description,
                            },
                            requires_user_action: false,
                            processed: true,
                            result: None,
                        });
                    }
                    Ok(None) => {} // already closed (idempotent)
                    Err(e) => eprintln!("[ASSEMBLY] error: {}", e),
                }
                // The close-season also ages and develops every player, and the
                // summer's rest restores condition.
                match crate::fsim_bridge::age_and_develop(&pool, game.season as i64).await {
                    Ok(changed) => eprintln!("[DEVELOP] season {}: {} players' ability moved", game.season, changed),
                    Err(e) => eprintln!("[DEVELOP] error: {}", e),
                }
                // The committee reviews the user's season (standing, benefactor, no
                // confidence), and the co-op founds/grows and pays the divi.
                let pos: Option<(i64,)> = sqlx::query_as(
                    "SELECT position FROM sheffield_standings WHERE club_id = ? AND season = ? LIMIT 1")
                    .bind(&game.user_club_id).bind(game.season as i64).fetch_optional(&pool).await.ok().flatten();
                if let Some((finished,)) = pos {
                    let total: (i64,) = sqlx::query_as(
                        "SELECT COUNT(*) FROM sheffield_league_clubs WHERE division_id = \
                         (SELECT division_id FROM sheffield_league_clubs WHERE club_id = ? LIMIT 1)")
                        .bind(&game.user_club_id).fetch_one(&pool).await.unwrap_or((16,));
                    let _ = crate::fsim_bridge::review_committee_season(
                        &pool, &game.user_club_id, finished as u32, total.0.max(1) as u32,
                        game.season as i64, &next_date).await;
                }
                let _ = crate::fsim_bridge::coop_status(&pool, game.season + 1).await;
                let _ = crate::fsim_bridge::coop_pay_dividend(&pool, &game.user_club_id, game.season).await;

                // Sponsored cups: existing ones face their season, and perhaps a
                // new man of means comes forward with one of his own.
                if let Err(e) = crate::fsim_bridge::tick_sponsored_cups(&pool, game.season as i64, &next_date).await {
                    eprintln!("[SPONSORED CUP] tick error: {}", e);
                }
                match crate::fsim_bridge::maybe_propose_cup(&pool, game.season as i64 + 1).await {
                    Ok(Some(p)) => eprintln!("[SPONSORED CUP] {} proposes {}", p.sponsor_name, p.cup_name),
                    Ok(None) => {}
                    Err(e) => eprintln!("[SPONSORED CUP] propose error: {}", e),
                }

                // The club's identity may change with its fortunes, your own name
                // grows or fades, and a better club may come asking.
                let standing = crate::fsim_bridge::committee_for(&pool, &game.user_club_id).await.standing;
                match crate::fsim_bridge::season_patronage_event(
                    &pool, &game.user_club_id, standing, &next_date, game.season as i64).await {
                    Ok(Some(change)) => eprintln!("[PATRONAGE] {}: {}", game.season, change.headline()),
                    Ok(None) => {}
                    Err(e) => eprintln!("[PATRONAGE] error: {}", e),
                }
                if let Some((finished,)) = pos {
                    let total: (i64,) = sqlx::query_as(
                        "SELECT COUNT(*) FROM sheffield_league_clubs WHERE division_id = \
                         (SELECT division_id FROM sheffield_league_clubs WHERE club_id = ? LIMIT 1)")
                        .bind(&game.user_club_id).fetch_one(&pool).await.unwrap_or((16,));
                    let won_honour = finished == 1;
                    if let Err(e) = crate::fsim_bridge::secretary_after_season(
                        &pool, &game.user_club_id, finished as u32, total.0.max(1) as u32,
                        won_honour, standing, &next_date, game.season as i64).await {
                        eprintln!("[SECRETARY] error: {}", e);
                    }
                }
            }
        }
    }

    let result = game.process_day_advance();

    // Check for pending challenge invitation responses in the active game database
    let current_date = game.current_date.clone();
    let pool_result = database::get_active_game_pool(&game).await.map_err(|e| e.to_string());
    if let Ok(pool) = pool_result.clone() {
        if let Ok(processed_invitations) = database::challenge_invitations::check_pending_responses(&pool, &current_date).await {
            // Fetch all news items created for today (response news items were just created)
            if let Ok(news_items) = database::challenge_invitations::get_news_for_date(&pool, &current_date).await {
                // For each processed invitation, find its response news item and add to pending events
                for invitation in processed_invitations {
                    // Find the response news item for this invitation
                    if let Some(news_item) = news_items.iter().find(|n| {
                        n.related_invitation_id.as_deref() == Some(&invitation.id) &&
                        (n.article_type == "challenge_accepted" || n.article_type == "challenge_declined")
                    }) {
                        let event = crate::game::GameEvent {
                            id: news_item.id.clone(),
                            date: news_item.publish_date.clone(),
                            event_type: crate::game::EventType::HistoricalAnnouncement {
                                title: news_item.headline.clone(),
                                description: news_item.body_text.clone(),
                            },
                            requires_user_action: false,
                            processed: true,  // Mark as processed since we're showing it immediately
                            result: None,
                        };

                        game.pending_events.push(event);
                    }
                }
            }
        }

        // The morning post: AI clubs occasionally issue challenges of their own —
        // to rivals (resolved entirely by the pipeline) or to the user (a letter
        // in the inbox; the Hon. Secretary answers it if the user does not).
        match crate::fsim_bridge::generate_ai_challenges(&pool, &current_date, &game.user_club_id).await {
            Ok(posted) => for p in &posted {
                eprintln!("[POST] {} challenges {}{}", p.sender_club_id, p.recipient_club_id,
                          if p.to_user { " (the user's club!)" } else { "" });
            },
            Err(e) => eprintln!("[POST] error: {}", e),
        }

        // Play any accepted friendly whose match day has arrived, through the real
        // match engine (deterministic per match id). Results land in
        // sheffield_matches + sheffield_news_items; surface them as today's events.
        match crate::fsim_bridge::play_due_friendlies(&pool, &current_date).await {
            Ok(played) => {
                for p in &played {
                    eprintln!("[FRIENDLY] played {}: {} {}-{} {}",
                        p.match_id, p.home_club_id, p.home_score, p.away_score, p.away_club_id);
                    game.pending_events.push(crate::game::GameEvent {
                        id: uuid::Uuid::new_v4().to_string(),
                        date: current_date.clone(),
                        event_type: crate::game::EventType::HistoricalAnnouncement {
                            title: p.headline.clone(),
                            description: format!("Full-time: {} {}-{} {}.",
                                p.home_club_id, p.home_score, p.away_score, p.away_club_id),
                        },
                        requires_user_action: false,
                        processed: true,
                        result: None,
                    });
                }
            }
            Err(e) => eprintln!("[FRIENDLY] engine error: {}", e),
        }

        // Check if any cup competitions should announce today
        // Calculate weeks elapsed from season start (assuming Jan 7, 1867 start date for now)
        use chrono::NaiveDate;
        if let Ok(current_naive_date) = NaiveDate::parse_from_str(&current_date, "%Y-%m-%d") {
            // Get the start date for this season (using season year + Jan 7)
            if let Ok(start_date) = NaiveDate::parse_from_str(&format!("{}-01-07", game.season), "%Y-%m-%d") {
                let weeks_elapsed = (current_naive_date - start_date).num_weeks();
                eprintln!("[CUP CHECK] Current date: {}, Season: {}, Start: {}, Weeks elapsed: {}",
                    current_date, game.season, start_date.format("%Y-%m-%d"), weeks_elapsed);

                // Get all cup competitions for this season
                match database::cup_competitions::get_competitions_for_season(&pool, game.season as i64).await {
                    Ok(competitions) => {
                        eprintln!("[CUP CHECK] Found {} competitions", competitions.len());
                        for competition in competitions {
                            eprintln!("[CUP CHECK] Competition '{}': announcement_week = {}, current week = {}",
                                competition.name, competition.announcement_week, weeks_elapsed);
                            // Check if this is announcement week
                            if competition.announcement_week == weeks_elapsed {
                                eprintln!("[CUP ANNOUNCEMENT] Week {}: Announcement week for {}", weeks_elapsed, competition.name);

                                // Clone data needed for async calls to avoid borrow checker issues
                                let comp_name = competition.name.clone();
                                let comp_id = competition.id.clone();
                                let current_date_str = current_date.clone();

                                // Extract cup type (Youdan or Cromwell) for duplicate checking
                                let cup_type = if comp_name.contains("Youdan") {
                                    "Youdan"
                                } else if comp_name.contains("Cromwell") {
                                    "Cromwell"
                                } else {
                                    &comp_name
                                };

                                // Check if we've already created a news item for this announcement (avoid duplicates)
                                // Only check headline pattern, not publish_date, since announcement_week spans 7 days
                                let headline_pattern = format!("%{}%Announced%", cup_type);
                                let existing_news: Result<Option<(String,)>, _> = sqlx::query_as(
                                    "SELECT id FROM sheffield_news_items WHERE headline LIKE ? AND article_type = 'cup_announcement'"
                                )
                                .bind(&headline_pattern)
                                .fetch_optional(&pool)
                                .await;

                                match existing_news {
                                    Ok(Some(_)) => {
                                        eprintln!("[CUP CHECK] Announcement news for {} already exists in database, skipping", comp_name);
                                    },
                                    Ok(None) => {
                                        eprintln!("[CUP ANNOUNCEMENT] Creating news item for {}", comp_name);
                                        // Clone the competition to pass ownership to async function
                                        let comp_clone = competition.clone();
                                        match database::cup_competitions::create_cup_announcement_news(
                                            &pool,
                                            &comp_clone,
                                            &current_date_str,
                                        ).await {
                                            Ok(news_id) => {
                                                eprintln!("[CUP ANNOUNCEMENT] Successfully created announcement news with ID: {}", news_id);
                                            },
                                            Err(e) => {
                                                eprintln!("[CUP ANNOUNCEMENT] Error creating announcement news: {}", e);
                                            }
                                        }
                                    },
                                    Err(e) => {
                                        eprintln!("[CUP CHECK] Error checking for existing announcement: {}", e);
                                    }
                                }
                            }

                            // Check if this is draw week
                            if competition.draw_week == weeks_elapsed {
                                eprintln!("[CUP DRAW] Week {}: Draw week for {}", weeks_elapsed, competition.name);

                                // Clone data needed for async calls
                                let comp_name = competition.name.clone();
                                let comp_id = competition.id.clone();
                                let current_date_str = current_date.clone();

                                // Extract cup type for duplicate checking
                                let cup_type = if comp_name.contains("Youdan") {
                                    "Youdan"
                                } else if comp_name.contains("Cromwell") {
                                    "Cromwell"
                                } else {
                                    &comp_name
                                };

                                // Check if draw has already been made (avoid duplicates)
                                let headline_pattern = format!("%{}%Draw%", cup_type);
                                let existing_draw_news: Result<Option<(String,)>, _> = sqlx::query_as(
                                    "SELECT id FROM sheffield_news_items WHERE headline LIKE ? AND article_type = 'cup_draw'"
                                )
                                .bind(&headline_pattern)
                                .fetch_optional(&pool)
                                .await;

                                match existing_draw_news {
                                    Ok(Some(_)) => {
                                        eprintln!("[CUP DRAW] Draw news for {} already exists in database, skipping", comp_name);
                                    },
                                    Ok(None) => {
                                        eprintln!("[CUP DRAW] Generating draw for {}", comp_name);

                                        // Generate the cup draw (unseeded for Youdan Cup 1867)
                                        if let Err(e) = database::cup_competitions::generate_cup_draw(&pool, &comp_id, false).await {
                                            eprintln!("[CUP DRAW] Error generating draw: {}", e);
                                            continue;
                                        }

                                        eprintln!("[CUP DRAW] Successfully generated draw");

                                        // Fetch the bracket to get the ties
                                        let bracket = match database::cup_competitions::get_cup_bracket(&pool, &comp_id).await {
                                            Ok(b) => b,
                                            Err(e) => {
                                                eprintln!("[CUP DRAW] Error fetching bracket: {}", e);
                                                continue;
                                            }
                                        };
                                        eprintln!("[CUP DRAW] Bracket has {} ties", bracket.len());

                                        // Create a news item for the draw
                                        let (title, description) = database::cup_competitions::get_draw_announcement_text(&comp_name, game.season as i64, &bracket);

                                        let news_id = uuid::Uuid::new_v4().to_string();
                                        if let Err(e) = sqlx::query(
                                            "INSERT INTO sheffield_news_items (id, headline, body_text, publish_date, article_type, is_read)
                                             VALUES (?, ?, ?, ?, ?, ?)"
                                        )
                                        .bind(&news_id)
                                        .bind(&title)
                                        .bind(&description)
                                        .bind(&current_date_str)
                                        .bind("cup_draw")
                                        .bind(false)
                                        .execute(&pool)
                                        .await {
                                            eprintln!("[CUP DRAW] Error creating draw news: {}", e);
                                        } else {
                                            eprintln!("[CUP DRAW] Successfully created draw news with ID: {}", news_id);
                                        }

                                        // Convert bracket to JSON for event
                                        let bracket_json: Vec<serde_json::Value> = bracket.iter().map(|tie| {
                                            serde_json::json!({
                                                "round": tie.round_number,
                                                "tieNumber": tie.tie_number,
                                                "homeClub": tie.home_club_name,
                                                "awayClub": tie.away_club_name,
                                            })
                                        }).collect();

                                        // Add CupDraw event to pending events
                                        let draw_event = crate::game::GameEvent {
                                            id: uuid::Uuid::new_v4().to_string(),
                                            date: current_date_str.clone(),
                                            event_type: crate::game::EventType::CupDraw {
                                                cup_name: comp_name.clone(),
                                                competition_id: comp_id.clone(),
                                                title: title.clone(),
                                                description: description.clone(),
                                                draw_bracket: bracket_json,
                                            },
                                            requires_user_action: false,
                                            processed: true,  // Mark as processed since we're showing it immediately
                                            result: None,
                                        };

                                        game.pending_events.push(draw_event);
                                        eprintln!("[CUP DRAW] Added CupDraw event to pending events");
                                    },
                                    Err(e) => {
                                        eprintln!("[CUP DRAW] Error checking for existing draw: {}", e);
                                    }
                                }
                            }
                        }
                    },
                    Err(e) => {
                        eprintln!("[CUP CHECK] Error loading competitions: {}", e);
                    }
                }
            } else {
                eprintln!("[CUP CHECK] Failed to parse start date for season {}", game.season);
            }
        } else {
            eprintln!("[CUP CHECK] Failed to parse current date: {}", current_date);
        }

        // Load cup match fixtures from database
        eprintln!("[CUP_MATCHES] Creating/loading cup match fixtures...");
        let start_date_str = format!("{}-01-07", game.season);
        match database::create_cup_match_fixtures(game.season as i64, &start_date_str).await {
            Ok(cup_matches) => {
                if !cup_matches.is_empty() {
                    eprintln!("[CUP_MATCHES] Created {} cup match fixtures", cup_matches.len());
                    // Add cup matches to game state
                    game.matches.extend(cup_matches);
                }
            },
            Err(e) => {
                eprintln!("[CUP_MATCHES] Error creating cup match fixtures: {}", e);
            }
        }

        // Load friendly matches from database and add them to game state
        eprintln!("[FRIENDLY_MATCHES] Loading friendly matches from database...");
        match database::challenge_invitations::load_friendly_matches(&pool).await {
            Ok(friendly_matches) => {
                eprintln!("[FRIENDLY_MATCHES] Found {} friendly matches", friendly_matches.len());
                // Remove any existing friendly matches (gameweek = 0) to avoid duplicates
                // Note: This also removes cup matches (also gameweek = 0), but we re-add them above
                game.matches.retain(|m| m.gameweek != 0);
                // Add the loaded friendly matches
                game.matches.extend(friendly_matches);
                eprintln!("[FRIENDLY_MATCHES] Game now has {} total matches", game.matches.len());
            },
            Err(e) => {
                eprintln!("[FRIENDLY_MATCHES] Error loading friendly matches: {}", e);
            }
        }
    }

    eprintln!("[ADVANCE_DAY] Game now has {} pending events", game.pending_events.len());
    for (i, event) in game.pending_events.iter().enumerate() {
        eprintln!("[ADVANCE_DAY]   Event {}: {:?} on {}", i, event.event_type, event.date);
    }

    // Process cup match results and generate next rounds if needed
    if let Ok(pool) = pool_result.clone() {
        eprintln!("[CUP_PROCESSING] Checking for completed cup matches...");

        // Find all played cup matches (gameweek = 0) and update corresponding cup ties
        let cup_matches: Vec<&crate::game::Match> = game.matches
            .iter()
            .filter(|m| m.gameweek == 0 && m.played && m.home_score.is_some() && m.away_score.is_some())
            .collect();

        eprintln!("[CUP_PROCESSING] Found {} played cup matches", cup_matches.len());

        for cup_match in cup_matches {
            // Find the corresponding cup tie by match_id
            let tie_result: Result<Option<(String,)>, _> = sqlx::query_as(
                "SELECT id FROM sheffield_cup_ties WHERE match_id = ? AND played = 0"
            )
            .bind(&cup_match.id)
            .fetch_optional(&pool)
            .await;

            if let Ok(Some((tie_id,))) = tie_result {
                eprintln!("[CUP_PROCESSING] Updating tie {} with result {} - {}",
                    tie_id, cup_match.home_score.unwrap(), cup_match.away_score.unwrap());

                // Update the tie with the result
                if let Err(e) = database::update_cup_tie_result(
                    &pool,
                    &tie_id,
                    cup_match.home_score.unwrap() as i64,
                    cup_match.away_score.unwrap() as i64
                ).await {
                    eprintln!("[CUP_PROCESSING] Error updating cup tie: {}", e);
                }
            }
        }

        // Check if any rounds are complete and generate next rounds
        let processing_succeeded = {
            match database::cup_competitions::process_cup_match_results(&pool, game.season as i64).await {
                Ok(_) => {
                    eprintln!("[CUP_PROCESSING] Cup processing complete");
                    true
                }
                Err(e) => {
                    eprintln!("[CUP_PROCESSING] Error processing cup results: {}", e);
                    false
                }
            }
        };

        if processing_succeeded {
            // Reload cup matches as new rounds may have been created
            let start_date_str = format!("{}-01-07", game.season);
            match database::create_cup_match_fixtures(game.season as i64, &start_date_str).await {
                Ok(new_cup_matches) => {
                    if !new_cup_matches.is_empty() {
                        eprintln!("[CUP_PROCESSING] Created {} new cup match fixtures for next round", new_cup_matches.len());
                        // Remove old cup matches and add new ones
                        game.matches.retain(|m| m.gameweek != 0 || m.played);
                        game.matches.extend(new_cup_matches);
                    }
                },
                Err(e) => {
                    eprintln!("[CUP_PROCESSING] Error creating new cup fixtures: {}", e);
                }
            }
        }
    }

    // Auto-save to loaded file if it was loaded from one
    if let Some(save_file) = &game.loaded_from_file {
        let sanitized_name = save_file.trim_end_matches(".json")
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
            .collect::<String>();
        database::save_game_named(&game, &sanitized_name).await.map_err(|e| e.to_string())?;
    } else {
        database::save_game(&game).await.map_err(|e| e.to_string())?;
    }

    Ok(result)
}

#[tauri::command]
pub async fn get_standings(game: GameState) -> Result<Vec<crate::game::Standing>, String> {
    Ok(game.standings)
}

/// Arrange a training session for the user's club. Casual by nature — who turns up
/// is composed from each man's census life (his trade, household, neighbours,
/// temperament). Attendees sharpen up; absentees are named. Deterministic per day.
#[tauri::command]
pub async fn arrange_training(game: GameState) -> Result<serde_json::Value, String> {
    let pool = database::get_active_game_pool(&game).await.map_err(|e| e.to_string())?;
    let seed = crate::fsim_bridge::seed_from(&format!("training-{}-{}", game.user_club_id, game.current_date));
    let report = crate::fsim_bridge::hold_training(&pool, &game.user_club_id, seed)
        .await.map_err(|e| e.to_string())?;
    Ok(serde_json::json!({
        "present": report.present,
        "absent": report.absent,
        "turnout": report.present.len(),
        "invited": report.present.len() + report.absent.len(),
    }))
}

/// Muster the user's club before a match — who is available, who is missing (and
/// why), and how many ringers must be found. Deterministic per match day.
#[tauri::command]
pub async fn muster_squad(game: GameState) -> Result<serde_json::Value, String> {
    let pool = database::get_active_game_pool(&game).await.map_err(|e| e.to_string())?;
    let seed = crate::fsim_bridge::seed_from(&format!("muster-{}-{}", game.user_club_id, game.current_date));
    let report = crate::fsim_bridge::muster_squad(&pool, &game.user_club_id, seed)
        .await.map_err(|e| e.to_string())?;
    Ok(serde_json::json!({
        "available": report.available,
        "missing": report.missing.iter().map(|(n, r)| serde_json::json!({"name": n, "reason": r})).collect::<Vec<_>>(),
        "ringersNeeded": report.ringers_needed,
        "ringers": report.ringers,
    }))
}

/// The committee's standing with the user's club — the members' confidence, their
/// verdict, whether they're imposing men on the side, and any benefactor.
#[tauri::command]
pub async fn committee_status(game: GameState) -> Result<serde_json::Value, String> {
    let pool = database::get_active_game_pool(&game).await.map_err(|e| e.to_string())?;
    let c = crate::fsim_bridge::committee_for(&pool, &game.user_club_id).await;
    Ok(serde_json::json!({
        "standing": c.standing,
        "verdict": format!("{:?}", c.verdict()),
        "selectionInterference": c.selection_interference(),
        "hasBenefactor": c.has_benefactor,
    }))
}

/// The club's identity and your own standing in the game — what institution is
/// behind the side, what that costs it, your name, and the club's rivals.
#[tauri::command]
pub async fn club_identity(game: GameState) -> Result<serde_json::Value, String> {
    let pool = database::get_active_game_pool(&game).await.map_err(|e| e.to_string())?;
    let p = crate::fsim_bridge::patronage_for(&pool, &game.user_club_id).await;
    let s = crate::fsim_bridge::secretary_for(&pool).await;
    let _ = crate::fsim_bridge::discover_rivalries(&pool, &game.user_club_id).await;
    let rivals: Vec<(String, String, i64)> = sqlx::query_as(
        "SELECT club_a, club_b, intensity FROM sheffield_rivalries WHERE club_a = ? OR club_b = ? \
         ORDER BY intensity DESC LIMIT 6")
        .bind(&game.user_club_id).bind(&game.user_club_id)
        .fetch_all(&pool).await.unwrap_or_default();
    Ok(serde_json::json!({
        "patronage": format!("{:?}", p),
        "patronageLabel": p.label(),
        "constraint": p.constraint(),
        "freeHand": p.free_hand(),
        "gateSharePct": p.gate_share_pct(),
        "localAffinity": p.local_affinity(),
        "secretary": { "name": s.name, "reputation": s.reputation,
                       "standing": s.standing_words(), "honours": s.honours,
                       "seasons": s.seasons_served },
        "rivals": rivals.iter().map(|(a, b, i)| {
            let other = if a == &game.user_club_id { b } else { a };
            serde_json::json!({ "club": other, "intensity": i })
        }).collect::<Vec<_>>(),
    }))
}

/// The sponsored cups currently running (or lapsed), for the fixture/history view.
#[tauri::command]
pub async fn sponsored_cups(game: GameState) -> Result<serde_json::Value, String> {
    let pool = database::get_active_game_pool(&game).await.map_err(|e| e.to_string())?;
    let rows: Vec<(String, String, i64, i64, i64, i64, i64, i64, i64)> = sqlx::query_as(
        "SELECT sponsor_name, cup_name, season_proposed, entrants, prize_shillings, \
         min_division_level, max_division_level, seasons_held, sponsor_still_backing \
         FROM sheffield_sponsored_cups ORDER BY season_proposed DESC")
        .fetch_all(&pool).await.map_err(|e| e.to_string())?;
    Ok(serde_json::json!(rows.iter().map(|r| serde_json::json!({
        "sponsorName": r.0, "cupName": r.1, "seasonProposed": r.2, "entrants": r.3,
        "prizeShillings": r.4, "minDivisionLevel": r.5, "maxDivisionLevel": r.6,
        "seasonsHeld": r.7, "stillBacking": r.8 != 0,
    })).collect::<Vec<_>>()))
}

/// The Co-operative Society's state and the user's club's membership.
#[tauri::command]
pub async fn coop_status(game: GameState) -> Result<serde_json::Value, String> {
    let pool = database::get_active_game_pool(&game).await.map_err(|e| e.to_string())?;
    let society = crate::fsim_bridge::coop_status(&pool, game.season).await;
    Ok(serde_json::json!({
        "founded": society.is_some(),
        "membership": society.map(|s| s.membership),
        "dividendRatePct": society.map(|s| s.dividend_rate_x10 as f64 / 10.0),
    }))
}

/// Buy goods for the user's club through the co-op (kit/ball/goalposts/…).
#[tauri::command]
pub async fn coop_buy(game: GameState, item: String) -> Result<serde_json::Value, String> {
    use fsim_core::coop::Purchase::*;
    let pool = database::get_active_game_pool(&game).await.map_err(|e| e.to_string())?;
    let purchase = match item.as_str() {
        "kit" => Kit, "ball" => Ball, "goalposts" => Goalposts,
        "shin_guards" => ShinGuards, "hall" => HallRental, _ => Refreshments,
    };
    crate::fsim_bridge::coop_join(&pool, &game.user_club_id, "the club", &game.current_date)
        .await.map_err(|e| e.to_string())?;
    let cost = crate::fsim_bridge::coop_purchase(&pool, &game.user_club_id, purchase, &game.current_date)
        .await.map_err(|e| e.to_string())?;
    Ok(serde_json::json!({ "boughtShillings": cost }))
}

#[tauri::command]
pub async fn get_squad(game: GameState, club_id: String) -> Result<Vec<crate::game::Player>, String> {
    Ok(game.players.into_iter().filter(|p| p.club_id == club_id).collect())
}

#[tauri::command]
pub async fn get_all_clubs() -> Result<Vec<ClubInfo>, String> {
    // Return all clubs from sheffield_rules, not from database
    // This ensures we get all 187 historical clubs
    let clubs = sheffield_rules::get_sheffield_clubs(None);
    let club_infos: Vec<ClubInfo> = clubs.iter().map(|c| {
        ClubInfo {
            id: c.id.clone(),
            name: c.name.clone(),
            short_name: c.name.chars().take(3).collect(),
            founded_year: c.founded_year as i32,
            ground_name: c.ground.clone(),
            ground_capacity: 0,
            city: c.city.clone().unwrap_or_default(),
            region: c.region.clone().unwrap_or_default(),
            origin: c.origin.clone(),
            primary_color: String::new(),
            secondary_color: String::new(),
            reserve_of_club_id: None,
            parent_club_name: None,
            where_from: None,
            additional_postcode: None,
        }
    }).collect();
    Ok(club_infos)
}

#[tauri::command]
pub async fn get_club_squad(club_id: String) -> Result<Vec<PlayerDetail>, String> {
    database::get_club_squad(&club_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_club_info(club_id: String) -> Result<ClubInfo, String> {
    database::get_club_info(&club_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_reserve_team_id(parent_club_id: String) -> Result<Option<String>, String> {
    database::get_reserve_team_id(&parent_club_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn complete_user_event(
    mut game: GameState,
    event_id: String,
    user_response: serde_json::Value,
) -> Result<GameState, String> {
    // Find event in pending_events
    if let Some(event) = game
        .pending_events
        .iter_mut()
        .find(|e| e.id == event_id)
    {
        // Mark as processed
        event.processed = true;
    }

    // Move to processed
    let processed: Vec<_> = game
        .pending_events
        .iter()
        .filter(|e| e.processed)
        .cloned()
        .collect();
    game.processed_events.extend(processed);
    game.pending_events.retain(|e| !e.processed);

    // Auto-save to loaded file if it was loaded from one
    if let Some(save_file) = &game.loaded_from_file {
        let sanitized_name = save_file.trim_end_matches(".json")
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
            .collect::<String>();
        database::save_game_named(&game, &sanitized_name).await.map_err(|e| e.to_string())?;
    } else {
        database::save_game(&game).await.map_err(|e| e.to_string())?;
    }

    Ok(game)
}

#[tauri::command]
pub async fn get_upcoming_fixtures(game: GameState, days: u8) -> Result<Vec<Match>, String> {
    let current_date = NaiveDate::parse_from_str(&game.current_date, "%Y-%m-%d")
        .map_err(|e| e.to_string())?;

    let end_date = current_date + chrono::Duration::days(days as i64);

    let fixtures: Vec<Match> = game
        .matches
        .iter()
        .filter(|m| {
            if let Ok(match_date) = NaiveDate::parse_from_str(&m.date, "%Y-%m-%d") {
                match_date >= current_date && match_date <= end_date && !m.played
            } else {
                false
            }
        })
        .cloned()
        .collect();

    Ok(fixtures)
}

// ============================================================================
// SHEFFIELD RULES GAME MODES
// ============================================================================

/// Get available Sheffield Rules game modes for selection
#[tauri::command]
pub async fn get_sheffield_game_modes() -> Result<Vec<sheffield_rules::GameModeOption>, String> {
    Ok(sheffield_rules::game_mode_options())
}

/// Create a new Sheffield Rules game with the selected game mode
#[tauri::command]
pub async fn new_sheffield_game(
    game_mode_option_id: String,
    club_id: String,
) -> Result<GameState, String> {
    let game_mode = sheffield_rules::create_game_mode_from_option(&game_mode_option_id)?;

    let mut game = GameState::new(club_id);
    // Store game mode metadata (would need to extend GameState)
    // For now, this just creates a normal game with the selected mode available

    database::save_game(&game).await.map_err(|e| e.to_string())?;
    Ok(game)
}

/// Get the ruleset for a specific season in a Sheffield Rules game
#[tauri::command]
pub async fn get_sheffield_ruleset_for_season(
    season_year: u32,
    game_mode_option_id: String,
) -> Result<String, String> {
    let game_mode = sheffield_rules::create_game_mode_from_option(&game_mode_option_id)?;

    let ruleset = game_mode.get_ruleset_for_season(season_year);
    Ok(format!(
        "Year: {}, Rules: {}, Goal Width: {:?} ft, Goal Height: {:?} ft, Has Rouge: {}",
        season_year,
        ruleset.name(),
        ruleset.goal_width(),
        ruleset.goal_height(),
        ruleset.scoring_system().has_rouge()
    ))
}

/// Get all available Sheffield Rules years for the ruleset explorer
#[tauri::command]
pub async fn get_all_sheffield_years() -> Result<Vec<(u32, String)>, String> {
    let years = sheffield_rules::all_ruleset_years();
    let result = years
        .iter()
        .map(|(year, name)| (*year, name.to_string()))
        .collect();
    Ok(result)
}

/// Get summary of rule changes for a specific year
#[tauri::command]
pub async fn get_sheffield_year_summary(year: u32) -> Result<String, String> {
    let summary = sheffield_rules::year_changes_summary(year);
    Ok(summary.to_string())
}

/// Get all Sheffield clubs from Sheffield1867.db
/// Returns clubs with database column mapping (ground_name -> ground in struct)
#[tauri::command]
pub async fn get_sheffield_clubs_from_db() -> Result<Vec<serde_json::Value>, String> {
    use sqlx::Row;

    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = database::sheffield_db::create_sheffield_database(db_path)
        .await
        .map_err(|e| format!("Failed to open database: {}", e))?;

    let rows = sqlx::query(
        "SELECT id, name, founded_year, ground_name, origin FROM sheffield_clubs ORDER BY name"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| format!("Failed to fetch clubs: {}", e))?;

    let clubs: Vec<serde_json::Value> = rows.iter().map(|row| {
        serde_json::json!({
            "id": row.get::<String, _>("id"),
            "name": row.get::<String, _>("name"),
            "founded_year": row.get::<i32, _>("founded_year"),
            "ground_name": row.get::<String, _>("ground_name"),
            "origin": row.get::<String, _>("origin"),
        })
    }).collect();

    Ok(clubs)
}

/// Get all Sheffield clubs (hardcoded - legacy)
#[tauri::command]
pub async fn get_sheffield_clubs(filter_year: Option<u32>) -> Result<Vec<sheffield_rules::SheffieldClub>, String> {
    Ok(sheffield_rules::get_sheffield_clubs(filter_year))
}

/// Get clubs available for a specific game mode at its start year
#[tauri::command]
pub async fn get_sheffield_clubs_for_mode(
    game_mode_option_id: String,
) -> Result<Vec<sheffield_rules::SheffieldClub>, String> {
    let game_mode = sheffield_rules::create_game_mode_from_option(&game_mode_option_id)?;

    // Sheffield & Hallamshire League uses ALL historical clubs, regardless of founding year
    if matches!(game_mode, sheffield_rules::GameMode::SheffieldAndHallamshireLeague { .. }) {
        return Ok(sheffield_rules::get_sheffield_clubs(None)); // None = all clubs
    }

    // Extract the start year from the game mode
    let start_year = match game_mode {
        sheffield_rules::GameMode::HistoricalTimeline => 1857,
        sheffield_rules::GameMode::AhistoricalSingleRuleset { year } => year,
        sheffield_rules::GameMode::HistoricalFromYear { start_year } => start_year,
        sheffield_rules::GameMode::SheffieldAndHallamshireLeague { .. } => unreachable!(),
    };

    Ok(sheffield_rules::get_clubs_for_game_mode(start_year))
}

/// Get the division and position information for a club in the Sheffield & Hallamshire League
#[tauri::command]
pub async fn get_club_league_division(
    club_id: String,
) -> Result<serde_json::Value, String> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = database::sheffield_db::create_sheffield_database(db_path)
        .await
        .map_err(|e| format!("Failed to create database: {}", e))?;

    // Get the club's division assignment
    let row = sqlx::query(
        "SELECT division_id, position_in_division, is_reserve_team, reserve_of_club_id
         FROM sheffield_league_clubs
         WHERE club_id = ?"
    )
    .bind(&club_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| format!("Database query failed: {}", e))?;

    if let Some(row) = row {
        use sqlx::Row;
        let division_id: String = row.get(0);
        let position: i32 = row.get(1);
        let is_reserve: bool = row.get(2);
        let reserve_of: Option<String> = row.get(3);

        Ok(serde_json::json!({
            "club_id": club_id,
            "division_id": division_id,
            "position_in_division": position,
            "is_reserve_team": is_reserve,
            "reserve_of_club_id": reserve_of,
        }))
    } else {
        Err(format!("Club '{}' not found in Sheffield league", club_id))
    }
}

/// Get division name and info for a specific club
#[tauri::command]
pub async fn get_club_division_info(
    club_id: String,
) -> Result<serde_json::Value, String> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867_temp.db";
    let pool = database::sheffield_db::create_sheffield_database(db_path)
        .await
        .map_err(|e| format!("Failed to create database: {}", e))?;

    // Get the club's division
    let row = sqlx::query(
        "SELECT slc.division_id, sld.name, slc.position_in_division
         FROM sheffield_league_clubs slc
         JOIN sheffield_league_divisions sld ON slc.division_id = sld.id
         WHERE slc.club_id = ?"
    )
    .bind(&club_id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| format!("Database query failed: {}", e))?;

    if let Some(row) = row {
        use sqlx::Row;
        let division_id: String = row.get(0);
        let division_name: String = row.get(1);
        let position: i32 = row.get(2);

        Ok(serde_json::json!({
            "division_id": division_id,
            "division_name": division_name,
            "position": position,
        }))
    } else {
        Err(format!("Club '{}' not found in Sheffield league", club_id))
    }
}

/// Get all clubs in the same division as the specified club (structural data only - no match stats)
#[tauri::command]
pub async fn get_division_clubs(
    division_id: String,
) -> Result<Vec<serde_json::Value>, String> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = database::sheffield_db::create_sheffield_database(db_path)
        .await
        .map_err(|e| format!("Failed to create database: {}", e))?;

    // Get all clubs in this division with division info
    let rows = sqlx::query(
        "SELECT slc.club_id, slc.position_in_division, sc.name, slc.is_reserve_team,
                sc.founded_year, sld.name as division_name, slc.division_id, sc.region
         FROM sheffield_league_clubs slc
         JOIN sheffield_clubs sc ON slc.club_id = sc.id
         JOIN sheffield_league_divisions sld ON slc.division_id = sld.id
         WHERE slc.division_id = ?
         ORDER BY slc.position_in_division ASC"
    )
    .bind(&division_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| format!("Database query failed: {}", e))?;

    let clubs = rows.iter().map(|row| {
        use sqlx::Row;
        serde_json::json!({
            "club_id": row.get::<String, _>(0),
            "position": row.get::<i32, _>(1),
            "club_name": row.get::<String, _>(2),
            "founded_year": row.get::<i32, _>(4),
            "division_name": row.get::<String, _>(5),
            "division_id": row.get::<String, _>(6),
            "postcode_area": row.get::<Option<String>, _>(7),
        })
    }).collect();

    Ok(clubs)
}

/// Get division standings with match statistics
/// Joins sheffield_standings with sheffield_league_clubs to get division-filtered standings
#[tauri::command]
pub async fn get_division_standings(
    division_id: String,
    season: i32,
) -> Result<Vec<serde_json::Value>, String> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867_temp.db";
    let pool = database::sheffield_db::create_sheffield_database(db_path)
        .await
        .map_err(|e| format!("Failed to create database: {}", e))?;

    println!("🔍 get_division_standings called: division={}, season={}", division_id, season);

    // Use LEFT JOIN to show all clubs in division even if no match data yet
    let rows = sqlx::query(
        "SELECT
            slc.club_id,
            sc.name as club_name,
            COALESCE(ss.played, 0) as played,
            COALESCE(ss.won, 0) as won,
            COALESCE(ss.drawn, 0) as drawn,
            COALESCE(ss.lost, 0) as lost,
            COALESCE(ss.goals_for, 0) as goals_for,
            COALESCE(ss.goals_against, 0) as goals_against,
            COALESCE(ss.goal_difference, 0) as goal_difference,
            COALESCE(ss.rouges_for, 0) as rouges_for,
            COALESCE(ss.rouges_against, 0) as rouges_against,
            COALESCE(ss.points, 0) as points,
            slc.division_id,
            sld.name as division_name
         FROM sheffield_league_clubs slc
         JOIN sheffield_clubs sc ON slc.club_id = sc.id
         JOIN sheffield_league_divisions sld ON slc.division_id = sld.id
         LEFT JOIN sheffield_standings ss ON slc.club_id = ss.club_id AND ss.season = ?
         WHERE slc.division_id = ?
         ORDER BY COALESCE(ss.points, 0) DESC, COALESCE(ss.goal_difference, 0) DESC, sc.name ASC"
    )
    .bind(season)
    .bind(&division_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| format!("Database query failed: {}", e))?;

    println!("📊 Query returned {} rows", rows.len());

    let standings: Vec<serde_json::Value> = rows.iter().enumerate().map(|(index, row)| {
        use sqlx::Row;
        serde_json::json!({
            "position": index + 1,
            "club_id": row.get::<String, _>(0),
            "club_name": row.get::<String, _>(1),
            "played": row.get::<i32, _>(2),
            "won": row.get::<i32, _>(3),
            "drawn": row.get::<i32, _>(4),
            "lost": row.get::<i32, _>(5),
            "goals_for": row.get::<i32, _>(6),
            "goals_against": row.get::<i32, _>(7),
            "goal_difference": row.get::<i32, _>(8),
            "rouges_for": row.get::<i32, _>(9),
            "rouges_against": row.get::<i32, _>(10),
            "points": row.get::<i32, _>(11),
            "division_id": row.get::<String, _>(12),
            "division_name": row.get::<String, _>(13),
        })
    }).collect();

    if !standings.is_empty() {
        println!("📋 First standing: {:?}", standings[0]);
    }

    Ok(standings)
}

/// Initialize cup competitions for a season and generate announcement/draw events
async fn initialize_season_cup_competitions(
    pool: &sqlx::SqlitePool,
    year: u32,
    start_date: &str,
) -> Result<Vec<crate::game::GameEvent>, Box<dyn std::error::Error>> {
    use crate::database::cup_competitions;

    let mut events = Vec::new();

    eprintln!("   Querying cup competitions for {} season...", year);
    // Get all competitions for this season
    let competitions = cup_competitions::get_competitions_for_season(pool, year as i64)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    if competitions.is_empty() {
        eprintln!("   ℹ️  No cup competitions found for {} season", year);
        return Ok(events);
    }

    eprintln!("   ✅ Found {} cup competition(s):", competitions.len());
    for comp in &competitions {
        eprintln!("      • {} (Divisions {}-{})", comp.name, comp.min_division_level.unwrap_or(1), comp.max_division_level.unwrap_or(10));
    }
    eprintln!();

    // Parse start date to calculate event dates
    let base_date = chrono::NaiveDate::parse_from_str(start_date, "%Y-%m-%d")?;

    eprintln!("   Scheduling cup events:");
    for (i, competition) in competitions.iter().enumerate() {
        eprintln!("\n   [{}/{}] {}:", i + 1, competitions.len(), competition.name);

        // Calculate announcement date (announcement_week weeks from start date)
        let announcement_date = base_date + chrono::Duration::weeks(competition.announcement_week);
        let announcement_date_str = announcement_date.format("%Y-%m-%d").to_string();

        eprintln!("      📰 Announcement: Week {} ({}) - will be generated dynamically", competition.announcement_week, announcement_date.format("%B %d, %Y"));

        // Note: Announcement events are now generated dynamically in advance_day when the date is reached
        // This prevents empty placeholder events from appearing from day 1

        // Generate the cup draw if it's time (at draw week)
        // For initialization, we'll generate the draw immediately if announcement_week == draw_week
        if competition.announcement_week == competition.draw_week {
            eprintln!("      🎲 Draw: Week {} (same as announcement)", competition.draw_week);

            // Generate draw
            cup_competitions::generate_cup_draw(pool, &competition.id, false).await?;

            // Calculate draw date
            let draw_date = base_date + chrono::Duration::weeks(competition.draw_week);
            let draw_date_str = draw_date.format("%Y-%m-%d").to_string();

            // Generate draw event
            let draw_event = cup_competitions::create_cup_draw_event(
                pool,
                &competition,
                &draw_date_str,
            ).await?;
            events.push(draw_event);
        } else {
            eprintln!("      🎲 Draw: Week {} ({})", competition.draw_week, (base_date + chrono::Duration::weeks(competition.draw_week)).format("%B %d, %Y"));
        }

        let start_date_formatted = (base_date + chrono::Duration::weeks(competition.start_week)).format("%B %d, %Y");
        eprintln!("      ⚽ Start: Week {} ({})", competition.start_week, start_date_formatted);
    }

    Ok(events)
}

/// Initialize a new Sheffield Rules game with database setup
#[tauri::command]
pub async fn initialize_sheffield_game(
    year: u32,
    game_mode: String,  // "historical-timeline" | "ahistorical-1862" | etc.
    club_id: Option<String>,
) -> Result<GameState, String> {
    // ========================================================================
    // INITIALIZATION START
    // ========================================================================
    eprintln!("\n╔══════════════════════════════════════════════════════════════════════════╗");
    eprintln!("║  INITIALIZING SHEFFIELD & HALLAMSHIRE FANTASY LEAGUE - 1867             ║");
    eprintln!("╚══════════════════════════════════════════════════════════════════════════╝\n");
    eprintln!("⚙️  Mode: {}", game_mode);
    eprintln!("📅 Year: {}", year);
    eprintln!("🏟️  Club: {:?}\n", club_id);

    // Determine if ahistorical or fantasy league (both should load ALL clubs)
    let is_ahistorical = game_mode.starts_with("ahistorical");
    let is_fantasy_league = game_mode == "sheffield-hallamshire-league";
    let load_all_clubs = is_ahistorical || is_fantasy_league;

    let master_db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let temp_db_path = "D:/projects/Saturday at Three/Sheffield1867_temp.db";
    let other_modes_db_path = "D:/projects/Saturday at Three/saturday_at_three.db";

    // Sheffield1867.db is ONLY ever touched by sheffield-hallamshire-league mode.
    // All other modes use saturday_at_three.db so player data in the master DB is never at risk.
    if is_fantasy_league {
        eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        eprintln!("📋 STEP 1/7: Copying Master Database");
        eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        eprintln!("   Source: Sheffield1867.db");
        eprintln!("   Target: Sheffield1867_temp.db");
        eprintln!("   Purpose: Preserve master data during gameplay\n");

        std::fs::copy(master_db_path, temp_db_path)
            .map_err(|e| format!("Failed to copy database: {}", e))?;

        eprintln!("   ✅ Database copied successfully\n");
    }

    let db_path = if is_fantasy_league {
        temp_db_path
    } else {
        other_modes_db_path
    };

    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("📋 STEP 2/7: Opening Database Connection");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("   Database: {}\n", db_path);

    let pool = database::sheffield_db::create_sheffield_database(db_path)
        .await
        .map_err(|e| format!("Failed to create database: {}", e))?;

    eprintln!("   ✅ Database connection established\n");

    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("📋 STEP 3/7: Migrating Database Schema");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // Only migrate sheffield_footballers for fantasy league mode (Sheffield1867.db)
    // Other modes use saturday_at_three.db which doesn't have this table
    if is_fantasy_league {
        eprintln!("   Checking sheffield_footballers table for updates...\n");

        // Migrate sheffield_footballers to add any columns that were added after the DB was first created
        database::migrate_sheffield_footballers(&pool)
            .await
            .map_err(|e| format!("Failed to migrate sheffield_footballers: {}", e))?;

        eprintln!("   ✅ Schema migration complete\n");
    } else {
        eprintln!("   Skipping sheffield_footballers migration (not fantasy league mode)\n");
    }

    // For fantasy league: the copied DB already has clubs, players, league structure, and
    // standings set up by the editor. Just clear game-state tables, initialize standings
    // fresh, save the ruleset, and go.
    // For other modes: initialize schema, clear tables, populate from hardcoded data.
    let (rule_year, selected_club, clubs) = if is_fantasy_league {
        let selected_club = club_id.ok_or_else(|| "No club selected".to_string())?;

        eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        eprintln!("📋 STEP 4/7: Clearing Previous Game State");
        eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        eprintln!("   Preserving: clubs, players, league structure, competitions");
        eprintln!("   Clearing: game state, standings, fixtures, matches, history\n");

        // Clear only game-state tables — preserve clubs, players, league structure
        sqlx::query("PRAGMA foreign_keys = OFF")
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to disable FK: {}", e))?;

        let tables_to_clear = vec![
            "sheffield_game_state",
            "sheffield_standings",
            "sheffield_fixtures",
            "sheffield_matches",
            "sheffield_match_incidents",
            "sheffield_rules_history"
        ];

        for (i, table) in tables_to_clear.iter().enumerate() {
            eprintln!("   [{}/{}] Clearing {}...", i + 1, tables_to_clear.len(), table);
            sqlx::query(&format!("DELETE FROM {}", table))
                .execute(&pool)
                .await
                .map_err(|e| format!("Failed to clear {}: {}", table, e))?;
        }

        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&pool)
            .await
            .map_err(|e| format!("Failed to enable FK: {}", e))?;

        eprintln!("\n   ✅ Game state cleared successfully\n");

        eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        eprintln!("📋 STEP 5/7: Loading League Structure");
        eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        eprintln!("   Reading league assignments from sheffield_league_clubs...\n");

        // Check if league structure is populated, if not, populate it
        let count_result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_league_clubs")
            .fetch_one(&pool)
            .await
            .map_err(|e| format!("Failed to check league clubs: {}", e))?;

        if count_result.0 == 0 {
            eprintln!("   ⚠️  League structure is empty - populating now...\n");
            database::sheffield_league::populate_sheffield_league(&pool)
                .await
                .map_err(|e| format!("Failed to populate league structure: {}", e))?;
            eprintln!("   ✅ League structure populated\n");
        }

        // Get clubs that are in the league (from sheffield_league_clubs which the editor populated)
        let league_club_rows = sqlx::query(
            "SELECT club_id FROM sheffield_league_clubs ORDER BY position_in_division"
        )
        .fetch_all(&pool)
        .await
        .map_err(|e| format!("Failed to fetch league clubs: {}", e))?;

        let league_clubs: Vec<String> = league_club_rows.iter().map(|row| {
            use sqlx::Row; row.get::<String, _>("club_id")
        }).collect();

        eprintln!("   ✅ Loaded {} clubs across 28 divisions", league_clubs.len());
        eprintln!("      • Levels 1-4: 16 clubs per division");
        eprintln!("      • Level 5: 13 clubs per division");
        eprintln!("      • Levels 6-7: 12 clubs per division\n");

        // Initialize standings for all clubs in the league
        eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        eprintln!("📋 STEP 6/7: Initializing League Standings");
        eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        eprintln!("   Creating fresh standings for {} clubs...", league_clubs.len());
        eprintln!("   (This may take 20-30 seconds)\n");

        if !league_clubs.is_empty() {
            database::populate::initialize_standings(&pool, year, league_clubs.clone())
                .await
                .map_err(|e| format!("Failed to initialize standings: {}", e))?;
        }

        eprintln!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        eprintln!("📋 STEP 7/7: Finalizing Game Setup");
        eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        eprintln!("   [1/2] Saving Sheffield Rules for {} season...", year);
        // Save ruleset
        database::rulesets::save_ruleset_to_db(&pool, year, year)
            .await
            .map_err(|e| format!("Failed to save ruleset: {}", e))?;
        eprintln!("   ✅ Sheffield Rules saved\n");

        eprintln!("   [2/2] Loading club database...");
        // Read club list from DB for the return value
        let rows = sqlx::query("SELECT id FROM sheffield_clubs ORDER BY name")
            .fetch_all(&pool)
            .await
            .map_err(|e| format!("Failed to fetch clubs: {}", e))?;
        let clubs: Vec<String> = rows.iter().map(|row| { use sqlx::Row; row.get::<String, _>("id") }).collect();
        eprintln!("   ✅ {} clubs loaded\n", clubs.len());

        (year, selected_club, clubs)
    } else {
        database::sheffield_db::initialize_schema(&pool)
            .await
            .map_err(|e| format!("Failed to initialize schema: {}", e))?;

        database::populate::populate_clubs_for_year(&pool, year, load_all_clubs)
            .await
            .map_err(|e| format!("Failed to populate clubs: {}", e))?;

        let clubs: Vec<String> = sheffield_rules::get_sheffield_clubs(if load_all_clubs { None } else { Some(year) })
            .iter()
            .map(|c| c.id.clone())
            .collect();

        if clubs.is_empty() {
            return Err("No clubs available for the selected year/mode combination.".to_string());
        }

        database::populate::populate_players_for_year(&pool, year)
            .await
            .map_err(|e| format!("Failed to populate players: {}", e))?;

        database::populate::initialize_standings(&pool, year, clubs.clone())
            .await
            .map_err(|e| format!("Failed to initialize standings: {}", e))?;

        database::rulesets::save_ruleset_to_db(&pool, year, year)
            .await
            .map_err(|e| format!("Failed to save ruleset: {}", e))?;

        let selected_club = if let Some(id) = club_id {
            let exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_clubs WHERE id = ?")
                .bind(&id)
                .fetch_one(&pool)
                .await
                .map_err(|e| format!("Failed to verify club exists: {}", e))?;
            if exists.0 == 0 {
                return Err(format!("Selected club '{}' not found in database", id));
            }
            id
        } else {
            clubs.first().cloned().unwrap_or_else(|| "sheffield_fc_1857".to_string())
        };

        (year, selected_club, clubs)
    };

    // Create game state record
    let game_state_id = uuid::Uuid::new_v4().to_string();

    // Start date: 1 week before Sheffield FA founding in January 1867
    // Sheffield FA founded mid-January, so start early January
    let start_date = format!("{:04}-01-07", year);  // January 7th - one week before Sheffield FA founding

    sqlx::query(
        "INSERT INTO sheffield_game_state (id, game_mode, rule_year, season_year, start_year, user_club_id, current_date, current_gameweek) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&game_state_id)
    .bind(&game_mode)
    .bind(rule_year as i32)
    .bind(year as i32)
    .bind(year as i32)
    .bind(&selected_club)
    .bind(&start_date)
    .bind(1)
    .execute(&pool)
    .await
    .map_err(|e| format!("Failed to create game state: {}", e))?;

    // Load clubs from sheffield_clubs table
    let club_rows: Vec<(String, String)> = sqlx::query_as(
        "SELECT id, name FROM sheffield_clubs ORDER BY name"
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_default();

    let clubs: Vec<Club> = club_rows
        .iter()
        .map(|(id, name)| Club {
            id: id.clone(),
            name: name.clone(),
            points: 0,
            played: 0,
            won: 0,
            drawn: 0,
            lost: 0,
            goals_for: 0,
            goals_against: 0,
            budget: 0,
            reputation: 0.0,
        })
        .collect();

    // Load standings from database (sorted by points descending, then by club name alphabetically)
    // For fantasy league mode, standings are loaded per-division, so leave empty here
    let standings: Vec<Standing> = if is_fantasy_league {
        // Fantasy league: standings are division-specific, loaded on-demand
        Vec::new()
    } else {
        // Other modes: load all standings into game state
        let standings_query = "SELECT ss.club_id, sc.name, ss.points, ss.played, ss.won, ss.drawn, ss.lost, ss.goals_for, ss.goals_against FROM sheffield_standings ss JOIN sheffield_clubs sc ON ss.club_id = sc.id ORDER BY ss.points DESC, sc.name ASC";

        let standings_raw: Vec<(String, String, u16, u8, u8, u8, u8, u16, u16)> =
            sqlx::query_as(standings_query)
                .fetch_all(&pool)
                .await
                .unwrap_or_default();

        // Convert standings with proper positioning
        standings_raw
        .into_iter()
        .enumerate()
        .map(|(position, (club_id, club_name, points, played, won, drawn, lost, goals_for, goals_against))| {
            Standing {
                club_id,
                club_name,
                position: (position as u16) + 1,
                points,
                played,
                won,
                drawn,
                lost,
                goals_for,
                goals_against,
                goal_difference: (goals_for as i16) - (goals_against as i16),
                rouges_for: 0,
                rouges_against: 0,
            }
        })
        .collect()
    };

    // Initialize cup competitions and generate events
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("🏆 Loading Cup Competitions");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let cup_events = initialize_season_cup_competitions(&pool, year, &start_date)
        .await
        .unwrap_or_else(|e| {
            eprintln!("   ⚠️  Warning: Failed to initialize cup competitions: {}", e);
            eprintln!("   Continuing without cup competitions...\n");
            Vec::new()
        });

    if !cup_events.is_empty() {
        eprintln!("   ✅ {} cup competition events scheduled\n", cup_events.len());
    }

    // Add Sheffield FA Formation announcement (January 12, 1867 - before Youdan Cup)
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("🏛️  Scheduling Sheffield FA Formation Event");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let cup_events_count = cup_events.len(); // Save count before moving
    let mut all_events = cup_events;

    // Sheffield FA was formed on January 12, 1867 (historically accurate date)
    let sheffield_fa_date = chrono::NaiveDate::from_ymd_opt(year as i32, 1, 12)
        .ok_or_else(|| "Failed to create Sheffield FA formation date".to_string())?;
    let sheffield_fa_date_str = sheffield_fa_date.format("%Y-%m-%d").to_string();

    let sheffield_fa_event = crate::game::GameEvent {
        id: format!("sheffield-fa-formation-{}", year),
        event_type: crate::game::EventType::HistoricalAnnouncement {
            title: "Sheffield Football Association Formed!".to_string(),
            description: format!(
                "SHEFFIELD, {} — A momentous occasion for Sheffield football! \
                 Representatives from the town's leading football clubs have assembled to form \
                 the Sheffield Football Association, creating the second-oldest football \
                 governing body in the world.\n\n\
                 Under the leadership of Harry Chambers, Secretary of Sheffield Football Club, \
                 the Association aims to organize competitive fixtures, standardize the Sheffield \
                 Rules across all clubs, and elevate the game to new heights.\n\n\
                 This historic development promises to usher in a new era of organized football \
                 in Sheffield and beyond. The Association has already announced plans for \
                 the first-ever football tournament, to be sponsored by local theatre proprietor \
                 Thomas Youdan.\n\n\
                 \"This is a proud day for Sheffield football,\" declared Mr. Chambers. \
                 \"With proper governance, our great game shall flourish for generations to come.\"",
                sheffield_fa_date.format("%B %d, %Y")
            ),
        },
        date: sheffield_fa_date_str.clone(),
        requires_user_action: false,
        processed: false,
        result: None,
    };

    all_events.push(sheffield_fa_event);
    eprintln!("   ✅ Sheffield FA formation event scheduled for {}\n", sheffield_fa_date.format("%B %d, %Y"));

    // Sort all events by date
    all_events.sort_by(|a, b| a.date.cmp(&b.date));

    // Return initialized game state
    eprintln!("╔══════════════════════════════════════════════════════════════════════════╗");
    eprintln!("║  ✅ INITIALIZATION COMPLETE                                              ║");
    eprintln!("╚══════════════════════════════════════════════════════════════════════════╝\n");
    eprintln!("🎮 Game ready to start!");
    eprintln!("📅 Season: {}-{}", year, (year % 100) + 1);
    eprintln!("🏟️  Your Club: {}", selected_club);
    eprintln!("⚽ Clubs in League: {}", standings.len());
    eprintln!("🏆 Cup Events: {}\n", cup_events_count);

    eprintln!("═══════════════════════════════════════════════════════════════════════════");
    eprintln!("DEBUG: Creating GameState to return to frontend");
    eprintln!("───────────────────────────────────────────────────────────────────────────");
    eprintln!("game_mode = {:?}", game_mode);
    eprintln!("clubs.len() = {}", clubs.len());
    eprintln!("standings.len() = {}", standings.len());
    eprintln!("═══════════════════════════════════════════════════════════════════════════\n");

    // Load friendly matches from database
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("🤝 Loading Friendly Matches");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let friendly_matches = database::challenge_invitations::load_friendly_matches(&pool)
        .await
        .unwrap_or_else(|e| {
            eprintln!("   ⚠️  Warning: Failed to load friendly matches: {}", e);
            Vec::new()
        });

    eprintln!("   ✅ Loaded {} friendly matches\n", friendly_matches.len());

    let mut game_state = GameState {
        id: game_state_id,
        season: year as u16,
        current_gameweek: 1,
        current_date: start_date,
        user_club_id: selected_club,
        game_mode: Some(game_mode.clone()),
        clubs,
        matches: friendly_matches,
        players: Vec::new(),
        standings,
        pending_events: all_events,
        processed_events: Vec::new(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        loaded_from_file: None,
    };

    eprintln!("═══════════════════════════════════════════════════════════════════════════");
    eprintln!("DEBUG: GameState created - about to serialize and return");
    eprintln!("───────────────────────────────────────────────────────────────────────────");
    eprintln!("game_state.game_mode = {:?}", game_state.game_mode);
    eprintln!("game_state.clubs.len() = {}", game_state.clubs.len());
    eprintln!("game_state.standings.len() = {}", game_state.standings.len());

    // Try to serialize to JSON to see exactly what will be sent
    match serde_json::to_string_pretty(&game_state) {
        Ok(json) => {
            eprintln!("GameState as JSON:");
            eprintln!("{}", json);
        }
        Err(e) => {
            eprintln!("Failed to serialize GameState to JSON: {}", e);
        }
    }
    eprintln!("═══════════════════════════════════════════════════════════════════════════\n");

    Ok(game_state)
}

/// Process promotions and relegations for the end of season
/// This command should be called when a season completes
#[tauri::command]
pub async fn process_season_promotions_relegations(
    season: u16,
) -> Result<serde_json::Value, String> {
    use std::collections::HashMap;

    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = database::sheffield_db::create_sheffield_database(db_path)
        .await
        .map_err(|e| format!("Failed to create database: {}", e))?;

    // Check if season has already been processed
    let already_processed = database::promotion_db::has_season_been_processed(&pool, season)
        .await
        .map_err(|e| format!("Failed to check if season processed: {}", e))?;

    if already_processed {
        return Err("Season has already been processed for promotions/relegations".to_string());
    }

    // Get all divisions
    let divisions = database::sheffield_league::get_all_divisions();
    let div_ids: Vec<String> = divisions
        .iter()
        .filter(|d| !d.id.starts_with("res-"))  // Only main divisions, not reserve
        .map(|d| d.id.clone())
        .collect();

    // Fetch standings for all divisions
    let mut standings_by_division: HashMap<String, Vec<sheffield_rules::Standing>> = HashMap::new();

    for div_id in &div_ids {
        match database::promotion_db::get_division_standings_simple(&pool, div_id, season).await {
            Ok(standings) => {
                standings_by_division.insert(div_id.clone(), standings);
            }
            Err(_) => {
                // Division might be empty or have no standings yet
                standings_by_division.insert(div_id.clone(), vec![]);
            }
        }
    }

    // Process promotions and relegations
    let report = sheffield_rules::process_season_end(standings_by_division, season);

    // Begin transaction for database updates
    let mut tx = pool.begin()
        .await
        .map_err(|e| format!("Failed to start transaction: {}", e))?;

    // Apply promotions
    for promotion in &report.promotions {
        // Update club division
        sqlx::query("DELETE FROM sheffield_league_clubs WHERE club_id = ?")
            .bind(&promotion.club_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Failed to delete old club assignment: {}", e))?;

        let new_id = format!("{}-{}", promotion.club_id, promotion.to_division_id);
        sqlx::query(
            "INSERT INTO sheffield_league_clubs
             (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
             VALUES (?, ?, ?, 1, 0, NULL)"
        )
        .bind(&new_id)
        .bind(&promotion.to_division_id)
        .bind(&promotion.club_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("Failed to insert new club assignment: {}", e))?;

        // Record movement history
        database::promotion_db::record_movement_event(
            &pool,
            "promotion",
            season,
            &promotion.club_id,
            &promotion.from_division_id,
            &promotion.to_division_id,
            promotion.final_position,
        )
        .await
        .map_err(|e| format!("Failed to record promotion: {}", e))?;
    }

    // Apply relegations
    for relegation in &report.relegations {
        // Update club division
        sqlx::query("DELETE FROM sheffield_league_clubs WHERE club_id = ?")
            .bind(&relegation.club_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Failed to delete old club assignment: {}", e))?;

        let new_id = format!("{}-{}", relegation.club_id, relegation.to_division_id);
        sqlx::query(
            "INSERT INTO sheffield_league_clubs
             (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
             VALUES (?, ?, ?, 1, 0, NULL)"
        )
        .bind(&new_id)
        .bind(&relegation.to_division_id)
        .bind(&relegation.club_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| format!("Failed to insert new club assignment: {}", e))?;

        // Record movement history
        database::promotion_db::record_movement_event(
            &pool,
            "relegation",
            season,
            &relegation.club_id,
            &relegation.from_division_id,
            &relegation.to_division_id,
            relegation.final_position,
        )
        .await
        .map_err(|e| format!("Failed to record relegation: {}", e))?;
    }

    tx.commit()
        .await
        .map_err(|e| format!("Failed to commit transaction: {}", e))?;

    // Return report as JSON
    Ok(serde_json::json!({
        "season": report.season,
        "promotions": report.promotions.iter().map(|p| serde_json::json!({
            "club_id": p.club_id,
            "club_name": p.club_name,
            "from_division": p.from_division_id,
            "to_division": p.to_division_id,
            "final_position": p.final_position,
        })).collect::<Vec<_>>(),
        "relegations": report.relegations.iter().map(|r| serde_json::json!({
            "club_id": r.club_id,
            "club_name": r.club_name,
            "from_division": r.from_division_id,
            "to_division": r.to_division_id,
            "final_position": r.final_position,
        })).collect::<Vec<_>>(),
        "divisions_affected": report.divisions_affected,
        "processed_at": report.processed_at,
    }))
}

// Database Editor Commands

#[tauri::command]
pub async fn db_get_all_clubs() -> Result<Vec<ClubInfo>, String> {
    database::get_all_clubs().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_get_club_players(club_id: String) -> Result<Vec<PlayerDetail>, String> {
    database::get_club_squad(&club_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_get_squad_simple(club_id: String) -> Result<Vec<serde_json::Value>, String> {
    use sqlx::Row;
    let pool = database::get_pool().await.map_err(|e| e.to_string())?;

    println!("[SQUAD SIMPLE] Getting players for club_id: '{}'", club_id);

    let rows = sqlx::query(
        "SELECT person_id as id, first_name || ' ' || surname as name, club_id, position, birth_year,
         CAST((1867 - COALESCE(birth_year, 1845)) AS INTEGER) as age
         FROM sheffield_footballers
         WHERE club_id = ?
         ORDER BY surname, first_name"
    )
    .bind(&club_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;

    println!("[SQUAD SIMPLE] Found {} players", rows.len());

    let players: Vec<serde_json::Value> = rows.iter().map(|row| {
        serde_json::json!({
            "id": row.get::<i64, _>(0),
            "name": row.get::<String, _>(1),
            "club_id": row.get::<String, _>(2),
            "position": row.get::<Option<String>, _>(3),
            "birth_year": row.get::<Option<i32>, _>(4),
            "age": row.get::<i32, _>(5)
        })
    }).collect();

    Ok(players)
}

#[tauri::command]
pub async fn db_bulk_add_players(
    club_id: String,
    player_names: Vec<String>,
    year: i32,
    has_goalkeeper: bool,
) -> Result<Vec<database::player_generator::GeneratedPlayer>, String> {
    database::bulk_add_players(&club_id, player_names, year, has_goalkeeper)
        .await
        .map_err(|e| e.to_string())
}

#[derive(serde::Deserialize)]
pub struct PlayerInput {
    pub name: String,
    pub birth_year: Option<i32>,
}

#[tauri::command]
pub async fn db_bulk_add_players_with_ages(
    club_id: String,
    player_inputs: Vec<PlayerInput>,
    year: i32,
    has_goalkeeper: bool,
) -> Result<Vec<database::player_generator::GeneratedPlayer>, String> {
    database::bulk_add_players_with_ages(&club_id, player_inputs, year, has_goalkeeper)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_update_club_location(
    club_id: String,
    city: Option<String>,
    region: Option<String>,
    origin: Option<String>,
) -> Result<(), String> {
    database::update_club_location(&club_id, city, region, origin)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_update_club_name(
    club_id: String,
    name: String,
) -> Result<(), String> {
    database::update_club_name(&club_id, name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_copy_parent_data_to_reserve(
    parent_club_id: String,
) -> Result<String, String> {
    database::copy_parent_data_to_reserve(&parent_club_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_create_club(
    club_id: String,
    club_name: String,
    founded_year: i32,
    ground_name: String,
    city: String,
    region: String,
    origin: String,
) -> Result<String, String> {
    database::create_club(
        &club_id,
        &club_name,
        founded_year,
        &ground_name,
        &city,
        &region,
        &origin,
    )
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_get_unassigned_clubs() -> Result<Vec<serde_json::Value>, String> {
    let clubs = database::get_unassigned_clubs()
        .await
        .map_err(|e| e.to_string())?;

    let result: Vec<serde_json::Value> = clubs
        .iter()
        .map(|(id, name, founded_year)| serde_json::json!({
            "id": id,
            "name": name,
            "founded_year": founded_year,
        }))
        .collect();
    Ok(result)
}

#[tauri::command]
pub async fn db_get_all_divisions() -> Result<Vec<serde_json::Value>, String> {
    let divisions = database::sheffield_league::get_all_divisions();
    let result: Vec<serde_json::Value> = divisions
        .iter()
        .map(|d| serde_json::json!({
            "id": d.id,
            "name": d.name,
            "level": d.level,
            "region": d.region,
        }))
        .collect();
    Ok(result)
}

#[tauri::command]
pub async fn db_update_club_division(
    club_id: String,
    division_id: String,
    position: i32,
) -> Result<(), String> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = database::sheffield_db::create_sheffield_database(db_path)
        .await
        .map_err(|e| format!("Failed to create database: {}", e))?;

    database::update_club_division(&pool, &club_id, &division_id, position)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_import_league_structure() -> Result<String, String> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let pool = database::sheffield_db::create_sheffield_database(db_path)
        .await
        .map_err(|e| format!("Failed to create database: {}", e))?;

    database::sheffield_league::populate_sheffield_league(&pool)
        .await
        .map_err(|e| format!("Failed to import league structure: {}", e))?;

    Ok("Successfully imported league structure".to_string())
}

#[derive(serde::Deserialize)]
pub struct PlayerStats {
    pub position: String,
    // Physical
    pub pace: i32,
    pub acceleration: i32,
    pub strength: i32,
    pub stamina: i32,
    pub balance: i32,
    pub jumping: i32,
    pub agility: i32,
    pub natural_fitness: i32,
    // Technical
    pub passing: i32,
    pub dribbling: i32,
    pub first_touch: i32,
    pub technique: i32,
    pub heading: i32,
    pub long_passing: i32,
    pub crossing: i32,
    pub long_shots: i32,
    pub tackling: i32,
    pub handling: i32,
    pub reflexes: i32,
    pub corners: i32,
    pub free_kicks: i32,
    pub throw_ins: i32,
    pub vision: i32,
    pub left_foot: i32,
    pub right_foot: i32,
    pub one_on_ones: i32,
    // Mental
    pub courage: i32,
    pub bravery: i32,
    pub concentration: i32,
    pub decision_making: i32,
    pub leadership: i32,
    pub aggression: i32,
    pub anticipation: i32,
    pub determination: i32,
    pub flair: i32,
    pub influence: i32,
    pub adaptability: i32,
    pub ambition: i32,
    pub loyalty: i32,
    pub pressure: i32,
    pub professionalism: i32,
    pub sportsmanship: i32,
    pub temperament: i32,
    // Positioning
    pub awareness: i32,
    pub marking: i32,
    pub positioning: i32,
    pub work_rate: i32,
    pub off_the_ball: i32,
    pub movement: i32,
    pub teamwork: i32,
    // Specialization
    pub finishing: i32,
    pub penalties: i32,
    pub set_pieces: i32,
    // Hidden
    pub consistency: i32,
    pub dirtiness: i32,
    pub versatility: i32,
    pub injury_proneness: i32,
    pub important_matches: i32,
    // Ability & Reputation
    pub current_ability: i32,
    pub potential_ability: i32,
    pub current_reputation: i32,
}

#[tauri::command]
pub async fn db_update_player_stats(
    player_id: String,
    stats: PlayerStats,
) -> Result<String, String> {
    database::update_player_stats(&player_id, &stats)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn transfer_player(
    player_id: String,
    new_club_id: String,
) -> Result<(), String> {
    database::transfer_player(&player_id, &new_club_id)
        .await
        .map_err(|e| e.to_string())
}

// ========================================
// Cup Competition Commands
// ========================================

#[tauri::command]
pub async fn db_create_annual_cups(season: i64, start_week: i64) -> Result<Vec<Competition>, String> {
    database::create_annual_cups(season, start_week)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_create_custom_cup(
    name: String,
    season: i64,
    min_division_level: Option<i64>,
    max_division_level: Option<i64>,
    start_week: i64,
    announcement_week: i64,
    draw_week: i64,
    prestige_level: String,
    rules_type: String,
) -> Result<Competition, String> {
    database::create_custom_cup(
        name,
        season,
        min_division_level,
        max_division_level,
        start_week,
        announcement_week,
        draw_week,
        prestige_level,
        rules_type,
    )
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_update_cup_competition(
    competition_id: String,
    name: String,
    min_division_level: Option<i64>,
    max_division_level: Option<i64>,
    start_week: i64,
    announcement_week: i64,
    draw_week: i64,
    prestige_level: String,
    rules_type: String,
) -> Result<Competition, String> {
    database::update_cup_competition(
        competition_id,
        name,
        min_division_level,
        max_division_level,
        start_week,
        announcement_week,
        draw_week,
        prestige_level,
        rules_type,
    )
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_get_eligible_clubs(competition_id: String) -> Result<Vec<EligibleClub>, String> {
    database::get_eligible_clubs(&competition_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_generate_cup_draw(competition_id: String, seeded: bool) -> Result<(), String> {
    database::generate_cup_draw(&competition_id, seeded)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_get_cup_bracket(competition_id: String) -> Result<Vec<CupTie>, String> {
    database::get_cup_bracket(&competition_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_get_league_config() -> Result<LeagueConfig, String> {
    database::get_league_config()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_save_league_config(config: LeagueConfig) -> Result<(), String> {
    database::save_league_config(config)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_create_league_config_table() -> Result<(), String> {
    database::create_league_config_table()
        .await
        .map_err(|e| e.to_string())
}

// League metadata commands
#[tauri::command]
pub async fn db_create_league_metadata_table() -> Result<(), String> {
    database::create_league_metadata_table()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_get_all_leagues() -> Result<Vec<LeagueMetadata>, String> {
    database::get_all_leagues()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_get_active_league() -> Result<Option<LeagueMetadata>, String> {
    database::get_active_league()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_create_league(
    name: String,
    description: String,
    season_year: i64,
) -> Result<LeagueMetadata, String> {
    database::create_league(name, description, season_year)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_set_active_league(league_id: String) -> Result<(), String> {
    database::set_active_league(league_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_delete_league(league_id: String) -> Result<(), String> {
    database::delete_league(league_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_get_competitions_for_season(season: i64) -> Result<Vec<Competition>, String> {
    database::get_competitions_for_season(season)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_delete_competitions_for_season(season: i64) -> Result<(), String> {
    database::delete_competitions_for_season(season)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_run_schema_migrations() -> Result<String, String> {
    let pool = database::get_pool().await.map_err(|e| e.to_string())?;

    // Run the schema initialization which will create any missing tables
    database::sheffield_db::initialize_schema(&pool)
        .await
        .map_err(|e| format!("Failed to run migrations: {}", e))?;

    Ok("Schema updated successfully".to_string())
}

// REMOVED: // Player creation with geographic data (no club assignment)
// REMOVED: #[derive(Debug, Clone, Serialize, Deserialize)]
// REMOVED: pub struct PlayerWithGeographicData {
// REMOVED:     pub name: String,
// REMOVED:     pub birth_year: i32,
// REMOVED:     pub nationality: Option<String>,
// REMOVED:     pub where_born: Option<String>,
// REMOVED:     pub birth_town: Option<String>,
// REMOVED:     pub birth_county: Option<String>,
// REMOVED:     pub birth_country: Option<String>,
// REMOVED:     pub civil_parish: Option<String>,
// REMOVED:     pub ecclesiastical_parish: Option<String>,
// REMOVED:     pub registration_district: Option<String>,
// REMOVED:     pub sub_registration_district: Option<String>,
// REMOVED:     pub census_age: Option<i32>,
// REMOVED:     pub census_relation: Option<String>,
// REMOVED:     pub census_gender: Option<String>,
// REMOVED:     pub census_ed: Option<String>,
// REMOVED:     pub census_household_schedule: Option<String>,
// REMOVED:     pub census_piece: Option<String>,
// REMOVED:     pub census_folio: Option<String>,
// REMOVED:     pub census_page: Option<String>,
// REMOVED: }
// REMOVED: 
// REMOVED: #[tauri::command]
// REMOVED: pub async fn db_create_players_with_geographic_data(
// REMOVED:     players: Vec<PlayerWithGeographicData>,
// REMOVED: ) -> Result<Vec<String>, String> {
// REMOVED:     database::create_players_with_geographic_data(players)
// REMOVED:         .await
// REMOVED:         .map_err(|e| e.to_string())
// REMOVED: }
// REMOVED: 
// REMOVED: #[tauri::command]
// REMOVED: pub async fn db_get_all_players_without_stats() -> Result<Vec<serde_json::Value>, String> {
// REMOVED:     database::get_all_players_without_stats()
// REMOVED:         .await
// REMOVED:         .map_err(|e| e.to_string())
// REMOVED: }
// REMOVED: 
// REMOVED: #[tauri::command]
// REMOVED: pub async fn db_get_all_players() -> Result<Vec<serde_json::Value>, String> {
// REMOVED:     database::get_all_players()
// REMOVED:         .await
// REMOVED:         .map_err(|e| e.to_string())
// REMOVED: }
// REMOVED: 
// REMOVED: #[tauri::command]
// REMOVED: pub async fn db_delete_player(player_id: String) -> Result<(), String> {
// REMOVED:     database::delete_player(&player_id)
// REMOVED:         .await
// REMOVED:         .map_err(|e| e.to_string())
// REMOVED: }
// REMOVED: 
// REMOVED: #[tauri::command]
// REMOVED: pub async fn db_assign_postcode_to_parish(
// REMOVED:     parish: String,
// REMOVED:     postcode: String,
// REMOVED: ) -> Result<usize, String> {
// REMOVED:     database::assign_postcode_to_parish(&parish, &postcode)
// REMOVED:         .await
// REMOVED:         .map_err(|e| e.to_string())
// REMOVED: }
// REMOVED: 
// REMOVED: #[tauri::command]
// REMOVED: pub async fn db_assign_parish_players_to_clubs(
// REMOVED:     parish: String,
// REMOVED:     postcode: String,
// REMOVED: ) -> Result<Vec<database::PlayerAssignment>, String> {
// REMOVED:     database::assign_parish_players_to_clubs(&parish, &postcode)
// REMOVED:         .await
// REMOVED:         .map_err(|e| e.to_string())
// REMOVED: }
// REMOVED: 
// ============================================================================
// PLAYER DATA COMMANDS
// ============================================================================

#[tauri::command]
pub async fn db_get_all_players() -> Result<Vec<serde_json::Value>, String> {
    database::get_all_players()
        .await
        .map_err(|e| e.to_string())
}

// ============================================================================
// PAGINATION AND STATISTICS COMMANDS
// ============================================================================

#[tauri::command]
pub async fn db_get_players_paginated(
    page: usize,
    page_size: usize,
    search: Option<String>,
    parish: Option<String>,
    birth_year: Option<i32>,
    assignment_filter: Option<String>,
) -> Result<database::PlayerQueryResult, String> {
    database::get_players_paginated(page, page_size, search, parish, birth_year, assignment_filter)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_get_age_statistics(year: i32) -> Result<Vec<database::AgeStatistic>, String> {
    database::get_age_statistics(year)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_get_first_name_statistics() -> Result<Vec<database::NameStatistic>, String> {
    database::get_first_name_statistics()
        .await
        .map_err(|e| e.to_string())
}

// REMOVED: #[tauri::command]
// REMOVED: pub async fn db_get_middle_name_statistics() -> Result<Vec<database::NameStatistic>, String> {
// REMOVED:     database::get_middle_name_statistics()
// REMOVED:         .await
// REMOVED:         .map_err(|e| e.to_string())
// REMOVED: }

#[tauri::command]
pub async fn db_get_surname_statistics() -> Result<Vec<database::NameStatistic>, String> {
    database::get_surname_statistics()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_create_player_indexes() -> Result<(), String> {
    database::create_player_indexes()
        .await
        .map_err(|e| e.to_string())
}

// REMOVED: #[tauri::command]
// REMOVED: pub async fn db_expand_name_abbreviations() -> Result<usize, String> {
// REMOVED:     database::expand_name_abbreviations()
// REMOVED:         .await
// REMOVED:         .map_err(|e| e.to_string())
// REMOVED: }

// ============================================================================
// GENEALOGY MATCHING TEST COMMAND
// ============================================================================

#[tauri::command]
pub async fn db_test_genealogy_matching(
    min_confidence: f32,
) -> Result<(Vec<database::MatchResult>, database::MatchStats), String> {
    use std::path::Path;

    // Read a sample of genealogy CSVs (just a few files for testing)
    let genealogy_dir = Path::new("D:/projects/Saturday at Three/genealogy");

    // Read just a few files for testing
    let test_files = vec![
        "tg_1835_S.csv",
        "tg_1836_S.csv",
        "tg_1837_S.csv",
        "tg_1835_B.csv",
        "tg_1836_B.csv",
    ];

    let mut all_records = Vec::new();
    for file_name in test_files {
        let file_path = genealogy_dir.join(file_name);
        if file_path.exists() {
            match database::read_genealogy_csv(&file_path).await {
                Ok(mut records) => {
                    all_records.append(&mut records);
                }
                Err(e) => {
                    eprintln!("Warning: Failed to read {}: {}", file_name, e);
                }
            }
        }
    }

    println!("Loaded {} genealogy records for testing", all_records.len());

    // Get database pool
    let pool = database::get_pool_write().await.map_err(|e| e.to_string())?;

    // Run matching test
    database::test_match_genealogy(&pool, all_records, min_confidence)
        .await
        .map_err(|e| e.to_string())
}

// ============================================================================
// MATCH ENGINE COMMANDS
// ============================================================================

use crate::match_engine::{MatchSimulator, MatchResult, MatchEvent, VisualState, MatchStatistics, PlayerAttributes};

/// Convert PlayerDetail to PlayerAttributes for match engine
fn player_detail_to_attributes(player: &PlayerDetail) -> PlayerAttributes {
    PlayerAttributes {
        pace: player.pace.unwrap_or(50),
        acceleration: player.acceleration.unwrap_or(50),
        strength: player.strength.unwrap_or(50),
        stamina: player.stamina.unwrap_or(50),
        balance: player.balance.unwrap_or(50),
        jumping: player.jumping.unwrap_or(50),
        agility: player.agility.unwrap_or(50),
        natural_fitness: player.natural_fitness.unwrap_or(50),
        passing: player.passing.unwrap_or(50),
        dribbling: player.dribbling.unwrap_or(50),
        first_touch: player.first_touch.unwrap_or(50),
        technique: player.technique.unwrap_or(50),
        heading: player.heading.unwrap_or(50),
        long_passing: player.long_passing.unwrap_or(50),
        crossing: player.crossing.unwrap_or(50),
        long_shots: player.long_shots.unwrap_or(50),
        tackling: player.tackling.unwrap_or(50),
        handling: player.handling.unwrap_or(50),
        reflexes: player.reflexes.unwrap_or(50),
        corners: player.corners.unwrap_or(50),
        free_kicks: player.free_kicks.unwrap_or(50),
        throw_ins: player.throw_ins.unwrap_or(50),
        vision: player.vision.unwrap_or(50),
        left_foot: player.left_foot.unwrap_or(50),
        right_foot: player.right_foot.unwrap_or(50),
        one_on_ones: player.one_on_ones.unwrap_or(50),
        courage: player.courage.unwrap_or(50),
        bravery: player.bravery.unwrap_or(50),
        concentration: player.concentration.unwrap_or(50),
        decision_making: player.decision_making.unwrap_or(50),
        leadership: player.leadership.unwrap_or(50),
        aggression: player.aggression.unwrap_or(50),
        anticipation: player.anticipation.unwrap_or(50),
        determination: player.determination.unwrap_or(50),
        flair: player.flair.unwrap_or(50),
        influence: player.influence.unwrap_or(50),
        adaptability: player.adaptability.unwrap_or(50),
        ambition: player.ambition.unwrap_or(50),
        loyalty: player.loyalty.unwrap_or(50),
        pressure: player.pressure.unwrap_or(50),
        professionalism: player.professionalism.unwrap_or(50),
        sportsmanship: player.sportsmanship.unwrap_or(50),
        temperament: player.temperament.unwrap_or(50),
        awareness: player.awareness.unwrap_or(50),
        marking: player.marking.unwrap_or(50),
        positioning: player.positioning.unwrap_or(50),
        work_rate: player.work_rate.unwrap_or(50),
        off_the_ball: player.off_the_ball.unwrap_or(50),
        movement: player.movement.unwrap_or(50),
        teamwork: player.teamwork.unwrap_or(50),
        finishing: player.finishing.unwrap_or(50),
        penalties: player.penalties.unwrap_or(50),
        set_pieces: player.set_pieces.unwrap_or(50),
        consistency: player.consistency.unwrap_or(50),
        important_matches: player.important_matches.unwrap_or(50),
        injury_proneness: player.injury_proneness.unwrap_or(50),
        versatility: player.versatility.unwrap_or(50),
        dirtiness: player.dirtiness.unwrap_or(50),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchUpdate {
    pub match_id: String,
    pub minute: i32,
    pub event: MatchEvent,
    pub visual_state: VisualState,
    pub statistics: MatchStatistics,
}

/// Simulate match in batch mode (fast, no streaming) - for AI vs AI matches
#[tauri::command]
pub async fn simulate_match_batch(
    match_id: String,
    home_club_id: String,
    away_club_id: String,
    rule_year: i32,
) -> Result<MatchResult, String> {
    // Load squads from database
    let home_players_detail = database::get_club_squad(&home_club_id)
        .await
        .map_err(|e| e.to_string())?;
    let away_players_detail = database::get_club_squad(&away_club_id)
        .await
        .map_err(|e| e.to_string())?;

    // Convert to match engine format
    let home_players: Vec<PlayerAttributes> = home_players_detail
        .iter()
        .map(player_detail_to_attributes)
        .collect();
    let away_players: Vec<PlayerAttributes> = away_players_detail
        .iter()
        .map(player_detail_to_attributes)
        .collect();

    // Generate seed from match_id
    let seed = match_id.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));

    // Create and run simulator
    let simulator = MatchSimulator::new(
        match_id,
        home_players,
        away_players,
        rule_year,
        seed,
    );

    let result = simulator.simulate_fast();

    Ok(result)
}

/// Simulate match with live streaming (emits events) - for user's team matches
#[tauri::command]
pub async fn simulate_match_live(
    app_handle: tauri::AppHandle,
    match_id: String,
    home_club_id: String,
    away_club_id: String,
    rule_year: i32,
) -> Result<String, String> {
    // Load squads
    let home_players_detail = database::get_club_squad(&home_club_id)
        .await
        .map_err(|e| e.to_string())?;
    let away_players_detail = database::get_club_squad(&away_club_id)
        .await
        .map_err(|e| e.to_string())?;

    // Convert to match engine format
    let home_players: Vec<PlayerAttributes> = home_players_detail
        .iter()
        .map(player_detail_to_attributes)
        .collect();
    let away_players: Vec<PlayerAttributes> = away_players_detail
        .iter()
        .map(player_detail_to_attributes)
        .collect();

    // Generate seed
    let seed = match_id.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));

    // Clone match_id for closure
    let match_id_clone = match_id.clone();

    // Spawn background task for simulation
    tokio::spawn(async move {
        let simulator = MatchSimulator::new(
            match_id_clone.clone(),
            home_players,
            away_players,
            rule_year,
            seed,
        );

        // Run with streaming callback
        simulator.simulate_with_streaming(|event, visual_state, statistics| {
            // Emit Tauri event
            let update = MatchUpdate {
                match_id: match_id_clone.clone(),
                minute: event.minute,
                event: event.clone(),
                visual_state: visual_state.clone(),
                statistics: statistics.clone(),
            };

            // Emit match update event using Tauri 2.0 API
            for window in app_handle.webview_windows().values() {
                let _ = window.emit("match_update", &update);
            }
        });
    });

    Ok(format!("Match {} simulation started", match_id))
}

/// Start a quick match: Sheffield FC vs Hallam FC, 1867 rules, live streaming.
/// Loads real player squads from the Sheffield 1867 DB.
/// Returns the match_id so the frontend knows which event stream to subscribe to.
#[tauri::command]
pub async fn start_quick_match(
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    let home_club_id = "sheffield-fc".to_string();
    let away_club_id = "hallam-fc".to_string();
    let rule_year: i32 = 1867;
    let match_id = format!("quick-{}", uuid::Uuid::new_v4());

    // Load squads from DB (same function used by simulate_match_live)
    let home_players_detail = database::get_club_squad(&home_club_id)
        .await
        .map_err(|e| format!("Failed to load Sheffield FC squad: {}", e))?;
    let away_players_detail = database::get_club_squad(&away_club_id)
        .await
        .map_err(|e| format!("Failed to load Hallam FC squad: {}", e))?;

    // Helper: build a fallback PlayerAttributes at a given quality level (1-20 scale)
    fn fallback_player(q: i32) -> PlayerAttributes {
        PlayerAttributes {
            pace: q, acceleration: q, strength: q, stamina: q, balance: q, jumping: q,
            agility: q, natural_fitness: q, passing: q, dribbling: q, first_touch: q,
            technique: q, heading: q, long_passing: q, crossing: q - 2, long_shots: q - 3,
            tackling: q, handling: 5, reflexes: 5, corners: q - 3, free_kicks: q - 2,
            throw_ins: q - 2, vision: q, left_foot: q - 1, right_foot: q + 1, one_on_ones: q - 1,
            courage: q, bravery: q, concentration: q, decision_making: q, leadership: q - 1,
            aggression: q - 2, anticipation: q, determination: q, flair: q - 1, influence: q - 2,
            adaptability: q - 1, ambition: q, loyalty: q, pressure: 8, professionalism: q,
            sportsmanship: q + 1, temperament: q, awareness: q, marking: q - 1, positioning: q,
            work_rate: q + 1, off_the_ball: q - 1, movement: q, teamwork: q, finishing: q,
            penalties: q - 1, set_pieces: q - 2, consistency: q, important_matches: q - 1,
            injury_proneness: 6, versatility: q - 2, dirtiness: 5,
        }
    }

    // If DB squads are empty, generate placeholder squads so the match runs
    let home_players: Vec<PlayerAttributes> = if home_players_detail.is_empty() {
        (0..11).map(|_| fallback_player(13)).collect()
    } else {
        home_players_detail.iter().map(player_detail_to_attributes).collect()
    };

    let away_players: Vec<PlayerAttributes> = if away_players_detail.is_empty() {
        (0..11).map(|_| fallback_player(12)).collect()
    } else {
        away_players_detail.iter().map(player_detail_to_attributes).collect()
    };

    let seed = match_id.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));
    let match_id_clone = match_id.clone();

    tokio::spawn(async move {
        let simulator = MatchSimulator::new(
            match_id_clone.clone(),
            home_players,
            away_players,
            rule_year,
            seed,
        );

        simulator.simulate_with_streaming(|event, visual_state, statistics| {
            let update = MatchUpdate {
                match_id: match_id_clone.clone(),
                minute: event.minute,
                event: event.clone(),
                visual_state: visual_state.clone(),
                statistics: statistics.clone(),
            };
            for window in app_handle.webview_windows().values() {
                let _ = window.emit("match_update", &update);
            }
        });
    });

    Ok(match_id)
}

/// Simulate a Sheffield 1867 match with live event streaming (emits rouge scoring)
/// Uses the SheffieldMatchSimulator with authentic Victorian-era commentary
#[tauri::command]
pub async fn simulate_sheffield_1867_match(
    app_handle: tauri::AppHandle,
    match_id: String,
    home_club_id: String,
    away_club_id: String,
    home_formation_code: Option<String>,
    away_formation_code: Option<String>,
) -> Result<String, String> {
    use crate::match_engine::sheffield_1867::{SheffieldMatchSimulator, MatchEvent as Sheffield1867Event, RealPlayerData};

    // Load club names from database
    let home_club_info = database::get_club_info(&home_club_id)
        .await
        .map_err(|e| format!("Failed to load home club info: {}", e))?;
    let away_club_info = database::get_club_info(&away_club_id)
        .await
        .map_err(|e| format!("Failed to load away club info: {}", e))?;

    let home_team_name = home_club_info.name;
    let away_team_name = away_club_info.name;

    // Load real player squads from database
    let home_squad_detail = database::get_club_squad(&home_club_id)
        .await
        .map_err(|e| format!("Failed to load home squad: {}", e))?;
    let away_squad_detail = database::get_club_squad(&away_club_id)
        .await
        .map_err(|e| format!("Failed to load away squad: {}", e))?;

    // Convert PlayerDetail to RealPlayerData
    let home_squad: Vec<RealPlayerData> = home_squad_detail.iter().map(|p| RealPlayerData {
        id: p.id.clone(),
        name: p.name.clone(),
        position: p.position.clone(),
        age: p.age,
        pace: p.pace,
        acceleration: p.acceleration,
        strength: p.strength,
        stamina: p.stamina,
        agility: p.agility,
        passing: p.passing,
        dribbling: p.dribbling,
        first_touch: p.first_touch,
        heading: p.heading,
        finishing: p.finishing,
        tackling: p.tackling,
        composure: p.concentration,  // Use concentration as composure equivalent
        vision: p.vision,
        decisions: p.decision_making,  // Use decision_making instead of decisions
        positioning: p.positioning,
        teamwork: p.teamwork,
        work_rate: p.work_rate,
    }).collect();

    let away_squad: Vec<RealPlayerData> = away_squad_detail.iter().map(|p| RealPlayerData {
        id: p.id.clone(),
        name: p.name.clone(),
        position: p.position.clone(),
        age: p.age,
        pace: p.pace,
        acceleration: p.acceleration,
        strength: p.strength,
        stamina: p.stamina,
        agility: p.agility,
        passing: p.passing,
        dribbling: p.dribbling,
        first_touch: p.first_touch,
        heading: p.heading,
        finishing: p.finishing,
        tackling: p.tackling,
        composure: p.concentration,  // Use concentration as composure equivalent
        vision: p.vision,
        decisions: p.decision_making,  // Use decision_making instead of decisions
        positioning: p.positioning,
        teamwork: p.teamwork,
        work_rate: p.work_rate,
    }).collect();

    let match_id_clone = match_id.clone();

    // Spawn async task to run simulation and emit events
    tokio::spawn(async move {
        let mut simulator = SheffieldMatchSimulator::new_with_squads_and_formations(
            home_team_name,
            away_team_name,
            home_squad,
            away_squad,
            home_formation_code,
            away_formation_code,
        );

        // Generate match events
        let events = simulator.simulate_match_live();

        // Emit events one by one with slight delay for live feel
        for event in events {
            #[derive(serde::Serialize, Clone)]
            struct SheffieldMatchUpdate {
                match_id: String,
                minute: i32,
                event: Sheffield1867Event,
            }

            let update = SheffieldMatchUpdate {
                match_id: match_id_clone.clone(),
                minute: event.minute,
                event: event.clone(),
            };

            // Emit to all windows
            for window in app_handle.webview_windows().values() {
                let _ = window.emit("sheffield_1867_match_update", &update);
            }

            // Small delay between events for streaming effect (adjust as needed)
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    });

    Ok(format!("Sheffield 1867 match {} simulation started", match_id))
}

/// Get available formations for a specific year
#[tauri::command]
pub fn get_available_formations(year: i32) -> Result<Vec<serde_json::Value>, String> {
    use crate::match_engine::formation::Formation;

    let formations = Formation::all_for_era(year);

    let formation_data: Vec<serde_json::Value> = formations.iter().map(|f| {
        serde_json::json!({
            "code": f.code,
            "name": f.name,
            "description": f.description,
            "era_start_year": f.era_start_year,
            "era_end_year": f.era_end_year,
            "player_count": f.player_count(),
        })
    }).collect();

    Ok(formation_data)
}

/// Get detailed information about a specific formation
#[tauri::command]
pub fn get_formation_details(formation_code: String) -> Result<serde_json::Value, String> {
    use crate::match_engine::formation::Formation;

    let formation = match formation_code.as_str() {
        "1-1-8" => Formation::ultra_attacking_1_1_8(),
        "1-2-7" => Formation::early_sheffield_1_2_7(),
        "1-3-6" => Formation::transitional_1_3_6(),
        "2-3-5" => Formation::pyramid_2_3_5(),
        "2-2-6" => Formation::attacking_2_2_6(),
        _ => return Err(format!("Invalid formation code: {}", formation_code)),
    };

    Ok(serde_json::to_value(&formation).map_err(|e| e.to_string())?)
}

/// Start a live match that streams FrameState at 60fps via Tauri events.
/// Emits "live_frame" events with payload FrameState until FullTime.
/// Returns the match_id so the frontend can filter events.
#[tauri::command]
pub async fn start_live_match(
    app_handle: tauri::AppHandle,
    home_club_id: String,
    away_club_id: String,
) -> Result<String, String> {
    use crate::match_engine::live_engine::LiveMatchEngine;

    let match_id = format!("live-{}", uuid::Uuid::new_v4());

    // Load squads from DB, fall back to defaults if empty
    let home_detail = database::get_club_squad(&home_club_id).await.unwrap_or_default();
    let away_detail = database::get_club_squad(&away_club_id).await.unwrap_or_default();

    fn to_live(p: &crate::commands::PlayerDetail) -> crate::match_engine::live_engine::PlayerAttribs {
        crate::match_engine::live_engine::PlayerAttribs {
            pace: p.pace.unwrap_or(50) as f32,
            acceleration: p.acceleration.unwrap_or(50) as f32,
            finishing: p.finishing.unwrap_or(50) as f32,
            passing: p.passing.unwrap_or(50) as f32,
            heading: p.heading.unwrap_or(50) as f32,
            tackling: p.tackling.unwrap_or(50) as f32,
            dribbling: p.dribbling.unwrap_or(50) as f32,
            reflexes: p.reflexes.unwrap_or(50) as f32,
            handling: p.handling.unwrap_or(50) as f32,
            positioning: p.positioning.unwrap_or(50) as f32,
        }
    }

    let default_a = crate::match_engine::live_engine::PlayerAttribs::default();
    let home_attribs: Vec<_> = if home_detail.is_empty() {
        (0..11).map(|_| default_a.clone()).collect()
    } else {
        home_detail.iter().map(to_live).collect()
    };
    let away_attribs: Vec<_> = if away_detail.is_empty() {
        (0..11).map(|_| default_a.clone()).collect()
    } else {
        away_detail.iter().map(to_live).collect()
    };

    let seed = match_id.bytes().fold(0u64, |a, b| a.wrapping_mul(31).wrapping_add(b as u64));
    let mid  = match_id.clone();

    tokio::spawn(async move {
        let mut engine = LiveMatchEngine::new(
            mid.clone(),
            home_attribs,
            away_attribs,
            &home_club_id,
            &away_club_id,
            seed,
        );

        while !engine.is_finished() {
            let frame = engine.step();
            for win in app_handle.webview_windows().values() {
                let _ = win.emit("live_frame", &frame);
            }
            // Sleep ~16ms (60fps). Use tokio sleep so we don't block the async runtime.
            tokio::time::sleep(tokio::time::Duration::from_millis(16)).await;
        }

        // Emit one final FullTime frame
        let frame = engine.step();
        for win in app_handle.webview_windows().values() {
            let _ = win.emit("live_frame", &frame);
        }
    });

    Ok(match_id)
}

/// Get match replay data (for viewing completed matches)
#[tauri::command]
pub async fn get_match_replay_data(
    match_id: String,
) -> Result<MatchResult, String> {
    // TODO: Load from database
    // For now, return error indicating not yet implemented
    Err("Match replay loading not yet implemented - will load from database".to_string())
}

// ============================================================================
// DATABASE BACKUP COMMANDS
// ============================================================================

#[tauri::command]
pub async fn db_create_backup() -> Result<String, String> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    database::backup::create_backup(db_path)
        .map_err(|e| e.to_string())
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct BackupInfo {
    pub path: String,
    pub filename: String,
    pub size: u64,
    pub created_timestamp: u64,
}

#[tauri::command]
pub async fn db_list_backups() -> Result<Vec<BackupInfo>, String> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let backups = database::backup::list_backups(db_path)
        .map_err(|e| e.to_string())?;

    Ok(backups.into_iter().map(|b| BackupInfo {
        path: b.path,
        filename: b.filename,
        size: b.size,
        created_timestamp: b.created
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs(),
    }).collect())
}

#[tauri::command]
pub async fn db_restore_backup(backup_path: String) -> Result<(), String> {
    let target_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    database::backup::restore_backup(&backup_path, target_path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn db_cleanup_old_backups(keep_count: usize) -> Result<usize, String> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    database::backup::cleanup_old_backups(db_path, keep_count)
        .map_err(|e| e.to_string())
}

// ============================================================================
// CENSUS CSV IMPORT COMMANDS
// ============================================================================

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CensusImportStats {
    pub total_files: usize,
    pub successful_files: usize,
    pub failed_files: usize,
    pub total_players: usize,
    pub errors: Vec<String>,
}

#[tauri::command]
pub async fn db_import_all_census_files() -> Result<CensusImportStats, String> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867.db";
    let census_dir = "D:/projects/Saturday at Three/sheffield census";

    // Get write pool
    let pool = database::get_pool_write()
        .await
        .map_err(|e| e.to_string())?;

    let stats = database::census_importer::import_all_census_files(&pool, census_dir)
        .await
        .map_err(|e| e.to_string())?;

    Ok(CensusImportStats {
        total_files: stats.total_files,
        successful_files: stats.successful_files,
        failed_files: stats.failed_files,
        total_players: stats.total_players,
        errors: stats.errors,
    })
}

#[tauri::command]
pub async fn db_import_census_file(file_path: String) -> Result<usize, String> {
    let pool = database::get_pool_write()
        .await
        .map_err(|e| e.to_string())?;

    database::census_importer::import_census_file(&pool, &file_path)
        .await
        .map_err(|e| e.to_string())
}

// ============================================================================
// WHITES DIRECTORY MATCHING COMMANDS
// ============================================================================

use crate::database::whites_matcher;

#[tauri::command]
pub async fn whites_get_unmatched_entries() -> Result<Vec<whites_matcher::WhitesEntry>, String> {
    let pool = database::get_pool_write()
        .await
        .map_err(|e| e.to_string())?;

    whites_matcher::get_unmatched_whites_entries(&pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn whites_get_player_candidates() -> Result<Vec<whites_matcher::PlayerCandidate>, String> {
    let pool = database::get_pool_write()
        .await
        .map_err(|e| e.to_string())?;

    whites_matcher::get_player_candidates(&pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn whites_search_player_candidates(
    search_term: String,
    limit: i64,
) -> Result<Vec<whites_matcher::PlayerCandidate>, String> {
    let pool = database::get_pool_write()
        .await
        .map_err(|e| e.to_string())?;

    whites_matcher::search_player_candidates(&pool, &search_term, limit)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn whites_generate_suggestions(
    min_confidence: f32,
    max_suggestions_per_entry: usize,
) -> Result<Vec<whites_matcher::WhitesMatchSuggestion>, String> {
    let pool = database::get_pool_write()
        .await
        .map_err(|e| e.to_string())?;

    whites_matcher::generate_match_suggestions(&pool, min_confidence, max_suggestions_per_entry)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn whites_accept_match(
    whites_entry_id: String,
    player_id: String,
) -> Result<(), String> {
    let pool = database::get_pool_write()
        .await
        .map_err(|e| e.to_string())?;

    whites_matcher::accept_manual_match(&pool, &whites_entry_id, &player_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn whites_reject_match(
    whites_entry_id: String,
    player_id: String,
    notes: Option<String>,
) -> Result<(), String> {
    let pool = database::get_pool_write()
        .await
        .map_err(|e| e.to_string())?;

    whites_matcher::reject_match(&pool, &whites_entry_id, &player_id, notes)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn whites_get_stats() -> Result<whites_matcher::WhitesMatchStats, String> {
    let pool = database::get_pool_write()
        .await
        .map_err(|e| e.to_string())?;

    whites_matcher::get_match_stats(&pool)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_database_matching_stats() -> Result<(i64, i64, i64, i64), String> {
    let pool = database::get_pool_write()
        .await
        .map_err(|e| e.to_string())?;

    // Count people with matched businesses
    let matched_people: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT person_id) FROM sheffield_businesses WHERE person_id IS NOT NULL"
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    // Count unmatched businesses
    let unmatched_businesses: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_businesses WHERE person_id IS NULL"
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    // Count total businesses
    let total_businesses: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_businesses"
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    // Count total people
    let total_people: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_people"
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok((matched_people.0, unmatched_businesses.0, total_businesses.0, total_people.0))
}

#[tauri::command]
pub async fn count_abraham_adams() -> Result<(i64, i64), String> {
    let pool = database::get_pool_write()
        .await
        .map_err(|e| e.to_string())?;

    // Total count
    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_people WHERE name LIKE '%Abraham Adams%' OR (first_name LIKE '%Abraham%' AND surname LIKE '%Adams%')"
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    // Unique count (grouped by name, birth_year, street_address)
    let unique: (i64,) = sqlx::query_as(
        "SELECT COUNT(DISTINCT name || COALESCE(birth_year, 0) || COALESCE(street_address, '')) FROM sheffield_people WHERE name LIKE '%Abraham Adams%' OR (first_name LIKE '%Abraham%' AND surname LIKE '%Adams%')"
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok((total.0, unique.0))
}

#[tauri::command]
pub async fn db_list_all_tables() -> Result<Vec<String>, String> {
    let pool = database::get_pool_write()
        .await
        .map_err(|e| e.to_string())?;

    let tables: Vec<(String,)> = sqlx::query_as(
        "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(tables.into_iter().map(|(name,)| name).collect())
}

#[tauri::command]
pub async fn db_get_table_schema(table_name: String) -> Result<String, String> {
    let pool = database::get_pool_write()
        .await
        .map_err(|e| e.to_string())?;

    let schema: Vec<(String,)> = sqlx::query_as(
        "SELECT sql FROM sqlite_master WHERE type='table' AND name=?"
    )
    .bind(&table_name)
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(schema.first().map(|(s,)| s.clone()).unwrap_or_default())
}

#[tauri::command]
pub async fn get_female_footballers_count() -> Result<i64, String> {
    database::player_management::get_female_footballers_count()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_female_footballers() -> Result<i64, String> {
    database::player_management::delete_female_footballers()
        .await
        .map_err(|e| e.to_string())
}

// ============================================================================
// PARISH MANAGEMENT COMMANDS
// ============================================================================

/// Get all parishes with their postcodes from the sheffield_parishes table
#[tauri::command]
pub async fn db_get_all_parishes() -> Result<Vec<serde_json::Value>, String> {
    let pool = database::get_pool_write()
        .await
        .map_err(|e| e.to_string())?;

    // Ensure additional_postcode column exists
    let _ = sqlx::query(
        "ALTER TABLE sheffield_parishes ADD COLUMN additional_postcode TEXT"
    )
    .execute(&pool)
    .await;  // Ignore error if column already exists

    let parishes: Vec<(String, String, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT id, name, postcode, additional_postcode FROM sheffield_parishes ORDER BY name"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(parishes.into_iter().map(|(id, name, postcode, additional_postcode)| {
        serde_json::json!({
            "id": id,
            "name": name,
            "postcode": postcode,
            "additional_postcode": additional_postcode,
        })
    }).collect())
}

/// Assign a postcode to all players in a parish who don't currently have one
#[tauri::command]
pub async fn db_assign_postcode_to_parish(
    parish: String,
    postcode: String,
) -> Result<usize, String> {
    let pool = database::get_pool_write()
        .await
        .map_err(|e| e.to_string())?;

    // Update sheffield_people records where:
    // - ecclesiastical_parish matches the specified parish
    // - postcode is NULL or empty
    // - the person is also a footballer (exists in sheffield_footballers)
    let result = sqlx::query(
        "UPDATE sheffield_people
         SET postcode = ?
         WHERE ecclesiastical_parish = ?
         AND (postcode IS NULL OR postcode = '')
         AND unique_id IN (SELECT person_id FROM sheffield_footballers)"
    )
    .bind(&postcode)
    .bind(&parish)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(result.rows_affected() as usize)
}

/// Assign players from a parish to clubs based on matching postcode areas
/// Matches only the area code (e.g., "S6") not the full postcode (e.g., "S6 6FL")
/// Assigns up to 15 players per club
#[tauri::command]
pub async fn db_assign_parish_players_to_clubs(
    parish: String,
) -> Result<serde_json::Value, String> {
    use std::collections::HashMap;
    use rand::seq::SliceRandom;

    let pool = database::get_pool_write()
        .await
        .map_err(|e| e.to_string())?;

    // Get parish info including main postcode and additional postcode
    let parish_info: Option<(String, Option<String>)> = sqlx::query_as(
        "SELECT postcode, additional_postcode FROM sheffield_parishes WHERE name = ?"
    )
    .bind(&parish)
    .fetch_optional(&pool)
    .await
    .map_err(|e| e.to_string())?;

    let (parish_main_postcode, parish_additional_postcode) = parish_info
        .unwrap_or((String::new(), None));

    // Get all footballers in this parish with their postcodes
    let players: Vec<(i64, String, Option<String>)> = sqlx::query_as(
        "SELECT f.id, p.surname || ', ' || p.first_name as name, p.postcode
         FROM sheffield_footballers f
         JOIN sheffield_people p ON f.person_id = p.unique_id
         WHERE p.ecclesiastical_parish = ?
         AND f.club_id = 'UNASSIGNED'
         ORDER BY RANDOM()"
    )
    .bind(&parish)
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;

    // Store the total count before consuming the vector
    let total_players = players.len();

    println!("\n========================================");
    println!("Starting parish player assignment");
    println!("Parish: {}", parish);
    println!("Parish main postcode: {}", parish_main_postcode);
    println!("Parish additional postcode: {:?}", parish_additional_postcode);
    println!("Total unassigned players found: {}", total_players);
    println!("========================================\n");

    let mut assigned_count = 0;
    let mut skipped_no_postcode = 0;
    let mut skipped_no_clubs = 0;
    let mut skipped_club_full = 0;
    let mut assignments = Vec::new();

    // Track how many players each club has (both existing + newly assigned)
    let mut club_player_counts: HashMap<String, i32> = HashMap::new();
    let mut club_names: HashMap<String, String> = HashMap::new();

    // Get current player counts for all clubs
    let current_counts: Vec<(String, String, i32)> = sqlx::query_as(
        "SELECT c.id, c.name, COUNT(f.id) as count
         FROM sheffield_clubs c
         LEFT JOIN sheffield_footballers f ON c.id = f.club_id
         GROUP BY c.id, c.name"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;

    // Build initial club state
    let mut initial_club_state = Vec::new();
    for (club_id, club_name, count) in current_counts {
        club_player_counts.insert(club_id.clone(), count);
        club_names.insert(club_id.clone(), club_name.clone());
        initial_club_state.push(serde_json::json!({
            "club_id": club_id,
            "club_name": club_name,
            "current_players": count,
            "spaces_available": 15 - count,
        }));
    }
    initial_club_state.sort_by(|a, b| {
        a["club_name"].as_str().unwrap().cmp(b["club_name"].as_str().unwrap())
    });

    for (footballer_id, player_name, postcode_opt) in players {
        // Skip if no postcode
        let postcode = match postcode_opt {
            Some(p) if !p.is_empty() => p,
            _ => {
                skipped_no_postcode += 1;
                continue;
            }
        };

        // Extract postcode area (e.g., "S6" from "S6 6FL")
        let postcode_area = postcode.split_whitespace().next().unwrap_or(&postcode);

        // Find clubs matching the player's postcode OR the parish's additional postcode areas
        let mut clubs: Vec<(String, String, i32)> = if let Some(ref additional) = parish_additional_postcode {
            let additional_area = additional.split_whitespace().next().unwrap_or(additional);

            // Get clubs from player's postcode AND parish additional postcode, with priority flag
            sqlx::query_as(
                "SELECT id, name,
                 CASE
                   WHEN region LIKE ?1 OR additional_postcode LIKE ?1 THEN 0
                   WHEN region LIKE ?2 OR additional_postcode LIKE ?2 THEN 1
                   ELSE 2
                 END as priority
                 FROM sheffield_clubs
                 WHERE (region LIKE ?1 OR additional_postcode LIKE ?1)
                    OR (region LIKE ?2 OR additional_postcode LIKE ?2)"
            )
            .bind(format!("{}%", postcode_area))
            .bind(format!("{}%", additional_area))
            .fetch_all(&pool)
            .await
            .map_err(|e| e.to_string())?
        } else {
            // No additional postcode - just match player's postcode
            sqlx::query_as(
                "SELECT id, name, 0 as priority
                 FROM sheffield_clubs
                 WHERE region LIKE ? OR additional_postcode LIKE ?"
            )
            .bind(format!("{}%", postcode_area))
            .bind(format!("{}%", postcode_area))
            .fetch_all(&pool)
            .await
            .map_err(|e| e.to_string())?
        };

        if clubs.is_empty() {
            skipped_no_clubs += 1;
            continue;
        }

        // Filter out clubs that already have 15 or more players
        clubs.retain(|(club_id, _, _)| {
            let count = club_player_counts.get(club_id).unwrap_or(&0);
            *count < 15
        });

        if clubs.is_empty() {
            skipped_club_full += 1;
            continue;
        }

        // Sort clubs by:
        // 1. Priority (0 = player's postcode first, 1 = parish additional postcode second)
        // 2. Current player count (ascending) to fill evenly
        clubs.sort_by_key(|(club_id, _, priority)| {
            let count = *club_player_counts.get(club_id).unwrap_or(&0);
            (*priority, count)
        });

        // Select the club with highest priority and fewest players
        let (club_id, club_name, _priority) = &clubs[0];

        // Get current player count for this club
        let club_count_before = *club_player_counts.get(club_id.as_str()).unwrap_or(&0);

        // Build list of available clubs with their current counts
        let available_clubs: Vec<serde_json::Value> = clubs.iter().map(|(cid, cname, priority)| {
            let current = *club_player_counts.get(cid.as_str()).unwrap_or(&0);
            serde_json::json!({
                "club_id": cid,
                "club_name": cname,
                "current_players": current,
                "spaces_left": 15 - current,
                "priority": priority,
                "selected": cid == club_id,
            })
        }).collect();

        // Update the footballer's club_id
        sqlx::query(
            "UPDATE sheffield_footballers SET club_id = ? WHERE id = ?"
        )
        .bind(club_id.as_str())
        .bind(footballer_id)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

        // Increment the count for this club
        *club_player_counts.entry(club_id.clone()).or_insert(0) += 1;

        assigned_count += 1;

        // Print progress to console for real-time feedback
        println!("[{}/{}] Assigned {} → {} ({})",
            assigned_count,
            total_players,
            player_name,
            club_name.as_str(),
            postcode_area
        );

        assignments.push(serde_json::json!({
            "assignment_number": assigned_count,
            "player_name": player_name,
            "postcode": postcode,
            "postcode_area": postcode_area,
            "club_id": club_id.as_str(),
            "club_name": club_name.as_str(),
            "club_count_before": club_count_before,
            "club_count_after": club_count_before + 1,
            "available_clubs": available_clubs,
            "num_available_clubs": clubs.len(),
        }));
    }

    // Build final club state
    let mut final_club_state = Vec::new();
    let mut clubs_that_received_players = Vec::new();

    for (club_id, final_count) in &club_player_counts {
        let club_name = club_names.get(club_id).unwrap_or(&"Unknown".to_string()).clone();
        let initial_count = initial_club_state.iter()
            .find(|c| c["club_id"].as_str().unwrap() == club_id)
            .and_then(|c| c["current_players"].as_i64())
            .unwrap_or(0) as i32;

        final_club_state.push(serde_json::json!({
            "club_id": club_id,
            "club_name": club_name,
            "players_before": initial_count,
            "players_after": final_count,
            "players_added": final_count - initial_count,
            "spaces_remaining": 15 - final_count,
        }));

        if final_count > &initial_count {
            clubs_that_received_players.push(serde_json::json!({
                "club_id": club_id,
                "club_name": club_name,
                "players_added": final_count - initial_count,
                "final_count": final_count,
            }));
        }
    }

    final_club_state.sort_by(|a, b| {
        a["club_name"].as_str().unwrap().cmp(b["club_name"].as_str().unwrap())
    });

    clubs_that_received_players.sort_by(|a, b| {
        b["players_added"].as_i64().unwrap().cmp(&a["players_added"].as_i64().unwrap())
    });

    println!("\n========================================");
    println!("Assignment Complete!");
    println!("----------------------------------------");
    println!("✓ Assigned: {}", assigned_count);
    println!("⊘ Skipped (no postcode): {}", skipped_no_postcode);
    println!("⊘ Skipped (no matching clubs): {}", skipped_no_clubs);
    println!("⊘ Skipped (clubs full): {}", skipped_club_full);
    println!("----------------------------------------");
    println!("Clubs that received players: {}", clubs_that_received_players.len());
    for club in &clubs_that_received_players {
        println!("  • {}: +{} players (now {})",
            club["club_name"].as_str().unwrap(),
            club["players_added"].as_i64().unwrap(),
            club["final_count"].as_i64().unwrap()
        );
    }
    println!("========================================\n");

    Ok(serde_json::json!({
        "parish": parish,
        "total_players_processed": total_players,
        "assigned_count": assigned_count,
        "skipped_no_postcode": skipped_no_postcode,
        "skipped_no_clubs": skipped_no_clubs,
        "skipped_club_full": skipped_club_full,
        "initial_club_state": initial_club_state,
        "final_club_state": final_club_state,
        "clubs_that_received_players": clubs_that_received_players,
        "assignments": assignments,
    }))
}

/// Update a club's additional postcode for recruitment
#[tauri::command]
pub async fn db_update_club_additional_postcode(
    club_id: String,
    additional_postcode: Option<String>,
) -> Result<(), String> {
    let pool = database::get_pool_write()
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query(
        "UPDATE sheffield_clubs SET additional_postcode = ? WHERE id = ?"
    )
    .bind(additional_postcode)
    .bind(&club_id)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// Update a parish's additional postcode for player assignment
#[tauri::command]
pub async fn db_update_parish_additional_postcode(
    parish_name: String,
    additional_postcode: Option<String>,
) -> Result<(), String> {
    let pool = database::get_pool_write()
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query(
        "UPDATE sheffield_parishes SET additional_postcode = ? WHERE name = ?"
    )
    .bind(additional_postcode)
    .bind(&parish_name)
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// Check how many players have been assigned to clubs
#[tauri::command]
pub async fn db_check_club_assignments() -> Result<serde_json::Value, String> {
    let pool = database::get_pool()
        .await
        .map_err(|e| e.to_string())?;

    // Count total players, assigned players, and unassigned players
    let stats: (i64, i64, i64) = sqlx::query_as(
        "SELECT
            COUNT(*) as total,
            SUM(CASE WHEN club_id != 'UNASSIGNED' AND club_id IS NOT NULL THEN 1 ELSE 0 END) as assigned,
            SUM(CASE WHEN club_id = 'UNASSIGNED' OR club_id IS NULL THEN 1 ELSE 0 END) as unassigned
         FROM sheffield_footballers"
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    // Get top 10 clubs by player count
    let club_counts: Vec<(String, String, i64)> = sqlx::query_as(
        "SELECT c.id, c.name, COUNT(f.id) as player_count
         FROM sheffield_clubs c
         LEFT JOIN sheffield_footballers f ON c.id = f.club_id
         GROUP BY c.id, c.name
         HAVING player_count > 0
         ORDER BY player_count DESC
         LIMIT 10"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(serde_json::json!({
        "total_players": stats.0,
        "assigned_players": stats.1,
        "unassigned_players": stats.2,
        "top_clubs": club_counts.iter().map(|(id, name, count)| {
            serde_json::json!({
                "club_id": id,
                "club_name": name,
                "player_count": count
            })
        }).collect::<Vec<_>>()
    }))
}

/// Check Sheffield1867_temp.db data for fantasy league diagnostics
#[tauri::command]
pub async fn check_temp_db_data() -> Result<serde_json::Value, String> {
    let db_path = "D:/projects/Saturday at Three/Sheffield1867_temp.db";
    let pool = database::sheffield_db::create_sheffield_database(db_path)
        .await
        .map_err(|e| format!("Failed to open temp database: {}", e))?;

    // Count clubs in sheffield_league_clubs
    let league_clubs_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_league_clubs")
        .fetch_one(&pool)
        .await
        .map_err(|e| format!("Failed to count league_clubs: {}", e))?;

    // Count standings entries
    let standings_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sheffield_standings")
        .fetch_one(&pool)
        .await
        .map_err(|e| format!("Failed to count standings: {}", e))?;

    // Count clubs in divisions
    let division_counts: Vec<(String, String, i64)> = sqlx::query_as(
        "SELECT slc.division_id, sld.name, COUNT(*) as club_count
         FROM sheffield_league_clubs slc
         JOIN sheffield_league_divisions sld ON slc.division_id = sld.id
         GROUP BY slc.division_id, sld.name
         ORDER BY sld.level, sld.region"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| format!("Failed to get division counts: {}", e))?;

    // Sample some clubs from div-1
    let div1_clubs: Vec<(String, String)> = sqlx::query_as(
        "SELECT slc.club_id, sc.name
         FROM sheffield_league_clubs slc
         JOIN sheffield_clubs sc ON slc.club_id = sc.id
         WHERE slc.division_id = 'div-1'
         LIMIT 5"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| format!("Failed to get div-1 clubs: {}", e))?;

    Ok(serde_json::json!({
        "league_clubs_count": league_clubs_count.0,
        "standings_count": standings_count.0,
        "division_counts": division_counts.iter().map(|(id, name, count)| {
            serde_json::json!({
                "division_id": id,
                "division_name": name,
                "club_count": count
            })
        }).collect::<Vec<_>>(),
        "div1_sample": div1_clubs.iter().map(|(id, name)| {
            serde_json::json!({
                "club_id": id,
                "club_name": name
            })
        }).collect::<Vec<_>>()
    }))
}

// ============================================================================
// CHALLENGE INVITATION SYSTEM COMMANDS
// ============================================================================

/// Send a challenge letter to another club
#[tauri::command]
pub async fn send_challenge_letter(
    mut game: GameState,
    sender_club_id: String,
    recipient_club_id: String,
    current_date: String,
    proposed_match_date: String,
    match_type: String,
    venue: String,
    stakes: String,
    tone: String,
    rules_type: String,
    match_duration: i32,
) -> Result<serde_json::Value, String> {
    let pool = database::get_active_game_pool(&game).await
        .map_err(|e| format!("Failed to get database pool: {}", e))?;

    let invitation = database::challenge_invitations::send_challenge_letter(
        &pool,
        &sender_club_id,
        &recipient_club_id,
        &current_date,
        &proposed_match_date,
        &match_type,
        &venue,
        &stakes,
        &tone,
        &rules_type,
        match_duration,
    )
    .await
    .map_err(|e| format!("Failed to send challenge letter: {}", e))?;

    // Fetch the news item that was just created and add it to pending events
    let news_items = database::challenge_invitations::get_news_for_date(&pool, &current_date)
        .await
        .map_err(|e| format!("Failed to get news: {}", e))?;

    // Find the news item we just created (it should be the last one for this invitation)
    if let Some(news_item) = news_items.iter().find(|n| n.related_invitation_id.as_deref() == Some(&invitation.id)) {
        let event = crate::game::GameEvent {
            id: news_item.id.clone(),
            date: news_item.publish_date.clone(),
            event_type: crate::game::EventType::HistoricalAnnouncement {
                title: news_item.headline.clone(),
                description: news_item.body_text.clone(),
            },
            requires_user_action: false,
            processed: true,  // Mark as processed since we're showing it immediately
            result: None,
        };

        game.pending_events.push(event);

        // Save the updated game state
        if let Some(save_file) = &game.loaded_from_file {
            let sanitized_name = save_file.trim_end_matches(".json")
                .chars()
                .map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
                .collect::<String>();
            database::save_game_named(&game, &sanitized_name).await.map_err(|e| e.to_string())?;
        } else {
            database::save_game(&game).await.map_err(|e| e.to_string())?;
        }
    }

    Ok(serde_json::to_value(invitation)
        .map_err(|e| format!("Failed to serialize invitation: {}", e))?)
}

/// Get all news items for a specific date
#[tauri::command]
pub async fn get_news_for_date(game: GameState, date: String) -> Result<Vec<serde_json::Value>, String> {
    let pool = database::get_active_game_pool(&game).await
        .map_err(|e| format!("Failed to get database pool: {}", e))?;

    let news_items = database::challenge_invitations::get_news_for_date(&pool, &date)
        .await
        .map_err(|e| format!("Failed to get news: {}", e))?;

    news_items.iter()
        .map(|item| serde_json::to_value(item)
            .map_err(|e| format!("Failed to serialize news item: {}", e)))
        .collect()
}

/// Get all news items (for historical event list)
#[tauri::command]
pub async fn get_all_news(game: GameState) -> Result<Vec<serde_json::Value>, String> {
    let pool = database::get_active_game_pool(&game).await
        .map_err(|e| format!("Failed to get database pool: {}", e))?;

    // Only the post that has actually arrived — nothing dated after today.
    let news_items = database::challenge_invitations::get_all_news(&pool, &game.current_date)
        .await
        .map_err(|e| format!("Failed to get all news: {}", e))?;

    news_items.iter()
        .map(|item| serde_json::to_value(item)
            .map_err(|e| format!("Failed to serialize news item: {}", e)))
        .collect()
}

/// Get all unread news items (for notification badge)
#[tauri::command]
pub async fn get_unread_news_count(game: GameState) -> Result<i64, String> {
    let pool = database::get_active_game_pool(&game).await
        .map_err(|e| format!("Failed to get database pool: {}", e))?;

    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM sheffield_news_items WHERE is_read = 0"
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| format!("Failed to get unread count: {}", e))?;

    Ok(count.0)
}

/// Mark a news item as read
#[tauri::command]
pub async fn mark_news_as_read(game: GameState, news_id: String) -> Result<(), String> {
    let pool = database::get_active_game_pool(&game).await
        .map_err(|e| format!("Failed to get database pool: {}", e))?;

    sqlx::query("UPDATE sheffield_news_items SET is_read = 1 WHERE id = ?")
        .bind(&news_id)
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to mark news as read: {}", e))?;

    Ok(())
}

/// Create "MATCH CONFIRMED" news when user reads an acceptance letter
#[tauri::command]
pub async fn create_match_confirmed_news(game: GameState, invitation_id: String) -> Result<(), String> {
    let pool = database::get_active_game_pool(&game).await
        .map_err(|e| format!("Failed to get database pool: {}", e))?;

    database::challenge_invitations::create_match_confirmed_news_for_invitation(&pool, &invitation_id)
        .await
        .map_err(|e| format!("Failed to create match confirmed news: {}", e))?;

    Ok(())
}

/// Get a specific invitation by ID (for reading the letter)
#[tauri::command]
pub async fn get_invitation_by_id(game: GameState, invitation_id: String) -> Result<serde_json::Value, String> {
    let pool = database::get_active_game_pool(&game).await
        .map_err(|e| format!("Failed to get database pool: {}", e))?;

    let invitation = database::challenge_invitations::get_invitation_by_id(&pool, &invitation_id)
        .await
        .map_err(|e| format!("Failed to get invitation: {}", e))?;

    Ok(serde_json::to_value(invitation)
        .map_err(|e| format!("Failed to serialize invitation: {}", e))?)
}

/// Generate trial players for a trial session (10-15 men aged 16-40)
#[tauri::command]
pub async fn generate_trial_players(
    club_id: String,
    current_year: i32,
) -> Result<Vec<PlayerDetail>, String> {
    use rand::Rng;

    // Generate random count before any await points to avoid Send issues
    let count = {
        let mut rng = rand::thread_rng();
        rng.gen_range(10..=15)
    };

    let pool = database::get_pool().await
        .map_err(|e| format!("Failed to get database pool: {}", e))?;

    // Get 10-15 random unassigned male players aged 16-40
    let min_birth_year = current_year - 40;
    let max_birth_year = current_year - 16;

    let players: Vec<PlayerDetail> = sqlx::query(
        "SELECT
            f.person_id,
            (f.first_name || ' ' || f.surname) as name,
            f.club_id,
            f.position,
            f.birth_year,
            CAST((? - COALESCE(f.birth_year, ?)) AS INTEGER) as age,
            f.nationality,
            p.profession,
            p.ecclesiastical_parish,
            p.street_address,
            COALESCE(p.where_born, p.birth_town) as birthplace,
            f.pace, f.acceleration, f.strength, f.stamina, f.balance, f.jumping, f.agility, f.natural_fitness,
            f.passing, f.dribbling, f.first_touch, f.technique, f.heading, f.long_passing, f.crossing, f.long_shots,
            f.tackling, f.handling, f.reflexes, f.corners, f.free_kicks, f.throw_ins, f.vision,
            f.left_foot, f.right_foot, f.one_on_ones,
            f.courage, f.bravery, f.concentration, f.decision_making,
            f.leadership, f.aggression, f.anticipation, f.determination, f.flair, f.influence,
            f.adaptability, f.ambition, f.loyalty, f.pressure, f.professionalism, f.sportsmanship, f.temperament,
            f.awareness, f.marking, f.positioning, f.work_rate, f.off_the_ball, f.movement, f.teamwork,
            f.finishing, f.penalties, f.set_pieces,
            f.consistency, f.dirtiness, f.versatility, f.injury_proneness, f.important_matches,
            f.current_ability, f.potential_ability, f.current_reputation
         FROM sheffield_footballers f
         LEFT JOIN sheffield_people p ON f.person_id = p.unique_id
         WHERE f.club_id = 'UNASSIGNED'
         AND f.has_stats = 1
         AND p.census_gender = 'Male'
         AND f.birth_year >= ?
         AND f.birth_year <= ?
         ORDER BY RANDOM()
         LIMIT ?"
    )
    .bind(current_year)
    .bind(current_year - 25)
    .bind(min_birth_year)
    .bind(max_birth_year)
    .bind(count)
    .fetch_all(&pool)
    .await
    .map_err(|e| format!("Failed to fetch trial players: {}", e))?
    .iter()
    .map(|row| {
        use sqlx::Row;
        PlayerDetail {
            id: row.get::<i64, _>(0).to_string(),
            name: row.get(1),
            club_id: row.get(2),
            position: row.get(3),
            birth_year: row.get(4),
            age: row.get(5),
            nationality: row.get(6),
            profession: row.get(7),
            parish: row.get(8),
            address: row.get(9),
            birthplace: row.get(10),
            pace: row.get(11),
            acceleration: row.get(12),
            strength: row.get(13),
            stamina: row.get(14),
            balance: row.get(15),
            jumping: row.get(16),
            agility: row.get(17),
            natural_fitness: row.get(18),
            passing: row.get(19),
            dribbling: row.get(20),
            first_touch: row.get(21),
            technique: row.get(22),
            heading: row.get(23),
            long_passing: row.get(24),
            crossing: row.get(25),
            long_shots: row.get(26),
            tackling: row.get(27),
            handling: row.get(28),
            reflexes: row.get(29),
            corners: row.get(30),
            free_kicks: row.get(31),
            throw_ins: row.get(32),
            vision: row.get(33),
            left_foot: row.get(34),
            right_foot: row.get(35),
            one_on_ones: row.get(36),
            courage: row.get(37),
            bravery: row.get(38),
            concentration: row.get(39),
            decision_making: row.get(40),
            leadership: row.get(41),
            aggression: row.get(42),
            anticipation: row.get(43),
            determination: row.get(44),
            flair: row.get(45),
            influence: row.get(46),
            adaptability: row.get(47),
            ambition: row.get(48),
            loyalty: row.get(49),
            pressure: row.get(50),
            professionalism: row.get(51),
            sportsmanship: row.get(52),
            temperament: row.get(53),
            awareness: row.get(54),
            marking: row.get(55),
            positioning: row.get(56),
            work_rate: row.get(57),
            off_the_ball: row.get(58),
            movement: row.get(59),
            teamwork: row.get(60),
            finishing: row.get(61),
            penalties: row.get(62),
            set_pieces: row.get(63),
            consistency: row.get(64),
            dirtiness: row.get(65),
            versatility: row.get(66),
            injury_proneness: row.get(67),
            important_matches: row.get(68),
            current_ability: row.get(69),
            potential_ability: row.get(70),
            current_reputation: row.get(71),
        }
    })
    .collect();

    Ok(players)
}

/// Check for pending responses (called during date advance)
#[tauri::command]
pub async fn check_pending_invitation_responses(game: GameState, current_date: String) -> Result<Vec<serde_json::Value>, String> {
    let pool = database::get_active_game_pool(&game).await
        .map_err(|e| format!("Failed to get database pool: {}", e))?;

    let processed = database::challenge_invitations::check_pending_responses(&pool, &current_date)
        .await
        .map_err(|e| format!("Failed to check pending responses: {}", e))?;

    processed.iter()
        .map(|inv| serde_json::to_value(inv)
            .map_err(|e| format!("Failed to serialize invitation: {}", e)))
        .collect()
}

/// Wipe all challenge invitations and news from the master Sheffield1867.db
#[tauri::command]
pub async fn wipe_master_challenge_data() -> Result<String, String> {
    let pool = database::get_pool_write().await
        .map_err(|e| format!("Failed to get database pool: {}", e))?;

    // Delete news first (child table with foreign key)
    sqlx::query("DELETE FROM sheffield_news_items")
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to delete news: {}", e))?;

    // Then delete invitations (parent table)
    sqlx::query("DELETE FROM sheffield_challenge_invitations")
        .execute(&pool)
        .await
        .map_err(|e| format!("Failed to delete invitations: {}", e))?;

    Ok("Successfully wiped all challenge letters and news from Sheffield1867.db".to_string())
}

/// Schedule a trial session for a future date
#[tauri::command]
pub async fn schedule_trial_session(
    mut game: GameState,
    trial_date: String,
) -> Result<GameState, String> {
    // Validate that the trial date is in the future
    use chrono::NaiveDate;
    let current_date = NaiveDate::parse_from_str(&game.current_date, "%Y-%m-%d")
        .map_err(|e| format!("Failed to parse current date: {}", e))?;
    let trial_date_parsed = NaiveDate::parse_from_str(&trial_date, "%Y-%m-%d")
        .map_err(|e| format!("Failed to parse trial date: {}", e))?;

    if trial_date_parsed <= current_date {
        return Err("Trial date must be in the future".to_string());
    }

    // Create a new TrialSession event
    let event = crate::game::GameEvent {
        id: uuid::Uuid::new_v4().to_string(),
        date: trial_date.clone(),
        event_type: crate::game::EventType::TrialSession {
            club_id: game.user_club_id.clone(),
            title: "Trial Session".to_string(),
            description: format!("Trial session scheduled for {}. Young men will arrive to demonstrate their skills.", trial_date),
        },
        requires_user_action: true,
        processed: false,
        result: None,
    };

    // Add event to pending events
    game.pending_events.push(event);

    // Sort pending events by date
    game.pending_events.sort_by(|a, b| a.date.cmp(&b.date));

    // Update the game state timestamp
    game.updated_at = chrono::Utc::now();

    Ok(game)
}
