const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('Expanding Name Abbreviations in Business Names');
console.log('='.repeat(70) + '\n');

// Common name abbreviations
const nameExpansions = {
    'Geo.': 'George',
    'Geo': 'George',
    'Wm.': 'William',
    'Wm': 'William',
    'Thos.': 'Thomas',
    'Thos': 'Thomas',
    'Jas.': 'James',
    'Jas': 'James',
    'Edwd.': 'Edward',
    'Edwd': 'Edward',
    'Saml.': 'Samuel',
    'Saml': 'Samuel',
    'Jno.': 'John',
    'Jno': 'John',
    'Robt.': 'Robert',
    'Robt': 'Robert',
    'Chas.': 'Charles',
    'Chas': 'Charles',
    'Jos.': 'Joseph',
    'Jos': 'Joseph',
    'Benj.': 'Benjamin',
    'Benj': 'Benjamin',
    'Richd.': 'Richard',
    'Richd': 'Richard',
    'Fred.': 'Frederick',
    'Fred': 'Frederick',
    'Fredk.': 'Frederick',
    'Fredk': 'Frederick',
    'Elizth.': 'Elizabeth',
    'Elizth': 'Elizabeth',
    'Eliz.': 'Elizabeth',
    'Eliz': 'Elizabeth',
    'Matt.': 'Matthew',
    'Matt': 'Matthew',
    'Dan.': 'Daniel',
    'Dan': 'Daniel',
    'Josh.': 'Joshua',
    'Josh': 'Joshua',
    'Steph.': 'Stephen',
    'Steph': 'Stephen',
    'Alex.': 'Alexander',
    'Alex': 'Alexander'
};

// Function to expand abbreviations in a name
function expandName(name) {
    if (!name) return name;

    let expanded = name;

    // Replace each abbreviation
    for (const [abbrev, full] of Object.entries(nameExpansions)) {
        // Match abbreviation at start of name or after a space, with optional period
        const cleanAbbrev = abbrev.replace('.', '');
        const regex = new RegExp(`\\b${cleanAbbrev}\\.?\\b`, 'g');
        expanded = expanded.replace(regex, full);
    }

    return expanded;
}

// Get all businesses
const businesses = db.prepare('SELECT * FROM sheffield_businesses').all();

console.log('Total businesses:', businesses.length.toLocaleString());

let updated = 0;
let samples = [];

const updateStmt = db.prepare('UPDATE sheffield_businesses SET full_name = ? WHERE id = ?');

businesses.forEach(business => {
    const original = business.full_name;
    const expanded = expandName(original);

    if (expanded !== original) {
        updateStmt.run(expanded, business.id);
        updated++;

        if (samples.length < 25) {
            samples.push({ original, expanded });
        }
    }
});

console.log('\nExpanded abbreviations in', updated.toLocaleString(), 'business names\n');

console.log('='.repeat(70));
console.log('SAMPLE EXPANSIONS (First 25)');
console.log('='.repeat(70) + '\n');

samples.forEach((s, i) => {
    console.log(`${i + 1}. "${s.original}" → "${s.expanded}"`);
});

console.log('\n' + '='.repeat(70));
console.log('✓ Name abbreviation expansion complete!');

db.close();
