const sqlite3 = require('better-sqlite3');

const db = new sqlite3('Sheffield1867.db', { timeout: 30000 });

console.log('='.repeat(70));
console.log('Matching Household Members to Addresses');
console.log('='.repeat(70) + '\n');

// Get all people WITH addresses who have household members
console.log('Loading household heads with addresses...');
const householdHeads = db.prepare(`
    SELECT
        id, name, first_name, surname, birth_year,
        census_piece, census_folio, census_household_schedule,
        census_household_members, street_address, profession,
        ecclesiastical_parish, postcode
    FROM sheffield_people
    WHERE street_address IS NOT NULL
    AND census_household_members IS NOT NULL
    AND census_household_members != ''
    AND census_piece IS NOT NULL
    AND census_folio IS NOT NULL
`).all();

console.log(`Found ${householdHeads.length} household heads with addresses\n`);

// Get all people WITHOUT addresses
console.log('Loading people without addresses...');
const unmatchedPeople = db.prepare(`
    SELECT
        id, name, first_name, surname, birth_year,
        census_piece, census_folio, census_household_schedule,
        street_address, profession, postcode
    FROM sheffield_people
    WHERE street_address IS NULL
    AND census_piece IS NOT NULL
    AND census_folio IS NOT NULL
    AND surname IS NOT NULL
`).all();

console.log(`Found ${unmatchedPeople.length} people without addresses\n`);

// Build index: piece -> folio -> schedule -> household head
console.log('Building household index...');
const householdIndex = {};

householdHeads.forEach(head => {
    const key = `${head.census_piece}|${head.census_folio}|${head.census_household_schedule}`;
    if (!householdIndex[key]) {
        householdIndex[key] = [];
    }
    householdIndex[key].push(head);
});

console.log(`Indexed ${Object.keys(householdIndex).length} unique households\n`);

// Common name abbreviations
const nameAbbreviations = {
    'william': ['wm', 'will', 'bill'],
    'thomas': ['thos', 'tom'],
    'james': ['jas', 'jim', 'jimmy'],
    'john': ['jno', 'jack'],
    'joseph': ['jos', 'joe'],
    'robert': ['robt', 'rob', 'bob'],
    'richard': ['richd', 'dick'],
    'charles': ['chas', 'charlie'],
    'edward': ['edw', 'ed', 'ted'],
    'henry': ['hy', 'harry', 'hal'],
    'george': ['geo'],
    'benjamin': ['benj', 'ben'],
    'samuel': ['saml', 'sam'],
    'elizabeth': ['eliz', 'lizzie', 'betty', 'liz'],
    'margaret': ['margt', 'maggie', 'meg'],
    'catherine': ['cath', 'kate'],
    'ann': ['anne', 'annie'],
    'sarah': ['sally']
};

// Function to get all variations of a name
function getNameVariations(name) {
    if (!name) return [];

    const normalized = name.toLowerCase().trim();
    const variations = [normalized];

    // Add abbreviations if it's a full name
    if (nameAbbreviations[normalized]) {
        variations.push(...nameAbbreviations[normalized]);
    }

    // Add full name if it's an abbreviation
    for (const [fullName, abbrevs] of Object.entries(nameAbbreviations)) {
        if (abbrevs.includes(normalized)) {
            variations.push(fullName);
        }
    }

    return variations;
}

// Function to check if a person's name appears in household members
function isInHousehold(personName, householdMembers) {
    if (!personName || !householdMembers) return false;

    const household = householdMembers.toLowerCase();
    const name = personName.toLowerCase().trim();

    // Try exact match first
    if (household.includes(name)) return true;

    // Try all name variations
    const variations = getNameVariations(name);
    for (const variant of variations) {
        if (household.includes(variant)) return true;
    }

    // Try just the first part of the name (ignore middle names)
    const firstPart = name.split(/\s+/)[0];
    if (firstPart !== name) {
        if (household.includes(firstPart)) return true;

        // Try variations of first part
        const firstPartVariations = getNameVariations(firstPart);
        for (const variant of firstPartVariations) {
            if (household.includes(variant)) return true;
        }
    }

    return false;
}

