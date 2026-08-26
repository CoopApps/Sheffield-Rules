const Database = require('better-sqlite3');
const db = new Database('./Sheffield1867.db', { readonly: true });

console.log('================================================================================');
console.log('SHEFFIELD1867.DB - SOURCE OF TRUTH CHECK');
console.log('================================================================================\n');

// What the code queries at initialization (commands.rs:661-666)
console.log('STEP 1: What clubs are in the league?');
console.log('Code queries: SELECT club_id FROM sheffield_league_clubs');
console.log('--------------------------------------------------------------------------------');

try {
    const leagueClubs = db.prepare('SELECT club_id, division_id, position_in_division, is_reserve_team FROM sheffield_league_clubs ORDER BY division_id, position_in_division').all();

    if (leagueClubs.length === 0) {
        console.log('❌ EMPTY! No clubs assigned to league.');
        console.log('Result: Game will initialize with 0 clubs in standings\n');
    } else {
        console.log(`✓ Found ${leagueClubs.length} club assignments\n`);

        // Group by division
        const byDivision = {};
        leagueClubs.forEach(c => {
            if (!byDivision[c.division_id]) byDivision[c.division_id] = [];
            byDivision[c.division_id].push(c);
        });

        console.log('Distribution across divisions:');
        Object.keys(byDivision).sort().forEach(divId => {
            console.log(`  ${divId}: ${byDivision[divId].length} clubs`);
        });

        console.log('\nSample from Division 1:');
        const div1 = byDivision['1'] || [];
        div1.slice(0, 5).forEach(c => {
            const reserve = c.is_reserve_team ? ' (RESERVE)' : '';
            console.log(`  Pos ${c.position_in_division}: ${c.club_id}${reserve}`);
        });
    }
} catch (e) {
    console.log(`❌ ERROR: ${e.message}`);
    console.log('Table may not exist!\n');
}

// What club data exists (commands.rs:687-691)
console.log('\n================================================================================');
console.log('STEP 2: What clubs exist in the database?');
console.log('Code queries: SELECT id FROM sheffield_clubs ORDER BY name');
console.log('--------------------------------------------------------------------------------');

try {
    const allClubs = db.prepare('SELECT id, name, founded_year FROM sheffield_clubs ORDER BY founded_year, name').all();
    console.log(`✓ Found ${allClubs.length} total clubs\n`);

    const reserves = allClubs.filter(c => c.id.includes('-reserves'));
    console.log(`  Main clubs: ${allClubs.length - reserves.length}`);
    console.log(`  Reserve teams: ${reserves.length}`);

    console.log('\nFirst 10 clubs by founding:');
    allClubs.slice(0, 10).forEach(c => {
        console.log(`  ${c.founded_year}: ${c.name} (${c.id})`);
    });
} catch (e) {
    console.log(`❌ ERROR: ${e.message}\n`);
}

// What divisions are defined (for league structure)
console.log('\n================================================================================');
console.log('STEP 3: What league structure is defined?');
console.log('Used by: Frontend display, promotion/relegation, cup eligibility');
console.log('--------------------------------------------------------------------------------');

try {
    const divisions = db.prepare('SELECT id, name, level, region FROM sheffield_league_divisions ORDER BY level, id').all();
    console.log(`✓ Found ${divisions.length} divisions\n`);

    const byLevel = {};
    divisions.forEach(d => {
        if (!byLevel[d.level]) byLevel[d.level] = [];
        byLevel[d.level].push(d);
    });

    Object.keys(byLevel).sort((a, b) => a - b).forEach(level => {
        const divs = byLevel[level];
        console.log(`  Level ${level}: ${divs.length} division(s)`);
        divs.forEach(d => {
            const region = d.region ? ` (${d.region})` : '';
            console.log(`    - ${d.id}: ${d.name}${region}`);
        });
    });
} catch (e) {
    console.log(`❌ ERROR: ${e.message}\n`);
}

// Check if game state tables are clear (they should be empty)
console.log('\n================================================================================');
console.log('STEP 4: Are game state tables clear?');
console.log('These should be EMPTY at initialization (get populated during gameplay)');
console.log('--------------------------------------------------------------------------------');

const stateTables = [
    'sheffield_game_state',
    'sheffield_standings',
    'sheffield_fixtures',
    'sheffield_matches'
];

