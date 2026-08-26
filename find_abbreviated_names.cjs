const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('Business Names with Abbreviations (containing periods)');
console.log('='.repeat(70) + '\n');

// Get business names with periods (likely abbreviations)
const businesses = db.prepare('SELECT full_name FROM sheffield_businesses WHERE full_name LIKE \'%.%\' LIMIT 50').all();

console.log('First 50 business names containing periods:\n');

businesses.forEach((b, i) => {
    console.log(`${i + 1}. "${b.full_name}"`);
});

console.log('\n' + '='.repeat(70));
console.log('Total found:', businesses.length);

db.close();
