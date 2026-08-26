const sqlite3 = require('better-sqlite3');

const db = new sqlite3('Sheffield1867.db', { timeout: 30000 });

console.log('='.repeat(70));
console.log('Linking sheffield_players to sheffield_people');
console.log('='.repeat(70) + '\n');

// Get all players that don't have a link yet
console.log('Finding players without people links...');
const unlinkedPlayers = db.prepare(`
    SELECT id, name, first_name, middle_name, surname, birth_year,
           birth_town, birth_county, ecclesiastical_parish
    FROM sheffield_players
    WHERE id NOT IN (
        SELECT DISTINCT player_id FROM sheffield_people WHERE player_id IS NOT NULL
    )
`).all();

console.log(`Found ${unlinkedPlayers.length} unlinked players\n`);

// Build index of people by surname
console.log('Building people index...');
const people = db.prepare(`
    SELECT id, name, first_name, surname, birth_year, player_id,
           birth_town, birth_county, ecclesiastical_parish, civil_parish
    FROM sheffield_people
    WHERE surname IS NOT NULL
`).all();

const bySurname = {};
people.forEach(person => {
    const surname = person.surname.toLowerCase().trim();
    if (!bySurname[surname]) bySurname[surname] = [];
    bySurname[surname].push(person);
});

console.log(`Indexed ${people.length} people by ${Object.keys(bySurname).length} surnames\n`);

let exactMatches = 0;
let closeMatches = 0;
let multipleMatches = 0;

const updatePersonLink = db.prepare(`
    UPDATE sheffield_people
    SET player_id = ?,
        is_player = 1
    WHERE id = ?
`);

console.log('Matching players to people...\n');

for (const player of unlinkedPlayers) {
    if (!player.surname) continue;

    const playerSurname = player.surname.toLowerCase().trim();
    const candidates = bySurname[playerSurname];

    if (!candidates || candidates.length === 0) continue;

    // Filter by birth year (exact match or within 1 year)
    let matches = candidates.filter(p =>
        p.birth_year === player.birth_year
    );

    // If no exact year match, try within 1 year
    if (matches.length === 0) {
        matches = candidates.filter(p =>
            p.birth_year && Math.abs(p.birth_year - player.birth_year) <= 1
        );
    }

    if (matches.length === 0) continue;

    // If multiple matches, try to narrow down by first name
    if (matches.length > 1 && player.first_name) {
        const firstNameMatches = matches.filter(p =>
            p.first_name &&
            p.first_name.toLowerCase().trim() === player.first_name.toLowerCase().trim()
        );

        if (firstNameMatches.length > 0) {
            matches = firstNameMatches;
        }
    }

    // If STILL multiple matches, try additional criteria
    if (matches.length > 1) {
        // Try parish match
        if (player.ecclesiastical_parish) {
            const parishMatches = matches.filter(p =>
                (p.ecclesiastical_parish && p.ecclesiastical_parish === player.ecclesiastical_parish) ||
                (p.civil_parish && p.civil_parish === player.ecclesiastical_parish)
            );
            if (parishMatches.length > 0) {
                matches = parishMatches;
            }
        }

        // If STILL multiple, try birth place
        if (matches.length > 1) {
            if (player.birth_town) {
                const townMatches = matches.filter(p =>
                    p.birth_town && p.birth_town.toLowerCase() === player.birth_town.toLowerCase()
                );
                if (townMatches.length > 0) {
                    matches = townMatches;
                }
            }

            if (matches.length > 1 && player.birth_county) {
                const countyMatches = matches.filter(p =>
                    p.birth_county && p.birth_county.toLowerCase() === player.birth_county.toLowerCase()
                );
                if (countyMatches.length > 0) {
                    matches = countyMatches;
                }
            }
        }
    }

    // Only accept if we narrowed it down to 1 unique match
    if (matches.length > 1) {
        multipleMatches++;
        continue; // Skip - too ambiguous
    }

    // If we have exactly one match (or narrowed down to one)
    if (matches.length === 1) {
        const person = matches[0];

        // Skip if this person is already linked to a different player
        if (person.player_id && person.player_id !== player.id) {
            continue;
        }

        updatePersonLink.run(player.id, person.id);

        if (player.birth_year === person.birth_year) {
            exactMatches++;
        } else {
            closeMatches++;
        }

        if (exactMatches + closeMatches <= 20) {
            console.log(`✓ ${player.name} (${player.birth_year}) → ${person.name}`);
            if (player.birth_year !== person.birth_year) {
                console.log(`  (birth year difference: ${Math.abs(player.birth_year - person.birth_year)})`);
            }
        }
    } else if (matches.length > 1) {
        multipleMatches++;
    }
}

console.log('\n' + '='.repeat(70));
console.log('Player Linking Complete!');
console.log('='.repeat(70));
console.log(`\nExact matches (same birth year): ${exactMatches}`);
console.log(`Close matches (1 year difference): ${closeMatches}`);
console.log(`Skipped (multiple matches): ${multipleMatches}`);
console.log(`Total linked: ${exactMatches + closeMatches}`);

// Now copy data FROM people TO players for newly linked players
console.log('\nCopying data from sheffield_people to sheffield_players...');

const copiedAddr = db.prepare(`
    UPDATE sheffield_players
    SET street_address = (
        SELECT street_address FROM sheffield_people
        WHERE sheffield_people.player_id = sheffield_players.id
        AND sheffield_people.street_address IS NOT NULL
        LIMIT 1
    )
    WHERE street_address IS NULL
    AND id IN (
        SELECT player_id FROM sheffield_people
        WHERE player_id IS NOT NULL AND street_address IS NOT NULL
    )
`).run();

const copiedProf = db.prepare(`
    UPDATE sheffield_players
    SET profession = (
        SELECT profession FROM sheffield_people
        WHERE sheffield_people.player_id = sheffield_players.id
        AND sheffield_people.profession IS NOT NULL
        LIMIT 1
    )
    WHERE profession IS NULL
    AND id IN (
        SELECT player_id FROM sheffield_people
        WHERE player_id IS NOT NULL AND profession IS NOT NULL
    )
`).run();

const copiedPostcode = db.prepare(`
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
        WHERE player_id IS NOT NULL AND postcode IS NOT NULL
    )
`).run();

console.log(`  Addresses: ${copiedAddr.changes} players`);
console.log(`  Professions: ${copiedProf.changes} players`);
console.log(`  Postcodes: ${copiedPostcode.changes} players`);

// Final statistics
const peopleLinked = db.prepare(`
    SELECT COUNT(*) as count
    FROM sheffield_people
    WHERE player_id IS NOT NULL
`).get();

const playerStats = db.prepare(`
    SELECT
        COUNT(*) as total,
        COUNT(street_address) as with_address,
        COUNT(profession) as with_profession,
        COUNT(postcode) as with_postcode
    FROM sheffield_players
`).get();

console.log('\nFinal Statistics:');
console.log(`  sheffield_people linked to players: ${peopleLinked.count}`);
console.log('  sheffield_players:');
console.log(`    Total: ${playerStats.total}`);
console.log(`    With addresses: ${playerStats.with_address} (${(playerStats.with_address / playerStats.total * 100).toFixed(1)}%)`);
console.log(`    With professions: ${playerStats.with_profession} (${(playerStats.with_profession / playerStats.total * 100).toFixed(1)}%)`);
console.log(`    With postcodes: ${playerStats.with_postcode} (${(playerStats.with_postcode / playerStats.total * 100).toFixed(1)}%)`);

db.close();
console.log('\n✓ Player linking complete!');
