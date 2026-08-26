-- Sheffield Rules Database Schema (1858-1877)

-- Clubs table: Historical Sheffield clubs with founding information
CREATE TABLE IF NOT EXISTS sheffield_clubs (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    founded_year INTEGER NOT NULL,
    disbanded_year INTEGER,
    ground_name TEXT,
    origin TEXT,  -- Church, Works, Hotel, School, Inn, etc.
    city TEXT,
    region TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Players table: Comprehensive player attributes with Sheffield-specific fields
CREATE TABLE IF NOT EXISTS sheffield_players (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    first_name TEXT,
    middle_name TEXT,
    surname TEXT,
    club_id TEXT,
    position TEXT,
    birth_year INTEGER,
    death_year INTEGER,
    nationality TEXT,
    height_cm INTEGER,

    -- Geographic/Census Information (for team assignment based on location)
    where_born TEXT,                    -- Full birth location: "Sheffield, Yorkshire, England"
    birth_town TEXT,                    -- Town/City of birth
    birth_county TEXT,                  -- County: "Yorkshire", "Derbyshire", etc.
    birth_country TEXT,                 -- Country: "England", "Scotland", "Wales", "Ireland"
    civil_parish TEXT,                  -- Civil parish from census
    ecclesiastical_parish TEXT,         -- Church parish: "St Philip", "St George", etc.
    registration_district TEXT,         -- Registration district: "Ecclesall Bierlow", etc.
    sub_registration_district TEXT,     -- Sub-registration district: "Nether Hallam", etc.
    census_age INTEGER,                 -- Age as recorded in census
    census_relation TEXT,               -- Relation to head: "Head", "Wife", "Son", etc.
    census_gender TEXT,                 -- Gender as recorded: "Male", "Female"
    census_ed TEXT,                     -- ED, institution, or vessel number
    census_household_schedule TEXT,     -- Household schedule number
    census_household_members TEXT,      -- Household members text from census
    census_piece TEXT,                  -- Piece reference number
    census_folio TEXT,                  -- Folio number
    census_page TEXT,                   -- Page number
    street_address TEXT,                -- Street address from genealogy data
    profession TEXT,                    -- Profession/occupation from genealogy data

    -- Physical Attributes (1-20)
    pace INTEGER,
    acceleration INTEGER,
    strength INTEGER,
    stamina INTEGER,
    balance INTEGER,
    jumping INTEGER,
    agility INTEGER,
    natural_fitness INTEGER,

    -- Technical Attributes (1-20)
    passing INTEGER,
    dribbling INTEGER,
    first_touch INTEGER,
    technique INTEGER,
    heading INTEGER,
    long_passing INTEGER,
    crossing INTEGER,
    long_shots INTEGER,
    tackling INTEGER,
    handling INTEGER,
    reflexes INTEGER,
    corners INTEGER,
    free_kicks INTEGER,
    throw_ins INTEGER,
    vision INTEGER,
    left_foot INTEGER,
    right_foot INTEGER,
    one_on_ones INTEGER,

    -- Mental Attributes (1-20)
    courage INTEGER,
    bravery INTEGER,
    concentration INTEGER,
    decision_making INTEGER,
    leadership INTEGER,
    aggression INTEGER,
    anticipation INTEGER,
    determination INTEGER,
    flair INTEGER,
    influence INTEGER,
    adaptability INTEGER,
    ambition INTEGER,
    loyalty INTEGER,
    pressure INTEGER,
    professionalism INTEGER,
    sportsmanship INTEGER,
    temperament INTEGER,

    -- Positioning Attributes (1-20)
    awareness INTEGER,
    marking INTEGER,
    positioning INTEGER,
    work_rate INTEGER,
    off_the_ball INTEGER,
    movement INTEGER,
    teamwork INTEGER,

    -- Specialization Attributes (1-20)
    finishing INTEGER,
    penalties INTEGER,
    set_pieces INTEGER,

    -- Hidden Attributes (1-20)
    consistency INTEGER,
    dirtiness INTEGER,
    versatility INTEGER,
    injury_proneness INTEGER,
    important_matches INTEGER,

    -- Calculated
    overall_rating INTEGER DEFAULT 0,

    -- Status
    morale INTEGER DEFAULT 10,
    form INTEGER DEFAULT 0,
    fitness INTEGER DEFAULT 100,
    is_injured BOOLEAN DEFAULT 0,
    injury_type TEXT,
    injury_weeks_remaining INTEGER DEFAULT 0,

    -- Contract
    contract_start_date TEXT,
    contract_end_date TEXT,
    weekly_wage INTEGER DEFAULT 0,

    -- Development
    personality_type TEXT,
    development_stage TEXT,
    potential_ceiling INTEGER,

    -- Ability & Reputation (Football Manager style, 1-200 scale)
    current_ability INTEGER DEFAULT 50,      -- Current skill level (1-200)
    potential_ability INTEGER DEFAULT 100,   -- Maximum potential (1-200)
    current_reputation INTEGER DEFAULT 10,   -- How well-known the player is (1-200)

    -- Career Stats
    career_appearances INTEGER DEFAULT 0,
    career_goals INTEGER DEFAULT 0,
    career_assists INTEGER DEFAULT 0,
    yellow_cards INTEGER DEFAULT 0,
    red_cards INTEGER DEFAULT 0,

    -- Sheffield Rules specific
    rouges_scored INTEGER DEFAULT 0,  -- For 1862-1868 rouge tracking
    is_real_player BOOLEAN DEFAULT 0,
    has_stats BOOLEAN DEFAULT 0,  -- Track whether stats have been assigned (for Database Editor)

    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (club_id) REFERENCES sheffield_clubs(id)
);

-- Matches table: Match results with rouge scoring
CREATE TABLE IF NOT EXISTS sheffield_matches (
    id TEXT PRIMARY KEY,
    gameweek INTEGER NOT NULL,
    season INTEGER NOT NULL,
    rule_year INTEGER NOT NULL,  -- May differ from season in ahistorical modes
    home_club_id TEXT NOT NULL,
    away_club_id TEXT NOT NULL,
    home_score INTEGER DEFAULT 0,
    away_score INTEGER DEFAULT 0,
    home_rouges INTEGER DEFAULT 0,  -- 1862-1868 only
    away_rouges INTEGER DEFAULT 0,  -- 1862-1868 only
    played BOOLEAN DEFAULT 0,
    match_date TEXT,

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
    FOREIGN KEY (home_club_id) REFERENCES sheffield_clubs(id),
    FOREIGN KEY (away_club_id) REFERENCES sheffield_clubs(id)
);

-- Match incidents table: Goals, rouges, cards, injuries, etc.
CREATE TABLE IF NOT EXISTS sheffield_match_incidents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    match_id TEXT NOT NULL,
    minute INTEGER,
    incident_type TEXT,  -- goal, rouge, yellow_card, red_card, injury, etc.
    player_id TEXT,
    club_id TEXT,
    description TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (match_id) REFERENCES sheffield_matches(id),
    FOREIGN KEY (player_id) REFERENCES sheffield_players(id),
    FOREIGN KEY (club_id) REFERENCES sheffield_clubs(id)
);

