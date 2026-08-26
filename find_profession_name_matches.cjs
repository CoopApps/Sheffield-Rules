const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('UNLINKED BUSINESSES - Name + Profession Match (Different Addresses)');
console.log('='.repeat(70) + '\n');

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

// Get ALL unlinked businesses
const unlinkedBusinesses = db.prepare('SELECT * FROM sheffield_businesses WHERE person_id IS NULL').all();

console.log('Checking', unlinkedBusinesses.length.toLocaleString(), 'unlinked businesses...\n');

const matches = [];

unlinkedBusinesses.forEach((business, index) => {
    if (index % 5000 === 0 && index > 0) {
        console.log(`Progress: ${index.toLocaleString()} / ${unlinkedBusinesses.length.toLocaleString()}`);
    }

    // Find people with same name
    const people = db.prepare('SELECT * FROM sheffield_people WHERE name = ?').all(business.full_name);

    if (people.length > 0) {
        people.forEach(person => {
            // Check if professions match
            if (professionsMatch(person.profession, business.occupation)) {
                // Check if addresses are DIFFERENT
                const personAddr = normalizeAddress(person.street_address);
                const businessAddr = normalizeAddress(business.address);

                const addressMatch = personAddr && businessAddr &&
                                    (personAddr.includes(businessAddr) || businessAddr.includes(personAddr));

                // Only include if profession matches but address DOESN'T match
                if (!addressMatch) {
                    matches.push({
                        business: business,
                        person: person,
                        peopleWithSameName: people.length,
                        personAddr: personAddr,
                        businessAddr: businessAddr
                    });
                }
            }
        });
    }
});

console.log('\n' + '='.repeat(70));
console.log('SUMMARY');
console.log('='.repeat(70));
console.log('Total matches (name + profession, different address):', matches.length.toLocaleString());
console.log('');

// Group by whether name is unique
const uniqueNameMatches = matches.filter(m => m.peopleWithSameName === 1);
const multipleNameMatches = matches.filter(m => m.peopleWithSameName > 1);

console.log('Unique name matches:', uniqueNameMatches.length.toLocaleString());
console.log('Multiple people with same name:', multipleNameMatches.length.toLocaleString());

console.log('\n' + '='.repeat(70));
console.log('UNIQUE NAME MATCHES (First 50)');
console.log('='.repeat(70) + '\n');

uniqueNameMatches.slice(0, 50).forEach((match, i) => {
    console.log(`${i + 1}. ${match.business.full_name}`);
    console.log(`   Business: ${match.business.occupation || '(none)'} at ${match.business.address}`);
    console.log(`   Person: ${match.person.profession || '(none)'} at ${match.person.street_address || '(none)'}`);
    if (match.peopleWithSameName > 1) {
        console.log(`   WARNING: ${match.peopleWithSameName} people with this name`);
    }
    console.log('');
});

console.log('='.repeat(70));
console.log('MULTIPLE NAME MATCHES (First 25)');
console.log('='.repeat(70) + '\n');

multipleNameMatches.slice(0, 25).forEach((match, i) => {
    console.log(`${i + 1}. ${match.business.full_name} (${match.peopleWithSameName} people with this name)`);
    console.log(`   Business: ${match.business.occupation || '(none)'} at ${match.business.address}`);
    console.log(`   Person: ${match.person.profession || '(none)'} at ${match.person.street_address || '(none)'}`);
    console.log('');
});

db.close();
