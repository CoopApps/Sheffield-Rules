const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('=== Cleaning sheffield_people duplicates ===\n');

const before = db.prepare('SELECT COUNT(*) as c FROM sheffield_people').get().c;
console.log('Before:', before.toLocaleString());

console.log('\nRemoving exact duplicates...');

// Delete duplicates, keeping only the record with the MIN(id)
db.exec(`
    DELETE FROM sheffield_people
    WHERE id NOT IN (
        SELECT MIN(id)
        FROM sheffield_people
        GROUP BY
            name,
            birth_year,
            gender,
            census_age,
            census_birth_place,
            census_relation,
            civil_parish,
            street_address,
            profession,
            source,
            census_piece,
            census_folio,
            census_page
    )
`);

const after = db.prepare('SELECT COUNT(*) as c FROM sheffield_people').get().c;
console.log('\nAfter:', after.toLocaleString());
console.log('Removed:', (before - after).toLocaleString(), 'exact duplicates\n');

// Verify Hannah Hall
console.log('Verifying Hannah Hall (1820):');
const hannahs = db.prepare(`
    SELECT COUNT(*) as c FROM sheffield_people
    WHERE name = 'Hannah Hall' AND birth_year = 1820
`).get().c;
console.log('  Now appears:', hannahs, 'time(s)\n');

db.close();
console.log('✓ Cleanup complete!');
