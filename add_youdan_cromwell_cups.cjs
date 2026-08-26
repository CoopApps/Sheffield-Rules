const Database = require('better-sqlite3');
const db = new Database('./Sheffield1867.db');

console.log('================================================================================');
console.log('ADDING YOUDAN CUP (1867) AND CROMWELL CUP (1868) TO SHEFFIELD1867.DB');
console.log('================================================================================\n');

// Check if tables exist
const tables = db.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name IN ('sheffield_competitions', 'sheffield_cup_ties', 'sheffield_competition_participants')").all();

console.log(`Found ${tables.length} competition tables:`);
tables.forEach(t => console.log(`  ✓ ${t.name}`));

if (tables.length < 3) {
    console.log('\n⚠️  WARNING: Not all competition tables exist. Schema may need to be updated.\n');
}

// Start transaction
db.prepare('BEGIN TRANSACTION').run();

try {
    console.log('\n================================================================================');
    console.log('YOUDAN CUP 1867');
    console.log('================================================================================\n');

    // Historical context:
    // - Held January-March 1867
    // - First ever football cup competition
    // - 12 teams entered
    // - Winner: Hallam FC
    // - Runner-up: Norfolk FC
    // - Divisions 1-3 (top 48 clubs)

    console.log('Inserting Youdan Cup 1867...');

    const youdanInsert = db.prepare(`
        INSERT OR REPLACE INTO sheffield_competitions (
            id, name, competition_type, season,
            min_division_level, max_division_level,
            current_round, total_rounds, is_active,
            winner_club_id, runner_up_club_id,
            rules_type, prestige_level,
            start_week, announcement_week, draw_week
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    `);

    youdanInsert.run(
        'youdan-cup-1867',              // id
        'Youdan Cup',                   // name
        'knockout_cup',                 // competition_type
        1867,                           // season
        1,                              // min_division_level (Divisions 1-3)
        3,                              // max_division_level
        0,                              // current_round (not started)
        4,                              // total_rounds (12 teams = R1, R2, SF, F)
        1,                              // is_active
        null,                           // winner_club_id (to be determined)
        null,                           // runner_up_club_id
        'sheffield_rules',              // rules_type
        'high',                         // prestige_level (first ever cup!)
        6,                              // start_week (early February)
        2,                              // announcement_week (late January)
        4                               // draw_week (week before start)
    );

    console.log('✓ Youdan Cup 1867 inserted');
    console.log('  - Open to: Divisions 1-3 (top 48 clubs)');
    console.log('  - Announcement: Week 2 (late January)');
    console.log('  - Draw: Week 4');
    console.log('  - Start: Week 6 (early February)');
    console.log('  - Format: 12-team knockout (4 rounds)');
    console.log('  - Prestige: HIGH (world\'s first football cup!)');
    console.log('  - Historical winner: Hallam FC');
    console.log('  - Historical runner-up: Norfolk FC');

    console.log('\n================================================================================');
    console.log('CROMWELL CUP 1868');
    console.log('================================================================================\n');

    // Historical context:
    // - Held February 1868
    // - Second ever football cup competition
    // - Only open to clubs founded within 2 years (1866+)
    // - 4 teams entered: Wednesday, Garrick, Exchange, Wellington
    // - Winner: Sheffield Wednesday FC
    // - Runner-up: Garrick FC
    // - For newer clubs (Divisions 4-7)

    console.log('Inserting Cromwell Cup 1868...');

    const cromwellInsert = db.prepare(`
        INSERT OR REPLACE INTO sheffield_competitions (
            id, name, competition_type, season,
            min_division_level, max_division_level,
            current_round, total_rounds, is_active,
            winner_club_id, runner_up_club_id,
            rules_type, prestige_level,
            start_week, announcement_week, draw_week
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    `);

    cromwellInsert.run(
        'cromwell-cup-1868',            // id
        'Cromwell Cup',                 // name
        'knockout_cup',                 // competition_type
        1867,                           // season (starts in 1867 season, held Feb 1868)
        4,                              // min_division_level (Divisions 4-7)
        7,                              // max_division_level
        0,                              // current_round (not started)
        2,                              // total_rounds (4 teams = SF, F)
        1,                              // is_active
        null,                           // winner_club_id (to be determined)
        null,                           // runner_up_club_id
        'sheffield_rules',              // rules_type
        'standard',                     // prestige_level
        30,                             // start_week (early February 1868)
        26,                             // announcement_week (January 1868)
        28                              // draw_week
    );

    console.log('✓ Cromwell Cup 1868 inserted');
    console.log('  - Open to: Divisions 4-7 (lower 324 clubs)');
    console.log('  - Announcement: Week 26 (January 1868)');
    console.log('  - Draw: Week 28');
    console.log('  - Start: Week 30 (February 1868)');
    console.log('  - Format: 4-team knockout (2 rounds: SF, F)');
    console.log('  - Prestige: STANDARD');
    console.log('  - Historical winner: Sheffield Wednesday FC');
    console.log('  - Historical runner-up: Garrick FC');
    console.log('  - Special rule: Originally open only to clubs under 2 years old');

    // Commit transaction
    db.prepare('COMMIT').run();
    console.log('\n✅ Transaction committed\n');

} catch (error) {
    db.prepare('ROLLBACK').run();
    console.error('❌ Error - transaction rolled back:', error.message);
    console.error(error.stack);
    process.exit(1);
}

// Verify results
console.log('================================================================================');
console.log('VERIFICATION:');
console.log('================================================================================\n');

const competitions = db.prepare(`
    SELECT id, name, season, min_division_level, max_division_level,
           prestige_level, start_week, announcement_week
    FROM sheffield_competitions
    ORDER BY season, start_week
`).all();

console.log(`Total competitions in database: ${competitions.length}\n`);

for (const comp of competitions) {
    console.log(`${comp.name} (${comp.season})`);
    console.log(`  ID: ${comp.id}`);
    console.log(`  Divisions: ${comp.min_division_level}-${comp.max_division_level}`);
    console.log(`  Prestige: ${comp.prestige_level}`);
    console.log(`  Announces: Week ${comp.announcement_week}, Starts: Week ${comp.start_week}`);
    console.log();
}

console.log('================================================================================');
console.log('SUMMARY:');
console.log('================================================================================\n');

console.log('✓ Youdan Cup 1867 added for top divisions (1-3)');
console.log('✓ Cromwell Cup 1868 added for lower divisions (4-7)');
console.log('\nBoth competitions will now be loaded when you start a Sheffield & Hallamshire');
console.log('Fantasy League game in 1867.\n');

console.log('WHAT HAPPENS IN GAME:');
console.log('  Week 2:  📰 Youdan Cup announced');
console.log('  Week 4:  🎲 Youdan Cup draw made');
console.log('  Week 6:  ⚽ Youdan Cup matches begin');
console.log('  Week 26: 📰 Cromwell Cup announced');
console.log('  Week 28: 🎲 Cromwell Cup draw made');
console.log('  Week 30: ⚽ Cromwell Cup matches begin\n');

console.log('Historical note:');
console.log('  The Youdan Cup (1867) preceded the FA Cup by 4 years and is the world\'s');
console.log('  oldest football trophy. The actual trophy is now valued at £350,000 and');
console.log('  held by Hallam FC, who won the original competition.\n');

console.log('================================================================================\n');

db.close();