-- League standings table: Points, goals, rouges tracked
CREATE TABLE IF NOT EXISTS sheffield_standings (
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
    rouges_for INTEGER DEFAULT 0,  -- 1862-1868 only
    rouges_against INTEGER DEFAULT 0,  -- 1862-1868 only
    points INTEGER DEFAULT 0,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (club_id) REFERENCES sheffield_clubs(id),
    UNIQUE(season, club_id)
);

-- Game state table: Track current game configuration
CREATE TABLE IF NOT EXISTS sheffield_game_state (
    id TEXT PRIMARY KEY,
    game_mode TEXT NOT NULL,  -- historical_timeline, ahistorical_1862, ahistorical_1868, ahistorical_1875, historical_from_year
    rule_year INTEGER NOT NULL,  -- Locked for ahistorical, evolves for historical
    season_year INTEGER NOT NULL,  -- Actual season being played
    start_year INTEGER NOT NULL,  -- Initial year selected
    user_club_id TEXT NOT NULL,
    current_date TEXT,
    current_gameweek INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_club_id) REFERENCES sheffield_clubs(id)
);

-- Rules history table: Track rule evolution
CREATE TABLE IF NOT EXISTS sheffield_rules_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    season_year INTEGER NOT NULL,
    rule_year INTEGER NOT NULL,
    ruleset_json TEXT,  -- JSON blob of active ruleset
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(season_year)
);

