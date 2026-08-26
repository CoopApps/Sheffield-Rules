const sqlite3 = require('better-sqlite3');

const db = new sqlite3('Sheffield1867.db');

console.log('='.repeat(70));
console.log('COMPREHENSIVE DATABASE STATISTICS');
console.log('='.repeat(70) + '\n');

console.log('TABLE COUNTS:');
console.log('-'.repeat(70));

const tables = ['sheffield_players', 'sheffield_people', 'sheffield_businesses', 'sheffield_workhouse', 'sheffield_asylum'];
tables.forEach(table => {
    const count = db.prepare('SELECT COUNT(*) as count FROM ' + table).get();
    console.log('  ' + table + ': ' + count.count.toLocaleString());
});

console.log('\nSHEFFIELD_PLAYERS:');
console.log('-'.repeat(70));

const playerStats = db.prepare(`
    SELECT
        COUNT(*) as total,
        COUNT(street_address) as with_address,
        COUNT(profession) as with_profession,
        COUNT(postcode) as with_postcode,
        COUNT(birth_town) as with_birth_town
    FROM sheffield_players
`).get();

console.log('  Total players: ' + playerStats.total.toLocaleString());
console.log('  With addresses: ' + playerStats.with_address.toLocaleString() + ' (' + (playerStats.with_address / playerStats.total * 100).toFixed(1) + '%)');
console.log('  With professions: ' + playerStats.with_profession.toLocaleString() + ' (' + (playerStats.with_profession / playerStats.total * 100).toFixed(1) + '%)');
console.log('  With postcodes: ' + playerStats.with_postcode.toLocaleString() + ' (' + (playerStats.with_postcode / playerStats.total * 100).toFixed(1) + '%)');
console.log('  With birth towns: ' + playerStats.with_birth_town.toLocaleString() + ' (' + (playerStats.with_birth_town / playerStats.total * 100).toFixed(1) + '%)');

console.log('\nSHEFFIELD_PEOPLE:');
console.log('-'.repeat(70));

const peopleStats = db.prepare(`
    SELECT
        COUNT(*) as total,
        COUNT(street_address) as with_address,
        COUNT(profession) as with_profession,
        COUNT(postcode) as with_postcode,
        COUNT(player_id) as linked_to_players,
        COUNT(birth_town) as with_birth_town
    FROM sheffield_people
`).get();

console.log('  Total people: ' + peopleStats.total.toLocaleString());
console.log('  With addresses: ' + peopleStats.with_address.toLocaleString() + ' (' + (peopleStats.with_address / peopleStats.total * 100).toFixed(1) + '%)');
console.log('  With professions: ' + peopleStats.with_profession.toLocaleString() + ' (' + (peopleStats.with_profession / peopleStats.total * 100).toFixed(1) + '%)');
console.log('  With postcodes: ' + peopleStats.with_postcode.toLocaleString() + ' (' + (peopleStats.with_postcode / peopleStats.total * 100).toFixed(1) + '%)');
console.log('  Linked to players: ' + peopleStats.linked_to_players.toLocaleString() + ' (' + (peopleStats.linked_to_players / peopleStats.total * 100).toFixed(1) + '%)');
console.log('  With birth towns: ' + peopleStats.with_birth_town.toLocaleString() + ' (' + (peopleStats.with_birth_town / peopleStats.total * 100).toFixed(1) + '%)');

const employers = db.prepare('SELECT COUNT(*) as count FROM sheffield_people WHERE profession LIKE ?').get('%employ%');
console.log('  Employers (employ/employing): ' + employers.count.toLocaleString());

console.log('\nSHEFFIELD_BUSINESSES:');
console.log('-'.repeat(70));

const bizStats = db.prepare(`
    SELECT
        COUNT(*) as total,
        COUNT(occupation) as with_occupation,
        COUNT(address) as with_address
    FROM sheffield_businesses
`).get();

console.log('  Total businesses: ' + bizStats.total.toLocaleString());
console.log('  With occupation: ' + bizStats.with_occupation.toLocaleString() + ' (' + (bizStats.with_occupation / bizStats.total * 100).toFixed(1) + '%)');
console.log('  With address: ' + bizStats.with_address.toLocaleString() + ' (' + (bizStats.with_address / bizStats.total * 100).toFixed(1) + '%)');

console.log('\nINSTITUTIONS:');
console.log('-'.repeat(70));

const workhouse = db.prepare('SELECT COUNT(*) as count FROM sheffield_workhouse').get();
const asylum = db.prepare('SELECT COUNT(*) as count FROM sheffield_asylum').get();

console.log('  Workhouse residents: ' + workhouse.count);
console.log('  Asylum residents: ' + asylum.count);

console.log('\n' + '='.repeat(70));
console.log('DATA SOURCES BREAKDOWN');
console.log('='.repeat(70) + '\n');

console.log('People by census year:');
const censusByYear = db.prepare(`
    SELECT
        SUBSTR(census_piece, 1, 4) as year,
        COUNT(*) as count
    FROM sheffield_people
    WHERE census_piece IS NOT NULL
    GROUP BY year
    ORDER BY year
`).all();

censusByYear.forEach(row => {
    console.log('  ' + (row.year || 'Unknown') + ': ' + row.count.toLocaleString());
});

db.close();

console.log('\n✓ Statistics complete!');
