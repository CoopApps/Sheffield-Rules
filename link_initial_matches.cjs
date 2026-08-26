const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('LINKING STRONG INITIAL MATCHES');
console.log('(Surname + Initial + Profession + Address)');
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

    const cleaned = name.replace(/[.,]/g, '').trim();
    const parts = cleaned.split(/\s+/);

    if (parts.length === 0) return { surname: '', initial: '' };

    const surname = parts[parts.length - 1].toLowerCase();
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

// Prepare update statements
const updateBusiness = db.prepare(`
    UPDATE sheffield_businesses
    SET person_id = ?, is_home_business = ?, profession_match = ?
    WHERE id = ?
`);

const updatePerson = db.prepare(`
    UPDATE sheffield_people
    SET owns_business = 1, business_at_home = ?
    WHERE id = ?
`);

// Get unlinked Sheffield businesses
const allBusinesses = db.prepare('SELECT * FROM sheffield_businesses WHERE person_id IS NULL').all();
const businesses = allBusinesses.filter(b => !isNonSheffieldLocation(b.address));

console.log('Checking', businesses.length.toLocaleString(), 'unlinked Sheffield businesses...\n');

let linked = 0;
let samples = [];
const linkedBusinessIds = new Set(); // Track which businesses we've already linked

businesses.forEach((business, index) => {
    if (index % 1000 === 0 && index > 0) {
        console.log(`Progress: ${index.toLocaleString()} / ${businesses.length.toLocaleString()} (${linked} linked)`);
    }

    // Skip if already linked
    if (linkedBusinessIds.has(business.id)) return;

    const businessName = extractNameParts(business.full_name);

    if (!businessName.surname || !businessName.initial) return;

    // Find people with same surname
    const people = db.prepare('SELECT * FROM sheffield_people WHERE LOWER(name) LIKE ?')
        .all(`%${businessName.surname}%`);

    for (const person of people) {
        const personName = extractNameParts(person.name);

        // Check if surname and initial match
        if (personName.surname !== businessName.surname) continue;
        if (personName.initial !== businessName.initial) continue;

        // Check if professions match
        if (!professionsMatch(person.profession, business.occupation)) continue;

        // Check address match
        const personAddr = normalizeAddress(person.street_address);
        const businessAddr = normalizeAddress(business.address);
        const addressMatch = personAddr && businessAddr &&
                            (personAddr.includes(businessAddr) || businessAddr.includes(personAddr));

        // ONLY link if address matches (strong match)
        if (addressMatch) {
            updateBusiness.run(person.id, 1, 1, business.id);
            updatePerson.run(1, person.id);
            linkedBusinessIds.add(business.id);
            linked++;

            if (samples.length < 25) {
                samples.push({
                    businessName: business.full_name,
                    personName: person.name,
                    occupation: business.occupation,
                    profession: person.profession,
                    address: business.address
                });
            }

            break; // Only link to first match
        }
    }
});

console.log('\n' + '='.repeat(70));
console.log('LINKING SUMMARY');
console.log('='.repeat(70));
console.log(`\nTotal businesses linked: ${linked.toLocaleString()}`);

console.log('\n' + '='.repeat(70));
console.log('SAMPLE LINKED BUSINESSES (First 25)');
console.log('='.repeat(70) + '\n');

samples.forEach((s, i) => {
    console.log(`${i + 1}. "${s.businessName}" → "${s.personName}"`);
    console.log(`   Business: ${s.occupation || '(none)'} at ${s.address}`);
    console.log(`   Person: ${s.profession || '(none)'}`);
    console.log('');
});

console.log('✓ Strong initial matches linked!');

db.close();