stateTables.forEach(table => {
    try {
        const count = db.prepare(`SELECT COUNT(*) as count FROM ${table}`).get();
        const status = count.count === 0 ? '✓ Empty (correct)' : `⚠ Has ${count.count} rows (should be empty)`;
        console.log(`  ${table}: ${status}`);
    } catch (e) {
        console.log(`  ${table}: ✗ Does not exist`);
    }
});

// Check players
console.log('\n================================================================================');
console.log('STEP 5: Player data (optional - can be generated)');
console.log('--------------------------------------------------------------------------------');

try {
    const playerCount = db.prepare('SELECT COUNT(*) as count FROM sheffield_players').get();
    console.log(`  sheffield_players: ${playerCount.count} players`);

    if (playerCount.count > 0) {
        const withClubs = db.prepare('SELECT COUNT(*) as count FROM sheffield_players WHERE club_id IS NOT NULL').get();
        console.log(`    Assigned to clubs: ${withClubs.count}`);
        console.log(`    Free agents: ${playerCount.count - withClubs.count}`);
    } else {
        console.log('    (Will be procedurally generated if empty)');
    }
} catch (e) {
    console.log(`  sheffield_players: ✗ Does not exist (will generate)`);
}

// FINAL VERDICT
console.log('\n================================================================================');
console.log('DATABASE AS SOURCE OF TRUTH - FINAL ANALYSIS');
console.log('================================================================================\n');

let isValid = true;
const issues = [];

// Check 1: League assignments
try {
    const leagueClubs = db.prepare('SELECT COUNT(*) as count FROM sheffield_league_clubs').get();
    if (leagueClubs.count === 0) {
        isValid = false;
        issues.push('❌ CRITICAL: sheffield_league_clubs is EMPTY - no clubs will be in the league');
    } else if (leagueClubs.count !== 372) {
        issues.push(`⚠ WARNING: Expected 372 club assignments, found ${leagueClubs.count}`);
    } else {
        console.log('✓ League assignments: 372 clubs properly assigned to divisions');
    }
} catch (e) {
    isValid = false;
    issues.push('❌ CRITICAL: sheffield_league_clubs table does not exist');
}

// Check 2: Clubs exist
try {
    const clubs = db.prepare('SELECT COUNT(*) as count FROM sheffield_clubs').get();
    if (clubs.count === 0) {
        isValid = false;
        issues.push('❌ CRITICAL: sheffield_clubs is EMPTY - no clubs defined');
    } else {
        console.log(`✓ Clubs defined: ${clubs.count} clubs available`);
    }
} catch (e) {
    isValid = false;
    issues.push('❌ CRITICAL: sheffield_clubs table does not exist');
}

// Check 3: Divisions exist
try {
    const divisions = db.prepare('SELECT COUNT(*) as count FROM sheffield_league_divisions').get();
    if (divisions.count === 0) {
        isValid = false;
        issues.push('❌ CRITICAL: sheffield_league_divisions is EMPTY - no pyramid structure');
    } else {
        console.log(`✓ League structure: ${divisions.count} divisions defined`);
    }
} catch (e) {
    isValid = false;
    issues.push('❌ CRITICAL: sheffield_league_divisions table does not exist');
}

console.log('\n' + '='.repeat(80));

if (issues.length > 0) {
    console.log('\nISSUES FOUND:\n');
    issues.forEach(issue => console.log(issue));
}

console.log('\n' + '='.repeat(80));

if (isValid && issues.length === 0) {
    console.log('\n✅ DATABASE IS VALID - Ready for Sheffield-Hallamshire League mode');
    console.log('\nThe game will use this database as the SOLE SOURCE OF TRUTH for:');
    console.log('  - Which clubs exist (sheffield_clubs)');
    console.log('  - Which divisions exist (sheffield_league_divisions)');
    console.log('  - Which clubs are in which divisions (sheffield_league_clubs)');
    console.log('  - Initial standings will be generated from sheffield_league_clubs');
    console.log('  - Fixtures will be generated based on divisional assignments\n');
} else {
    console.log('\n❌ DATABASE IS NOT READY');
    console.log('\nThe game REQUIRES a properly populated database to function.');
    console.log('Without it, the league will be empty or broken.\n');
}

console.log('='.repeat(80) + '\n');

db.close();
