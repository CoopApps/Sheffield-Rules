const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('Checking for Abbott/Geo entries...\n');

const abbotts = db.prepare('SELECT full_name FROM sheffield_businesses WHERE full_name LIKE \'%Abbott%\' OR full_name LIKE \'%Geo%\' LIMIT 20').all();

console.log('Found', abbotts.length, 'entries:\n');

abbotts.forEach((a, i) => {
    console.log(`${i + 1}. "${a.full_name}"`);
});

db.close();
