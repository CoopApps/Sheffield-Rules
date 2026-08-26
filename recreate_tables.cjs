const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('Recreating All Tables');
console.log('='.repeat(70) + '\n');

// Create institutional tables
console.log('Creating sheffield_workhouse table...');
db.exec(`
    CREATE TABLE IF NOT EXISTS sheffield_workhouse (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        first_name TEXT,
        middle_name TEXT,
        surname TEXT,
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
        workhouse_name TEXT,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    )
`);
console.log('✓ sheffield_workhouse created\n');

console.log('Creating sheffield_asylum table...');
db.exec(`
    CREATE TABLE IF NOT EXISTS sheffield_asylum (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        first_name TEXT,
        middle_name TEXT,
        surname TEXT,
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
        asylum_name TEXT,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    )
`);
console.log('✓ sheffield_asylum created\n');

console.log('Creating sheffield_paupers table...');
db.exec(`
    CREATE TABLE IF NOT EXISTS sheffield_paupers (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        first_name TEXT,
        middle_name TEXT,
        surname TEXT,
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
        pauper_status TEXT,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    )
`);
console.log('✓ sheffield_paupers created\n');

console.log('Creating unmatched_sheffield table...');
db.exec(`
    CREATE TABLE IF NOT EXISTS unmatched_sheffield (
        id TEXT PRIMARY KEY,
        year INTEGER NOT NULL,
        name TEXT NOT NULL,
        age INTEGER,
        gender TEXT,
        birth_year INTEGER,
        birth_place TEXT,
        relation TEXT,
        household_members TEXT,
        civil_parish TEXT,
        ecclesiastical_parish TEXT,
        registration_district TEXT,
        county TEXT,
        piece TEXT,
        folio TEXT,
        page TEXT,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    )
`);
console.log('✓ unmatched_sheffield created\n');

console.log('Creating unmatched_genealogy table...');
db.exec(`
    CREATE TABLE IF NOT EXISTS unmatched_genealogy (
        id TEXT PRIMARY KEY,
        year INTEGER NOT NULL,
        name TEXT NOT NULL,
        age INTEGER,
        birth_year INTEGER,
        birth_place TEXT,
        address TEXT,
        profession TEXT,
        spouse TEXT,
        relation TEXT,
        parish TEXT,
        area TEXT,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    )
`);
console.log('✓ unmatched_genealogy created\n');

// Create indexes
console.log('Creating indexes...');
db.exec('CREATE INDEX IF NOT EXISTS idx_unmatched_sheffield_name ON unmatched_sheffield(name)');
db.exec('CREATE INDEX IF NOT EXISTS idx_unmatched_sheffield_year ON unmatched_sheffield(year)');
db.exec('CREATE INDEX IF NOT EXISTS idx_unmatched_genealogy_name ON unmatched_genealogy(name)');
db.exec('CREATE INDEX IF NOT EXISTS idx_unmatched_genealogy_year ON unmatched_genealogy(year)');
console.log('✓ Indexes created\n');

console.log('='.repeat(70));
console.log('All tables recreated and ready');
console.log('='.repeat(70));

db.close();