-- Fixtures table: Pre-generated fixture list
CREATE TABLE IF NOT EXISTS sheffield_fixtures (
    id TEXT PRIMARY KEY,
    gameweek INTEGER NOT NULL,
    season INTEGER NOT NULL,
    home_club_id TEXT NOT NULL,
    away_club_id TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (home_club_id) REFERENCES sheffield_clubs(id),
    FOREIGN KEY (away_club_id) REFERENCES sheffield_clubs(id),
    UNIQUE(season, gameweek, home_club_id, away_club_id)
);

-- Sheffield & Hallamshire League (1867 Fantasy Mode) Tables
-- These tables support the fantasy league with all 186 historical clubs

-- League divisions: Structure of the fantasy league pyramid
CREATE TABLE IF NOT EXISTS sheffield_league_divisions (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,                    -- e.g., "First Division", "6C North"
    level INTEGER NOT NULL,                -- 1-10 (1=top, 10=bottom)
    region TEXT,                           -- Geographic region for lower divisions: North, East, West, South
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- League club assignments: Maps clubs to their divisions
CREATE TABLE IF NOT EXISTS sheffield_league_clubs (
    id TEXT PRIMARY KEY,
    division_id TEXT NOT NULL,
    club_id TEXT NOT NULL,
    position_in_division INTEGER,          -- 1-16 (starting position)
    is_reserve_team BOOLEAN DEFAULT 0,     -- 1 if this is a reserve/junior team
    reserve_of_club_id TEXT,               -- If reserve, which main club does it belong to
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (division_id) REFERENCES sheffield_league_divisions(id),
    FOREIGN KEY (club_id) REFERENCES sheffield_clubs(id),
    FOREIGN KEY (reserve_of_club_id) REFERENCES sheffield_clubs(id),
    UNIQUE(division_id, club_id)
);

-- League promotion/relegation structure: Defines which divisions feed into which
CREATE TABLE IF NOT EXISTS sheffield_league_promotion_rules (
    id TEXT PRIMARY KEY,
    from_division_id TEXT NOT NULL,
    to_division_id TEXT NOT NULL,
    slots INTEGER NOT NULL,                -- How many teams get promoted
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (from_division_id) REFERENCES sheffield_league_divisions(id),
    FOREIGN KEY (to_division_id) REFERENCES sheffield_league_divisions(id),
    UNIQUE(from_division_id, to_division_id)
);

-- League movement history: Tracks promotions and relegations between seasons
CREATE TABLE IF NOT EXISTS sheffield_league_movement_history (
    id TEXT PRIMARY KEY,
    season INTEGER NOT NULL,
    event_type TEXT NOT NULL,               -- 'promotion' or 'relegation'
    club_id TEXT NOT NULL,
    from_division_id TEXT NOT NULL,
    to_division_id TEXT NOT NULL,
    final_position INTEGER,                 -- Position in the division they left
    is_reserve_team BOOLEAN DEFAULT 0,      -- Track if reserve team was moved
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (club_id) REFERENCES sheffield_clubs(id),
    FOREIGN KEY (from_division_id) REFERENCES sheffield_league_divisions(id),
    FOREIGN KEY (to_division_id) REFERENCES sheffield_league_divisions(id)
);

-- Cup Competitions System (1867 Fantasy Mode)
-- Supports Youdan Cup (Divisions 1-4) and Cromwell Cup (Divisions 5-10)

-- Competitions table: Defines cup competitions and their eligibility rules
CREATE TABLE IF NOT EXISTS sheffield_competitions (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,                     -- e.g., "Youdan Cup 1867", "Cromwell Cup 1867"
    competition_type TEXT NOT NULL,         -- 'knockout_cup', 'league' (future)
    season INTEGER NOT NULL,
    min_division_level INTEGER,             -- 1 for Youdan, 5 for Cromwell, NULL for all
    max_division_level INTEGER,             -- 4 for Youdan, 10 for Cromwell, NULL for all
    current_round INTEGER DEFAULT 0,        -- 0=not started, 1=R1, 2=R2, etc.
    total_rounds INTEGER,                   -- Based on number of entrants
    is_active BOOLEAN DEFAULT 1,
    winner_club_id TEXT,
    runner_up_club_id TEXT,
    rules_type TEXT DEFAULT 'sheffield_rules',
    prestige_level TEXT DEFAULT 'standard', -- 'high' for Youdan, 'standard' for Cromwell
    start_week INTEGER,                     -- Week when competition begins
    announcement_week INTEGER,              -- Week when cup is announced (typically 4 weeks before start)
    draw_week INTEGER,                      -- Week when draw is made (typically 2 weeks before start)
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (winner_club_id) REFERENCES sheffield_clubs(id),
    FOREIGN KEY (runner_up_club_id) REFERENCES sheffield_clubs(id)
);

-- Cup ties: Bracket structure for knockout competitions
CREATE TABLE IF NOT EXISTS sheffield_cup_ties (
    id TEXT PRIMARY KEY,
    competition_id TEXT NOT NULL,
    round_number INTEGER NOT NULL,          -- 1, 2, 3, 4, 5, 6, etc.
    round_name TEXT,                        -- "Round 1", "Round 2", "Quarter-Final", "Semi-Final", "Final"
    tie_number INTEGER,                     -- Position in bracket (1, 2, 3, 4, etc.)
    home_club_id TEXT,
    away_club_id TEXT,                      -- NULL if bye
    scheduled_week INTEGER,                 -- Which week this tie is scheduled for
    played BOOLEAN DEFAULT 0,
    home_score INTEGER,
    away_score INTEGER,
    winner_club_id TEXT,
    match_id TEXT,                          -- References actual match record
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (competition_id) REFERENCES sheffield_competitions(id),
    FOREIGN KEY (home_club_id) REFERENCES sheffield_clubs(id),
    FOREIGN KEY (away_club_id) REFERENCES sheffield_clubs(id),
    FOREIGN KEY (winner_club_id) REFERENCES sheffield_clubs(id),
    FOREIGN KEY (match_id) REFERENCES sheffield_matches(id)
);

-- Competition participants: Tracks which clubs entered each competition
CREATE TABLE IF NOT EXISTS sheffield_competition_participants (
    id TEXT PRIMARY KEY,
    competition_id TEXT NOT NULL,
    club_id TEXT NOT NULL,
    seed_position INTEGER,                  -- For seeded draws
    eliminated_in_round INTEGER,            -- Which round they were eliminated (NULL if winner)
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (competition_id) REFERENCES sheffield_competitions(id),
    FOREIGN KEY (club_id) REFERENCES sheffield_clubs(id),
    UNIQUE(competition_id, club_id)
);

-- Competition winners history: Historical record of cup winners
CREATE TABLE IF NOT EXISTS sheffield_competition_winners (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    competition_name TEXT NOT NULL,
    season INTEGER NOT NULL,
    winner_club_id TEXT NOT NULL,
    runner_up_club_id TEXT,
    final_score TEXT,                       -- e.g., "3-1", "2-1 (aet)", "4-3 (pens)"
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (winner_club_id) REFERENCES sheffield_clubs(id),
    FOREIGN KEY (runner_up_club_id) REFERENCES sheffield_clubs(id),
    UNIQUE(competition_name, season)
);

-- ============================================================================
-- MATCH ARCHIVE SYSTEM: Complete historical match data storage
-- ============================================================================

-- Match lineups: Who actually played in each match
CREATE TABLE IF NOT EXISTS sheffield_match_lineups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    match_id TEXT NOT NULL,
    club_id TEXT NOT NULL,
    player_id TEXT NOT NULL,
    position TEXT NOT NULL,          -- GK, CB, FB, MID, FWD, WG
    jersey_number INTEGER,
    started BOOLEAN DEFAULT 1,       -- 1 if starting XI, 0 if substitute
    substituted_off_minute INTEGER,  -- NULL if played full match
    substituted_on_minute INTEGER,   -- NULL if started
    captain BOOLEAN DEFAULT 0,

    -- Match performance stats
    minutes_played INTEGER DEFAULT 0,
    goals_scored INTEGER DEFAULT 0,
    rouges_scored INTEGER DEFAULT 0,
    assists INTEGER DEFAULT 0,
    yellow_cards INTEGER DEFAULT 0,
    red_cards INTEGER DEFAULT 0,
    shots INTEGER DEFAULT 0,
    shots_on_target INTEGER DEFAULT 0,
    passes_completed INTEGER DEFAULT 0,
    passes_attempted INTEGER DEFAULT 0,
    tackles_won INTEGER DEFAULT 0,
    tackles_attempted INTEGER DEFAULT 0,

    -- Rating
    match_rating REAL,               -- 1.0 - 10.0

    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (match_id) REFERENCES sheffield_matches(id),
    FOREIGN KEY (club_id) REFERENCES sheffield_clubs(id),
    FOREIGN KEY (player_id) REFERENCES sheffield_players(id),
    UNIQUE(match_id, player_id)
);

