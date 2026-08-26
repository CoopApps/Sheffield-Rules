const sqlite3 = require('better-sqlite3');
const db = new sqlite3('Sheffield1867.db');

console.log('Creating sheffield_people table...\n');

// Create the sheffield_people table with all census and genealogy fields
const createTableSQL = `
CREATE TABLE IF NOT EXISTS sheffield_people (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    first_name TEXT,
    middle_name TEXT,
    surname TEXT,
    birth_year INTEGER,
    gender TEXT,

    -- Census data
    census_age INTEGER,
    census_birth_date TEXT,
    census_birth_place TEXT,
    census_county TEXT,
    census_relation TEXT,
    census_gender TEXT,
    census_ed TEXT,
    census_household_schedule TEXT,
    census_household_members TEXT,
    census_piece TEXT,
    census_folio TEXT,
    census_page TEXT,

    -- Birth location data
    where_born TEXT,
    birth_town TEXT,
    birth_county TEXT,
    birth_country TEXT,

    -- Parish data
    civil_parish TEXT,
    ecclesiastical_parish TEXT,
    registration_district TEXT,
    sub_registration_district TEXT,

    -- Genealogy data
    street_address TEXT,
    profession TEXT,

    -- Link to player if they are a player
    player_id TEXT,
    is_player BOOLEAN DEFAULT 0,

    -- Metadata
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (player_id) REFERENCES sheffield_players(id)
)
`;

try {
    db.exec(createTableSQL);
    console.log('✓ Created sheffield_people table');

    // Create indexes for efficient lookups
    db.exec('CREATE INDEX IF NOT EXISTS idx_people_name_birth ON sheffield_people(name, birth_year)');
    db.exec('CREATE INDEX IF NOT EXISTS idx_people_household ON sheffield_people(census_household_schedule, census_piece)');
    db.exec('CREATE INDEX IF NOT EXISTS idx_people_player_id ON sheffield_people(player_id)');
    db.exec('CREATE INDEX IF NOT EXISTS idx_people_is_player ON sheffield_people(is_player)');

    console.log('✓ Created indexes');

    // Check if table has data
    const count = db.prepare('SELECT COUNT(*) as count FROM sheffield_people').get();
    console.log(`\nCurrent records in sheffield_people: ${count.count}`);

} catch (error) {
    console.error('Error:', error.message);
}

db.close();
console.log('\nDatabase connection closed.');
