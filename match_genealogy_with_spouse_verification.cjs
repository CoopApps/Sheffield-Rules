const sqlite3 = require('better-sqlite3');
const fs = require('fs');
const path = require('path');

const db = new sqlite3('Sheffield1867.db', { timeout: 30000 });

console.log('='.repeat(70));
console.log('Matching genealogy data with SPOUSE VERIFICATION');
console.log('='.repeat(70) + '\n');

// Parse CSV (handle quoted fields with commas)
function parseCSV(content) {
    const lines = content.split('\n');
    if (lines.length < 2) return [];

    const headers = lines[0].split(',').map(h => h.trim());
    const rows = [];

    for (let i = 1; i < lines.length; i++) {
        const line = lines[i].trim();
        if (!line) continue;

        const values = [];
        let current = '';
        let inQuotes = false;

        for (let j = 0; j < line.length; j++) {
            const char = line[j];
            if (char === '"') {
                inQuotes = !inQuotes;
            } else if (char === ',' && !inQuotes) {
                values.push(current.trim());
                current = '';
            } else {
                current += char;
            }
        }
        values.push(current.trim());

        const row = {};
        headers.forEach((h, idx) => {
            row[h] = values[idx] || '';
        });
        rows.push(row);
    }

    return rows;
}

// Load all genealogy records
console.log('Loading genealogy files...');
const genealogyFiles = fs.readdirSync('.').filter(f => f.startsWith('tg_') && f.endsWith('.csv'));
console.log(`Found ${genealogyFiles.length} genealogy files\n`);

const allGenRecords = [];
genealogyFiles.forEach(file => {
    const content = fs.readFileSync(file, 'utf-8');
    const records = parseCSV(content);
    allGenRecords.push(...records);
});

console.log(`Loaded ${allGenRecords.length} total genealogy records\n`);

// Count records with spouse information
const recordsWithSpouse = allGenRecords.filter(r => r.Spouse && r.Spouse.trim() !== '');
console.log(`Records with spouse information: ${recordsWithSpouse.length}\n`);

// Function to check if a name appears in household members
function isSpouseInHousehold(spouseName, householdMembers) {
    if (!spouseName || !householdMembers) return false;

    // Extract first name and surname from spouse name
    const spouseParts = spouseName.trim().split(/\s+/);
    const spouseFirstName = spouseParts[0];
    const spouseSurname = spouseParts[spouseParts.length - 1];

    const household = householdMembers.toLowerCase();

    // Check if both first name and surname appear in household
    const hasFirst = spouseFirstName && household.includes(spouseFirstName.toLowerCase());
    const hasSurname = spouseSurname && household.includes(spouseSurname.toLowerCase());

    return hasFirst && hasSurname;
}

// Function to normalize name for matching
function normalizeName(name) {
    if (!name) return '';
    return name.toLowerCase().trim().replace(/\s+/g, ' ');
}

// Get all people from census with household members
const censusPeople = db.prepare(`
    SELECT
        id, name, first_name, surname, birth_year, gender,
        census_household_members, street_address, profession,
        ecclesiastical_parish
    FROM sheffield_people
    WHERE census_household_members IS NOT NULL
    AND census_household_members != ''
`).all();

console.log(`Census people with household members: ${censusPeople.length}\n`);

let totalMatches = 0;
let spouseVerified = 0;
let nameOnlyMatches = 0;
let addressUpdates = 0;
let professionUpdates = 0;

const updatePerson = db.prepare(`
    UPDATE sheffield_people
    SET street_address = COALESCE(street_address, ?),
        profession = COALESCE(profession, ?)
    WHERE id = ?
`);

console.log('Starting spouse-verified matching...\n');