-- Match timeline: Detailed chronological event log
CREATE TABLE IF NOT EXISTS sheffield_match_timeline (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    match_id TEXT NOT NULL,
    minute INTEGER NOT NULL,
    event_type TEXT NOT NULL,        -- kickoff, goal, rouge, foul, card, injury, substitution, half_time, full_time
    club_id TEXT,
    player_id TEXT,
    assist_player_id TEXT,           -- For goals
    description TEXT,                -- Rich description: "Header from corner", "Long-range effort"
    x_coordinate REAL,               -- 0.0-1.0 (left to right)
    y_coordinate REAL,               -- 0.0-1.0 (bottom to top)
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (match_id) REFERENCES sheffield_matches(id),
    FOREIGN KEY (club_id) REFERENCES sheffield_clubs(id),
    FOREIGN KEY (player_id) REFERENCES sheffield_players(id),
    FOREIGN KEY (assist_player_id) REFERENCES sheffield_players(id)
);

-- Match weather tracking: Detailed environmental conditions
CREATE TABLE IF NOT EXISTS sheffield_match_weather (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    match_id TEXT NOT NULL UNIQUE,
    condition TEXT NOT NULL,         -- clear, overcast, light_rain, heavy_rain, snow, fog
    severity INTEGER,                -- 1-10
    temperature_celsius INTEGER,
    wind_speed_mph INTEGER,
    precipitation_mm INTEGER,
    pitch_condition TEXT,            -- firm, soft, muddy, waterlogged, frozen, snow_covered
    visibility TEXT,                 -- excellent, good, moderate, poor
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (match_id) REFERENCES sheffield_matches(id)
);

