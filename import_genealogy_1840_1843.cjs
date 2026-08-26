const sqlite3 = require('better-sqlite3');
const fs = require('fs');
const path = require('path');

const db = new sqlite3('Sheffield1867.db', { timeout: 30000 });

console.log('='.repeat(70));
console.log('Importing Genealogy 1840-1843 and Matching to Census Data');
console.log('='.repeat(70) + '\n');

// Parse CSV function
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

// Find genealogy files for 1840-1843
const genealogyFolder = path.join('.', 'Genealogy');
const allFiles = fs.readdirSync(genealogyFolder);

const years = ['1840', '1841', '1842', '1843'];
let genealogyRecords = [];

console.log('Loading genealogy files for 1840-1843...\n');

years.forEach(year => {
    const yearFiles = allFiles.filter(f => f.startsWith(`tg_${year}_`) && f.endsWith('.csv'));
    console.log(`  ${year}: ${yearFiles.length} files`);

    yearFiles.forEach(file => {
        const filePath = path.join(genealogyFolder, file);
        const content = fs.readFileSync(filePath, 'utf-8');
        const records = parseCSV(content);
        genealogyRecords = genealogyRecords.concat(records);
    });
});

console.log(`\nTotal genealogy records loaded: ${genealogyRecords.length}\n`);

// Get all unmatched people from census
console.log('Loading unmatched census people...');
const unmatchedPeople = db.prepare(`
    SELECT id, name, first_name, surname, birth_year,
           street_address, profession, census_piece, census_folio,
           census_household_schedule, census_household_members,
           ecclesiastical_parish, civil_parish
    FROM sheffield_people
    WHERE surname IS NOT NULL
    AND birth_year >= 1840 AND birth_year <= 1843
`).all();

console.log(`Found ${unmatchedPeople.length} census people born 1840-1843\n`);

// Build surname index for census people
console.log('Building census index by surname...');
const bySurname = {};
unmatchedPeople.forEach(person => {
    const surname = person.surname.toLowerCase().trim();
    if (!bySurname[surname]) bySurname[surname] = [];
    bySurname[surname].push(person);
});

console.log(`Indexed ${Object.keys(bySurname).length} surnames\n`);

// Name matching helper
const nameAbbreviations = {
    'william': ['wm', 'will', 'bill', 'w'],
    'thomas': ['thos', 'tom', 'tho', 't'],
    'james': ['jas', 'jim', 'jimmy', 'j'],
    'john': ['jno', 'jack', 'j'],
    'joseph': ['jos', 'joe', 'j'],
    'robert': ['robt', 'rob', 'bob', 'r'],
    'richard': ['richd', 'dick', 'r'],
    'charles': ['chas', 'charlie', 'c'],
    'edward': ['edw', 'ed', 'ted', 'e'],
    'henry': ['hy', 'harry', 'hal', 'h'],
    'george': ['geo', 'g'],
    'benjamin': ['benj', 'ben', 'b'],
    'samuel': ['saml', 'sam', 's'],
    'frederick': ['fred', 'fredk', 'f'],
    'alfred': ['alf', 'a']
};

function namesMatch(name1, name2) {
    if (!name1 || !name2) return false;

    const n1 = name1.toLowerCase().trim().replace(/\./g, '');
    const n2 = name2.toLowerCase().trim().replace(/\./g, '');

    if (n1 === n2) return true;

    const n1First = n1.split(/\s+/)[0];
    const n2First = n2.split(/\s+/)[0];

    // Initial matching
    if (n1First.length === 1 || n2First.length === 1) {
        if (n1First[0] === n2First[0]) return true;
    }

    // Try abbreviations
    for (const [fullName, abbrevs] of Object.entries(nameAbbreviations)) {
        if ((n1First === fullName && abbrevs.includes(n2First)) ||
            (n2First === fullName && abbrevs.includes(n1First)) ||
            (abbrevs.includes(n1First) && abbrevs.includes(n2First))) {
            return true;
        }
    }

    return false;
}

// Spouse verification
function isSpouseInHousehold(spouseName, householdMembers) {
    if (!spouseName || !householdMembers) return false;

    const spouseParts = spouseName.trim().split(/\s+/);
    const spouseFirstName = spouseParts[0];
    const spouseSurname = spouseParts[spouseParts.length - 1];

    const household = householdMembers.toLowerCase();

    return household.includes(spouseFirstName.toLowerCase()) &&
           household.includes(spouseSurname.toLowerCase());
}

let spouseMatches = 0;
let uniqueMatches = 0;
let newAddresses = 0;
let newProfessions = 0;

const updatePerson = db.prepare(`
    UPDATE sheffield_people
    SET street_address = COALESCE(street_address, ?),
        profession = COALESCE(profession, ?)
    WHERE id = ?
`);

