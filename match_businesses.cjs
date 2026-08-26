const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('Matching Businesses with People');
console.log('='.repeat(70) + '\n');

// Helper function to normalize addresses for comparison
function normalizeAddress(addr) {
    if (!addr) return '';
    return addr.toLowerCase()
        .replace(/~/g, '')
        // Add space before common street suffixes if not already there
        .replace(/([a-z])(road|street|lane|avenue|drive|terrace|place|square|row|grove|crescent|court|hill|way)/gi, '$1 $2')
        .replace(/\s+/g, ' ')
        .replace(/,/g, ' ')
        .replace(/\./g, '')
        // Now replace the full words with abbreviations
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

    // Exact match
    if (p1 === p2) return true;

    // One contains the other
    if (p1.includes(p2) || p2.includes(p1)) return true;

    // Common profession synonyms (only for same trade)
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

// Helper function to detect if address indicates employment (like "yard", "works", "factory")
function hasEmploymentIndicators(addr) {
    if (!addr) return false;
    const lowerAddr = addr.toLowerCase();
    return lowerAddr.includes('yard') ||
           lowerAddr.includes('works') ||
           lowerAddr.includes('factory') ||
           lowerAddr.includes('mill') ||
           lowerAddr.includes('shop') ||
           lowerAddr.includes('warehouse');
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

// Get all businesses and people
const allBusinesses = db.prepare('SELECT * FROM sheffield_businesses').all();
const people = db.prepare('SELECT * FROM sheffield_people').all();

// Filter out non-Sheffield locations
const businesses = allBusinesses.filter(b => !isNonSheffieldLocation(b.address));

console.log('Total businesses in database:', allBusinesses.length.toLocaleString());
console.log('Businesses to match (Sheffield only):', businesses.length.toLocaleString());
console.log('Excluded (non-Sheffield):', (allBusinesses.length - businesses.length).toLocaleString());
console.log('People in database:', people.length.toLocaleString());
console.log('');

// Create indexes for faster lookup
const peopleByName = new Map();
const peopleByAddress = new Map();

people.forEach(person => {
    // Index by name
    if (!peopleByName.has(person.name)) {
        peopleByName.set(person.name, []);
    }
    peopleByName.get(person.name).push(person);

    // Index by normalized address
    const normAddr = normalizeAddress(person.street_address);
    if (normAddr) {
        if (!peopleByAddress.has(normAddr)) {
            peopleByAddress.set(normAddr, []);
        }
        peopleByAddress.get(normAddr).push(person);
    }
});

// Prepare update statement
const updateBusiness = db.prepare(`
    UPDATE sheffield_businesses
    SET person_id = ?, is_home_business = ?, profession_match = ?, employs_people = ?
    WHERE id = ?
`);

// Also add business ownership flag to sheffield_people (if columns don't exist, this will be skipped gracefully)
try {
    db.exec('ALTER TABLE sheffield_people ADD COLUMN owns_business BOOLEAN DEFAULT 0');
} catch (e) {
    // Column might already exist
}

try {
    db.exec('ALTER TABLE sheffield_people ADD COLUMN business_at_home BOOLEAN DEFAULT 0');
} catch (e) {
    // Column might already exist
}

const updatePerson = db.prepare(`
    UPDATE sheffield_people
    SET owns_business = 1, business_at_home = ?
    WHERE id = ?
`);

let matched = 0;
let nameMatches = 0;
let addressMatches = 0;
let professionMatches = 0;
let homeBusinesses = 0;
let employers = 0;

console.log('Matching businesses to people...\n');

businesses.forEach((business, index) => {
    if (index % 1000 === 0) {
        console.log(`Progress: ${index.toLocaleString()} / ${businesses.length.toLocaleString()} (${matched.toLocaleString()} matches)`);
    }

    // Try to find matching person
    const potentialMatches = peopleByName.get(business.full_name) || [];

    let bestMatch = null;
    let isHomeBusiness = false;
    let professionMatch = false;
    let employsePeople = hasEmploymentIndicators(business.address);

    for (const person of potentialMatches) {
        nameMatches++;

        // Check if addresses match
        const personAddr = normalizeAddress(person.street_address);
        const businessAddr = normalizeAddress(business.address);

        const addressMatch = personAddr && businessAddr &&
                            (personAddr.includes(businessAddr) || businessAddr.includes(personAddr));

        // Check if professions match
        const profMatch = professionsMatch(person.profession, business.occupation);

        // ONLY link if profession matches (address alone is not enough - could be different people with same name)
        if (profMatch) {
            bestMatch = person;
            isHomeBusiness = addressMatch;
            professionMatch = profMatch;

            if (addressMatch) addressMatches++;
            if (profMatch) professionMatches++;

            break; // Take first good match
        }
    }

    if (bestMatch) {
        matched++;
        if (isHomeBusiness) homeBusinesses++;
        if (employsePeople) employers++;

        updateBusiness.run(
            bestMatch.id,
            isHomeBusiness ? 1 : 0,
            professionMatch ? 1 : 0,
            employsePeople ? 1 : 0,
            business.id
        );

        updatePerson.run(
            isHomeBusiness ? 1 : 0,
            bestMatch.id
        );
    }
});

console.log('\n' + '='.repeat(70));
console.log('MATCHING SUMMARY');
console.log('='.repeat(70));
console.log(`\nTotal businesses matched to people: ${matched.toLocaleString()}`);
console.log(`  - Name matches found: ${nameMatches.toLocaleString()}`);
console.log(`  - Address matches: ${addressMatches.toLocaleString()}`);
console.log(`  - Profession matches: ${professionMatches.toLocaleString()}`);
console.log(`  - Home-based businesses: ${homeBusinesses.toLocaleString()}`);
console.log(`  - Businesses with employment indicators: ${employers.toLocaleString()}`);

// Sample matched businesses
console.log('\n' + '='.repeat(70));
console.log('SAMPLE MATCHED BUSINESSES');
console.log('='.repeat(70));

const samples = db.prepare(`
    SELECT b.*, p.name as person_name, p.street_address, p.profession
    FROM sheffield_businesses b
    JOIN sheffield_people p ON b.person_id = p.id
    WHERE b.is_home_business = 1
    LIMIT 10
`).all();

samples.forEach((s, i) => {
    console.log(`\n${i+1}. ${s.full_name} - ${s.occupation}`);
    console.log(`   Business Address: ${s.address}`);
    console.log(`   Home Address: ${s.street_address}`);
    console.log(`   Profession: ${s.profession}`);
    console.log(`   Employs People: ${s.employs_people ? 'Yes' : 'No'}`);
});

db.close();
console.log('\n✓ Business matching complete!');