-- Historical newspaper archive: Match reports and articles
CREATE TABLE IF NOT EXISTS sheffield_newspaper_archive (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    publication_name TEXT NOT NULL,  -- "Sheffield Independent", "The Field", etc.
    edition_date TEXT NOT NULL,      -- ISO 8601 date
    article_type TEXT NOT NULL,      -- match_report, preview, opinion, rule_discussion, club_news
    headline TEXT NOT NULL,
    body TEXT NOT NULL,              -- Full article text
    match_id TEXT,                   -- NULL if not match-related
    clubs_mentioned TEXT,            -- JSON array of club_ids
    players_mentioned TEXT,          -- JSON array of player_ids
    season INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (match_id) REFERENCES sheffield_matches(id)
);

-- Season archives: Complete season summaries
CREATE TABLE IF NOT EXISTS sheffield_season_archives (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    season_year INTEGER NOT NULL UNIQUE,
    rule_year INTEGER NOT NULL,
    total_matches_played INTEGER DEFAULT 0,
    total_goals_scored INTEGER DEFAULT 0,
    total_rouges_scored INTEGER DEFAULT 0,
    highest_attendance INTEGER,
    highest_attendance_match_id TEXT,
    league_champion_club_id TEXT,
    top_scorer_player_id TEXT,
    top_scorer_goals INTEGER,
    most_rouges_player_id TEXT,
    most_rouges_count INTEGER,
    fair_play_club_id TEXT,          -- Fewest cards/fouls
    summary TEXT,                    -- Rich season narrative
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (highest_attendance_match_id) REFERENCES sheffield_matches(id),
    FOREIGN KEY (league_champion_club_id) REFERENCES sheffield_clubs(id),
    FOREIGN KEY (top_scorer_player_id) REFERENCES sheffield_players(id),
    FOREIGN KEY (most_rouges_player_id) REFERENCES sheffield_players(id),
    FOREIGN KEY (fair_play_club_id) REFERENCES sheffield_clubs(id)
);

