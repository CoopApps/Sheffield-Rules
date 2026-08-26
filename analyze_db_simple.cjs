const Database = require('better-sqlite3');
const db = new Database('./Sheffield1867.db', { readonly: true });

console.log('================================================================================');
console.log('SHEFFIELD1867.DB ANALYSIS FOR SHEFFIELD-HALLAMSHIRE LEAGUE MODE');
console.log('================================================================================\n');

// Get all tables
const tables = db.prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name").all();
const tableNames = tables.map(t => t.name);

console.log(`Total tables in database: ${tables.length}\n`);

// Required tables
const requiredTables = [
    'sheffield_clubs',
    'sheffield_league_clubs',
    'sheffield_league_divisions',
    'sheffield_players',
    'sheffield_game_state',
    'sheffield_standings',
    'sheffield_fixtures',
    'sheffield_matches'
];

console.log('REQUIRED TABLES CHECK:');
console.log('--------------------------------------------------------------------------------');
requiredTables.forEach(tableName => {
    const exists = tableNames.includes(tableName);
    console.log(`  ${exists ? '✓' : '✗'} ${tableName}`);
});

// Check sheffield_clubs
console.log('\n================================================================================');
console.log('SHEFFIELD_CLUBS TABLE:');
console.log('================================================================================');
if (tableNames.includes('sheffield_clubs')) {
    const clubCount = db.prepare('SELECT COUNT(*) as count FROM sheffield_clubs').get();
    console.log(`Total clubs: ${clubCount.count}`);

    const sampleClubs = db.prepare('SELECT id, name, founded_year FROM sheffield_clubs ORDER BY founded_year LIMIT 10').all();
    console.log('\nFirst 10 clubs by founding year:');
    sampleClubs.forEach(c => console.log(`  - ${c.name} (${c.id}) - Founded: ${c.founded_year}`));

    const reserveCount = db.prepare("SELECT COUNT(*) as count FROM sheffield_clubs WHERE id LIKE '%-reserves'").get();
    console.log(`\nReserve teams: ${reserveCount.count}`);
    console.log(`Main teams: ${clubCount.count - reserveCount.count}`);
} else {
    console.log('✗ TABLE DOES NOT EXIST!');
}

// Check sheffield_league_divisions
console.log('\n================================================================================');
console.log('SHEFFIELD_LEAGUE_DIVISIONS TABLE:');
console.log('================================================================================');
if (tableNames.includes('sheffield_league_divisions')) {
    const divisions = db.prepare('SELECT id, name, level, region FROM sheffield_league_divisions ORDER BY level, id').all();
    console.log(`Total divisions: ${divisions.length}\n`);

    if (divisions.length > 0) {
        console.log('Division structure:');
        divisions.forEach(d => {
            const region = d.region ? ` (${d.region})` : '';
            console.log(`  Level ${d.level}: ${d.name} (${d.id})${region}`);
        });
    } else {
        console.log('⚠ NO DIVISIONS DEFINED!');
    }
} else {
    console.log('✗ TABLE DOES NOT EXIST!');
}

// Check sheffield_league_clubs
console.log('\n================================================================================');
console.log('SHEFFIELD_LEAGUE_CLUBS TABLE (CRITICAL FOR FANTASY LEAGUE):');
console.log('================================================================================');
if (tableNames.includes('sheffield_league_clubs')) {
    const leagueClubCount = db.prepare('SELECT COUNT(*) as count FROM sheffield_league_clubs').get();
    console.log(`Clubs assigned to league: ${leagueClubCount.count}`);

    if (leagueClubCount.count > 0) {
        // Group by division
        const divisionCounts = db.prepare(`
            SELECT division_id, COUNT(*) as count
            FROM sheffield_league_clubs
            GROUP BY division_id
            ORDER BY division_id
        `).all();

        console.log('\nClubs per division:');
        divisionCounts.forEach(d => console.log(`  ${d.division_id}: ${d.count} clubs`));

        // Check for reserve teams
        const reserveTeamsInLeague = db.prepare('SELECT COUNT(*) as count FROM sheffield_league_clubs WHERE is_reserve_team = 1').get();
        console.log(`\nReserve teams in league: ${reserveTeamsInLeague.count}`);
        console.log(`Main teams in league: ${leagueClubCount.count - reserveTeamsInLeague.count}`);

        // Sample assignments
        const sampleAssignments = db.prepare(`
            SELECT slc.club_id, slc.division_id, slc.is_reserve_team, slc.position_in_division, sc.name
            FROM sheffield_league_clubs slc
            LEFT JOIN sheffield_clubs sc ON slc.club_id = sc.id
            WHERE slc.division_id = '1'
            ORDER BY slc.position_in_division
            LIMIT 16
        `).all();

        console.log('\nFirst Division clubs:');
        sampleAssignments.forEach((c, idx) => {
            const reserveFlag = c.is_reserve_team ? ' (RESERVE)' : '';
            const position = c.position_in_division || (idx + 1);
            console.log(`  ${position}. ${c.name || c.club_id}${reserveFlag}`);
        });
    } else {
        console.log('\n⚠ NO CLUBS ASSIGNED TO LEAGUE!');
    }
} else {
    console.log('✗ TABLE DOES NOT EXIST!');
}

// Check sheffield_players
console.log('\n================================================================================');
console.log('SHEFFIELD_PLAYERS TABLE:');
console.log('================================================================================');
if (tableNames.includes('sheffield_players')) {
    const playerCount = db.prepare('SELECT COUNT(*) as count FROM sheffield_players').get();
    console.log(`Total players: ${playerCount.count}`);

    if (playerCount.count > 0) {
        const samplePlayers = db.prepare('SELECT id, name, club_id, position FROM sheffield_players LIMIT 5').all();
        console.log('\nSample players:');
        samplePlayers.forEach(p => console.log(`  - ${p.name} (${p.club_id || 'Unassigned'}) - ${p.position || 'No position'}`));
    }
} else {
    console.log('✗ TABLE DOES NOT EXIST!');
}

// Final assessment
console.log('\n================================================================================');
console.log('FINAL ASSESSMENT:');
console.log('================================================================================\n');

const missingTables = requiredTables.filter(t => !tableNames.includes(t));

if (missingTables.length > 0) {
    console.log('❌ DATABASE IS INCOMPLETE\n');
    console.log('Missing required tables:');
    missingTables.forEach(t => console.log(`  - ${t}`));
} else {
    console.log('✓ All required tables exist\n');

    // Check if league structure is populated
    if (tableNames.includes('sheffield_league_clubs')) {
        const leagueClubCount = db.prepare('SELECT COUNT(*) as count FROM sheffield_league_clubs').get();
        if (leagueClubCount.count > 0) {
            console.log('✓ League structure is populated with clubs');

            if (tableNames.includes('sheffield_league_divisions')) {
                const divCount = db.prepare('SELECT COUNT(*) as count FROM sheffield_league_divisions').get();
                if (divCount.count > 0) {
                    console.log('✓ League divisions are defined');
                    console.log('\n✅ DATABASE IS READY FOR SHEFFIELD-HALLAMSHIRE LEAGUE MODE\n');
                } else {
                    console.log('✗ League divisions are NOT defined');
                    console.log('\n⚠ DATABASE NEEDS DIVISION STRUCTURE');
                }
            }
        } else {
            console.log('✗ League structure exists but is EMPTY');
            console.log('\n❌ DATABASE NEEDS CLUBS TO BE ASSIGNED TO DIVISIONS');
        }
    }
}

console.log('================================================================================');

db.close();