console.log('='.repeat(70));
console.log('MATCHING WITH SPOUSE VERIFICATION');
console.log('='.repeat(70) + '\n');

for (const record of genealogyRecords) {
    const genName = record.Name?.trim();
    const genBirthYear = parseInt(record['Born Approx'] || record['Birth Year']);
    const genAddress = record.Address?.trim();
    const genOccupation = record.Profession?.trim() || record.Occupation?.trim();
    const genSpouse = record.Spouse?.trim();
    const genParish = (record['Ecclesiastical District'] || record.Parish || '').toLowerCase().trim();

    if (!genName || !genBirthYear) continue;

    // Extract surname from full name (last part)
    const nameParts = genName.split(/\s+/);
    const genSurname = nameParts[nameParts.length - 1];

    if (!genSurname) continue;

    const genSurnameLower = genSurname.toLowerCase().trim();
    const candidates = bySurname[genSurnameLower];

    if (!candidates || candidates.length === 0) continue;

    // Filter by birth year (exact or within 1 year)
    let matches = candidates.filter(p =>
        Math.abs(p.birth_year - genBirthYear) <= 1
    );

    if (matches.length === 0) continue;

    // Try first name match
    if (matches.length > 1 && genName) {
        const firstNameMatches = matches.filter(p =>
            namesMatch(genName, p.first_name)
        );
        if (firstNameMatches.length > 0) {
            matches = firstNameMatches;
        }
    }

    // Try spouse verification for males
    if (matches.length > 1 && genSpouse) {
        const spouseMatches = matches.filter(p =>
            p.census_household_members &&
            isSpouseInHousehold(genSpouse, p.census_household_members)
        );
        if (spouseMatches.length > 0) {
            matches = spouseMatches;
        }
    }

    // Try parish match
    if (matches.length > 1 && genParish) {
        const parishMatches = matches.filter(p => {
            const personParish = (p.ecclesiastical_parish || p.civil_parish || '').toLowerCase().trim();
            return personParish === genParish;
        });
        if (parishMatches.length > 0) {
            matches = parishMatches;
        }
    }

    // Only accept unique matches
    if (matches.length !== 1) continue;

    const person = matches[0];

    // Skip if same data
    if (person.street_address === genAddress && person.profession === genOccupation) continue;

    const hadAddress = person.street_address ? true : false;
    const hadProfession = person.profession ? true : false;

    updatePerson.run(
        genAddress || null,
        genOccupation || null,
        person.id
    );

    if (genSpouse && person.census_household_members &&
        isSpouseInHousehold(genSpouse, person.census_household_members)) {
        spouseMatches++;
    } else {
        uniqueMatches++;
    }

    if (!hadAddress && genAddress) newAddresses++;
    if (!hadProfession && genOccupation) newProfessions++;

    if (spouseMatches + uniqueMatches <= 30) {
        console.log(`✓ ${person.name} (${person.birth_year})`);
        if (genSpouse) console.log(`  Spouse verified: ${genSpouse}`);
        if (genAddress) console.log(`  Address: ${genAddress}`);
        if (genOccupation) console.log(`  Occupation: ${genOccupation}`);
        console.log('');
    }
}

console.log('='.repeat(70));
console.log('Matching Complete!');
console.log('='.repeat(70));
console.log(`\nSpouse-verified matches: ${spouseMatches}`);
console.log(`Unique matches: ${uniqueMatches}`);
console.log(`Total matches: ${spouseMatches + uniqueMatches}`);
console.log(`\nNew addresses: ${newAddresses}`);
console.log(`New professions: ${newProfessions}`);

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

// Final statistics
const stats = db.prepare(`
    SELECT
        COUNT(*) as total_people,
        COUNT(CASE WHEN birth_year BETWEEN 1840 AND 1843 THEN 1 END) as born_1840_1843,
        COUNT(CASE WHEN birth_year BETWEEN 1840 AND 1843 AND street_address IS NOT NULL THEN 1 END) as with_address,
        COUNT(CASE WHEN birth_year BETWEEN 1840 AND 1843 AND profession IS NOT NULL THEN 1 END) as with_profession
    FROM sheffield_people
`).get();

console.log('\nFinal Statistics for people born 1840-1843:');
console.log(`  Total: ${stats.born_1840_1843.toLocaleString()}`);
console.log(`  With addresses: ${stats.with_address.toLocaleString()} (${(stats.with_address / stats.born_1840_1843 * 100).toFixed(1)}%)`);
console.log(`  With professions: ${stats.with_profession.toLocaleString()} (${(stats.with_profession / stats.born_1840_1843 * 100).toFixed(1)}%)`);

db.close();

console.log('\n✓ Import and matching complete!');
