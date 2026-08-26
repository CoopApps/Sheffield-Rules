const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

const daughter = db.prepare('SELECT * FROM unmatched_genealogy WHERE relation = ? LIMIT 1').get('Daughter');

console.log('Sample daughter:');
console.log('  Name:', daughter.name);
console.log('  Year:', daughter.year);
console.log('  Birth Year:', daughter.birth_year);
console.log('');

function getSurname(fullName) {
    if (!fullName) return '';
    const parts = fullName.trim().split(/\s+/);
    return parts[parts.length - 1];
}

const surname = getSurname(daughter.name);
console.log('  Surname:', surname);
console.log('');

const menWithSameSurname = db.prepare('SELECT name, census_household_members FROM sheffield_people WHERE surname = ? AND census_gender = ? LIMIT 5').all(surname, 'Male');

console.log('Men with same surname in sheffield_people:');
menWithSameSurname.forEach(m => {
    console.log('  ' + m.name);
    console.log('    Household: ' + (m.census_household_members ? m.census_household_members.substring(0, 150) : 'none'));

    if (m.census_household_members) {
        const members = m.census_household_members.split(',').map(n => n.trim());
        console.log('    Members array:', members.slice(0, 5));
        console.log('    Includes "' + daughter.name + '"?', members.includes(daughter.name));
    }
    console.log('');
});

db.close();