-- Player season stats: Per-season statistical records
CREATE TABLE IF NOT EXISTS sheffield_player_season_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id TEXT NOT NULL,
    club_id TEXT NOT NULL,
    season_year INTEGER NOT NULL,
    appearances INTEGER DEFAULT 0,
    starts INTEGER DEFAULT 0,
    minutes_played INTEGER DEFAULT 0,
    goals INTEGER DEFAULT 0,
    rouges INTEGER DEFAULT 0,
    assists INTEGER DEFAULT 0,
    yellow_cards INTEGER DEFAULT 0,
    red_cards INTEGER DEFAULT 0,
    average_rating REAL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (player_id) REFERENCES sheffield_players(id),
    FOREIGN KEY (club_id) REFERENCES sheffield_clubs(id),
    UNIQUE(player_id, season_year)
);

-- Club season stats: Per-season club performance
CREATE TABLE IF NOT EXISTS sheffield_club_season_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    club_id TEXT NOT NULL,
    season_year INTEGER NOT NULL,
    division_id TEXT,
    final_position INTEGER,
    played INTEGER DEFAULT 0,
    won INTEGER DEFAULT 0,
    drawn INTEGER DEFAULT 0,
    lost INTEGER DEFAULT 0,
    goals_for INTEGER DEFAULT 0,
    goals_against INTEGER DEFAULT 0,
    goal_difference INTEGER DEFAULT 0,
    rouges_for INTEGER DEFAULT 0,
    rouges_against INTEGER DEFAULT 0,
    points INTEGER DEFAULT 0,
    total_attendance INTEGER DEFAULT 0,
    average_attendance INTEGER DEFAULT 0,
    highest_attendance INTEGER DEFAULT 0,
    cup_wins INTEGER DEFAULT 0,       -- How many cups won this season
    promoted BOOLEAN DEFAULT 0,
    relegated BOOLEAN DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (club_id) REFERENCES sheffield_clubs(id),
    FOREIGN KEY (division_id) REFERENCES sheffield_league_divisions(id),
    UNIQUE(club_id, season_year)
);

-- Co-operative Movement System Tables

-- Co-operative system state
CREATE TABLE IF NOT EXISTS sheffield_cooperative_system (
    id INTEGER PRIMARY KEY CHECK (id = 1),  -- Singleton table
    is_founded BOOLEAN DEFAULT 0,
    founding_date TEXT,
    membership_count INTEGER DEFAULT 0,
    annual_sales INTEGER DEFAULT 0,
    dividend_rate REAL DEFAULT 0.0,
    last_updated TEXT
);

-- Insert default row
INSERT OR IGNORE INTO sheffield_cooperative_system (id, is_founded) VALUES (1, 0);

-- Club memberships in the Co-operative
CREATE TABLE IF NOT EXISTS sheffield_cooperative_memberships (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    club_id TEXT NOT NULL UNIQUE,
    club_name TEXT NOT NULL,
    joined_date TEXT NOT NULL,
    member_number INTEGER NOT NULL UNIQUE,
    share_capital INTEGER DEFAULT 20,  -- In shillings
    total_purchases INTEGER DEFAULT 0,  -- Lifetime purchases in shillings
    dividend_earned INTEGER DEFAULT 0,  -- Total dividend in shillings
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (club_id) REFERENCES sheffield_clubs(id)
);

