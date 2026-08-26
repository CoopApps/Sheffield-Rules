const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('Expanding Name Abbreviations');
console.log('='.repeat(70) + '\n');

// Common name abbreviations mapping
const abbreviations = {
    // First names - men
    'Wm': 'William',
    'Thos': 'Thomas',
    'Jas': 'James',
    'Geo': 'George',
    'Chas': 'Charles',
    'Benj': 'Benjamin',
    'Benjn': 'Benjamin',
    'Benjm': 'Benjamin',
    'Jos': 'Joseph',
    'Jno': 'John',
    'Robt': 'Robert',
    'Richd': 'Richard',
    'Edwd': 'Edward',
    'Edw': 'Edward',
    'Saml': 'Samuel',
    'Danl': 'Daniel',
    'Michl': 'Michael',
    'Fras': 'Francis',
    'Fred': 'Frederick',
    'Fredk': 'Frederick',
    'Andw': 'Andrew',
    'Alexr': 'Alexander',
    'Nath': 'Nathaniel',
    'Nathl': 'Nathaniel',
    'Christr': 'Christopher',
    'Patk': 'Patrick',
    'Matthw': 'Matthew',
    'Matt': 'Matthew',
    'Steph': 'Stephen',

    // First names - women
    'Eliz': 'Elizabeth',
    'Elizth': 'Elizabeth',
    'Cath': 'Catherine',
    'Margt': 'Margaret',
    'Margrt': 'Margaret',
    'Mgt': 'Margaret',
    'Ann': 'Ann',
    'Elizbth': 'Elizabeth',
    'Chas': 'Charlotte',
    'Fras': 'Frances',

    // Titles and other abbreviations
    'Jnr': 'Junior',
    'Snr': 'Senior',
    'Jr': 'Junior',
    'Sr': 'Senior',
};

function expandAbbreviations(name) {
    if (!name) return name;

    let expanded = name;

    // Process each abbreviation
    Object.entries(abbreviations).forEach(([abbr, full]) => {
        // Match as whole word (with word boundaries)
        // Handle cases: "Wm Smith", "John Wm Smith", "Wm"
        const regex = new RegExp('\\b' + abbr + '\\b', 'g');
        expanded = expanded.replace(regex, full);
    });

    return expanded;
}

// Process Sheffield Census table
console.log('Processing unmatched_sheffieldcensus...');
const sheffieldRecords = db.prepare('SELECT id, name FROM unmatched_sheffieldcensus').all();
const updateSheffield = db.prepare('UPDATE unmatched_sheffieldcensus SET name = ? WHERE id = ?');

let sheffieldUpdated = 0;
sheffieldRecords.forEach(record => {
    const expanded = expandAbbreviations(record.name);
    if (expanded !== record.name) {
        updateSheffield.run(expanded, record.id);
        sheffieldUpdated++;
    }
});

console.log(`✓ Sheffield Census: ${sheffieldUpdated.toLocaleString()} names expanded\n`);

// Process Genealogy table
console.log('Processing unmatched_genealogy...');
const genealogyRecords = db.prepare('SELECT id, name FROM unmatched_genealogy').all();
const updateGenealogy = db.prepare('UPDATE unmatched_genealogy SET name = ? WHERE id = ?');

let genealogyUpdated = 0;
genealogyRecords.forEach(record => {
    const expanded = expandAbbreviations(record.name);
    if (expanded !== record.name) {
        updateGenealogy.run(expanded, record.id);
        genealogyUpdated++;
    }
});

console.log(`✓ Genealogy: ${genealogyUpdated.toLocaleString()} names expanded\n`);

// Show some examples
console.log('='.repeat(70));
console.log('Sample Expansions (Sheffield Census)');
console.log('='.repeat(70));
const samples = db.prepare('SELECT name FROM unmatched_sheffieldcensus WHERE name LIKE ? LIMIT 5').all('%William%');
samples.forEach(r => console.log('  ' + r.name));

console.log('\n' + '='.repeat(70));
console.log('SUMMARY');
console.log('='.repeat(70));
console.log(`Sheffield Census names updated: ${sheffieldUpdated.toLocaleString()}`);
console.log(`Genealogy names updated: ${genealogyUpdated.toLocaleString()}`);
console.log(`Total expansions: ${(sheffieldUpdated + genealogyUpdated).toLocaleString()}`);

db.close();
console.log('\n✓ Abbreviation expansion complete!');
