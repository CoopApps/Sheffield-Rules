const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('STREET-ONLY MATCHES');
console.log('(Name + Profession + Street Name match, but house numbers differ/missing)');
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

// Helper function to extract street name (without house number)
function extractStreetName(addr) {
    if (!addr) return '';
    const normalized = normalizeAddress(addr);
    // Remove leading numbers and whitespace
    return normalized.replace(/^[\d\s]+/, '').trim();
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
    if (!name) return { surname: '', initial: '', fullFirst: '' };

    const cleaned = name.replace(/[.,]/g, '').trim();
    const parts = cleaned.split(/\s+/);

    if (parts.length === 0) return { surname: '', initial: '', fullFirst: '' };

    const surname = parts[parts.length - 1].toLowerCase();
    const initial = parts[0] ? parts[0][0].toLowerCase() : '';
    const fullFirst = parts[0] ? parts[0].toLowerCase() : '';

    return { surname, initial, fullFirst };
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
    if (!businessName.surname) return;

    const businessStreet = extractStreetName(business.address);
    if (!businessStreet) return; // Skip if no street name

    // Find people with same surname
    const people = db.prepare('SELECT * FROM sheffield_people WHERE LOWER(name) LIKE ?')
        .all(`%${businessName.surname}%`);

    people.forEach(person => {
        const personName = extractNameParts(person.name);

        // Check if surname matches
        if (personName.surname !== businessName.surname) return;

        // Check if first name or initial matches
        const nameMatch = personName.initial === businessName.initial ||
                         personName.fullFirst === businessName.fullFirst;
        if (!nameMatch) return;

        // Check if professions match
        if (!professionsMatch(person.profession, business.occupation)) return;

        // Check if street name matches
        const personStreet = extractStreetName(person.street_address);
        if (!personStreet) return;

        const streetMatch = personStreet.includes(businessStreet) ||
                           businessStreet.includes(personStreet);

        if (!streetMatch) return;

        // Check if it's an EXACT address match (we want to exclude these - they should already be linked)
        const personAddr = normalizeAddress(person.street_address);
        const businessAddr = normalizeAddress(business.address);
        const exactMatch = personAddr && businessAddr &&
                          (personAddr.includes(businessAddr) || businessAddr.includes(personAddr));

        // Only include if street matches but NOT an exact address match
        if (streetMatch && !exactMatch) {
            matches.push({
                business: business,
                person: person,
                businessName: business.full_name,
                personName: person.name,
                businessAddr: business.address,
                personAddr: person.street_address,
                businessStreet: businessStreet,
                personStreet: personStreet
            });
        }
    });
});

console.log('\n' + '='.repeat(70));
console.log('SUMMARY');
console.log('='.repeat(70));
console.log('Total street-only matches found:', matches.length.toLocaleString());
console.log('(Name + Profession + Street match, but house numbers differ/missing)');

console.log('\n' + '='.repeat(70));
console.log('STREET-ONLY MATCHES (First 100)');
console.log('='.repeat(70) + '\n');

matches.slice(0, 100).forEach((match, i) => {
    console.log(`${i + 1}. "${match.businessName}" → "${match.personName}"`);
    console.log(`   Business: ${match.business.occupation || '(none)'} at ${match.businessAddr}`);
    console.log(`   Person: ${match.person.profession || '(none)'} at ${match.personAddr || '(none)'}`);
    console.log(`   Street: "${match.businessStreet}" matches "${match.personStreet}"`);
    console.log('');
});

if (matches.length > 100) {
    console.log(`... and ${(matches.length - 100).toLocaleString()} more street-only matches\n`);
}

db.close();
