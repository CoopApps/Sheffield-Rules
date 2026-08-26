const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('NAME MATCHES - Unlinked Businesses with People Who Have Professions');
console.log('='.repeat(70) + '\n');

// Get first 1000 unlinked businesses
const unlinkedBusinesses = db.prepare('SELECT * FROM sheffield_businesses WHERE person_id IS NULL LIMIT 1000').all();

console.log('Checking', unlinkedBusinesses.length.toLocaleString(), 'unlinked businesses...\n');

let count = 0;

for (const business of unlinkedBusinesses) {
    if (count >= 25) break;

    // Find people with same name who have a profession
    const people = db.prepare(`
        SELECT * FROM sheffield_people
        WHERE name = ?
        AND profession IS NOT NULL
        AND profession != ''
    `).all(business.full_name);

    if (people.length > 0) {
        people.forEach(person => {
            if (count >= 25) return;

            count++;
            console.log(`${count}. ${business.full_name}`);
            console.log(`   Business: ${business.occupation || '(no occupation listed)'}`);
            console.log(`   Business Address: ${business.address}`);
            console.log(`   Person Profession: ${person.profession}`);
            console.log(`   Person Address: ${person.street_address || '(none)'}`);
            console.log(`   Multiple people with this name: ${people.length > 1 ? 'YES (' + people.length + ' people)' : 'NO'}`);
            console.log('');
        });
    }
}

console.log('='.repeat(70));
console.log('Total name matches shown:', count);

db.close();
