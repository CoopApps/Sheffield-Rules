const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('Linking Businesses with Unique Name Matches');
console.log('='.repeat(70) + '\n');

// Get all unlinked businesses
const unlinkedBusinesses = db.prepare('SELECT * FROM sheffield_businesses WHERE person_id IS NULL').all();

console.log('Unlinked businesses:', unlinkedBusinesses.length.toLocaleString());
console.log('');

const updateBusiness = db.prepare(`
    UPDATE sheffield_businesses
    SET person_id = ?, profession_match = ?
    WHERE id = ?
`);

const updatePerson = db.prepare(`
    UPDATE sheffield_people
    SET owns_business = 1
    WHERE id = ?
`);

let uniqueMatches = 0;
let professionMatches = 0;

console.log('Finding unique name matches...\n');

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
        ['dealer', 'merchant'],
        ['keeper', 'keeping']
    ];

    for (const [syn1, syn2] of synonyms) {
        if ((p1.includes(syn1) && p2.includes(syn2)) || (p1.includes(syn2) && p2.includes(syn1))) {
            return true;
        }
    }

    return false;
}

unlinkedBusinesses.forEach((business, index) => {
    if (index % 1000 === 0) {
        console.log(`Progress: ${index.toLocaleString()} / ${unlinkedBusinesses.length.toLocaleString()} (${uniqueMatches.toLocaleString()} unique matches)`);
    }

    // Find all people with this exact name
    const people = db.prepare('SELECT * FROM sheffield_people WHERE name = ?').all(business.full_name);

    // Only match if there's exactly one person with this name
    if (people.length === 1) {
        const person = people[0];
        const profMatch = professionsMatch(person.profession, business.occupation);

        if (profMatch) professionMatches++;

        updateBusiness.run(
            person.id,
            profMatch ? 1 : 0,
            business.id
        );

        updatePerson.run(person.id);

        uniqueMatches++;
    }
});

console.log('\n' + '='.repeat(70));
console.log('LINKING SUMMARY');
console.log('='.repeat(70));
console.log(`\nUnique name matches linked: ${uniqueMatches.toLocaleString()}`);
console.log(`  - With profession match: ${professionMatches.toLocaleString()}`);
console.log(`  - Without profession match: ${(uniqueMatches - professionMatches).toLocaleString()}`);

// Final statistics
const totalLinked = db.prepare('SELECT COUNT(*) as c FROM sheffield_businesses WHERE person_id IS NOT NULL').get().c;
const totalUnlinked = db.prepare('SELECT COUNT(*) as c FROM sheffield_businesses WHERE person_id IS NULL').get().c;
const totalBusinessOwners = db.prepare('SELECT COUNT(*) as c FROM sheffield_people WHERE owns_business = 1').get().c;

console.log('\n' + '='.repeat(70));
console.log('OVERALL BUSINESS STATISTICS');
console.log('='.repeat(70));
console.log(`\nTotal businesses: ${(totalLinked + totalUnlinked).toLocaleString()}`);
console.log(`  - Linked to people: ${totalLinked.toLocaleString()}`);
console.log(`  - Unlinked: ${totalUnlinked.toLocaleString()}`);
console.log(`\nPeople who own businesses: ${totalBusinessOwners.toLocaleString()}`);

// Sample new matches
console.log('\n' + '='.repeat(70));
console.log('SAMPLE NEWLY LINKED BUSINESSES');
console.log('='.repeat(70));

const samples = db.prepare(`
    SELECT b.full_name, b.occupation, b.address, p.profession, p.street_address
    FROM sheffield_businesses b
    JOIN sheffield_people p ON b.person_id = p.id
    WHERE b.is_home_business IS NULL OR b.is_home_business = 0
    LIMIT 10
`).all();

samples.forEach((s, i) => {
    console.log(`\n${i+1}. ${s.full_name}`);
    console.log(`   Business: ${s.occupation} at ${s.address}`);
    console.log(`   Person: ${s.profession || 'No profession'} at ${s.street_address || 'No address'}`);
});

db.close();
console.log('\n✓ Unique name linking complete!');
