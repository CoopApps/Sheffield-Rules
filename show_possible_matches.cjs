const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('POSSIBLE BUSINESS MATCHES (Currently Unlinked)');
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

// Get unlinked businesses
const unlinkedBusinesses = db.prepare('SELECT * FROM sheffield_businesses WHERE person_id IS NULL LIMIT 500').all();

console.log('Analyzing', unlinkedBusinesses.length.toLocaleString(), 'unlinked businesses...\n');

const possibleMatches = [];

unlinkedBusinesses.forEach(business => {
    // Find people with same name
    const people = db.prepare('SELECT * FROM sheffield_people WHERE name = ?').all(business.full_name);

    if (people.length > 0) {
        people.forEach(person => {
            const personAddr = normalizeAddress(person.street_address);
            const businessAddr = normalizeAddress(business.address);

            // Check if addresses are similar
            const addressSimilar = personAddr && businessAddr &&
                                  (personAddr.includes(businessAddr) || businessAddr.includes(personAddr));

            // Check if professions are somewhat related
            const p1 = (person.profession || '').toLowerCase();
            const p2 = (business.occupation || '').toLowerCase();

            const professionSimilar = p1 && p2 && (
                p1.includes(p2) ||
                p2.includes(p1) ||
                // Check for related words
                (p1.includes('shop') && p2.includes('shop')) ||
                (p1.includes('keeper') && p2.includes('keeper')) ||
                (p1.includes('dealer') && p2.includes('dealer')) ||
                (p1.includes('maker') && p2.includes('maker'))
            );

            if (addressSimilar || professionSimilar) {
                possibleMatches.push({
                    business: business,
                    person: person,
                    addressSimilar: addressSimilar,
                    professionSimilar: professionSimilar,
                    personAddr: personAddr,
                    businessAddr: businessAddr
                });
            }
        });
    }
});

console.log('Found', possibleMatches.length.toLocaleString(), 'possible matches\n');
console.log('='.repeat(70));
console.log('TOP 25 POSSIBLE MATCHES');
console.log('='.repeat(70) + '\n');

possibleMatches.slice(0, 25).forEach((match, i) => {
    const reasons = [];
    if (match.addressSimilar) reasons.push('ADDRESS SIMILAR');
    if (match.professionSimilar) reasons.push('PROFESSION SIMILAR');

    console.log(`${i + 1}. ${match.business.full_name}`);
    console.log(`   Business: ${match.business.occupation || '(none)'} at ${match.business.address}`);
    console.log(`   Person: ${match.person.profession || '(none)'} at ${match.person.street_address || '(none)'}`);
    console.log(`   Match reason: ${reasons.join(' + ')}`);
    console.log(`   Normalized addresses: "${match.businessAddr}" vs "${match.personAddr}"`);
    console.log('');
});

db.close();
