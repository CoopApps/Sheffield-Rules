const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('Linking unique businesses to people...\n');

// Find unique matches with house numbers
const findMatches = db.prepare(`
    SELECT
        b.id as business_id,
        p.id as person_id,
        b.name,
        b.street_address
    FROM sheffield_businesses b
    JOIN sheffield_people p ON
        b.name = p.name
        AND LOWER(REPLACE(b.street_address, ' ', '')) = LOWER(REPLACE(p.street_address, ' ', ''))
    WHERE b.street_address GLOB '*[0-9]*'
      AND p.street_address GLOB '*[0-9]*'
`);

const allMatches = findMatches.all();

// Group by business_id to find unique matches
const matchesByBusiness = new Map();
allMatches.forEach(match => {
    if (!matchesByBusiness.has(match.business_id)) {
        matchesByBusiness.set(match.business_id, []);
    }
    matchesByBusiness.get(match.business_id).push(match);
});

// Filter to only unique (1-to-1) matches
const uniqueMatches = [];
matchesByBusiness.forEach((matches, businessId) => {
    if (matches.length === 1) {
        uniqueMatches.push(matches[0]);
    }
});

console.log(`Found ${uniqueMatches.length} unique 1-to-1 matches`);

// Update businesses
const updateBusiness = db.prepare(`
    UPDATE sheffield_businesses
    SET matched_to_genealogy_id = ?
    WHERE id = ?
`);

const updateMany = db.transaction((matches) => {
    for (const match of matches) {
        updateBusiness.run(match.person_id, match.business_id);
    }
});

updateMany(uniqueMatches);

// Verify
const linked = db.prepare('SELECT COUNT(*) as count FROM sheffield_businesses WHERE matched_to_genealogy_id IS NOT NULL').get();
console.log(`\nBusinesses now linked: ${linked.count}`);

// Show examples
console.log('\nSample linked records:');
const samples = db.prepare(`
    SELECT
        b.id,
        b.name,
        b.street_address,
        b.profession as business_profession,
        p.profession as person_profession
    FROM sheffield_businesses b
    JOIN sheffield_people p ON b.matched_to_genealogy_id = p.id
    LIMIT 10
`).all();

samples.forEach(s => {
    console.log(`  ${s.name} at ${s.street_address}`);
    console.log(`    Business: ${s.business_profession}`);
    console.log(`    Person: ${s.person_profession}\n`);
});

db.close();
console.log('✓ Complete!');
