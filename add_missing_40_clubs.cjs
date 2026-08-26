const Database = require('better-sqlite3');
const db = new Database('./Sheffield1867.db');

console.log('================================================================================');
console.log('ADDING 40 MISSING CLUBS TO SHEFFIELD-HALLAMSHIRE LEAGUE');
console.log('================================================================================\n');

// Get current state
const before = db.prepare('SELECT COUNT(*) as count FROM sheffield_league_clubs').get();
console.log(`Clubs in league BEFORE: ${before.count}\n`);

// Get all unassigned clubs
const unassigned = db.prepare(`
    SELECT id, name, founded_year
    FROM sheffield_clubs
    WHERE id NOT IN (SELECT club_id FROM sheffield_league_clubs)
    ORDER BY founded_year, name
`).all();

console.log(`Found ${unassigned.length} unassigned clubs\n`);

// Prepare insert statement
const insert = db.prepare(`
    INSERT INTO sheffield_league_clubs (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
    VALUES (?, ?, ?, ?, ?, ?)
`);

// Distribution strategy: Fill lower divisions evenly
const divisions = {
    'div-7a': { current: 10, target: 15, clubs: [] },
    'div-7b': { current: 10, target: 15, clubs: [] },
    'div-6a': { current: 13, target: 15, clubs: [] },
    'div-6b': { current: 13, target: 15, clubs: [] },
    'div-6c': { current: 13, target: 14, clubs: [] },
    'res-div-7a': { current: 10, target: 15, clubs: [] },
    'res-div-7b': { current: 10, target: 15, clubs: [] },
    'res-div-6a': { current: 13, target: 15, clubs: [] },
    'res-div-6b': { current: 13, target: 15, clubs: [] },
    'res-div-6c': { current: 13, target: 14, clubs: [] },
};

let addedCount = 0;

// Separate main and reserve teams
const mainTeams = unassigned.filter(c => !c.id.includes('-reserves'));
const reserveTeams = unassigned.filter(c => c.id.includes('-reserves'));

console.log(`Main teams to add: ${mainTeams.length}`);
console.log(`Reserve teams to add: ${reserveTeams.length}\n`);

// Distribute main teams
const mainDivisions = ['div-7a', 'div-7b', 'div-6a', 'div-6b', 'div-6c'];
let mainDivIndex = 0;

for (const club of mainTeams) {
    const divId = mainDivisions[mainDivIndex % mainDivisions.length];
    const div = divisions[divId];

    if (div.clubs.length < (div.target - div.current)) {
        div.clubs.push(club);
        mainDivIndex++;
    }
}

// Distribute reserve teams
const reserveDivisions = ['res-div-7a', 'res-div-7b', 'res-div-6a', 'res-div-6b', 'res-div-6c'];
let resDivIndex = 0;

for (const club of reserveTeams) {
    const divId = reserveDivisions[resDivIndex % reserveDivisions.length];
    const div = divisions[divId];

    if (div.clubs.length < (div.target - div.current)) {
        div.clubs.push(club);
        resDivIndex++;
    }
}

// Insert clubs
console.log('Adding clubs to divisions:\n');

db.prepare('BEGIN TRANSACTION').run();

try {
    for (const [divId, div] of Object.entries(divisions)) {
        if (div.clubs.length === 0) continue;

        console.log(`${divId}: Adding ${div.clubs.length} clubs`);

        for (let i = 0; i < div.clubs.length; i++) {
            const club = div.clubs[i];
            const position = div.current + i + 1;
            const isReserve = club.id.includes('-reserves') ? 1 : 0;
            const parentClub = isReserve ? club.id.replace('-reserves', '') : null;
            const leagueId = `league-${club.id}`;

            insert.run(leagueId, divId, club.id, position, isReserve, parentClub);
            addedCount++;

            console.log(`  Pos ${position}: ${club.name}`);
        }
        console.log();
    }

    db.prepare('COMMIT').run();
    console.log('✅ Transaction committed\n');
} catch (error) {
    db.prepare('ROLLBACK').run();
    console.error('❌ Error - transaction rolled back:', error.message);
    process.exit(1);
}

// Verify
const after = db.prepare('SELECT COUNT(*) as count FROM sheffield_league_clubs').get();
console.log('================================================================================');
console.log('RESULTS:');
console.log('================================================================================\n');
console.log(`Clubs in league BEFORE: ${before.count}`);
console.log(`Clubs added: ${addedCount}`);
console.log(`Clubs in league AFTER: ${after.count}\n`);

if (after.count === 372) {
    console.log('✅ SUCCESS! All 372 clubs are now in the league\n');
} else {
    console.log(`⚠ WARNING: Expected 372 clubs, got ${after.count}\n`);
}

// Show updated division sizes
console.log('Updated division sizes:');
const divSizes = db.prepare(`
    SELECT division_id, COUNT(*) as count
    FROM sheffield_league_clubs
    GROUP BY division_id
    ORDER BY division_id
`).all();

divSizes.forEach(d => {
    console.log(`  ${d.division_id}: ${d.count} clubs`);
});

console.log('\n================================================================================\n');

db.close();
