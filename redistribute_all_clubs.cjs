const Database = require('better-sqlite3');
const db = new Database('./Sheffield1867.db');

console.log('================================================================================');
console.log('REDISTRIBUTING ALL 372 CLUBS - 16 PER DIVISION FROM TOP DOWN');
console.log('================================================================================\n');

// Backup check
console.log('⚠️  WARNING: This will DELETE all current league assignments and rebuild them!\n');
console.log('Starting redistribution NOW...\n');

// Get current state
const beforeCount = db.prepare('SELECT COUNT(*) as count FROM sheffield_league_clubs').get();
console.log(`Current assignments: ${beforeCount.count}\n`);

// Get all clubs, separated by main and reserve
const mainClubs = db.prepare(`
    SELECT id, name, founded_year
    FROM sheffield_clubs
    WHERE id NOT LIKE '%-reserves'
    ORDER BY founded_year, name
`).all();

const reserveClubs = db.prepare(`
    SELECT id, name, founded_year, parent_club_id
    FROM sheffield_clubs
    WHERE id LIKE '%-reserves'
    ORDER BY founded_year, name
`).all();

console.log(`Main clubs: ${mainClubs.length}`);
console.log(`Reserve clubs: ${reserveClubs.length}`);
console.log(`Total: ${mainClubs.length + reserveClubs.length}\n`);

// Division structure (matching the parallel pyramid structure)
const divisionStructure = [
    // Level 1
    { id: 'div-1', name: 'First Division', level: 1, type: 'main', capacity: 16 },
    { id: 'res-div-1', name: 'Reserve Division 1', level: 1, type: 'reserve', capacity: 16 },

    // Level 2
    { id: 'div-2', name: 'Second Division', level: 2, type: 'main', capacity: 16 },
    { id: 'res-div-2', name: 'Reserve Division 2', level: 2, type: 'reserve', capacity: 16 },

    // Level 3
    { id: 'div-3', name: 'Third Division', level: 3, type: 'main', capacity: 16 },
    { id: 'res-div-3', name: 'Reserve Division 3', level: 3, type: 'reserve', capacity: 16 },

    // Level 4
    { id: 'div-4', name: 'Fourth Division', level: 4, type: 'main', capacity: 16 },
    { id: 'res-div-4', name: 'Reserve Division 4', level: 4, type: 'reserve', capacity: 16 },

    // Level 5
    { id: 'div-5a', name: 'Fifth Division A', level: 5, type: 'main', capacity: 13 },
    { id: 'div-5b', name: 'Fifth Division B', level: 5, type: 'main', capacity: 13 },
    { id: 'res-div-5a', name: 'Reserve Division 5A', level: 5, type: 'reserve', capacity: 13 },
    { id: 'res-div-5b', name: 'Reserve Division 5B', level: 5, type: 'reserve', capacity: 13 },

    // Level 6
    { id: 'div-6a', name: 'Sixth Division West', level: 6, type: 'main', capacity: 12 },
    { id: 'div-6b', name: 'Sixth Division East', level: 6, type: 'main', capacity: 12 },
    { id: 'div-6c', name: 'Sixth Division North', level: 6, type: 'main', capacity: 12 },
    { id: 'div-6d', name: 'Sixth Division South', level: 6, type: 'main', capacity: 12 },
    { id: 'res-div-6a', name: 'Reserve Division 6A', level: 6, type: 'reserve', capacity: 12 },
    { id: 'res-div-6b', name: 'Reserve Division 6B', level: 6, type: 'reserve', capacity: 12 },
    { id: 'res-div-6c', name: 'Reserve Division 6C', level: 6, type: 'reserve', capacity: 12 },
    { id: 'res-div-6d', name: 'Reserve Division 6D', level: 6, type: 'reserve', capacity: 12 },

    // Level 7
    { id: 'div-7a', name: 'Seventh Division West', level: 7, type: 'main', capacity: 12 },
    { id: 'div-7b', name: 'Seventh Division East', level: 7, type: 'main', capacity: 12 },
    { id: 'div-7c', name: 'Seventh Division North', level: 7, type: 'main', capacity: 12 },
    { id: 'div-7d', name: 'Seventh Division South', level: 7, type: 'main', capacity: 12 },
    { id: 'res-div-7a', name: 'Reserve Division 7A', level: 7, type: 'reserve', capacity: 12 },
    { id: 'res-div-7b', name: 'Reserve Division 7B', level: 7, type: 'reserve', capacity: 12 },
    { id: 'res-div-7c', name: 'Reserve Division 7C', level: 7, type: 'reserve', capacity: 12 },
    { id: 'res-div-7d', name: 'Reserve Division 7D', level: 7, type: 'reserve', capacity: 12 },
];

console.log('Target distribution:');
console.log('  Levels 1-4: 16 clubs per division');
console.log('  Level 5: 13 clubs per division');
console.log('  Levels 6-7: 12 clubs per division\n');

