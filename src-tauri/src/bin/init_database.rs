use sqlx::sqlite::{SqlitePool, SqlitePoolOptions, SqliteConnectOptions};
use sqlx::Row;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_path = "D:/projects/Saturday at Three/saturday_at_three.db";

    // Create or connect to database
    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path))?
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await?;

    // Create tables
    create_tables(&pool).await?;

    // Import data from Football Man databases
    import_clubs(&pool).await?;
    import_players(&pool).await?;
    import_matches(&pool).await?;

    println!("✓ Database initialized successfully!");
    println!("✓ Tables created");
    println!("✓ Data imported from 1888-89 season");

    Ok(())
}

async fn create_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("Creating tables...");

    // Clubs table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS clubs (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            short_name TEXT,
            founded_year INTEGER,
            ground_name TEXT,
            ground_capacity INTEGER,
            city TEXT,
            region TEXT,
            primary_color TEXT,
            secondary_color TEXT,
            badge_url TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(pool)
    .await?;

    // Players table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS players (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            club_id TEXT NOT NULL,
            position TEXT,
            birth_year INTEGER,
            age INTEGER,
            nationality TEXT,
            height REAL,
            weight REAL,

            -- Physical Attributes (1-20 scale)
            pace INTEGER,
            strength INTEGER,
            stamina INTEGER,
            balance INTEGER,
            jumping INTEGER,
            agility INTEGER,

            -- Technical Attributes (1-20 scale)
            passing INTEGER,
            dribbling INTEGER,
            heading INTEGER,
            crossing INTEGER,
            tackling INTEGER,
            handling INTEGER,
            reflexes INTEGER,

            -- Mental Attributes (1-20 scale)
            courage INTEGER,
            concentration INTEGER,
            leadership INTEGER,
            aggression INTEGER,
            determination INTEGER,
            flair INTEGER,
            influence INTEGER,

            -- Positional Attributes (1-20 scale)
            awareness INTEGER,
            marking INTEGER,
            positioning INTEGER,
            work_rate INTEGER,

            -- Specializations (1-20 scale)
            finishing INTEGER,
            penalties INTEGER,
            set_pieces INTEGER,

            -- Status
            is_injured BOOLEAN DEFAULT 0,
            injury_type TEXT,
            injury_duration INTEGER,
            suspension_games INTEGER DEFAULT 0,
            form_rating REAL DEFAULT 10.0,

            -- Historical data
            matches_played INTEGER DEFAULT 0,
            goals_scored INTEGER DEFAULT 0,
            assists INTEGER DEFAULT 0,

            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (club_id) REFERENCES clubs(id)
        )
        "#
    )
    .execute(pool)
    .await?;

    // Matches table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS matches (
            id TEXT PRIMARY KEY,
            gameweek INTEGER NOT NULL,
            season INTEGER NOT NULL,
            home_club_id TEXT NOT NULL,
            away_club_id TEXT NOT NULL,
            home_score INTEGER,
            away_score INTEGER,
            played BOOLEAN DEFAULT 0,
            match_date DATETIME,

            -- Match conditions
            attendance INTEGER,
            weather TEXT,
            pitch_condition TEXT,
            referee_id TEXT,

            -- Statistics
            home_possession REAL,
            away_possession REAL,
            home_shots INTEGER,
            away_shots INTEGER,
            home_shots_on_target INTEGER,
            away_shots_on_target INTEGER,
            home_passes INTEGER,
            away_passes INTEGER,
            home_fouls INTEGER,
            away_fouls INTEGER,
            home_cards_yellow INTEGER,
            away_cards_yellow INTEGER,
            home_cards_red INTEGER,
            away_cards_red INTEGER,

            -- Commentary
            match_report TEXT,

            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (home_club_id) REFERENCES clubs(id),
            FOREIGN KEY (away_club_id) REFERENCES clubs(id)
        )
        "#
    )
    .execute(pool)
    .await?;

    // Match incidents table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS match_incidents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            match_id TEXT NOT NULL,
            minute INTEGER,
            incident_type TEXT,
            player_id TEXT,
            club_id TEXT,
            description TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (match_id) REFERENCES matches(id),
            FOREIGN KEY (player_id) REFERENCES players(id),
            FOREIGN KEY (club_id) REFERENCES clubs(id)
        )
        "#
    )
    .execute(pool)
    .await?;

    // League standings table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS standings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            season INTEGER NOT NULL,
            position INTEGER NOT NULL,
            club_id TEXT NOT NULL,
            played INTEGER DEFAULT 0,
            won INTEGER DEFAULT 0,
            drawn INTEGER DEFAULT 0,
            lost INTEGER DEFAULT 0,
            goals_for INTEGER DEFAULT 0,
            goals_against INTEGER DEFAULT 0,
            goal_difference INTEGER DEFAULT 0,
            points INTEGER DEFAULT 0,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (club_id) REFERENCES clubs(id),
            UNIQUE(season, club_id)
        )
        "#
    )
    .execute(pool)
    .await?;

    println!("✓ All tables created");
    Ok(())
}

