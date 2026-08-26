pub mod sheffield_db;
pub mod populate;
pub mod rulesets;
pub mod sheffield_league;
pub mod promotion_db;
pub mod player_generator;
pub mod cup_competitions;
pub mod league_config;
pub mod league_metadata;
pub mod player_management;
pub mod backup;
pub mod census_importer;
pub mod genealogy_importer;
pub mod genealogy_matcher;
pub mod whites_matcher;
pub mod challenge_invitations;

pub use sheffield_db::{create_sheffield_database, initialize_schema, migrate_sheffield_footballers};
pub use populate::{populate_clubs_for_year, populate_players_for_year, initialize_standings, create_fixtures_for_season};
pub use rulesets::{get_ruleset_for_year, save_ruleset_to_db};
pub use sheffield_league::populate_sheffield_league;
pub use promotion_db::{
    get_division_standings_simple, update_club_division, batch_update_club_divisions,
    record_movement_event, recalculate_division_positions,
};
// Cup competitions module - wrappers defined below
pub use cup_competitions::{Competition, CupTie, EligibleClub};
pub use league_config::LeagueConfig;
pub use league_metadata::LeagueMetadata;
pub use player_management::{
    get_all_players,
    get_players_paginated,
    get_age_statistics,
    get_first_name_statistics,
    get_surname_statistics,
    create_player_indexes,
    PlayerQueryResult,
    AgeStatistic,
    NameStatistic,
};
pub use genealogy_importer::{read_genealogy_csv, read_all_genealogy_csvs};
pub use genealogy_matcher::{test_match_genealogy, MatchResult, MatchStats};

// Legacy game save/load functions (JSON-based)
use crate::game::GameState;
use std::path::PathBuf;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions, SqliteConnectOptions};
use std::str::FromStr;

fn get_db_path() -> String {
    "D:/projects/Saturday at Three/Sheffield1867.db".to_string()
}

fn get_temp_db_path() -> String {
    "D:/projects/Saturday at Three/sheffield1867_temp.db".to_string()
}

/// Read-only pool — used by game-play code. Cannot modify Sheffield1867.db.
pub async fn get_pool() -> Result<SqlitePool, Box<dyn std::error::Error>> {
    let db_path = get_db_path();
    let opts = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?
        .read_only(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(opts)
        .await?;
    Ok(pool)
}

/// Read-write pool — used only by the Sheffield Rules Editor and Claude Code.
/// Game-play code must NOT call this.
pub async fn get_pool_write() -> Result<SqlitePool, Box<dyn std::error::Error>> {
    let db_path = get_db_path();
    let opts = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?
        .create_if_missing(false);
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(opts)
        .await?;
    Ok(pool)
}

pub async fn init(_app_dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

/// Initialize a new game by copying Sheffield1867.db to sheffield1867_temp.db
pub async fn init_new_game() -> Result<(), Box<dyn std::error::Error>> {
    let master_db = get_db_path();
    let temp_db = get_temp_db_path();

    // Copy master database to temp
    std::fs::copy(&master_db, &temp_db)?;

    println!("[NEW GAME] Copied {} to {}", master_db, temp_db);

    // The real 1867 Youdan Cup happened before any save begins — write it into
    // the record books as history, not a match to be played.
    let opts = SqliteConnectOptions::from_str(&format!("sqlite:{}", temp_db))?.create_if_missing(false);
    let pool = SqlitePoolOptions::new().max_connections(1).connect_with(opts).await?;
    match crate::fsim_bridge::seed_1867_youdan_cup_history(&pool).await {
        Ok(true) => println!("[NEW GAME] Seeded the 1867 Youdan Cup as history"),
        Ok(false) => {}
        Err(e) => eprintln!("[NEW GAME] Could not seed Youdan Cup history: {}", e),
    }
    pool.close().await;

    Ok(())
}

/// Get a pool for the active game database (either temp or a .sav file)
/// If game.loaded_from_file is set, use that .sav file, otherwise use temp
pub async fn get_active_game_pool(game: &GameState) -> Result<SqlitePool, Box<dyn std::error::Error>> {
    let db_path = if let Some(save_file) = &game.loaded_from_file {
        if save_file.ends_with(".sav") {
            // Using a saved game file
            let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
            let save_path = home_dir.join(".saturday_at_three").join(save_file);
            save_path.to_string_lossy().to_string()
        } else {
            // Using temp database
            get_temp_db_path()
        }
    } else {
        // Using temp database
        get_temp_db_path()
    };

    let opts = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?
        .create_if_missing(false);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(opts)
        .await?;

    Ok(pool)
}

pub async fn save_game(game: &GameState) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(game)?;
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let save_dir = home_dir.join(".saturday_at_three");
    std::fs::create_dir_all(&save_dir)?;

    let save_path = save_dir.join(format!("{}.json", game.id));
    std::fs::write(save_path, json)?;

    Ok(())
}

pub async fn load_game(game_id: &str) -> Result<GameState, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let save_path = home_dir.join(".saturday_at_three").join(format!("{}.json", game_id));

    let json = std::fs::read_to_string(save_path)?;
    let game: GameState = serde_json::from_str(&json)?;

    Ok(game)
}