-- Co-operative transactions (purchases, donations, etc.)
CREATE TABLE IF NOT EXISTS sheffield_cooperative_transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    transaction_id TEXT NOT NULL UNIQUE,
    club_id TEXT NOT NULL,
    transaction_date TEXT NOT NULL,
    transaction_type TEXT NOT NULL,  -- 'KitPurchase', 'EquipmentPurchase', etc.
    item_description TEXT NOT NULL,
    cost_shillings INTEGER NOT NULL,
    dividend_eligible BOOLEAN DEFAULT 1,
    dividend_earned INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (club_id) REFERENCES sheffield_clubs(id)
);

-- Co-operative events (for newspaper integration)
CREATE TABLE IF NOT EXISTS sheffield_cooperative_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_type TEXT NOT NULL,  -- 'Founding', 'NewShopOpening', etc.
    event_date TEXT NOT NULL,
    headline TEXT,
    description TEXT NOT NULL,
    impact TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Match Engine: Visual States for 2D Replay
-- Stores ball and player positions at each minute for visual replay
CREATE TABLE IF NOT EXISTS sheffield_match_visual_states (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    match_id TEXT NOT NULL,
    minute INTEGER NOT NULL,
    ball_x REAL NOT NULL,          -- Ball X position (0.0-1.0 normalized)
    ball_y REAL NOT NULL,          -- Ball Y position (0.0-1.0 normalized)
    player_positions TEXT NOT NULL, -- JSON array of PlayerPosition objects
    possession_team TEXT NOT NULL,  -- 'home' or 'away'
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (match_id) REFERENCES sheffield_matches(id)
);

-- Match Engine: Detailed Event Timeline
-- Stores all match events with positions for commentary and replay
CREATE TABLE IF NOT EXISTS sheffield_match_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    match_id TEXT NOT NULL,
    minute INTEGER NOT NULL,
    second INTEGER DEFAULT 0,       -- Second within minute (0-59)
    event_type TEXT NOT NULL,       -- 'Pass', 'Shot', 'Goal', 'Tackle', 'Foul', etc.
    description TEXT NOT NULL,      -- Human-readable commentary text
    position_x REAL,                -- Event X position (0.0-1.0 normalized)
    position_y REAL,                -- Event Y position (0.0-1.0 normalized)
    players_involved TEXT,          -- JSON array of player IDs/names
    team_side TEXT NOT NULL,        -- 'home' or 'away'
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (match_id) REFERENCES sheffield_matches(id)
);

-- Match Engine: Formation Templates
-- Stores team formations (2-3-5 pyramid, 2-2-6, etc.) for different eras
CREATE TABLE IF NOT EXISTS sheffield_formations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,             -- '2-3-5 Pyramid', '2-2-6 Attacking', etc.
    era_start_year INTEGER,         -- Earliest year this formation was used
    era_end_year INTEGER,           -- Latest year this formation was used
    formation_code TEXT NOT NULL,   -- '2-3-5', '2-2-6', etc.
    positions TEXT NOT NULL,        -- JSON array of position coordinates
    description TEXT,
    is_default BOOLEAN DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for match engine performance
CREATE INDEX IF NOT EXISTS idx_visual_states_match ON sheffield_match_visual_states(match_id, minute);
CREATE INDEX IF NOT EXISTS idx_match_events_match ON sheffield_match_events(match_id, minute, second);
CREATE INDEX IF NOT EXISTS idx_formations_era ON sheffield_formations(era_start_year, era_end_year);

-- ============================================================================
-- CHALLENGE INVITATION SYSTEM: Victorian-era correspondence and match proposals
-- ============================================================================