// For each genealogy record with a spouse
for (const genRecord of recordsWithSpouse) {
    const genName = normalizeName(genRecord.Name);
    const genSpouse = normalizeName(genRecord.Spouse);
    const genAddress = genRecord.Address?.trim();
    const genProfession = genRecord.Profession?.trim();

    if (!genName || !genSpouse) continue;

    // Try to parse birth year from Age/Born Approx
    let genBirthYear = null;
    if (genRecord['Born Approx']) {
        genBirthYear = parseInt(genRecord['Born Approx']);
    } else if (genRecord.Age) {
        // Assuming census year is 1871
        const age = parseInt(genRecord.Age);
        if (!isNaN(age)) {
            genBirthYear = 1871 - age;
        }
    }

    // Find potential matches in census
    for (const person of censusPeople) {
        const censusName = normalizeName(person.name);

        // Name must match (at least surname)
        const genSurname = genName.split(/\s+/).pop();
        const censusSurname = normalizeName(person.surname);

        if (genSurname !== censusSurname) continue;

        // Birth year should be close (within 3 years tolerance)
        if (genBirthYear && person.birth_year) {
            const yearDiff = Math.abs(genBirthYear - person.birth_year);
            if (yearDiff > 3) continue;
        }

        // SPOUSE VERIFICATION: Check if spouse appears in household members
        const spouseMatches = isSpouseInHousehold(genRecord.Spouse, person.census_household_members);

        if (spouseMatches) {
            // HIGH CONFIDENCE MATCH - spouse verified!
            totalMatches++;
            spouseVerified++;

            // Update address and profession
            const hadAddress = person.street_address ? true : false;
            const hadProfession = person.profession ? true : false;

            updatePerson.run(
                genAddress || null,
                genProfession || null,
                person.id
            );

            if (genAddress && !hadAddress) addressUpdates++;
            if (genProfession && !hadProfession) professionUpdates++;

            if (spouseVerified <= 15) {
                console.log(`✓ SPOUSE VERIFIED: ${person.name} (${person.birth_year})`);
                console.log(`  Spouse: ${genRecord.Spouse} found in household`);
                console.log(`  Address: ${genAddress || 'N/A'}`);
                console.log(`  Profession: ${genProfession || 'N/A'}`);
                console.log('');
            }

            break; // Found verified match, move to next genealogy record
        }
    }
}

console.log('\n' + '='.repeat(70));
console.log('Spouse-Verified Matching Complete!');
console.log('='.repeat(70));
console.log(`\nTotal spouse-verified matches: ${spouseVerified}`);
console.log(`  New addresses added: ${addressUpdates}`);
console.log(`  New professions added: ${professionUpdates}`);

// Copy to sheffield_players
console.log('\nCopying data to sheffield_players...');
const copiedAddress = db.prepare(`
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
        WHERE player_id IS NOT NULL
        AND street_address IS NOT NULL
    )
`).run();

const copiedProfession = db.prepare(`
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
        WHERE player_id IS NOT NULL
        AND profession IS NOT NULL
    )
`).run();

console.log(`  Addresses copied to ${copiedAddress.changes} players`);
console.log(`  Professions copied to ${copiedProfession.changes} players`);

// Final statistics
const peopleStats = db.prepare(`
    SELECT
        COUNT(*) as total,
        COUNT(street_address) as with_address,
        COUNT(profession) as with_profession
    FROM sheffield_people
`).get();

const playerStats = db.prepare(`
    SELECT
        COUNT(*) as total,
        COUNT(street_address) as with_address,
        COUNT(profession) as with_profession
    FROM sheffield_players
`).get();

console.log('\nFinal Statistics:');
console.log('  sheffield_people:');
console.log(`    Total: ${peopleStats.total}`);
console.log(`    With addresses: ${peopleStats.with_address} (${(peopleStats.with_address / peopleStats.total * 100).toFixed(1)}%)`);
console.log(`    With professions: ${peopleStats.with_profession} (${(peopleStats.with_profession / peopleStats.total * 100).toFixed(1)}%)`);
console.log('  sheffield_players:');
console.log(`    Total: ${playerStats.total}`);
console.log(`    With addresses: ${playerStats.with_address} (${(playerStats.with_address / playerStats.total * 100).toFixed(1)}%)`);
console.log(`    With professions: ${playerStats.with_profession} (${(playerStats.with_profession / playerStats.total * 100).toFixed(1)}%)`);

db.close();
console.log('\n✓ Spouse-verified genealogy matching complete!');
