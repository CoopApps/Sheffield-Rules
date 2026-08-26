const sqlite3 = require('better-sqlite3');

const db = new sqlite3('Sheffield1867.db', { timeout: 30000 });

console.log('='.repeat(70));
console.log('White\'s Directory Address-First Matching');
console.log('='.repeat(70) + '\n');

// Get all people WITH addresses
console.log('Loading people with addresses...');
const people = db.prepare(`
    SELECT id, name, first_name, surname, birth_year, street_address, profession
    FROM sheffield_people
    WHERE street_address IS NOT NULL
    AND surname IS NOT NULL
`).all();

console.log(`Found ${people.length} people with addresses\n`);

// Get all businesses
console.log('Loading businesses...');
const businesses = db.prepare('SELECT * FROM sheffield_businesses').all();
console.log(`Found ${businesses.length} businesses\n`);

// Normalize address
function normalizeAddress(addr) {
    if (!addr) return '';
    return addr.toLowerCase()
        .replace(/\s+/g, ' ')
        .replace(/road/g, 'rd')
        .replace(/street/g, 'st')
        .replace(/lane/g, 'ln')
        .replace(/avenue/g, 'av')
        .replace(/terrace/g, 'ter')
        .replace(/house/g, 'hs')
        .replace(/\./g, '')
        .replace(/,/g, '')
        .replace(/~/g, '')
        .replace(/\(/g, '')
        .replace(/\)/g, '')
        .trim();
}

// Normalize profession
function normalizeProfession(prof) {
    if (!prof) return '';
    return prof.toLowerCase()
        .replace(/\s+/g, ' ')
        .replace(/\./g, '')
        .trim();
}

// Name matching with abbreviations
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

    // If one is just an initial, check if it matches the first letter
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

// Check if addresses are similar
function addressesSimilar(addr1, addr2) {
    const a1 = normalizeAddress(addr1);
    const a2 = normalizeAddress(addr2);

    if (a1 === a2) return 'exact';

    // Extract significant parts (ignore small words)
    const parts1 = a1.split(' ').filter(p => p.length > 2);
    const parts2 = a2.split(' ').filter(p => p.length > 2);

    if (parts1.length === 0 || parts2.length === 0) return 'none';

    // Find common parts
    const common = parts1.filter(p => parts2.includes(p));

    // Strong match: 2+ common parts or 1 long (6+) part
    if (common.length >= 2 || common.some(p => p.length >= 6)) {
        return 'strong';
    }

    // Weak match: 1 common part
    if (common.length === 1) {
        return 'weak';
    }

    return 'none';
}

// Build address index for businesses
console.log('Building address index...');
const byAddress = {};

businesses.forEach(biz => {
    const addr = normalizeAddress(biz.address);
    if (!addr) return;

    if (!byAddress[addr]) byAddress[addr] = [];
    byAddress[addr].push(biz);
});

console.log(`Indexed ${Object.keys(byAddress).length} unique addresses\n`);

let exactAddressMatches = 0;
let strongAddressMatches = 0;
let weakAddressMatches = 0;
let addressWithNameMatch = 0;
let addressWithProfessionMatch = 0;
let newProfessions = 0;

const updatePerson = db.prepare(`
    UPDATE sheffield_people
    SET profession = COALESCE(profession, ?)
    WHERE id = ?
`);

console.log('Matching by address first...\n');

for (const person of people) {
    const personAddr = normalizeAddress(person.street_address);
    if (!personAddr) continue;

    // Find businesses with similar addresses
    let matches = [];

    for (const [bizAddr, bizList] of Object.entries(byAddress)) {
        const similarity = addressesSimilar(person.street_address, bizAddr);

        if (similarity === 'exact' || similarity === 'strong') {
            bizList.forEach(biz => {
                matches.push({ biz, similarity });
            });
        }
    }

    if (matches.length === 0) continue;

    // Filter by name if we have multiple matches at same address
    const nameMatches = matches.filter(m =>
        namesMatch(person.first_name, m.biz.forename) &&
        person.surname.toLowerCase() === m.biz.surname.toLowerCase()
    );

    // Prefer name matches
    if (nameMatches.length === 1) {
        const match = nameMatches[0];
        const biz = match.biz;

        // Skip if same profession
        if (normalizeProfession(person.profession) === normalizeProfession(biz.occupation)) {
            continue;
        }

        if (!biz.occupation) continue;

        updatePerson.run(biz.occupation, person.id);

        if (match.similarity === 'exact') exactAddressMatches++;
        else strongAddressMatches++;
        addressWithNameMatch++;
        if (!person.profession) newProfessions++;

        if ((exactAddressMatches + strongAddressMatches) <= 30) {
            console.log(`✓ ${person.name} (${person.birth_year}) - ${match.similarity.toUpperCase()} ADDRESS + NAME MATCH`);
            console.log(`  Person address: ${person.street_address}`);
            console.log(`  Business address: ${biz.address}`);
            console.log(`  Current profession: ${person.profession || 'NONE'}`);
            console.log(`  Business occupation: ${biz.occupation}`);
            console.log('');
        }
    }
    // If no name match, check profession match
    else if (person.profession && matches.length > 0) {
        const personProf = normalizeProfession(person.profession);

        const profMatches = matches.filter(m => {
            const bizProf = normalizeProfession(m.biz.occupation);
            return bizProf && (
                personProf.includes(bizProf) ||
                bizProf.includes(personProf) ||
                personProf.split(' ')[0] === bizProf.split(' ')[0]
            );
        });

        if (profMatches.length === 1) {
            const match = profMatches[0];
            const biz = match.biz;

            // This is just confirmation - addresses and professions match
            if (match.similarity === 'exact') exactAddressMatches++;
            else strongAddressMatches++;
            addressWithProfessionMatch++;

            if ((exactAddressMatches + strongAddressMatches) <= 30) {
                console.log(`✓ ${person.name} (${person.birth_year}) - ${match.similarity.toUpperCase()} ADDRESS + PROFESSION MATCH`);
                console.log(`  Person address: ${person.street_address}`);
                console.log(`  Business address: ${biz.address}`);
                console.log(`  Profession: ${person.profession} ≈ ${biz.occupation}`);
                console.log('');
            }
        }
    }
}

console.log('='.repeat(70));
console.log('Address-First Matching Complete!');
console.log('='.repeat(70));
console.log(`\nExact address matches: ${exactAddressMatches}`);
console.log(`Strong address matches: ${strongAddressMatches}`);
console.log(`  With name confirmation: ${addressWithNameMatch}`);
console.log(`  With profession confirmation: ${addressWithProfessionMatch}`);
console.log(`\nNew professions added: ${newProfessions}`);

// Copy to players
console.log('\nCopying to sheffield_players...');
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
console.log('\n✓ Address-first matching complete!');
