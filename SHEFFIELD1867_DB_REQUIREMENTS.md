# Sheffield1867.db Database Requirements

## Overview
This document describes the complete requirements for the `Sheffield1867.db` database file used by the **Sheffield-Hallamshire League** fantasy mode in Saturday at Three.

## How Database Loading Works

### Selection Flow
1. User selects **1867 Sheffield and Hallamshire League** in year selector
2. User selects a club from the club list
3. Frontend calls: `invoke('initialize_sheffield_game', { year: 1867, gameMode: "sheffield-hallamshire-league", clubId })`
4. Backend detects `gameMode == "sheffield-hallamshire-league"` → uses `Sheffield1867.db`

### Critical Code Path
**File**: `src-tauri/src/commands.rs:604-625`

```rust
let is_fantasy_league = game_mode == "sheffield-hallamshire-league";  // TRUE

let master_db_path = "D:/projects/Saturday at Three/Sheffield1867.db";  // HARDCODED
let temp_db_path = "D:/projects/Saturday at Three/Sheffield1867_temp.db";

if is_fantasy_league {
    // Copy master to temp (to preserve original)
    std::fs::copy(master_db_path, temp_db_path)?;
}

let db_path = temp_db_path;  // Game uses the temp copy
```

**Key Point**: The filename `Sheffield1867.db` is **HARDCODED** - it's NOT derived from the year selection.

---

## Required Database Schema

### Core Tables (MUST EXIST)

#### 1. `sheffield_clubs`
**Purpose**: All 186 historical Sheffield clubs + their reserve teams (372 total)