let matched = 0;
let addressUpdates = 0;
let professionUpdates = 0;
let postcodeUpdates = 0;

const updatePerson = db.prepare(`
    UPDATE sheffield_people
    SET street_address = COALESCE(street_address, ?),
        profession = COALESCE(profession, ?),
        postcode = COALESCE(postcode, ?)
    WHERE id = ?
`);

console.log('Matching household members...\n');

// For each unmatched person
for (const person of unmatchedPeople) {
    const householdKey = `${person.census_piece}|${person.census_folio}|${person.census_household_schedule}`;

    // Find matching household heads
    const heads = householdIndex[householdKey];
    if (!heads || heads.length === 0) continue;

    // Check each head in this household
    for (const head of heads) {
        // Must have same surname
        if (person.surname.toLowerCase() !== head.surname.toLowerCase()) continue;

        // Check if person's name appears in household members
        if (!isInHousehold(person.first_name || person.name, head.census_household_members)) continue;

        // MATCH! Assign the household head's address
        const hadAddress = person.street_address ? true : false;
        const hadProfession = person.profession ? true : false;
        const hadPostcode = person.postcode ? true : false;

        updatePerson.run(
            head.street_address,
            head.profession, // Don't assign profession to children
            head.postcode,
            person.id
        );

        matched++;
        if (!hadAddress) addressUpdates++;
        if (!hadProfession && head.profession) professionUpdates++;
        if (!hadPostcode && head.postcode) postcodeUpdates++;

        if (matched <= 20) {
            console.log(`✓ ${person.name} (${person.birth_year}) → household of ${head.name}`);
            console.log(`  Address: ${head.street_address}`);
            console.log(`  Postcode: ${head.postcode || 'N/A'}`);
            console.log('');
        }

        break; // Found match, move to next person
    }
}

console.log('='.repeat(70));
console.log('Household Matching Complete!');
console.log('='.repeat(70));
console.log(`\nMatched household members: ${matched}`);
console.log(`  New addresses: ${addressUpdates}`);
console.log(`  New professions: ${professionUpdates}`);
console.log(`  New postcodes: ${postcodeUpdates}`);

// Copy to players
console.log('\nCopying to sheffield_players...');
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

console.log(`  ${copiedAddr.changes} players got addresses`);
console.log(`  ${copiedProf.changes} players got professions`);
console.log(`  ${copiedPostcode.changes} players got postcodes`);

// Final stats
const peopleStats = db.prepare(`
    SELECT
        COUNT(*) as total,
        COUNT(street_address) as with_address,
        COUNT(profession) as with_profession,
        COUNT(postcode) as with_postcode
    FROM sheffield_people
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
console.log('  sheffield_people:');
console.log(`    Total: ${peopleStats.total}`);
console.log(`    Addresses: ${peopleStats.with_address} (${(peopleStats.with_address / peopleStats.total * 100).toFixed(1)}%)`);
console.log(`    Professions: ${peopleStats.with_profession} (${(peopleStats.with_profession / peopleStats.total * 100).toFixed(1)}%)`);
console.log(`    Postcodes: ${peopleStats.with_postcode} (${(peopleStats.with_postcode / peopleStats.total * 100).toFixed(1)}%)`);
console.log('  sheffield_players:');
console.log(`    Total: ${playerStats.total}`);
console.log(`    Addresses: ${playerStats.with_address} (${(playerStats.with_address / playerStats.total * 100).toFixed(1)}%)`);
console.log(`    Professions: ${playerStats.with_profession} (${(playerStats.with_profession / playerStats.total * 100).toFixed(1)}%)`);
console.log(`    Postcodes: ${playerStats.with_postcode} (${(playerStats.with_postcode / playerStats.total * 100).toFixed(1)}%)`);

db.close();
console.log('\n✓ Household member matching complete!');
