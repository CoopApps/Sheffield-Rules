// Copy clubs from saturday_at_three.db to Sheffield1867.db as 1867fantasyclubs table
const Database = require('better-sqlite3');
const path = require('path');

const sourceDb = new Database('./saturday_at_three.db', { readonly: true });
const targetDb = new Database('./Sheffield1867.db');

console.log('\n=== Creating 1867fantasyclubs table ===\n');

// Create the table with all columns from clubs plus postcode fields
targetDb.exec(`
    CREATE TABLE IF NOT EXISTS "1867fantasyclubs" (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        short_name TEXT,
        founded_year INTEGER,
        ground_name TEXT,
        ground_capacity INTEGER,
        city TEXT,
        region TEXT,
        origin TEXT,
        primary_color TEXT,
        secondary_color TEXT,
        badge_url TEXT,
        reserve_of_club_id TEXT,
        where_from TEXT,

        -- Postcode columns
        postcode TEXT,
        postcode_area TEXT,
        postcode_district TEXT,
        postcode_sector TEXT,
        postcode_unit TEXT,
        latitude REAL,
        longitude REAL,

        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    )
`);

console.log('✓ Table created\n');

// Get all clubs from source database
const clubs = sourceDb.prepare('SELECT * FROM clubs').all();
console.log(`Found ${clubs.length} clubs in saturday_at_three.db\n`);

// Insert clubs into target database
const insert = targetDb.prepare(`
    INSERT OR REPLACE INTO "1867fantasyclubs" (
        id, name, short_name, founded_year, ground_name, ground_capacity,
        city, region, origin, primary_color, secondary_color, badge_url,
        reserve_of_club_id, where_from
    ) VALUES (
        @id, @name, @short_name, @founded_year, @ground_name, @ground_capacity,
        @city, @region, @origin, @primary_color, @secondary_color, @badge_url,
        @reserve_of_club_id, @where_from
    )
`);

let imported = 0;
for (const club of clubs) {
    insert.run({
        id: club.id,
        name: club.name,
        short_name: club.short_name || null,
        founded_year: club.founded_year || null,
        ground_name: club.ground_name || null,
        ground_capacity: club.ground_capacity || null,
        city: club.city || null,
        region: club.region || null,
        origin: club.origin || null,
        primary_color: club.primary_color || null,
        secondary_color: club.secondary_color || null,
        badge_url: club.badge_url || null,
        reserve_of_club_id: club.reserve_of_club_id || null,
        where_from: club.where_from || null
    });
    imported++;
    console.log(`✓ Imported: ${club.name}`);
}

console.log(`\n=== Import Complete ===`);
console.log(`Total clubs imported: ${imported}\n`);

sourceDb.close();
targetDb.close();