```sql
CREATE TABLE IF NOT EXISTS sheffield_clubs (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    founded_year INTEGER NOT NULL,
    disbanded_year INTEGER,
    ground_name TEXT,
    origin TEXT,
    city TEXT,
    region TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

**Requirements**:
- Must contain 372 clubs (186 main + 186 reserves)
- Reserve team IDs follow pattern: `{main-club-id}-reserves`
- Examples: `sheffield-fc`, `sheffield-fc-reserves`, `hallam-fc`, `hallam-fc-reserves`

---

#### 2. `sheffield_league_divisions`
**Purpose**: Defines the 10-tier pyramid structure

```sql
CREATE TABLE IF NOT EXISTS sheffield_league_divisions (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    level INTEGER NOT NULL,  -- 1 (top) to 10 (bottom)
    region TEXT,             -- NULL for levels 1-4, 'North'/'East'/'South'/'West' for 5-10
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

**Requirements**:
- Must contain divisions for all 10 levels
- Level 1: `'1'` - "First Division" (16 clubs)
- Level 2: `'2'` - "Second Division" (16 clubs)
- Level 3: `'3'` - "Third Division" (16 clubs)
- Level 4: `'4'` - "Fourth Division" (16 clubs)
- Level 5: 4 divisions of 16 clubs each (`'5N'`, `'5E'`, `'5S'`, `'5W'`)
- Level 6: 4 divisions of 16 clubs each (`'6N'`, `'6E'`, `'6S'`, `'6W'`)
- Level 7: 4 divisions of 16 clubs each (`'7N'`, `'7E'`, `'7S'`, `'7W'`)
- Level 8: 4 divisions of 16 clubs each (`'8N'`, `'8E'`, `'8S'`, `'8W'`)
- Level 9: 4 divisions of 16 clubs each (`'9N'`, `'9E'`, `'9S'`, `'9W'`)
- Level 10: 4 divisions of 12 clubs each (`'10N'`, `'10E'`, `'10S'`, `'10W'`)

**Total**: 36 divisions, 372 club slots

---

#### 3. `sheffield_league_clubs` ⚠️ **CRITICAL**
**Purpose**: Maps clubs to their assigned divisions (THIS IS THE MOST IMPORTANT TABLE)

```sql
CREATE TABLE IF NOT EXISTS sheffield_league_clubs (
    id TEXT PRIMARY KEY,
    division_id TEXT NOT NULL,
    club_id TEXT NOT NULL,
    position_in_division INTEGER,          -- Starting position 1-16
    is_reserve_team BOOLEAN DEFAULT 0,     -- 1 if reserve team
    reserve_of_club_id TEXT,               -- Parent club for reserves
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (division_id) REFERENCES sheffield_league_divisions(id),
    FOREIGN KEY (club_id) REFERENCES sheffield_clubs(id),
    FOREIGN KEY (reserve_of_club_id) REFERENCES sheffield_clubs(id),
    UNIQUE(division_id, club_id)
);
```

**Requirements**:
- **MUST BE POPULATED** - this is what the code queries first (line 661-666 in commands.rs)
- Must contain exactly 372 entries (one per club)
- Each club assigned to exactly one division
- Reserve teams should have `is_reserve_team = 1` and valid `reserve_of_club_id`
- `position_in_division` should be 1-16 for most divisions, 1-12 for level 10

**Code Dependency** (commands.rs:660-672):
```rust
// Get clubs that are in the league (from sheffield_league_clubs which the editor populated)
let league_club_rows = sqlx::query(
    "SELECT club_id FROM sheffield_league_clubs ORDER BY position_in_division"
)
.fetch_all(&pool)
.await
.map_err(|e| format!("Failed to fetch league clubs: {}", e))?;

eprintln!("DEBUG: {} clubs in league", league_clubs.len());

// Initialize standings for all clubs in the league
if !league_clubs.is_empty() {
    database::populate::initialize_standings(&pool, year, league_clubs)
        .await
        .map_err(|e| format!("Failed to initialize standings: {}", e))?;
}
```

**If this table is empty, the game will initialize with NO CLUBS IN THE LEAGUE.**

---

#### 4. `sheffield_game_state`
**Purpose**: Tracks current game session state

```sql
CREATE TABLE IF NOT EXISTS sheffield_game_state (
    id TEXT PRIMARY KEY,
    game_mode TEXT NOT NULL,
    rule_year INTEGER NOT NULL,
    season_year INTEGER NOT NULL,
    start_year INTEGER NOT NULL,
    user_club_id TEXT NOT NULL,
    current_date TEXT,
    current_gameweek INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_club_id) REFERENCES sheffield_clubs(id)
);
```

**Requirements**:
- Must be **empty** at initialization (gets populated by commands.rs:748-761)
- Gets cleared at start of each new game

---

#### 5. `sheffield_standings`
**Purpose**: League table for current season

```sql
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
    rouges_for INTEGER DEFAULT 0,
    rouges_against INTEGER DEFAULT 0,
    points INTEGER DEFAULT 0,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (club_id) REFERENCES sheffield_clubs(id),
    UNIQUE(season, club_id)
);
```

**Requirements**:
- Must be **empty** at initialization
- Gets populated by `initialize_standings()` based on `sheffield_league_clubs`

---

#### 6. `sheffield_fixtures`
**Purpose**: Pre-generated match schedule

```sql
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
```

**Requirements**:
- Must be **empty** at initialization
- Gets populated during season generation

---

#### 7. `sheffield_matches`
**Purpose**: Completed match results

```sql
CREATE TABLE IF NOT EXISTS sheffield_matches (
    id TEXT PRIMARY KEY,
    gameweek INTEGER NOT NULL,
    season INTEGER NOT NULL,
    rule_year INTEGER NOT NULL,
    home_club_id TEXT NOT NULL,
    away_club_id TEXT NOT NULL,
    home_score INTEGER DEFAULT 0,
    away_score INTEGER DEFAULT 0,
    home_rouges INTEGER DEFAULT 0,
    away_rouges INTEGER DEFAULT 0,
    played BOOLEAN DEFAULT 0,
    match_date TEXT,
    -- ... (additional stats columns)
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (home_club_id) REFERENCES sheffield_clubs(id),
    FOREIGN KEY (away_club_id) REFERENCES sheffield_clubs(id)
);
```

**Requirements**:
- Must be **empty** at initialization
- Gets populated as matches are played

---

#### 8. `sheffield_players` (Optional but recommended)
**Purpose**: Player pool for all clubs

```sql
CREATE TABLE IF NOT EXISTS sheffield_players (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    first_name TEXT,
    surname TEXT,
    club_id TEXT,
    position TEXT,
    -- ... (many attribute columns)
    FOREIGN KEY (club_id) REFERENCES sheffield_clubs(id)
);
```

**Requirements**:
- Can be empty (players generated procedurally if missing)
- If populated, provides historical player names and attributes

---

## Additional Tables (For Full Experience)

### Competition System
- `sheffield_competitions` - Cup competitions (Youdan, Cromwell)
- `sheffield_cup_ties` - Cup bracket structure
- `sheffield_competition_participants` - Cup entrants
- `sheffield_competition_winners` - Historical cup winners

### Match Archive
- `sheffield_match_incidents` - Goals, cards, injuries
- `sheffield_match_lineups` - Who played
- `sheffield_match_timeline` - Minute-by-minute events

### Historical Data
- `sheffield_season_archives` - Season summaries
- `sheffield_player_season_stats` - Per-season player stats
- `sheffield_club_season_stats` - Per-season club records

---

## Database Population Status Check

### Quick Check Script
Create `check_db_status.cjs`:

```javascript
const Database = require('better-sqlite3');
const db = new Database('./Sheffield1867.db', { readonly: true });

console.log('=== Sheffield1867.db Status ===\n');

// Check clubs
const clubs = db.prepare('SELECT COUNT(*) as count FROM sheffield_clubs').get();
console.log(`✓ Clubs table: ${clubs.count} clubs`);

// Check divisions
const divs = db.prepare('SELECT COUNT(*) as count FROM sheffield_league_divisions').get();
console.log(`✓ Divisions table: ${divs.count} divisions`);

// Check league assignments (CRITICAL)
const assignments = db.prepare('SELECT COUNT(*) as count FROM sheffield_league_clubs').get();
console.log(`${assignments.count > 0 ? '✓' : '✗'} League clubs: ${assignments.count} assignments`);

if (assignments.count === 0) {
    console.log('\n❌ CRITICAL: sheffield_league_clubs is EMPTY!');
    console.log('The game will not work without club assignments.\n');
} else if (assignments.count === 372) {
    console.log('\n✅ Database is properly configured!\n');
} else {
    console.log(`\n⚠ WARNING: Expected 372 assignments, found ${assignments.count}\n`);
}

db.close();
```

---

## Common Issues

### Issue 1: "No clubs in league" on game start
**Cause**: `sheffield_league_clubs` table is empty
**Solution**: Populate the table with all 372 club assignments

### Issue 2: Database not found
**Cause**: Looking for `Sheffield1867.db` in wrong location
**Solution**: Must be at project root: `D:\projects\Saturday at Three\Sheffield1867.db`

### Issue 3: Clubs exist but league is empty
**Cause**: `sheffield_clubs` populated but `sheffield_league_clubs` not
**Solution**: Run league assignment script to populate divisions

---

## How to Populate the Database

### Option 1: Use existing populate script
```bash
node populate_league_clubs.cjs
```

### Option 2: Manual SQL insertion
See `build_sheffield_db_complete.sql` for club data
Need separate script to assign clubs to divisions based on:
- Founded year (earlier = higher division)
- Historical significance
- Reserve teams in same/lower division as parent

---

## Summary: What Makes Sheffield1867.db Valid?

### ✅ MUST HAVE:
1. ✓ `sheffield_clubs` table with 372 clubs
2. ✓ `sheffield_league_divisions` table with 36 divisions
3. ✓ `sheffield_league_clubs` table with 372 assignments ⚠️ **MOST CRITICAL**
4. ✓ Empty `sheffield_game_state` table
5. ✓ Empty `sheffield_standings` table
6. ✓ Empty `sheffield_fixtures` table
7. ✓ Empty `sheffield_matches` table

### 👍 SHOULD HAVE:
- `sheffield_players` table (can be empty, will generate if missing)

### 🎁 NICE TO HAVE:
- Competition tables (for cup system)
- Archive tables (for history)
- Match detail tables (for 2D replay)

---

## File Location
**Master**: `D:\projects\Saturday at Three\Sheffield1867.db`
**Temp (used in game)**: `D:\projects\Saturday at Three\Sheffield1867_temp.db`

The game copies master → temp at initialization to preserve the original.

---

## Reference Files
- Schema: `src-tauri/src/database/sheffield_schema.sql`
- Initialization: `src-tauri/src/commands.rs` (lines 595-800)
- Club data: `build_sheffield_db_complete.sql`
- Assignment script: `populate_league_clubs.cjs`
