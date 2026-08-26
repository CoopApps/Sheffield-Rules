const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('WIPING DATABASE - Starting Fresh');
console.log('='.repeat(70) + '\n');

// Drop all tables
console.log('Dropping all tables...\n');

db.exec('DROP TABLE IF EXISTS sheffield_people');
db.exec('DROP TABLE IF EXISTS sheffield_players');
db.exec('DROP TABLE IF EXISTS sheffield_workhouse');
db.exec('DROP TABLE IF EXISTS sheffield_asylum');
db.exec('DROP TABLE IF EXISTS sheffield_paupers');
db.exec('DROP TABLE IF EXISTS unmatched_sheffield');
db.exec('DROP TABLE IF EXISTS unmatched_genealogy');

console.log('✓ All tables dropped\n');

// Create fresh sheffield_people table
console.log('Creating fresh sheffield_people table...');
db.exec(`
    CREATE TABLE sheffield_people (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        first_name TEXT,
        middle_name TEXT,
        surname TEXT,
        source TEXT NOT NULL,
        birth_year INTEGER,
        gender TEXT,
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
        where_born TEXT,
        birth_town TEXT,
        birth_county TEXT,
        birth_country TEXT,
        civil_parish TEXT,
        ecclesiastical_parish TEXT,
        registration_district TEXT,
        sub_registration_district TEXT,
        street_address TEXT,
        profession TEXT,
        player_id TEXT,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (player_id) REFERENCES sheffield_players(id)
    )
`);
console.log('✓ sheffield_people created\n');

// Create fresh sheffield_players table
console.log('Creating fresh sheffield_players table...');
db.exec(`
    CREATE TABLE sheffield_players (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        birth_year INTEGER,
        gender TEXT,
        birth_place TEXT,
        profession TEXT,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    )
`);
console.log('✓ sheffield_players created\n');

console.log('='.repeat(70));
console.log('Database wiped clean and ready for fresh import');
console.log('='.repeat(70));

db.close();