pub async fn get_latest_game() -> Result<GameState, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let save_dir = home_dir.join(".saturday_at_three");

    if !save_dir.exists() {
        return Ok(GameState::new("default_club".to_string()));
    }

    let mut latest_game: Option<GameState> = None;
    let mut latest_time = std::time::SystemTime::UNIX_EPOCH;

    for entry in std::fs::read_dir(&save_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "json") {
            if let Ok(metadata) = path.metadata() {
                if let Ok(modified) = metadata.modified() {
                    if modified > latest_time {
                        latest_time = modified;
                        if let Ok(json) = std::fs::read_to_string(&path) {
                            if let Ok(game) = serde_json::from_str::<GameState>(&json) {
                                latest_game = Some(game);
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(latest_game.unwrap_or_else(|| GameState::new("default_club".to_string())))
}

pub async fn save_game_named(game: &GameState, save_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(game)?;
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let save_dir = home_dir.join(".saturday_at_three");
    std::fs::create_dir_all(&save_dir)?;

    // Sanitize the save name to create a valid filename
    let sanitized_name = save_name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
        .collect::<String>();

    let save_path = save_dir.join(format!("{}.sav", sanitized_name));
    let temp_db_path = get_temp_db_path();

    // If temp database exists, copy it to the .sav file (first save of a new game)
    if PathBuf::from(&temp_db_path).exists() && !save_path.exists() {
        println!("[SAVE] Copying temp database to {}", save_path.display());
        std::fs::copy(&temp_db_path, &save_path)?;

        // Delete temp database after successful copy
        std::fs::remove_file(&temp_db_path)?;
        println!("[SAVE] Deleted temp database");
    }

    // Open the .sav file
    let db_url = format!("sqlite:{}", save_path.to_string_lossy());
    let opts = SqliteConnectOptions::from_str(&db_url)?
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(opts)
        .await?;

    // Ensure the game_state table exists
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS game_state (
            id TEXT PRIMARY KEY,
            data TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(&pool)
    .await?;

    // Insert or update the game state
    sqlx::query(
        "INSERT OR REPLACE INTO game_state (id, data, updated_at)
         VALUES (?, ?, CURRENT_TIMESTAMP)"
    )
    .bind(&game.id)
    .bind(&json)
    .execute(&pool)
    .await?;

    println!("[SAVE] Saved game '{}' to {}", save_name, save_path.display());

    Ok(())
}

pub async fn load_game_named(save_name: &str) -> Result<GameState, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));

    // Sanitize the save name to match the save file
    let sanitized_name = save_name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
        .collect::<String>();

    // Open .sav file as SQLite database
    let save_path = home_dir.join(".saturday_at_three").join(format!("{}.sav", sanitized_name));

    if !save_path.exists() {
        return Err(format!("Save file not found: {}", save_path.display()).into());
    }

    let db_url = format!("sqlite:{}", save_path.to_string_lossy());

    let opts = SqliteConnectOptions::from_str(&db_url)?
        .create_if_missing(false);
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(opts)
        .await?;

    // Load the game state from the database
    let json: String = sqlx::query_scalar(
        "SELECT data FROM game_state ORDER BY updated_at DESC LIMIT 1"
    )
    .fetch_one(&pool)
    .await?;

    let mut game: GameState = serde_json::from_str(&json)?;

    // Track which file this game was loaded from
    game.loaded_from_file = Some(format!("{}.sav", sanitized_name));

    println!("[LOAD] Loaded game '{}' from {}", save_name, save_path.display());

    Ok(game)
}

pub async fn get_save_list() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let save_dir = home_dir.join(".saturday_at_three");

    if !save_dir.exists() {
        return Ok(vec![]);
    }

    let mut saves = vec![];

    for entry in std::fs::read_dir(&save_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "sav") {
            if let Some(file_name) = path.file_stem() {
                if let Some(name) = file_name.to_str() {
                    saves.push(name.to_string());
                }
            }
        }
    }

    saves.sort();
    Ok(saves)
}

pub async fn delete_save_file(save_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));

    // Sanitize the save name to match the save file
    let sanitized_name = save_name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
        .collect::<String>();

    let save_path = home_dir.join(".saturday_at_three").join(format!("{}.json", sanitized_name));

    if save_path.exists() {
        std::fs::remove_file(save_path)?;
    }

    Ok(())
}

