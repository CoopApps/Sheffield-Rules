const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('UNLINKED BUSINESSES - Name + Profession Match');
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

// Get unlinked businesses
const unlinkedBusinesses = db.prepare('SELECT * FROM sheffield_businesses WHERE person_id IS NULL LIMIT 5000').all();

console.log('Checking', unlinkedBusinesses.length.toLocaleString(), 'unlinked businesses...\n');

const matches = [];

unlinkedBusinesses.forEach(business => {
    // Find people with same name
    const people = db.prepare('SELECT * FROM sheffield_people WHERE name = ?').all(business.full_name);

    if (people.length > 0) {
        people.forEach(person => {
            if (professionsMatch(person.profession, business.occupation)) {
                matches.push({
                    business: business,
                    person: person,
                    peopleWithSameName: people.length
                });
            }
        });
    }
});

console.log('Found', matches.length.toLocaleString(), 'potential matches\n');
console.log('='.repeat(70));
console.log('SHOWING FIRST 50 MATCHES');
console.log('='.repeat(70) + '\n');

matches.slice(0, 50).forEach((match, i) => {
    console.log(`${i + 1}. ${match.business.full_name}`);
    console.log(`   Business: ${match.business.occupation || '(none)'} at ${match.business.address}`);
    console.log(`   Person: ${match.person.profession || '(none)'} at ${match.person.street_address || '(none)'}`);
    console.log(`   People with this name in database: ${match.peopleWithSameName}`);
    console.log('');
});

console.log('='.repeat(70));
console.log('Total profession+name matches found:', matches.length.toLocaleString());
console.log('These were not auto-matched because current logic requires exact profession match');

db.close();
