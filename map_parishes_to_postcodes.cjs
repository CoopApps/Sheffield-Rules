const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('\n========================================');
console.log('MAPPING ECCLESIASTICAL PARISHES TO MODERN POSTCODES');
console.log('========================================\n');

// Parish to postcode mapping based on the ecclesiastical boundaries document
const parishToPostcode = {
    // Central districts
    "St. Peter's": 'S1',
    "St. Paul's": 'S1',
    "Porter Street": 'S1',
    "St. James's": 'S1',
    "Carver Street": 'S1',
    "Eldon": 'S1',

    // South and South East
    "Heeley": 'S2',
    "St. John's": 'S2',
    "Dyer's Hill": 'S2',
    "St. Mary's": 'S2',

    // North and North East
    "Wicker": 'S3',
    "Pitsmoor": 'S3',
    "Netherthorpe": 'S3',
    "Broomhall": 'S3',
    "Moorfields": 'S3',
    "Hollis Croft": 'S3',

    // Brightside area
    "Brightside": 'S4',
    "Attercliffe": 'S9',
    "Attercliffe-cum-Darnall": 'S9',
    "Darnall": 'S9',

    // North Sheffield
    "St. Philip's": 'S6',

    // West Sheffield
    "St. George's": 'S10',
    "Crookes": 'S10',
    "Fulwood": 'S10',
    "Broomhill": 'S10',
    "Gillcar": 'S10',

    // South West
    "Ecclesall": 'S11',
    "Sharrow": 'S11'
};

console.log('Parish to Postcode Mapping:');
console.log('---------------------------');
Object.entries(parishToPostcode).forEach(([parish, postcode]) => {
    console.log('  ' + parish.padEnd(30) + ' -> ' + postcode);
});
console.log('\n');

// First, let's see what parishes we actually have
console.log('Analyzing parishes in database...\n');
const parishes = db.prepare('SELECT DISTINCT ecclesiastical_parish, COUNT(*) as count FROM sheffield_players WHERE ecclesiastical_parish IS NOT NULL GROUP BY ecclesiastical_parish ORDER BY count DESC').all();

console.log('Parishes found in database:');
console.log('---------------------------');
parishes.forEach(p => {
    console.log('  ' + (p.ecclesiastical_parish || 'NULL').padEnd(40) + ' ' + p.count.toLocaleString());
});
console.log('\n');

// Update players with postcodes based on parish
let updated = 0;
let alreadyHadPostcode = 0;
let noParish = 0;
let unmatchedParishes = new Set();

const players = db.prepare('SELECT id, ecclesiastical_parish, postcode FROM sheffield_players').all();

console.log('Total players: ' + players.length.toLocaleString() + '\n');
console.log('Processing...\n');

for (const player of players) {
    if (!player.ecclesiastical_parish) {
        noParish++;
        continue;
    }

    if (player.postcode) {
        alreadyHadPostcode++;
        continue;
    }

    // Try to match the parish
    let matched = false;
    for (const [parish, postcode] of Object.entries(parishToPostcode)) {
        if (player.ecclesiastical_parish.includes(parish) || parish.includes(player.ecclesiastical_parish)) {
            db.prepare('UPDATE sheffield_players SET postcode = ? WHERE id = ?')
                .run(postcode, player.id);
            updated++;
            matched = true;
            break;
        }
    }

    if (!matched) {
        unmatchedParishes.add(player.ecclesiastical_parish);
    }
}

console.log('\n========================================');
console.log('RESULTS');
console.log('========================================');
console.log('Already had postcode:     ' + alreadyHadPostcode.toLocaleString());
console.log('Updated with postcode:    ' + updated.toLocaleString());
console.log('No parish data:           ' + noParish.toLocaleString());
console.log('Total players:            ' + players.length.toLocaleString());

if (unmatchedParishes.size > 0) {
    console.log('\nUnmatched parishes:');
    unmatchedParishes.forEach(p => console.log('  - ' + p));
}

// Show final statistics
const withPostcode = db.prepare('SELECT COUNT(*) as c FROM sheffield_players WHERE postcode IS NOT NULL').get();
const withoutPostcode = db.prepare('SELECT COUNT(*) as c FROM sheffield_players WHERE postcode IS NULL').get();

console.log('\n========================================');
console.log('FINAL POSTCODE COVERAGE');
console.log('========================================');
console.log('With postcode:    ' + withPostcode.c.toLocaleString() + ' (' + (withPostcode.c / players.length * 100).toFixed(1) + '%)');
console.log('Without postcode: ' + withoutPostcode.c.toLocaleString() + ' (' + (withoutPostcode.c / players.length * 100).toFixed(1) + '%)');
console.log('========================================\n');

// Show breakdown by postcode
console.log('Players by Postcode:');
console.log('-------------------');
const byPostcode = db.prepare('SELECT postcode, COUNT(*) as count FROM sheffield_players WHERE postcode IS NOT NULL GROUP BY postcode ORDER BY postcode').all();

byPostcode.forEach(row => {
    console.log('  ' + row.postcode + ': ' + row.count.toLocaleString());
});

console.log('\n');
db.close();