pub async fn get_all_clubs() -> Result<Vec<crate::commands::ClubInfo>, Box<dyn std::error::Error>> {
    let pool = get_pool().await?;

    let rows = sqlx::query(
        "SELECT
            c.id,
            c.name,
            c.founded_year,
            c.ground_name,
            c.city,
            c.region,
            c.origin,
            lc.reserve_of_club_id,
            parent.name as parent_club_name,
            c.region as where_from,
            c.additional_postcode
         FROM sheffield_clubs c
         LEFT JOIN sheffield_league_clubs lc ON c.id = lc.club_id AND lc.is_reserve_team = 1
         LEFT JOIN sheffield_clubs parent ON lc.reserve_of_club_id = parent.id
         ORDER BY c.name"
    )
    .fetch_all(&pool)
    .await?;

    let clubs = rows.iter().map(|row| {
        use sqlx::Row;
        crate::commands::ClubInfo {
            id: row.get(0),
            name: row.get(1),
            short_name: row.get::<String, _>(1).chars().take(3).collect(),
            founded_year: row.get(2),
            ground_name: row.get(3),
            ground_capacity: 0,
            city: row.get::<Option<String>, _>(4).unwrap_or_default(),
            region: row.get::<Option<String>, _>(5).unwrap_or_default(),
            origin: row.get::<Option<String>, _>(6).unwrap_or_default(),
            primary_color: String::new(),
            secondary_color: String::new(),
            reserve_of_club_id: row.get(7),
            parent_club_name: row.get(8),
            where_from: row.get(9),
            additional_postcode: row.get(10),
        }
    }).collect();

    Ok(clubs)
}

pub async fn get_reserve_team_id(parent_club_id: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let pool = get_pool().await?;

    let reserve_club_id: Option<String> = sqlx::query_scalar(
        "SELECT club_id FROM sheffield_league_clubs
         WHERE reserve_of_club_id = ? AND is_reserve_team = 1
         LIMIT 1"
    )
    .bind(parent_club_id)
    .fetch_optional(&pool)
    .await?;

    Ok(reserve_club_id)
}

