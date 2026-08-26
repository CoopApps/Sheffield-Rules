const sqlite3 = require('better-sqlite3');
const fs = require('fs');

const db = new sqlite3('Sheffield1867.db', { timeout: 30000 });

console.log('='.repeat(70));
console.log('OPTIMIZED Spouse-Verified Genealogy Matching');
console.log('='.repeat(70) + '\n');

// Parse CSV
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

// Load genealogy records
console.log('Loading genealogy files...');
const genealogyFiles = fs.readdirSync('.').filter(f => f.startsWith('tg_') && f.endsWith('.csv'));
console.log(`Found ${genealogyFiles.length} genealogy files\n`);

const allGenRecords = [];
genealogyFiles.forEach(file => {
    const content = fs.readFileSync(file, 'utf-8');
    const records = parseCSV(content);
    allGenRecords.push(...records);
});

console.log(`Loaded ${allGenRecords.length} total genealogy records`);

// Filter to records with spouse AND (address OR profession)
const relevantRecords = allGenRecords.filter(r =>
    r.Spouse && r.Spouse.trim() !== '' &&
    (r.Address?.trim() || r.Profession?.trim())
);

console.log(`Records with spouse + data: ${relevantRecords.length}\n`);

// Get census people and INDEX BY SURNAME for fast lookup
console.log('Loading and indexing census data...');
const censusPeople = db.prepare(`
    SELECT
        id, name, first_name, surname, birth_year,
        census_household_members, street_address, profession
    FROM sheffield_people
    WHERE census_household_members IS NOT NULL
    AND census_household_members != ''
    AND surname IS NOT NULL
`).all();

// Build surname index
const bySurname = {};
censusPeople.forEach(person => {
    const surname = person.surname.toLowerCase().trim();
    if (!bySurname[surname]) bySurname[surname] = [];
    bySurname[surname].push(person);
});

console.log(`Indexed ${censusPeople.length} census people by surname`);
console.log(`Unique surnames: ${Object.keys(bySurname).length}\n`);

// Spouse checking function
function isSpouseInHousehold(spouseName, householdMembers) {
    if (!spouseName || !householdMembers) return false;

    const spouseParts = spouseName.trim().split(/\s+/);
    const spouseFirstName = spouseParts[0];
    const spouseSurname = spouseParts[spouseParts.length - 1];

    const household = householdMembers.toLowerCase();

    const hasFirst = spouseFirstName && household.includes(spouseFirstName.toLowerCase());
    const hasSurname = spouseSurname && household.includes(spouseSurname.toLowerCase());

    return hasFirst && hasSurname;
}

let spouseVerified = 0;
let addressUpdates = 0;
let professionUpdates = 0;

const updatePerson = db.prepare(`
    UPDATE sheffield_people
    SET street_address = COALESCE(street_address, ?),
        profession = COALESCE(profession, ?)
    WHERE id = ?
`);

console.log('Starting optimized spouse-verified matching...\n');

// Process each genealogy record
for (const genRecord of relevantRecords) {
    const genName = genRecord.Name?.trim();
    if (!genName) continue;

    // Extract surname
    const nameParts = genName.split(/\s+/);
    const genSurname = nameParts[nameParts.length - 1].toLowerCase();

    // Look up census people with this surname (FAST!)
    const candidates = bySurname[genSurname];
    if (!candidates) continue;

    // Parse genealogy birth year
    let genBirthYear = null;
    if (genRecord['Born Approx']) {
        genBirthYear = parseInt(genRecord['Born Approx']);
    } else if (genRecord.Age) {
        const age = parseInt(genRecord.Age);
        if (!isNaN(age)) {
            genBirthYear = 1871 - age;
        }
    }

    // Check each candidate
    for (const person of candidates) {
        // Birth year should be close (within 3 years)
        if (genBirthYear && person.birth_year) {
            const yearDiff = Math.abs(genBirthYear - person.birth_year);
            if (yearDiff > 3) continue;
        }

        // SPOUSE VERIFICATION
        const spouseMatches = isSpouseInHousehold(genRecord.Spouse, person.census_household_members);

        if (spouseMatches) {
            const genAddress = genRecord.Address?.trim();
            const genProfession = genRecord.Profession?.trim();

            const hadAddress = person.street_address ? true : false;
            const hadProfession = person.profession ? true : false;

            updatePerson.run(
                genAddress || null,
                genProfession || null,
                person.id
            );

            spouseVerified++;
            if (genAddress && !hadAddress) addressUpdates++;
            if (genProfession && !hadProfession) professionUpdates++;

            if (spouseVerified <= 20) {
                console.log(`✓ ${person.name} (${person.birth_year})`);
                console.log(`  Spouse: ${genRecord.Spouse}`);
                console.log(`  Address: ${genAddress || 'N/A'}`);
                console.log(`  Profession: ${genProfession || 'N/A'}`);
                console.log('');
            }

            break; // Found match, move to next record
        }
    }
}

console.log('='.repeat(70));
console.log('Matching Complete!');
console.log('='.repeat(70));
console.log(`\nSpouse-verified matches: ${spouseVerified}`);
console.log(`  New addresses: ${addressUpdates}`);
console.log(`  New professions: ${professionUpdates}`);

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

console.log(`  ${copiedAddr.changes} players got addresses`);
console.log(`  ${copiedProf.changes} players got professions`);

// Final stats
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
console.log(`    Addresses: ${peopleStats.with_address} (${(peopleStats.with_address / peopleStats.total * 100).toFixed(1)}%)`);
console.log(`    Professions: ${peopleStats.with_profession} (${(peopleStats.with_profession / peopleStats.total * 100).toFixed(1)}%)`);
console.log('  sheffield_players:');
console.log(`    Total: ${playerStats.total}`);
console.log(`    Addresses: ${playerStats.with_address} (${(playerStats.with_address / playerStats.total * 100).toFixed(1)}%)`);
console.log(`    Professions: ${playerStats.with_profession} (${(playerStats.with_profession / playerStats.total * 100).toFixed(1)}%)`);

db.close();
console.log('\n✓ Spouse-verified matching complete!');
