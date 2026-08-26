const sqlite3 = require('better-sqlite3');
const fs = require('fs');

const db = new sqlite3('Sheffield1867.db', { timeout: 30000 });

console.log('='.repeat(70));
console.log('UNIQUE-ONLY Genealogy Matching (Name + Birth Year + Parish)');
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

// Filter to records with address OR profession (and NOT already matched via spouse)
const relevantRecords = allGenRecords.filter(r =>
    (r.Address?.trim() || r.Profession?.trim())
);

console.log(`Records with data to import: ${relevantRecords.length}\n`);

// Get census people WITHOUT addresses/professions (haven't been matched yet)
console.log('Loading unmatched census records...');
const unmatchedPeople = db.prepare(`
    SELECT
        id, name, first_name, surname, birth_year,
        ecclesiastical_parish, civil_parish,
        street_address, profession
    FROM sheffield_people
    WHERE surname IS NOT NULL
`).all();

console.log(`Census records to potentially match: ${unmatchedPeople.length}\n`);

// Build multi-level index: surname -> parish -> birth_year -> [people]
console.log('Building uniqueness index...');
const index = {};

unmatchedPeople.forEach(person => {
    const surname = person.surname.toLowerCase().trim();
    const parish = (person.ecclesiastical_parish || person.civil_parish || '').toLowerCase().trim();
    const year = person.birth_year;

    if (!index[surname]) index[surname] = {};
    if (!index[surname][parish]) index[surname][parish] = {};
    if (!index[surname][parish][year]) index[surname][parish][year] = [];

    index[surname][parish][year].push(person);
});

console.log('Index built!\n');

let uniqueMatches = 0;
let skippedDuplicates = 0;
let addressUpdates = 0;
let professionUpdates = 0;

const updatePerson = db.prepare(`
    UPDATE sheffield_people
    SET street_address = COALESCE(street_address, ?),
        profession = COALESCE(profession, ?)
    WHERE id = ?
`);

console.log('Starting unique-only matching...\n');

// Process each genealogy record
for (const genRecord of relevantRecords) {
    const genName = genRecord.Name?.trim();
    if (!genName) continue;

    // Extract surname
    const nameParts = genName.split(/\s+/);
    const genSurname = nameParts[nameParts.length - 1].toLowerCase();

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

    if (!genBirthYear) continue;

    // Get parish from genealogy
    const genParish = (genRecord.Parish || genRecord.Area || '').toLowerCase().trim();
    if (!genParish) continue;

    // Check if surname exists in index
    if (!index[genSurname]) continue;

    // Check if parish exists for this surname
    if (!index[genSurname][genParish]) continue;

    // Check if birth year exists for this surname+parish
    const candidates = index[genSurname][genParish][genBirthYear];
    if (!candidates) continue;

    // UNIQUENESS CHECK: Only match if there's EXACTLY ONE person
    if (candidates.length !== 1) {
        skippedDuplicates++;
        if (skippedDuplicates <= 10) {
            console.log(`✗ SKIPPED (${candidates.length} duplicates): ${genName} (${genBirthYear}) in ${genRecord.Parish}`);
        }
        continue;
    }

    // We have a UNIQUE match!
    const person = candidates[0];

    const genAddress = genRecord.Address?.trim();
    const genProfession = genRecord.Profession?.trim();

    const hadAddress = person.street_address ? true : false;
    const hadProfession = person.profession ? true : false;

    updatePerson.run(
        genAddress || null,
        genProfession || null,
        person.id
    );

    uniqueMatches++;
    if (genAddress && !hadAddress) addressUpdates++;
    if (genProfession && !hadProfession) professionUpdates++;

    if (uniqueMatches <= 20) {
        console.log(`✓ UNIQUE: ${person.name} (${person.birth_year}) in ${person.ecclesiastical_parish || person.civil_parish}`);
        console.log(`  Address: ${genAddress || 'N/A'}`);
        console.log(`  Profession: ${genProfession || 'N/A'}`);
        console.log('');
    }
}

console.log('='.repeat(70));
console.log('Unique-Only Matching Complete!');
console.log('='.repeat(70));
console.log(`\nUnique matches: ${uniqueMatches}`);
console.log(`  New addresses: ${addressUpdates}`);
console.log(`  New professions: ${professionUpdates}`);
console.log(`\nSkipped due to duplicates: ${skippedDuplicates}`);

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
console.log('\n✓ Unique-only matching complete!');
