-- Whites Directory Matching System
-- Stores business directory entries and matches them to sheffield_players

-- Whites directory entries: Business listings from Whites Directory
CREATE TABLE IF NOT EXISTS sheffield_whites_entries (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,                     -- Business owner/proprietor name
    first_name TEXT,
    surname TEXT,
    business_name TEXT,                     -- e.g., "Smith & Sons", "The Crown Inn"
    business_type TEXT,                     -- e.g., "Cutler", "Inn", "Grocer"
    profession TEXT,                        -- Occupation/trade
    street_address TEXT NOT NULL,           -- Full street address
    district TEXT,                          -- e.g., "Park", "Ecclesall", "Brightside"
    parish TEXT,                            -- Civil or ecclesiastical parish
    postcode TEXT,                          -- If available
    notes TEXT,                             -- Additional information
    year INTEGER NOT NULL,                  -- Directory publication year
    page_number INTEGER,                    -- Page in directory

    -- Matching metadata
    is_matched BOOLEAN DEFAULT 0,           -- Whether matched to a player
    matched_player_id TEXT,                 -- FK to sheffield_players
    match_confidence REAL,                  -- 0.0-1.0 confidence score
    match_method TEXT,                      -- 'automatic', 'manual', 'reviewed'
    match_timestamp TEXT,

    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (matched_player_id) REFERENCES sheffield_players(id)
);

-- Match history: Track all match attempts and decisions
CREATE TABLE IF NOT EXISTS sheffield_whites_match_history (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    whites_entry_id TEXT NOT NULL,
    player_id TEXT NOT NULL,
    confidence_score REAL NOT NULL,         -- 0.0-1.0
    match_factors TEXT,                     -- JSON: {"name": 0.5, "address": 0.3, "profession": 0.2}
    match_reason TEXT,                      -- Human-readable explanation
    status TEXT NOT NULL,                   -- 'suggested', 'accepted', 'rejected', 'review'
    decided_by TEXT,                        -- 'automatic', 'user'
    decided_at TEXT,
    notes TEXT,                             -- User notes on why accepted/rejected

    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (whites_entry_id) REFERENCES sheffield_whites_entries(id),
    FOREIGN KEY (player_id) REFERENCES sheffield_players(id)
);

-- Matching rules: User-defined rules for automated matching
CREATE TABLE IF NOT EXISTS sheffield_whites_match_rules (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    rule_name TEXT NOT NULL,
    rule_type TEXT NOT NULL,                -- 'address', 'profession', 'name', 'parish'
    pattern TEXT NOT NULL,                  -- Match pattern or value
    weight REAL DEFAULT 0.0,                -- Contribution to confidence (0.0-1.0)
    is_active BOOLEAN DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Unmatched businesses: Quick view of businesses needing review
CREATE VIEW IF NOT EXISTS sheffield_whites_unmatched AS
SELECT
    id,
    name,
    business_name,
    business_type,
    profession,
    street_address,
    district,
    parish,
    year
FROM sheffield_whites_entries
WHERE is_matched = 0
ORDER BY surname, first_name;

-- Matched businesses: Quick view of confirmed matches
CREATE VIEW IF NOT EXISTS sheffield_whites_matched AS
SELECT
    w.id as whites_id,
    w.name as business_owner,
    w.business_name,
    w.business_type,
    w.profession as business_profession,
    w.street_address,
    w.district,
    w.parish as business_parish,
    p.id as player_id,
    p.name as player_name,
    p.birth_year,
    p.civil_parish as player_parish,
    p.profession as player_profession,
    p.street_address as player_address,
    w.match_confidence,
    w.match_method
FROM sheffield_whites_entries w
JOIN sheffield_players p ON w.matched_player_id = p.id
WHERE w.is_matched = 1
ORDER BY w.match_confidence DESC;

-- Suggested matches: Potential matches for review
CREATE VIEW IF NOT EXISTS sheffield_whites_suggestions AS
SELECT
    w.id as whites_id,
    w.name as business_owner,
    w.profession as business_profession,
    w.street_address as business_address,
    w.parish as business_parish,
    p.id as player_id,
    p.name as player_name,
    p.birth_year,
    p.profession as player_profession,
    p.street_address as player_address,
    p.civil_parish as player_parish,
    h.confidence_score,
    h.match_reason,
    h.status
FROM sheffield_whites_match_history h
JOIN sheffield_whites_entries w ON h.whites_entry_id = w.id
JOIN sheffield_players p ON h.player_id = p.id
WHERE h.status = 'suggested'
ORDER BY h.confidence_score DESC;

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_whites_surname ON sheffield_whites_entries(surname);
CREATE INDEX IF NOT EXISTS idx_whites_parish ON sheffield_whites_entries(parish);
CREATE INDEX IF NOT EXISTS idx_whites_profession ON sheffield_whites_entries(profession);
CREATE INDEX IF NOT EXISTS idx_whites_matched ON sheffield_whites_entries(is_matched);
CREATE INDEX IF NOT EXISTS idx_whites_year ON sheffield_whites_entries(year);
CREATE INDEX IF NOT EXISTS idx_match_history_status ON sheffield_whites_match_history(status);
CREATE INDEX IF NOT EXISTS idx_match_history_confidence ON sheffield_whites_match_history(confidence_score);