pub async fn get_club_squad(club_id: &str) -> Result<Vec<crate::commands::PlayerDetail>, Box<dyn std::error::Error>> {
    let pool = get_pool().await?;

    println!("[SQUAD DEBUG] Getting squad for club_id: '{}'", club_id);

    // Join with sheffield_people to get census data
    let rows = sqlx::query(
        "SELECT
            f.person_id,
            (f.first_name || ' ' || f.surname) as name,
            f.club_id,
            f.position,
            f.birth_year,
            CAST((1867 - COALESCE(f.birth_year, 1845)) AS INTEGER) as age,
            f.nationality,
            p.profession,
            p.ecclesiastical_parish,
            p.street_address,
            COALESCE(p.where_born, p.birth_town || CASE WHEN p.birth_county IS NOT NULL THEN ', ' || p.birth_county ELSE '' END) as birthplace,
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
         WHERE f.club_id = ?
         ORDER BY f.surname, f.first_name"
    )
    .bind(club_id)
    .fetch_all(&pool)
    .await?;

    println!("[SQUAD DEBUG] Found {} players for club_id: '{}'", rows.len(), club_id);

    let players = rows.iter().map(|row| {
        use sqlx::Row;
        crate::commands::PlayerDetail {
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
    }).collect();

    Ok(players)
}

pub async fn get_club_info(club_id: &str) -> Result<crate::commands::ClubInfo, Box<dyn std::error::Error>> {
    let pool = get_pool().await?;

    let row = sqlx::query(
        "SELECT id, name, founded_year, ground_name, city, region, origin FROM sheffield_clubs WHERE id = ?"
    )
    .bind(club_id)
    .fetch_one(&pool)
    .await?;

    use sqlx::Row;
    let name: String = row.get(1);
    let club = crate::commands::ClubInfo {
        id: row.get(0),
        name: name.clone(),
        short_name: name.chars().take(3).collect(),
        founded_year: row.get(2),
        ground_name: row.get(3),
        ground_capacity: 0,
        city: row.get::<Option<String>, _>(4).unwrap_or_default(),
        region: row.get::<Option<String>, _>(5).unwrap_or_default(),
        origin: row.get::<Option<String>, _>(6).unwrap_or_default(),
        primary_color: String::new(),
        secondary_color: String::new(),
        reserve_of_club_id: None,
        parent_club_name: None,
        where_from: None,
        additional_postcode: None,
    };

    Ok(club)
}

pub async fn bulk_add_players(
    club_id: &str,
    player_names: Vec<String>,
    year: i32,
    has_goalkeeper: bool,
) -> Result<Vec<player_generator::GeneratedPlayer>, Box<dyn std::error::Error>> {
    let pool = get_pool_write().await?;

    let mut generated_players = Vec::new();

    for name in player_names {
        let input = player_generator::PlayerInput {
            name,
            position: None, // Auto-assign position
            birth_year: None, // Auto-assign age
        };

        let player = player_generator::generate_player(&input, club_id, year, has_goalkeeper);

        // Insert into database
        sqlx::query(
            r#"
            INSERT INTO sheffield_footballers (
                id, name, club_id, position, birth_year, nationality, height_cm,
                pace, strength, stamina, balance, jumping, agility,
                passing, dribbling, first_touch, heading, long_passing, crossing, long_shots, tackling, handling, reflexes,
                courage, concentration, decision_making, leadership, aggression, anticipation, determination, flair, influence,
                awareness, marking, positioning, work_rate, off_the_ball, teamwork,
                finishing, penalties, set_pieces,
                consistency, dirtiness, versatility,
                overall_rating, morale, form, fitness, is_real_player
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?,
                ?, ?, ?,
                ?, ?, ?,
                ?, ?, ?, ?, ?
            )
            "#
        )
        .bind(&player.id)
        .bind(&player.name)
        .bind(if player.club_id.is_empty() { None } else { Some(player.club_id.clone()) })
        .bind(if player.position.is_empty() { None } else { Some(player.position.clone()) })
        .bind(if player.birth_year == 0 { None } else { Some(player.birth_year) })
        .bind(&player.nationality)
        .bind(player.height_cm)
        .bind(player.pace)
        .bind(player.strength)
        .bind(player.stamina)
        .bind(player.balance)
        .bind(player.jumping)
        .bind(player.agility)
        .bind(player.passing)
        .bind(player.dribbling)
        .bind(player.first_touch)
        .bind(player.heading)
        .bind(player.long_passing)
        .bind(player.crossing)
        .bind(player.long_shots)
        .bind(player.tackling)
        .bind(player.handling)
        .bind(player.reflexes)
        .bind(player.courage)
        .bind(player.concentration)
        .bind(player.decision_making)
        .bind(player.leadership)
        .bind(player.aggression)
        .bind(player.anticipation)
        .bind(player.determination)
        .bind(player.flair)
        .bind(player.influence)
        .bind(player.awareness)
        .bind(player.marking)
        .bind(player.positioning)
        .bind(player.work_rate)
        .bind(player.off_the_ball)
        .bind(player.teamwork)
        .bind(player.finishing)
        .bind(player.penalties)
        .bind(player.set_pieces)
        .bind(player.consistency)
        .bind(player.dirtiness)
        .bind(player.versatility)
        .bind(player.overall_rating)
        .bind(player.morale)
        .bind(player.form)
        .bind(player.fitness)
        .bind(player.is_real_player as i32)
        .execute(&pool)
        .await?;

        generated_players.push(player);
    }

    Ok(generated_players)
}

pub async fn bulk_add_players_with_ages(
    club_id: &str,
    player_inputs: Vec<crate::commands::PlayerInput>,
    year: i32,
    has_goalkeeper: bool,
) -> Result<Vec<player_generator::GeneratedPlayer>, Box<dyn std::error::Error>> {
    let pool = get_pool_write().await?;

    let mut generated_players = Vec::new();

    for input in player_inputs {
        let player_input = player_generator::PlayerInput {
            name: input.name,
            position: None, // Auto-assign position
            birth_year: input.birth_year, // Use provided birth_year or None for random
        };

        let player = player_generator::generate_player(&player_input, club_id, year, has_goalkeeper);

        // Insert into database
        sqlx::query(
            r#"
            INSERT INTO sheffield_footballers (
                id, name, club_id, position, birth_year, nationality, height_cm,
                pace, strength, stamina, balance, jumping, agility,
                passing, dribbling, first_touch, heading, long_passing, crossing, long_shots, tackling, handling, reflexes,
                courage, concentration, decision_making, leadership, aggression, anticipation, determination, flair, influence,
                awareness, marking, positioning, work_rate, off_the_ball, teamwork,
                finishing, penalties, set_pieces,
                consistency, dirtiness, versatility,
                overall_rating, morale, form, fitness, is_real_player
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?, ?, ?, ?,
                ?, ?, ?, ?, ?, ?,
                ?, ?, ?,
                ?, ?, ?,
                ?, ?, ?, ?, ?
            )
            "#
        )
        .bind(&player.id)
        .bind(&player.name)
        .bind(if player.club_id.is_empty() { None } else { Some(player.club_id.clone()) })
        .bind(if player.position.is_empty() { None } else { Some(player.position.clone()) })
        .bind(if player.birth_year == 0 { None } else { Some(player.birth_year) })
        .bind(&player.nationality)
        .bind(player.height_cm)
        .bind(player.pace)
        .bind(player.strength)
        .bind(player.stamina)
        .bind(player.balance)
        .bind(player.jumping)
        .bind(player.agility)
        .bind(player.passing)
        .bind(player.dribbling)
        .bind(player.first_touch)
        .bind(player.heading)
        .bind(player.long_passing)
        .bind(player.crossing)
        .bind(player.long_shots)
        .bind(player.tackling)
        .bind(player.handling)
        .bind(player.reflexes)
        .bind(player.courage)
        .bind(player.concentration)
        .bind(player.decision_making)
        .bind(player.leadership)
        .bind(player.aggression)
        .bind(player.anticipation)
        .bind(player.determination)
        .bind(player.flair)
        .bind(player.influence)
        .bind(player.awareness)
        .bind(player.marking)
        .bind(player.positioning)
        .bind(player.work_rate)
        .bind(player.off_the_ball)
        .bind(player.teamwork)
        .bind(player.finishing)
        .bind(player.penalties)
        .bind(player.set_pieces)
        .bind(player.consistency)
        .bind(player.dirtiness)
        .bind(player.versatility)
        .bind(player.overall_rating)
        .bind(player.morale)
        .bind(player.form)
        .bind(player.fitness)
        .bind(player.is_real_player as i32)
        .execute(&pool)
        .await?;

        generated_players.push(player);
    }

    Ok(generated_players)
}


pub async fn update_club_location(
    club_id: &str,
    city: Option<String>,
    region: Option<String>,
    origin: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let pool = get_pool_write().await?;

    sqlx::query(
        "UPDATE sheffield_clubs SET city = ?, region = ?, origin = ? WHERE id = ?"
    )
    .bind(city)
    .bind(region)
    .bind(origin)
    .bind(club_id)
    .execute(&pool)
    .await?;

    Ok(())
}

pub async fn update_club_name(
    club_id: &str,
    name: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let pool = get_pool_write().await?;

    sqlx::query(
        "UPDATE sheffield_clubs SET name = ? WHERE id = ?"
    )
    .bind(name)
    .bind(club_id)
    .execute(&pool)
    .await?;

    Ok(())
}

pub async fn copy_parent_data_to_reserve(
    parent_club_id: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let pool = get_pool_write().await?;

    // Find the reserve team for this parent club
    let reserve_club_id: Option<String> = sqlx::query_scalar(
        "SELECT club_id FROM sheffield_league_clubs
         WHERE reserve_of_club_id = ? AND is_reserve_team = 1
         LIMIT 1"
    )
    .bind(parent_club_id)
    .fetch_optional(&pool)
    .await?;

    if let Some(reserve_id) = reserve_club_id {
        // Get parent club data
        let parent_data: (i32, String, Option<String>, Option<String>, Option<String>) = sqlx::query_as(
            "SELECT founded_year, ground_name, city, region, origin
             FROM sheffield_clubs WHERE id = ?"
        )
        .bind(parent_club_id)
        .fetch_one(&pool)
        .await?;

        // Update reserve team with parent's data
        sqlx::query(
            "UPDATE sheffield_clubs
             SET founded_year = ?,
                 ground_name = ?,
                 city = ?,
                 region = ?,
                 origin = ?
             WHERE id = ?"
        )
        .bind(parent_data.0)
        .bind(parent_data.1)
        .bind(parent_data.2)
        .bind(parent_data.3)
        .bind(parent_data.4)
        .bind(&reserve_id)
        .execute(&pool)
        .await?;

        Ok(format!("Copied data to reserve team: {}", reserve_id))
    } else {
        Ok("No reserve team found for this club".to_string())
    }
}

pub async fn create_club(
    club_id: &str,
    club_name: &str,
    founded_year: i32,
    ground_name: &str,
    city: &str,
    region: &str,
    origin: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let pool = get_pool_write().await?;

    // Check if club already exists
    let existing: Option<String> = sqlx::query_scalar(
        "SELECT id FROM sheffield_clubs WHERE id = ?"
    )
    .bind(club_id)
    .fetch_optional(&pool)
    .await?;

    if existing.is_some() {
        return Err(format!("Club with ID '{}' already exists", club_id).into());
    }

    // Insert the new club
    sqlx::query(
        "INSERT INTO sheffield_clubs (id, name, founded_year, ground_name, city, region, origin)
         VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(club_id)
    .bind(club_name)
    .bind(founded_year)
    .bind(ground_name)
    .bind(city)
    .bind(region)
    .bind(origin)
    .execute(&pool)
    .await?;

    Ok(format!("Successfully created club: {}", club_name))
}

pub async fn get_unassigned_clubs() -> Result<Vec<(String, String, i32)>, Box<dyn std::error::Error>> {
    let pool = get_pool().await?;

    // Get all clubs that are NOT in sheffield_league_clubs
    let clubs: Vec<(String, String, i32)> = sqlx::query_as(
        "SELECT sc.id, sc.name, sc.founded_year
         FROM sheffield_clubs sc
         WHERE sc.id NOT IN (SELECT club_id FROM sheffield_league_clubs)
         ORDER BY sc.name"
    )
    .fetch_all(&pool)
    .await?;

    Ok(clubs)
}

pub async fn update_player_stats(
    player_id: &str,
    stats: &crate::commands::PlayerStats,
) -> Result<String, Box<dyn std::error::Error>> {
    let pool = get_pool_write().await?;

    println!("DEBUG: Attempting to update stats for player_id: {}", player_id);
    println!("DEBUG: pace = {}, current_ability = {}", stats.pace, stats.current_ability);

    let result = sqlx::query(
        "UPDATE sheffield_footballers
         SET position = ?, pace = ?, acceleration = ?, strength = ?, stamina = ?, balance = ?, jumping = ?, agility = ?, natural_fitness = ?,
             passing = ?, dribbling = ?, first_touch = ?, technique = ?, heading = ?, long_passing = ?, crossing = ?, long_shots = ?,
             tackling = ?, handling = ?, reflexes = ?, corners = ?, free_kicks = ?, throw_ins = ?, vision = ?,
             left_foot = ?, right_foot = ?, one_on_ones = ?,
             courage = ?, bravery = ?, concentration = ?, decision_making = ?, leadership = ?,
             aggression = ?, anticipation = ?, determination = ?, flair = ?, influence = ?,
             adaptability = ?, ambition = ?, loyalty = ?, pressure = ?, professionalism = ?,
             sportsmanship = ?, temperament = ?,
             awareness = ?, marking = ?, positioning = ?, work_rate = ?, off_the_ball = ?, movement = ?, teamwork = ?,
             finishing = ?, penalties = ?, set_pieces = ?,
             consistency = ?, dirtiness = ?, versatility = ?, injury_proneness = ?, important_matches = ?,
             current_ability = ?, potential_ability = ?, current_reputation = ?
         WHERE person_id = ?"
    )
    .bind(&stats.position)
    .bind(stats.pace).bind(stats.acceleration).bind(stats.strength).bind(stats.stamina).bind(stats.balance).bind(stats.jumping).bind(stats.agility).bind(stats.natural_fitness)
    .bind(stats.passing).bind(stats.dribbling).bind(stats.first_touch).bind(stats.technique).bind(stats.heading).bind(stats.long_passing).bind(stats.crossing).bind(stats.long_shots)
    .bind(stats.tackling).bind(stats.handling).bind(stats.reflexes).bind(stats.corners).bind(stats.free_kicks).bind(stats.throw_ins).bind(stats.vision)
    .bind(stats.left_foot).bind(stats.right_foot).bind(stats.one_on_ones)
    .bind(stats.courage).bind(stats.bravery).bind(stats.concentration).bind(stats.decision_making).bind(stats.leadership)
    .bind(stats.aggression).bind(stats.anticipation).bind(stats.determination).bind(stats.flair).bind(stats.influence)
    .bind(stats.adaptability).bind(stats.ambition).bind(stats.loyalty).bind(stats.pressure).bind(stats.professionalism)
    .bind(stats.sportsmanship).bind(stats.temperament)
    .bind(stats.awareness).bind(stats.marking).bind(stats.positioning).bind(stats.work_rate).bind(stats.off_the_ball).bind(stats.movement).bind(stats.teamwork)
    .bind(stats.finishing).bind(stats.penalties).bind(stats.set_pieces)
    .bind(stats.consistency).bind(stats.dirtiness).bind(stats.versatility).bind(stats.injury_proneness).bind(stats.important_matches)
    .bind(stats.current_ability).bind(stats.potential_ability).bind(stats.current_reputation)
    .bind(player_id)
    .execute(&pool)
    .await?;

    let rows_affected = result.rows_affected();
    println!("DEBUG: Rows affected: {}", rows_affected);

    if rows_affected == 0 {
        println!("WARNING: No rows updated for player_id: {}", player_id);
    }

    Ok(format!("Successfully updated player stats for {} (rows affected: {})", player_id, rows_affected))
}

pub async fn transfer_player(
    player_id: &str,
    new_club_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let pool = get_pool_write().await?;

    println!("DEBUG: Transferring player {} to club {}", player_id, new_club_id);

    let result = sqlx::query(
        "UPDATE sheffield_footballers
         SET club_id = ?
         WHERE person_id = ?"
    )
    .bind(new_club_id)
    .bind(player_id)
    .execute(&pool)
    .await?;

    let rows_affected = result.rows_affected();
    println!("DEBUG: Transfer complete, rows affected: {}", rows_affected);

    if rows_affected == 0 {
        return Err(format!("No player found with person_id: {}", player_id).into());
    }

    Ok(())
}

// Cup Competitions Wrappers
pub async fn create_annual_cups(season: i64, start_week: i64) -> Result<Vec<Competition>, Box<dyn std::error::Error>> {
    let pool = get_pool_write().await?;
    cup_competitions::create_annual_cups(&pool, season, start_week).await
}

pub async fn create_custom_cup(
    name: String,
    season: i64,
    min_division_level: Option<i64>,
    max_division_level: Option<i64>,
    start_week: i64,
    announcement_week: i64,
    draw_week: i64,
    prestige_level: String,
    rules_type: String,
) -> Result<Competition, Box<dyn std::error::Error>> {
    let pool = get_pool_write().await?;
    cup_competitions::create_custom_cup(
        &pool,
        name,
        season,
        min_division_level,
        max_division_level,
        start_week,
        announcement_week,
        draw_week,
        prestige_level,
        rules_type,
    ).await
}

pub async fn update_cup_competition(
    competition_id: String,
    name: String,
    min_division_level: Option<i64>,
    max_division_level: Option<i64>,
    start_week: i64,
    announcement_week: i64,
    draw_week: i64,
    prestige_level: String,
    rules_type: String,
) -> Result<Competition, Box<dyn std::error::Error>> {
    let pool = get_pool_write().await?;
    cup_competitions::update_cup_competition(
        &pool,
        competition_id,
        name,
        min_division_level,
        max_division_level,
        start_week,
        announcement_week,
        draw_week,
        prestige_level,
        rules_type,
    ).await
}

pub async fn get_eligible_clubs(competition_id: &str) -> Result<Vec<EligibleClub>, Box<dyn std::error::Error>> {
    let pool = get_pool().await?;
    cup_competitions::get_eligible_clubs(&pool, competition_id).await
}

pub async fn generate_cup_draw(competition_id: &str, seeded: bool) -> Result<(), Box<dyn std::error::Error>> {
    let pool = get_pool_write().await?;
    cup_competitions::generate_cup_draw(&pool, competition_id, seeded).await
}

pub async fn get_cup_bracket(competition_id: &str) -> Result<Vec<CupTie>, Box<dyn std::error::Error>> {
    let pool = get_pool().await?;
    cup_competitions::get_cup_bracket(&pool, competition_id).await
}

pub async fn get_competitions_for_season(season: i64) -> Result<Vec<Competition>, Box<dyn std::error::Error>> {
    let pool = get_pool().await?;
    cup_competitions::get_competitions_for_season(&pool, season)
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

pub async fn delete_competitions_for_season(season: i64) -> Result<(), Box<dyn std::error::Error>> {
    let pool = get_pool_write().await?;
    cup_competitions::delete_competitions_for_season(&pool, season).await
}

pub async fn initialize_season_cup_competitions(year: i32, start_date: &str) -> Result<Vec<crate::game::GameEvent>, Box<dyn std::error::Error>> {
    let pool = get_pool_write().await?;
    cup_competitions::initialize_season_cup_competitions(&pool, year, start_date).await
}

pub async fn create_cup_match_fixtures(season: i64, start_date: &str) -> Result<Vec<crate::game::Match>, Box<dyn std::error::Error>> {
    let pool = get_pool_write().await?;
    cup_competitions::create_cup_match_fixtures(&pool, season, start_date).await
}

pub use cup_competitions::{update_cup_tie_result, process_cup_match_results};

// League configuration wrappers
pub async fn get_league_config() -> Result<LeagueConfig, Box<dyn std::error::Error>> {
    let pool = get_pool().await?;
    league_config::get_league_config(&pool).await
}

pub async fn save_league_config(config: LeagueConfig) -> Result<(), Box<dyn std::error::Error>> {
    let pool = get_pool_write().await?;
    league_config::save_league_config(&pool, &config).await
}

pub async fn create_league_config_table() -> Result<(), Box<dyn std::error::Error>> {
    let pool = get_pool_write().await?;
    league_config::create_league_config_table(&pool).await
}

// League metadata wrappers
pub async fn create_league_metadata_table() -> Result<(), Box<dyn std::error::Error>> {
    let pool = get_pool_write().await?;
    league_metadata::create_league_metadata_table(&pool).await
}

pub async fn get_all_leagues() -> Result<Vec<LeagueMetadata>, Box<dyn std::error::Error>> {
    let pool = get_pool().await?;
    league_metadata::get_all_leagues(&pool).await
}

pub async fn get_active_league() -> Result<Option<LeagueMetadata>, Box<dyn std::error::Error>> {
    let pool = get_pool().await?;
    league_metadata::get_active_league(&pool).await
}

pub async fn create_league(
    name: String,
    description: String,
    season_year: i64,
) -> Result<LeagueMetadata, Box<dyn std::error::Error>> {
    let pool = get_pool_write().await?;
    league_metadata::create_league(&pool, name, description, season_year).await
}

pub async fn set_active_league(league_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let pool = get_pool_write().await?;
    league_metadata::set_active_league(&pool, league_id).await
}

pub async fn delete_league(league_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let pool = get_pool_write().await?;
    league_metadata::delete_league(&pool, league_id).await
}

pub async fn initialize_default_league() -> Result<(), Box<dyn std::error::Error>> {
    let pool = get_pool_write().await?;
    league_metadata::initialize_default_league(&pool).await
}
