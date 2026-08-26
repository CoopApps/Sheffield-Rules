const Database = require('better-sqlite3');
const path = require('path');

const dbPath = path.join(__dirname, 'Sheffield1867.db');

console.log('='.repeat(80));
console.log('SHEFFIELD1867.DB ANALYSIS');
console.log('='.repeat(80));

try {
    const db = new Database(dbPath, { readonly: true });

    console.log('\n✓ Database file opened successfully\n');

    // Get all tables
    const tables = db.prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name").all();

    console.log(`Total tables: ${tables.length}\n`);
    console.log('TABLES IN DATABASE:');
    console.log('-'.repeat(80));
    tables.forEach(t => console.log(`  - ${t.name}`));

    // Required tables for Sheffield-Hallamshire League mode
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

    console.log('\n' + '='.repeat(80));
    console.log('REQUIRED TABLES CHECK:');
    console.log('='.repeat(80));

    const existingTables = tables.map(t => t.name);
    requiredTables.forEach(tableName => {
        const exists = existingTables.includes(tableName);
        console.log(`  ${exists ? '✓' : '✗'} ${tableName}`);
    });

    // Check sheffield_clubs table
    console.log('\n' + '='.repeat(80));
    console.log('SHEFFIELD_CLUBS TABLE:');
    console.log('='.repeat(80));

    if (existingTables.includes('sheffield_clubs')) {
        const clubCount = db.prepare('SELECT COUNT(*) as count FROM sheffield_clubs').get();
        console.log(`  Total clubs: ${clubCount.count}`);

        const sampleClubs = db.prepare('SELECT id, name, founded_year FROM sheffield_clubs LIMIT 10').all();
        console.log('\n  Sample clubs:');
        sampleClubs.forEach(c => console.log(`    - ${c.name} (${c.id}) - Founded: ${c.founded_year}`));

        // Count reserves
        const reserveCount = db.prepare("SELECT COUNT(*) as count FROM sheffield_clubs WHERE id LIKE '%-reserves' OR name LIKE '%Reserves%' OR name LIKE '%Second%' OR name LIKE '%Junior%' OR name LIKE '%B Team%' OR name LIKE '%Colts%'").get();
        console.log(`\n  Reserve teams: ${reserveCount.count}`);
        console.log(`  Main teams: ${clubCount.count - reserveCount.count}`);
    } else {
        console.log('  ✗ Table does not exist!');
    }

    // Check sheffield_league_clubs table
    console.log('\n' + '='.repeat(80));
    console.log('SHEFFIELD_LEAGUE_CLUBS TABLE:');
    console.log('='.repeat(80));

    if (existingTables.includes('sheffield_league_clubs')) {
        const leagueClubCount = db.prepare('SELECT COUNT(*) as count FROM sheffield_league_clubs').get();
        console.log(`  Clubs assigned to league: ${leagueClubCount.count}`);

        // Group by division
        const divisionCounts = db.prepare(`
            SELECT division_id, COUNT(*) as count
            FROM sheffield_league_clubs
            GROUP BY division_id
            ORDER BY division_id
        `).all();

        console.log('\n  Clubs per division:');
        divisionCounts.forEach(d => console.log(`    - ${d.division_id}: ${d.count} clubs`));

        // Check for reserve teams
        const reserveTeamsInLeague = db.prepare('SELECT COUNT(*) as count FROM sheffield_league_clubs WHERE is_reserve_team = 1').get();
        console.log(`\n  Reserve teams in league: ${reserveTeamsInLeague.count}`);

        // Sample clubs
        const sampleLeagueClubs = db.prepare(`
            SELECT slc.club_id, slc.division_id, slc.is_reserve_team, sc.name
            FROM sheffield_league_clubs slc
            LEFT JOIN sheffield_clubs sc ON slc.club_id = sc.id
            LIMIT 10
        `).all();

        console.log('\n  Sample league assignments:');
        sampleLeagueClubs.forEach(c => {
            const reserveFlag = c.is_reserve_team ? ' (RESERVE)' : '';
            console.log(`    - ${c.name || c.club_id} → ${c.division_id}${reserveFlag}`);
        });
    } else {
        console.log('  ✗ Table does not exist!');
        console.log('  ⚠ This is CRITICAL - the league structure is missing!');
    }

    // Check sheffield_league_divisions table
    console.log('\n' + '='.repeat(80));
    console.log('SHEFFIELD_LEAGUE_DIVISIONS TABLE:');
    console.log('='.repeat(80));

    if (existingTables.includes('sheffield_league_divisions')) {
        const divisions = db.prepare('SELECT id, name, level, region FROM sheffield_league_divisions ORDER BY level, id').all();
        console.log(`  Total divisions: ${divisions.length}\n`);

        console.log('  Division structure:');
        divisions.forEach(d => {
            const region = d.region ? ` (${d.region})` : '';
            console.log(`    - Level ${d.level}: ${d.name} (${d.id})${region}`);
        });
    } else {
        console.log('  ✗ Table does not exist!');
        console.log('  ⚠ This is CRITICAL - the league structure is missing!');
    }

    // Check sheffield_players table
    console.log('\n' + '='.repeat(80));
    console.log('SHEFFIELD_PLAYERS TABLE:');
    console.log('='.repeat(80));

    if (existingTables.includes('sheffield_players')) {
        const playerCount = db.prepare('SELECT COUNT(*) as count FROM sheffield_players').get();
        console.log(`  Total players: ${playerCount.count}`);

        if (playerCount.count > 0) {
            const samplePlayers = db.prepare('SELECT id, name, club_id, position FROM sheffield_players LIMIT 10').all();
            console.log('\n  Sample players:');
            samplePlayers.forEach(p => console.log(`    - ${p.name} (${p.club_id}) - ${p.position}`));
        } else {
            console.log('  ⚠ No players in database');
        }
    } else {
        console.log('  ✗ Table does not exist!');
    }

    // Final assessment
    console.log('\n' + '='.repeat(80));
    console.log('ASSESSMENT FOR SHEFFIELD-HALLAMSHIRE LEAGUE MODE:');
    console.log('='.repeat(80));

    const missingTables = requiredTables.filter(t => !existingTables.includes(t));

    if (missingTables.length === 0) {
        console.log('\n✓ All required tables exist\n');

        // Check if league structure is populated
        if (existingTables.includes('sheffield_league_clubs')) {
            const leagueClubCount = db.prepare('SELECT COUNT(*) as count FROM sheffield_league_clubs').get();
            if (leagueClubCount.count > 0) {
                console.log('✓ League structure is populated with clubs');
                console.log('\n✅ DATABASE IS READY FOR SHEFFIELD-HALLAMSHIRE LEAGUE MODE');
            } else {
                console.log('✗ League structure exists but is EMPTY');
                console.log('\n❌ DATABASE NEEDS LEAGUE CLUBS TO BE ASSIGNED');
            }
        }
    } else {
        console.log('\n✗ Missing required tables:');
        missingTables.forEach(t => console.log(`    - ${t}`));
        console.log('\n❌ DATABASE IS NOT READY - MISSING TABLES');
    }

    console.log('\n' + '='.repeat(80));

    db.close();
} catch (error) {
    console.error('\n✗ Error analyzing database:', error.message);
    console.error('\nFull error:', error);
}