-- Challenge invitations table: Stores all challenge letters sent and received
CREATE TABLE IF NOT EXISTS sheffield_challenge_invitations (
    id TEXT PRIMARY KEY,
    sender_club_id TEXT NOT NULL,
    recipient_club_id TEXT NOT NULL,

    -- Dates
    sent_date TEXT NOT NULL,              -- When the letter was sent
    response_date TEXT NOT NULL,          -- When response will arrive (sent_date + 3-4 days)
    proposed_match_date TEXT NOT NULL,    -- Proposed date for the match

    -- Match details from the letter
    match_type TEXT NOT NULL,             -- 'friendly', 'practice', 'challenge'
    venue TEXT NOT NULL,                  -- 'home', 'away', 'neutral'
    stakes TEXT NOT NULL,                 -- 'honor', 'small', 'medium', 'trophy', 'silver-cup', 'dinner', 'charity'
    tone TEXT NOT NULL,                   -- 'cordial', 'formal', 'respectful', 'confident', 'humble', 'competitive', 'bold'
    rules_type TEXT NOT NULL,             -- 'sheffield', 'fa', 'rugby'
    match_duration INTEGER NOT NULL,      -- 60, 80, 90, 120 (in minutes)

    -- Status tracking
    status TEXT NOT NULL DEFAULT 'sent',  -- 'sent', 'delivered', 'accepted', 'declined', 'expired'
    response_text TEXT,                   -- The Victorian-era response text
    acceptance_likelihood INTEGER,        -- 0-100 calculated when sent
    decline_reason TEXT,                  -- If declined: 'schedule_conflict', 'prestige_mismatch', 'too_soon', etc.

    -- News integration
    sent_news_id TEXT,                    -- Reference to news story about sending
    response_news_id TEXT,                -- Reference to news story about receiving response

    -- Match scheduling (if accepted)
    scheduled_match_id TEXT,              -- Reference to the actual match created

    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (sender_club_id) REFERENCES sheffield_clubs(id),
    FOREIGN KEY (recipient_club_id) REFERENCES sheffield_clubs(id),
    FOREIGN KEY (scheduled_match_id) REFERENCES sheffield_matches(id)
);

-- News/Inbox system for in-game events
CREATE TABLE IF NOT EXISTS sheffield_news_items (
    id TEXT PRIMARY KEY,
    headline TEXT NOT NULL,
    article_type TEXT NOT NULL,           -- 'challenge_sent', 'challenge_received', 'match_result', 'cup_announcement', etc.
    publish_date TEXT NOT NULL,           -- When this news appears (ISO 8601 date)
    body_text TEXT NOT NULL,              -- Main article text

    -- Status
    is_read BOOLEAN DEFAULT 0,
    is_important BOOLEAN DEFAULT 0,
    requires_action BOOLEAN DEFAULT 0,    -- Does this need user interaction?

    -- Related entities (JSON arrays)
    related_club_ids TEXT,                -- JSON array of club IDs mentioned
    related_player_ids TEXT,              -- JSON array of player IDs mentioned
    related_match_id TEXT,                -- If about a match
    related_invitation_id TEXT,           -- If about a challenge invitation

    -- Display
    has_action_button BOOLEAN DEFAULT 0,  -- Does this news have a button (e.g., "Read Letter")
    action_button_text TEXT,              -- Text for the button
    action_type TEXT,                     -- 'read_letter', 'view_match', 'accept_invitation', etc.
    action_data TEXT,                     -- JSON data for the action

    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (related_match_id) REFERENCES sheffield_matches(id),
    FOREIGN KEY (related_invitation_id) REFERENCES sheffield_challenge_invitations(id)
);

-- Indexes for challenge invitation system
CREATE INDEX IF NOT EXISTS idx_invitations_sender ON sheffield_challenge_invitations(sender_club_id);
CREATE INDEX IF NOT EXISTS idx_invitations_recipient ON sheffield_challenge_invitations(recipient_club_id);
CREATE INDEX IF NOT EXISTS idx_invitations_response_date ON sheffield_challenge_invitations(response_date);
CREATE INDEX IF NOT EXISTS idx_invitations_status ON sheffield_challenge_invitations(status);
CREATE INDEX IF NOT EXISTS idx_news_publish_date ON sheffield_news_items(publish_date);
CREATE INDEX IF NOT EXISTS idx_news_is_read ON sheffield_news_items(is_read);
CREATE INDEX IF NOT EXISTS idx_news_requires_action ON sheffield_news_items(requires_action);
CREATE INDEX IF NOT EXISTS idx_news_invitation ON sheffield_news_items(related_invitation_id);

-- Game state table (stores the GameState JSON for .sav files)
CREATE TABLE IF NOT EXISTS game_state (
    id TEXT PRIMARY KEY,
    data TEXT NOT NULL,  -- JSON serialized GameState
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
