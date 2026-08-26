const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('POTENTIAL MATCHES - Same Surname, Initial & Profession');
console.log('='.repeat(70) + '\n');

// Helper function to normalize addresses for comparison
function normalizeAddress(addr) {
    if (!addr) return '';
    return addr.toLowerCase()
        .replace(/~/g, '')
        .replace(/([a-z])(road|street|lane|avenue|drive|terrace|place|square|row|grove|crescent|court|hill|way)/gi, '$1 $2')
        .replace(/\s+/g, ' ')
        .replace(/,/g, ' ')
        .replace(/\./g, '')
        .replace(/\broad\b/g, 'rd')
        .replace(/\bstreet\b/g, 'st')
        .replace(/\bavenue\b/g, 'ave')
        .replace(/\blane\b/g, 'ln')
        .replace(/\bdrive\b/g, 'dr')
        .replace(/\bterrace\b/g, 'ter')
        .replace(/\bplace\b/g, 'pl')
        .replace(/\bsquare\b/g, 'sq')
        .trim();
}

// Helper function to check if professions are similar
function professionsMatch(prof1, prof2) {
    if (!prof1 || !prof2) return false;

    const p1 = prof1.toLowerCase().trim();
    const p2 = prof2.toLowerCase().trim();

    if (p1 === p2) return true;
    if (p1.includes(p2) || p2.includes(p1)) return true;

    const synonyms = [
        ['grinder', 'grinding'],
        ['forger', 'forging'],
        ['cutler', 'cutter'],
        ['smith', 'smithing'],
        ['maker', 'making'],
        ['builder', 'building'],
        ['keeper', 'keeping']
    ];

    for (const [syn1, syn2] of synonyms) {
        if ((p1.includes(syn1) && p2.includes(syn2)) || (p1.includes(syn2) && p2.includes(syn1))) {
            return true;
        }
    }

    return false;
}

// Helper function to extract surname and first initial
function extractNameParts(name) {
    if (!name) return { surname: '', initial: '' };

    // Remove punctuation
    const cleaned = name.replace(/[.,]/g, '').trim();
    const parts = cleaned.split(/\s+/);

    if (parts.length === 0) return { surname: '', initial: '' };

    // Last part is surname
    const surname = parts[parts.length - 1].toLowerCase();

    // First part's first character is initial
    const initial = parts[0] ? parts[0][0].toLowerCase() : '';

    return { surname, initial };
}

// Helper function to check if business is in non-Sheffield location
function isNonSheffieldLocation(address) {
    if (!address) return false;
    const addr = address.toLowerCase();
    const nonSheffieldLocations = [
        'rotherham', 'rawmarsh', 'parkgate', 'ecclesfield',
        'whiston', 'guilthwaite', 'canklow'
    ];
    return nonSheffieldLocations.some(loc => addr.includes(loc));
}

// Get unlinked Sheffield businesses
const allBusinesses = db.prepare('SELECT * FROM sheffield_businesses WHERE person_id IS NULL').all();
const businesses = allBusinesses.filter(b => !isNonSheffieldLocation(b.address));

console.log('Checking', businesses.length.toLocaleString(), 'unlinked Sheffield businesses...\n');

const matches = [];

businesses.forEach((business, index) => {
    if (index % 5000 === 0 && index > 0) {
        console.log(`Progress: ${index.toLocaleString()} / ${businesses.length.toLocaleString()}`);
    }

    const businessName = extractNameParts(business.full_name);

    // Skip if we don't have surname and initial
    if (!businessName.surname || !businessName.initial) return;

    // Find people with same surname
    const people = db.prepare('SELECT * FROM sheffield_people WHERE LOWER(name) LIKE ?')
        .all(`%${businessName.surname}%`);

    people.forEach(person => {
        const personName = extractNameParts(person.name);

        // Check if surname and initial match
        if (personName.surname !== businessName.surname) return;
        if (personName.initial !== businessName.initial) return;

        // Check if professions match
        if (!professionsMatch(person.profession, business.occupation)) return;

        // Check address match
        const personAddr = normalizeAddress(person.street_address);
        const businessAddr = normalizeAddress(business.address);
        const addressMatch = personAddr && businessAddr &&
                            (personAddr.includes(businessAddr) || businessAddr.includes(personAddr));

        matches.push({
            business: business,
            person: person,
            addressMatch: addressMatch,
            businessName: business.full_name,
            personName: person.name,
            businessAddr: businessAddr,
            personAddr: personAddr
        });
    });
});

console.log('\n' + '='.repeat(70));
console.log('SUMMARY');
console.log('='.repeat(70));
console.log('Total matches found:', matches.length.toLocaleString());

const addressMatches = matches.filter(m => m.addressMatch);
const noAddressMatches = matches.filter(m => !m.addressMatch);

console.log('  - With address match (STRONG):', addressMatches.length.toLocaleString());
console.log('  - Without address match:', noAddressMatches.length.toLocaleString());

console.log('\n' + '='.repeat(70));
console.log('STRONG MATCHES - Address + Name Initial + Profession (First 50)');
console.log('='.repeat(70) + '\n');

addressMatches.slice(0, 50).forEach((match, i) => {
    console.log(`${i + 1}. "${match.businessName}" → "${match.personName}"`);
    console.log(`   Business: ${match.business.occupation || '(none)'} at ${match.business.address}`);
    console.log(`   Person: ${match.person.profession || '(none)'} at ${match.person.street_address || '(none)'}`);
    console.log('');
});

if (addressMatches.length > 50) {
    console.log(`... and ${(addressMatches.length - 50).toLocaleString()} more strong matches\n`);
}

console.log('='.repeat(70));
console.log('POSSIBLE MATCHES - Name Initial + Profession Only (First 50)');
console.log('='.repeat(70) + '\n');

noAddressMatches.slice(0, 50).forEach((match, i) => {
    console.log(`${i + 1}. "${match.businessName}" → "${match.personName}"`);
    console.log(`   Business: ${match.business.occupation || '(none)'} at ${match.business.address}`);
    console.log(`   Person: ${match.person.profession || '(none)'} at ${match.person.street_address || '(none)'}`);
    console.log('');
});

if (noAddressMatches.length > 50) {
    console.log(`... and ${(noAddressMatches.length - 50).toLocaleString()} more possible matches\n`);
}

db.close();
