const Database = require('better-sqlite3');
const fs = require('fs');
const db = new Database('Sheffield1867.db');

console.log('\n========================================');
console.log('SYNCING CLUBS FROM RUST SOURCE TO SHEFFIELD_CLUBS TABLE');
console.log('========================================\n');

// Read the Rust file
const rustFile = fs.readFileSync('src-tauri/src/sheffield_rules/clubs.rs', 'utf8');

// Parse club data from Rust Vec
const clubs = [];
const clubRegex = /SheffieldClub\s*\{[\s\S]*?id:\s*"([^"]+)"[\s\S]*?name:\s*"([^"]+)"[\s\S]*?founded_year:\s*(\d+)[\s\S]*?ground:\s*"([^"]*)"[\s\S]*?origin:\s*"([^"]*)"[\s\S]*?city:\s*(?:Some\("([^"]*)"\)|None)[\s\S]*?region:\s*(?:Some\("([^"]*)"\)|None)[\s\S]*?\}/g;

let match;
while ((match = clubRegex.exec(rustFile)) !== null) {
    clubs.push({
        id: match[1],
        name: match[2],
        founded: parseInt(match[3]),
        ground: match[4],
        origin: match[5],
        location: match[6] || null,
        postcode: match[7] || null
    });
}

console.log(`Parsed ${clubs.length} clubs from Rust source\n`);

// Clear existing sheffield_clubs table (this is the table the game reads from!)
db.exec('DELETE FROM sheffield_clubs');
console.log('Cleared existing sheffield_clubs table\n');

// Insert all clubs into sheffield_clubs (matching the schema used by the game)
const insert = db.prepare(`
    INSERT INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region)
    VALUES (?, ?, ?, ?, ?, ?, ?)
`);

const insertMany = db.transaction((clubs) => {
    for (const club of clubs) {
        insert.run(
            club.id,
            club.name,
            club.founded,
            club.ground,
            club.origin,
            club.location,
            club.postcode
        );
    }
});

insertMany(clubs);

// Verify
const count = db.prepare('SELECT COUNT(*) as c FROM sheffield_clubs').get();
const withPostcode = db.prepare('SELECT COUNT(*) as c FROM sheffield_clubs WHERE region IS NOT NULL').get();
const withoutPostcode = db.prepare('SELECT COUNT(*) as c FROM sheffield_clubs WHERE region IS NULL').get();

console.log('========================================');
console.log('SYNC COMPLETE');
console.log('========================================');
console.log(`Total clubs: ${count.c}`);
console.log(`With postcode: ${withPostcode.c}`);
console.log(`Without postcode: ${withoutPostcode.c}`);
console.log('========================================\n');

// Show clubs without postcodes
if (withoutPostcode.c > 0) {
    console.log('Clubs without postcodes:');
    const noPc = db.prepare('SELECT name, ground_name FROM sheffield_clubs WHERE region IS NULL').all();
    noPc.forEach(c => console.log(`  - ${c.name} (${c.ground_name})`));
    console.log('');
}

// Show sample with postcodes
console.log('Sample clubs with postcodes:');
const sample = db.prepare('SELECT name, founded_year, region FROM sheffield_clubs WHERE region IS NOT NULL LIMIT 10').all();
sample.forEach(c => console.log(`  ${c.name} (${c.founded_year}) - ${c.region}`));
console.log('');

db.close();
