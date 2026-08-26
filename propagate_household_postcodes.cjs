const sqlite3 = require('better-sqlite3');

const db = new sqlite3('Sheffield1867.db', { timeout: 30000 });

console.log('='.repeat(60));
console.log('Propagating postcodes within households');
console.log('='.repeat(60) + '\n');

// Find households where at least one person has a postcode
const householdsWithPostcodes = db.prepare(`
    SELECT DISTINCT
        census_piece,
        census_folio,
        census_household_schedule
    FROM sheffield_people
    WHERE postcode IS NOT NULL
    AND census_piece IS NOT NULL
    AND census_folio IS NOT NULL
`).all();

console.log(`Found ${householdsWithPostcodes.length} households with at least one postcode\n`);

let totalUpdated = 0;

// For each household, get the postcode(s) and apply to all members without postcodes
for (const household of householdsWithPostcodes) {
    const { census_piece, census_folio, census_household_schedule } = household;

    // Get all postcodes for this household (should be the same, but take the most common one)
    const postcodes = db.prepare(`
        SELECT postcode, COUNT(*) as count
        FROM sheffield_people
        WHERE census_piece = ?
        AND census_folio = ?
        AND census_household_schedule = ?
        AND postcode IS NOT NULL
        GROUP BY postcode
        ORDER BY count DESC
        LIMIT 1
    `).get(census_piece, census_folio, census_household_schedule);

    if (!postcodes) continue;

    const postcode = postcodes.postcode;

    // Update all people in this household who don't have a postcode
    const result = db.prepare(`
        UPDATE sheffield_people
        SET postcode = ?
        WHERE census_piece = ?
        AND census_folio = ?
        AND census_household_schedule = ?
        AND postcode IS NULL
    `).run(postcode, census_piece, census_folio, census_household_schedule);

    if (result.changes > 0) {
        totalUpdated += result.changes;
        if (totalUpdated <= 5) {
            console.log(`✓ Household (Piece: ${census_piece}, Folio: ${census_folio}, Schedule: ${census_household_schedule})`);
            console.log(`  Updated ${result.changes} people with postcode: ${postcode}`);
        }
    }
}

console.log('\n' + '='.repeat(60));
console.log(`Total people updated: ${totalUpdated}`);

// Copy new postcodes to sheffield_players
console.log('\nCopying new postcodes to sheffield_players...');
const copied = db.prepare(`
    UPDATE sheffield_players
    SET postcode = (
        SELECT postcode FROM sheffield_people
        WHERE sheffield_people.player_id = sheffield_players.id
        AND sheffield_people.postcode IS NOT NULL
        LIMIT 1
    )
    WHERE postcode IS NULL
    AND id IN (
        SELECT player_id FROM sheffield_people
        WHERE player_id IS NOT NULL
        AND postcode IS NOT NULL
    )
`).run();

console.log(`✓ Copied postcodes to ${copied.changes} additional players`);

// Final statistics
const peopleStats = db.prepare(`
    SELECT
        COUNT(*) as total,
        COUNT(postcode) as with_postcode,
        COUNT(street_address) as with_address
    FROM sheffield_people
`).get();

const playerStats = db.prepare(`
    SELECT
        COUNT(*) as total,
        COUNT(postcode) as with_postcode
    FROM sheffield_players
`).get();

console.log('\nFinal Statistics:');
console.log(`  sheffield_people:`);
console.log(`    Total: ${peopleStats.total}`);
console.log(`    With addresses: ${peopleStats.with_address}`);
console.log(`    With postcodes: ${peopleStats.with_postcode} (${(peopleStats.with_postcode / peopleStats.total * 100).toFixed(1)}%)`);
console.log(`  sheffield_players:`);
console.log(`    Total: ${playerStats.total}`);
console.log(`    With postcodes: ${playerStats.with_postcode} (${(playerStats.with_postcode / playerStats.total * 100).toFixed(1)}%)`);

db.close();
console.log('\n✓ Household postcode propagation complete!');
