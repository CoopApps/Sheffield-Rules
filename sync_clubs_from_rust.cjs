const Database = require('better-sqlite3');
const fs = require('fs');
const db = new Database('Sheffield1867.db');

console.log('\n========================================');
console.log('SYNCING CLUBS FROM RUST SOURCE');
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

// Clear existing clubs table
db.exec('DELETE FROM clubs');
console.log('Cleared existing clubs table\n');

// Insert all clubs
const insert = db.prepare(`
    INSERT INTO clubs (id, name, founded, ground, origin, location, postcode)
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
const count = db.prepare('SELECT COUNT(*) as c FROM clubs').get();
const withPostcode = db.prepare('SELECT COUNT(*) as c FROM clubs WHERE postcode IS NOT NULL').get();
const withoutPostcode = db.prepare('SELECT COUNT(*) as c FROM clubs WHERE postcode IS NULL').get();

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
    const noPc = db.prepare('SELECT name, ground FROM clubs WHERE postcode IS NULL').all();
    noPc.forEach(c => console.log(`  - ${c.name} (${c.ground})`));
    console.log('');
}

// Show sample with postcodes
console.log('Sample clubs with postcodes:');
const sample = db.prepare('SELECT name, founded, postcode FROM clubs WHERE postcode IS NOT NULL LIMIT 10').all();
sample.forEach(c => console.log(`  ${c.name} (${c.founded}) - ${c.postcode}`));
console.log('');

db.close();