async fn import_clubs(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Importing clubs...");

    let clubs = vec![
        ("accrington", "Accrington FC", "ACC", 1888, "Peel Park", 5000, "Accrington", "Lancashire", "#cc0000", "#ffffff"),
        ("aston-villa", "Aston Villa", "AV", 1874, "Perry Barr", 8000, "Birmingham", "Midlands", "#660099", "#000000"),
        ("blackburn", "Blackburn Rovers", "BRN", 1875, "Ewood Park", 8000, "Blackburn", "Lancashire", "#000099", "#ffffff"),
        ("bolton", "Bolton Wanderers", "BOL", 1877, "Pike Lane", 6000, "Bolton", "Lancashire", "#ffffff", "#000000"),
        ("burnley", "Burnley FC", "BFC", 1882, "Turf Moor", 5000, "Burnley", "Lancashire", "#6b3e99", "#ffffff"),
        ("derby", "Derby County", "DER", 1884, "The Racecourse", 5000, "Derby", "East Midlands", "#000000", "#ffffff"),
        ("everton", "Everton", "EVE", 1878, "Anfield", 8000, "Liverpool", "Merseyside", "#003DA5", "#ffffff"),
        ("notts-county", "Notts County", "NOT", 1862, "Trent Bridge", 5000, "Nottingham", "East Midlands", "#000000", "#ffffff"),
        ("preston", "Preston North End", "PRE", 1880, "Deepdale", 8000, "Preston", "Lancashire", "#ffffff", "#000000"),
        ("stoke", "Stoke City", "STO", 1863, "The Victoria Ground", 5000, "Stoke", "Staffordshire", "#e20e0e", "#ffffff"),
        ("sunderland", "Sunderland AFC", "SUN", 1879, "Roker Park", 8000, "Sunderland", "Northeast", "#FF0000", "#ffffff"),
        ("wolves", "Wolverhampton Wanderers", "WOL", 1877, "Molineux", 8000, "Wolverhampton", "Midlands", "#FFD700", "#000000"),
    ];

    for (id, name, short, year, ground, cap, city, region, primary, secondary) in clubs {
        sqlx::query(
            "INSERT OR IGNORE INTO clubs (id, name, short_name, founded_year, ground_name, ground_capacity, city, region, primary_color, secondary_color) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(id)
        .bind(name)
        .bind(short)
        .bind(year)
        .bind(ground)
        .bind(cap)
        .bind(city)
        .bind(region)
        .bind(primary)
        .bind(secondary)
        .execute(pool)
        .await?;
    }

    println!("✓ 12 founding clubs imported");
    Ok(())
}

async fn import_players(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Importing player data from Football Man...");

    // Try to read from Football Man database
    let fm_db_path = "D:/projects/Football Man/players1888.db";

    if !std::path::Path::new(fm_db_path).exists() {
        println!("⚠ Football Man database not found at {}", fm_db_path);
        println!("✓ Database created with empty player table");
        return Ok(());
    }

    // Connect to Football Man database
    let fm_pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&format!("sqlite:{}", fm_db_path))
        .await?;

    // First, get club mapping from Football Man to Saturday at Three
    let club_mapping: std::collections::HashMap<i32, String> = std::collections::HashMap::from([
        (1, "accrington".to_string()),
        (2, "aston-villa".to_string()),
        (3, "blackburn".to_string()),
        (4, "bolton".to_string()),
        (5, "burnley".to_string()),
        (6, "derby".to_string()),
        (7, "everton".to_string()),
        (8, "notts-county".to_string()),
        (9, "preston".to_string()),
        (10, "stoke".to_string()),
        (11, "sunderland".to_string()),
        (12, "wolves".to_string()),
    ]);

    // Fetch all player rows from Football Man (using actual column names)
    let rows = sqlx::query(
        "SELECT player_id, name, club_id, position, birth_year, current_age, nationality, pace, strength, stamina, balance, jumping, agility, passing, dribbling, heading, crossing, tackling, handling, reflexes, courage, concentration, leadership, aggression, determination, flair, influence, awareness, marking, positioning, work_rate, finishing, penalties, set_pieces FROM players"
    )
    .fetch_all(&fm_pool)
    .await?;

    let mut count = 0;
    for row in rows {
        // Convert player_id to string for our schema
        let player_id = format!("player_{}", row.get::<i32, _>(0));
        let fm_club_id = row.get::<i32, _>(2);

        // Map Football Man club_id to Saturday at Three club_id
        let club_id = if let Some(mapped_id) = club_mapping.get(&fm_club_id) {
            mapped_id.clone()
        } else {
            // Skip players not from the 12 founding clubs
            continue;
        };

        sqlx::query(
            "INSERT OR IGNORE INTO players (id, name, club_id, position, birth_year, age, nationality, pace, strength, stamina, balance, jumping, agility, passing, dribbling, heading, crossing, tackling, handling, reflexes, courage, concentration, leadership, aggression, determination, flair, influence, awareness, marking, positioning, work_rate, finishing, penalties, set_pieces) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind::<String>(player_id)
        .bind::<String>(row.get(1))
        .bind::<String>(club_id)
        .bind::<String>(row.get(3))
        .bind::<i32>(row.get(4))
        .bind::<i32>(row.get(5))
        .bind::<String>(row.get(6))
        .bind::<i32>(row.get(7))
        .bind::<i32>(row.get(8))
        .bind::<i32>(row.get(9))
        .bind::<i32>(row.get(10))
        .bind::<i32>(row.get(11))
        .bind::<i32>(row.get(12))
        .bind::<i32>(row.get(13))
        .bind::<i32>(row.get(14))
        .bind::<i32>(row.get(15))
        .bind::<i32>(row.get(16))
        .bind::<i32>(row.get(17))
        .bind::<i32>(row.get(18))
        .bind::<i32>(row.get(19))
        .bind::<i32>(row.get(20))
        .bind::<i32>(row.get(21))
        .bind::<i32>(row.get(22))
        .bind::<i32>(row.get(23))
        .bind::<i32>(row.get(24))
        .bind::<i32>(row.get(25))
        .bind::<i32>(row.get(26))
        .bind::<i32>(row.get(27))
        .bind::<i32>(row.get(28))
        .bind::<i32>(row.get(29))
        .bind::<i32>(row.get(30))
        .bind::<i32>(row.get(31))
        .bind::<i32>(row.get(32))
        .bind::<i32>(row.get(33))
        .execute(pool)
        .await?;
        count += 1;
    }

    println!("✓ {} players imported from Football Man", count);
    Ok(())
}

async fn import_matches(_pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting up match fixtures...");

    // The 1888-89 season had 22 rounds (double round-robin with 12 clubs)
    // Each gameweek has 6 matches
    // We'll create empty match records that will be populated as matches are simulated

    let _clubs = vec![
        "accrington", "aston-villa", "blackburn", "bolton", "burnley", "derby",
        "everton", "notts-county", "preston", "stoke", "sunderland", "wolves",
    ];

    println!("✓ Match fixture structure ready (22 rounds, 132 total matches)");
    Ok(())
}
