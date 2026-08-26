const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('BUSINESSES WITH ~~ ADDRESS AND UNIQUE NAMES');
console.log('='.repeat(70) + '\n');

// Get businesses with ~~ in address but full name (no ~ in name)
const businesses = db.prepare('SELECT * FROM sheffield_businesses WHERE address LIKE ? AND full_name NOT LIKE ?')
    .all('%~~%', '%~%');

console.log('Total businesses with ~~ in address:', businesses.length);

const uniqueMatches = [];
const multipleMatches = [];
const noMatches = [];

businesses.forEach(business => {
    // Find people with exact same name
    const people = db.prepare('SELECT * FROM sheffield_people WHERE name = ?')
        .all(business.full_name);

    if (people.length === 0) {
        noMatches.push({
            business,
            peopleCount: 0
        });
    } else if (people.length === 1) {
        uniqueMatches.push({
            business,
            person: people[0],
            peopleCount: 1
        });
    } else {
        multipleMatches.push({
            business,
            people: people,
            peopleCount: people.length
        });
    }
});

console.log('\n=== SUMMARY ===');
console.log('Unique name matches (1 person):', uniqueMatches.length);
console.log('Multiple name matches (2+ people):', multipleMatches.length);
console.log('No matches (0 people):', noMatches.length);

console.log('\n' + '='.repeat(70));
console.log('UNIQUE NAME MATCHES (First 50)');
console.log('='.repeat(70) + '\n');

uniqueMatches.slice(0, 50).forEach((match, i) => {
    console.log(`${i + 1}. "${match.business.full_name}"`);
    console.log(`   Business: ${match.business.occupation || '(none)'} at ${match.business.address}`);
    console.log(`   Person: ${match.person.profession || '(none)'} at ${match.person.street_address || '(none)'}`);
    console.log(`   Already linked: ${match.business.person_id ? 'YES' : 'NO'}`);
    console.log('');
});

if (uniqueMatches.length > 50) {
    console.log(`... and ${uniqueMatches.length - 50} more\n`);
}

console.log('='.repeat(70));
console.log('MULTIPLE NAME MATCHES (First 25)');
console.log('='.repeat(70) + '\n');

multipleMatches.slice(0, 25).forEach((match, i) => {
    console.log(`${i + 1}. "${match.business.full_name}" (${match.peopleCount} people with this name)`);
    console.log(`   Business: ${match.business.occupation || '(none)'} at ${match.business.address}`);
    match.people.forEach((p, idx) => {
        console.log(`   Person ${idx + 1}: ${p.profession || '(none)'} at ${p.street_address || '(none)'}`);
    });
    console.log('');
});

if (multipleMatches.length > 25) {
    console.log(`... and ${multipleMatches.length - 25} more\n`);
}

console.log('='.repeat(70));
console.log('NO MATCHES (First 25)');
console.log('='.repeat(70) + '\n');

noMatches.slice(0, 25).forEach((match, i) => {
    console.log(`${i + 1}. "${match.business.full_name}"`);
    console.log(`   Business: ${match.business.occupation || '(none)'} at ${match.business.address}`);
    console.log('');
});

if (noMatches.length > 25) {
    console.log(`... and ${noMatches.length - 25} more\n`);
}

db.close();
