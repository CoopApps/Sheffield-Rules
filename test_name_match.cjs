const sqlite3 = require('better-sqlite3');

const db = new sqlite3('Sheffield1867.db');

// Get the person
const person = db.prepare('SELECT * FROM sheffield_people WHERE surname = ? AND first_name LIKE ?').get('Abrahams', '%Chas%');
console.log('Person from sheffield_people:');
console.log('  Name:', person.name);
console.log('  First name:', person.first_name);
console.log('  Surname:', person.surname);

// Get the business
const biz = db.prepare('SELECT * FROM sheffield_businesses WHERE surname = ? LIMIT 1').get('Abrahams');
console.log('\nBusiness from sheffield_businesses:');
console.log('  Forename:', biz.forename);
console.log('  Surname:', biz.surname);
console.log('  Occupation:', biz.occupation);

// Test name matching
const nameAbbreviations = {
    'charles': ['chas', 'charlie', 'c']
};

function namesMatch(name1, name2) {
    if (!name1 || !name2) return false;

    const n1 = name1.toLowerCase().trim().replace(/\./g, '');
    const n2 = name2.toLowerCase().trim().replace(/\./g, '');

    // Exact match
    if (n1 === n2) return true;

    // Try abbreviations
    for (const [fullName, abbrevs] of Object.entries(nameAbbreviations)) {
        if ((n1 === fullName && abbrevs.includes(n2)) ||
            (n2 === fullName && abbrevs.includes(n1)) ||
            (abbrevs.includes(n1) && abbrevs.includes(n2))) {
            return true;
        }
    }

    return false;
}

console.log('\nName matching tests:');
console.log('  "' + person.first_name + '" vs "' + biz.forename + '": ' + namesMatch(person.first_name, biz.forename));
console.log('  "Chas S" vs "C S": ' + namesMatch('Chas S', 'C S'));
console.log('  "Chas" vs "C": ' + namesMatch('Chas', 'C'));

db.close();
