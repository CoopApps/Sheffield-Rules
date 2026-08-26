const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('UNLINKED BUSINESSES - Name Matches Only (No Profession Match)');
console.log('='.repeat(70) + '\n');

// Get ALL unlinked businesses
const unlinkedBusinesses = db.prepare('SELECT * FROM sheffield_businesses WHERE person_id IS NULL').all();

console.log('Checking', unlinkedBusinesses.length.toLocaleString(), 'unlinked businesses...\n');

const nameMatches = [];

unlinkedBusinesses.forEach((business, index) => {
    if (index % 5000 === 0 && index > 0) {
        console.log(`Progress: ${index.toLocaleString()} / ${unlinkedBusinesses.length.toLocaleString()}`);
    }

    // Find people with same name
    const people = db.prepare('SELECT * FROM sheffield_people WHERE name = ?').all(business.full_name);

    if (people.length > 0) {
        nameMatches.push({
            businessName: business.full_name,
            businessOccupation: business.occupation,
            businessAddress: business.address,
            matchingPeopleCount: people.length,
            people: people
        });
    }
});

console.log('\n' + '='.repeat(70));
console.log('SUMMARY');
console.log('='.repeat(70));
console.log('Total unlinked businesses with name matches:', nameMatches.length.toLocaleString());
console.log('');

// Group by number of matching people
const uniqueNameMatches = nameMatches.filter(m => m.matchingPeopleCount === 1);
const multipleNameMatches = nameMatches.filter(m => m.matchingPeopleCount > 1);

console.log('Unique name matches (1 person):', uniqueNameMatches.length.toLocaleString());
console.log('Multiple people with same name:', multipleNameMatches.length.toLocaleString());

console.log('\n' + '='.repeat(70));
console.log('SAMPLE UNIQUE NAME MATCHES (First 25)');
console.log('='.repeat(70) + '\n');

uniqueNameMatches.slice(0, 25).forEach((match, i) => {
    const person = match.people[0];
    console.log(`${i + 1}. ${match.businessName}`);
    console.log(`   Business: ${match.businessOccupation || '(no occupation)'} at ${match.businessAddress}`);
    console.log(`   Person: ${person.profession || '(no profession)'} at ${person.street_address || '(no address)'}`);
    console.log('');
});

console.log('='.repeat(70));
console.log('SAMPLE MULTIPLE NAME MATCHES (First 10)');
console.log('='.repeat(70) + '\n');

multipleNameMatches.slice(0, 10).forEach((match, i) => {
    console.log(`${i + 1}. ${match.businessName} (${match.matchingPeopleCount} people)`);
    console.log(`   Business: ${match.businessOccupation || '(no occupation)'} at ${match.businessAddress}`);
    console.log(`   Matching people:`);
    match.people.forEach((person, j) => {
        console.log(`      ${j + 1}. ${person.profession || '(no profession)'} at ${person.street_address || '(no address)'}`);
    });
    console.log('');
});

db.close();