// Calculate totals
const mainDivisions = divisionStructure.filter(d => d.type === 'main');
const reserveDivisions = divisionStructure.filter(d => d.type === 'reserve');
const mainCapacity = mainDivisions.reduce((sum, d) => sum + d.capacity, 0);
const reserveCapacity = reserveDivisions.reduce((sum, d) => sum + d.capacity, 0);

console.log(`Main pyramid capacity: ${mainCapacity}`);
console.log(`Reserve pyramid capacity: ${reserveCapacity}`);
console.log(`Total capacity: ${mainCapacity + reserveCapacity}\n`);

// Start transaction
db.prepare('BEGIN TRANSACTION').run();

try {
    // Clear existing assignments
    console.log('Clearing existing assignments...');
    db.prepare('DELETE FROM sheffield_league_clubs').run();
    console.log('✓ Cleared\n');

    // Distribute main clubs
    console.log('Distributing MAIN clubs:\n');
    let mainClubIndex = 0;
    const insertStmt = db.prepare(`
        INSERT INTO sheffield_league_clubs (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
        VALUES (?, ?, ?, ?, 0, NULL)
    `);

    for (const division of mainDivisions) {
        console.log(`  ${division.id} (${division.name}):`);

        for (let pos = 1; pos <= division.capacity && mainClubIndex < mainClubs.length; pos++) {
            const club = mainClubs[mainClubIndex];
            const leagueId = `${club.id}-${division.id}`;

            insertStmt.run(leagueId, division.id, club.id, pos);

            if (pos <= 3 || pos === division.capacity) {
                console.log(`    ${pos}. ${club.name}`);
            } else if (pos === 4) {
                console.log(`    ... (${division.capacity - 3} more clubs)`);
            }

            mainClubIndex++;
        }
        console.log(`    Total: ${Math.min(division.capacity, mainClubs.length - mainClubIndex + division.capacity)} clubs\n`);
    }

    // Distribute reserve clubs
    console.log('Distributing RESERVE clubs:\n');
    let reserveClubIndex = 0;
    const insertReserveStmt = db.prepare(`
        INSERT INTO sheffield_league_clubs (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
        VALUES (?, ?, ?, ?, 1, ?)
    `);

    for (const division of reserveDivisions) {
        console.log(`  ${division.id} (${division.name}):`);

        for (let pos = 1; pos <= division.capacity && reserveClubIndex < reserveClubs.length; pos++) {
            const club = reserveClubs[reserveClubIndex];
            const parentClub = club.id.replace('-reserves', '');
            const leagueId = `${club.id}-${division.id}`;

            insertReserveStmt.run(leagueId, division.id, club.id, pos, parentClub);

            if (pos <= 3 || pos === division.capacity) {
                console.log(`    ${pos}. ${club.name}`);
            } else if (pos === 4) {
                console.log(`    ... (${division.capacity - 3} more clubs)`);
            }

            reserveClubIndex++;
        }
        console.log(`    Total: ${Math.min(division.capacity, reserveClubs.length - reserveClubIndex + division.capacity)} clubs\n`);
    }

    // Commit transaction
    db.prepare('COMMIT').run();
    console.log('✅ Transaction committed\n');

} catch (error) {
    db.prepare('ROLLBACK').run();
    console.error('❌ Error - transaction rolled back:', error.message);
    process.exit(1);
}

// Verify results
console.log('================================================================================');
console.log('VERIFICATION:');
console.log('================================================================================\n');

const afterCount = db.prepare('SELECT COUNT(*) as count FROM sheffield_league_clubs').get();
const unassigned = db.prepare(`
    SELECT COUNT(*) as count
    FROM sheffield_clubs
    WHERE id NOT IN (SELECT club_id FROM sheffield_league_clubs)
`).get();

console.log(`Clubs assigned: ${afterCount.count}`);
console.log(`Clubs unassigned: ${unassigned.count}`);
console.log(`Total clubs in database: ${mainClubs.length + reserveClubs.length}\n`);

if (afterCount.count === mainClubs.length + reserveClubs.length) {
    console.log('✅ SUCCESS! All 372 clubs are now assigned to divisions\n');
} else {
    console.log(`⚠️  WARNING: Expected 372 assignments, got ${afterCount.count}\n`);
}

// Show division sizes
console.log('Final distribution by division:\n');
const divSizes = db.prepare(`
    SELECT division_id, COUNT(*) as count
    FROM sheffield_league_clubs
    GROUP BY division_id
    ORDER BY division_id
`).all();

let currentLevel = 0;
for (const div of divSizes) {
    const divInfo = divisionStructure.find(d => d.id === div.division_id);
    if (divInfo && divInfo.level !== currentLevel) {
        if (currentLevel > 0) console.log();
        console.log(`Level ${divInfo.level}:`);
        currentLevel = divInfo.level;
    }
    console.log(`  ${div.division_id}: ${div.count} clubs`);
}

console.log('\n================================================================================\n');

db.close();
