const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('SEARCHING FOR BEARDSHAW');
console.log('='.repeat(70) + '\n');

const people = db.prepare('SELECT * FROM sheffield_people WHERE name LIKE ?').all('%Beardshaw%');

console.log('People with surname Beardshaw:', people.length);
console.log('');

people.forEach((p, i) => {
    console.log(`${i+1}. ${p.name}`);
    console.log(`   Profession: ${p.profession || '(none)'}`);
    console.log(`   Address: ${p.street_address || '(none)'}`);
    console.log('');
});

const businesses = db.prepare('SELECT * FROM sheffield_businesses WHERE full_name LIKE ?').all('%Beardshaw%');

console.log('\n' + '='.repeat(70));
console.log('Businesses with Beardshaw:', businesses.length);
console.log('='.repeat(70) + '\n');

businesses.forEach((b, i) => {
    console.log(`${i+1}. "${b.full_name}"`);
    console.log(`   ${b.occupation || '(none)'} at ${b.address}`);
    console.log(`   Linked to person_id: ${b.person_id || 'NO'}`);

    if (b.person_id) {
        const person = db.prepare('SELECT * FROM sheffield_people WHERE id = ?').get(b.person_id);
        if (person) {
            console.log(`   Owner: ${person.name} - ${person.profession || '(no profession)'}`);
        }
    }
    console.log('');
});

db.close();
