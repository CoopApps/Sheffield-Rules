const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('Removing Extra Periods After Expanded Names');
console.log('='.repeat(70) + '\n');

// Common full names that shouldn't have periods after them
const fullNames = [
    'George', 'William', 'Thomas', 'James', 'Edward', 'Samuel',
    'John', 'Robert', 'Charles', 'Joseph', 'Benjamin', 'Richard',
    'Frederick', 'Elizabeth', 'Matthew', 'Daniel', 'Joshua',
    'Stephen', 'Alexander'
];

const updateStmt = db.prepare('UPDATE sheffield_businesses SET full_name = ? WHERE id = ?');
const businesses = db.prepare('SELECT * FROM sheffield_businesses').all();

let updated = 0;
let samples = [];

businesses.forEach(business => {
    let cleaned = business.full_name;

    // Remove period after full names (but not after initials like "A.")
    fullNames.forEach(name => {
        // Match the full name followed by a period and then a space or end of string
        // This avoids matching initials like "A."
        const regex = new RegExp(`\\b${name}\\.\\s`, 'g');
        cleaned = cleaned.replace(regex, `${name} `);

        // Also handle case where period is at end of name before end of string
        const regexEnd = new RegExp(`\\b${name}\\.$`, 'g');
        cleaned = cleaned.replace(regexEnd, name);
    });

    if (cleaned !== business.full_name) {
        updateStmt.run(cleaned, business.id);
        updated++;

        if (samples.length < 25) {
            samples.push({ original: business.full_name, cleaned });
        }
    }
});

console.log('Removed extra periods from', updated.toLocaleString(), 'business names\n');

console.log('='.repeat(70));
console.log('SAMPLE CLEANUPS (First 25)');
console.log('='.repeat(70) + '\n');

samples.forEach((s, i) => {
    console.log(`${i + 1}. "${s.original}" → "${s.cleaned}"`);
});

console.log('\n' + '='.repeat(70));
console.log('✓ Period cleanup complete!');

db.close();
