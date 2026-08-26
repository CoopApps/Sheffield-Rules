const sqlite3 = require('better-sqlite3');

const db = new sqlite3('Sheffield1867.db');

console.log('='.repeat(70));
console.log('CHECKING OVERLAP: SHEFFIELD_PEOPLE vs WHITE\'S DIRECTORY');
console.log('='.repeat(70) + '\n');

const totalPeople = db.prepare('SELECT COUNT(*) as count FROM sheffield_people').get();
const totalBusinesses = db.prepare('SELECT COUNT(*) as count FROM sheffield_businesses').get();

console.log('Total people in database: ' + totalPeople.count.toLocaleString());
console.log('Total businesses in White\'s Directory: ' + totalBusinesses.count.toLocaleString() + '\n');

// Find people who have matching names in White's Directory
const matches = db.prepare(`
    SELECT COUNT(DISTINCT p.id) as count
    FROM sheffield_people p
    JOIN sheffield_businesses b
        ON LOWER(TRIM(p.surname)) = LOWER(TRIM(b.surname))
        AND LOWER(TRIM(p.first_name)) = LOWER(TRIM(b.forename))
`).get();

console.log('People with matching names in White\'s Directory: ' + matches.count.toLocaleString());

// Find people who have professions and are also in White's
const withProfFromWhites = db.prepare(`
    SELECT COUNT(DISTINCT p.id) as count
    FROM sheffield_people p
    WHERE p.profession IS NOT NULL
    AND EXISTS (
        SELECT 1 FROM sheffield_businesses b
        WHERE LOWER(TRIM(p.surname)) = LOWER(TRIM(b.surname))
        AND LOWER(TRIM(p.first_name)) = LOWER(TRIM(b.forename))
    )
`).get();

console.log('People with professions who are also in White\'s: ' + withProfFromWhites.count.toLocaleString());

// Find people who are employers/patrons and in White's
const patronsInWhites = db.prepare(`
    SELECT COUNT(DISTINCT p.id) as count
    FROM sheffield_people p
    WHERE p.is_patron = 1
    AND EXISTS (
        SELECT 1 FROM sheffield_businesses b
        WHERE LOWER(TRIM(p.surname)) = LOWER(TRIM(b.surname))
        AND LOWER(TRIM(p.first_name)) = LOWER(TRIM(b.forename))
    )
`).get();

console.log('Patrons who are also in White\'s: ' + patronsInWhites.count.toLocaleString());

console.log('\n' + '='.repeat(70));
console.log('SAMPLE PEOPLE FOUND IN BOTH (UNIQUE PEOPLE ONLY):');
console.log('='.repeat(70) + '\n');

// Get unique people first, then show their matches
const uniquePeople = db.prepare(`
    SELECT DISTINCT
        p.id,
        p.name,
        p.birth_year,
        p.profession,
        p.street_address,
        p.is_patron
    FROM sheffield_people p
    JOIN sheffield_businesses b
        ON LOWER(TRIM(p.surname)) = LOWER(TRIM(b.surname))
        AND LOWER(TRIM(p.first_name)) = LOWER(TRIM(b.forename))
    WHERE p.profession IS NOT NULL
    LIMIT 20
`).all();

// For each unique person, get their White's matches
uniquePeople.forEach((person, i) => {
    const matches = db.prepare(`
        SELECT occupation, address
        FROM sheffield_businesses
        WHERE LOWER(TRIM(surname)) = LOWER(TRIM(?))
        AND LOWER(TRIM(forename)) = LOWER(TRIM(?))
    `).all(person.name.split(' ').pop(), person.name.split(' ')[0]);

    console.log((i + 1) + '. ' + person.name + ' (' + person.birth_year + ')' + (person.is_patron ? ' [PATRON]' : ''));
    console.log('   Census profession: ' + person.profession);
    if (person.street_address) console.log('   Home address: ' + person.street_address);
    console.log('   White\'s Directory matches: ' + matches.length);

    matches.slice(0, 3).forEach((m, j) => {
        console.log('     ' + (j + 1) + '. ' + (m.occupation || 'N/A') + ' at ' + m.address);
    });
    if (matches.length > 3) {
        console.log('     ... and ' + (matches.length - 3) + ' more');
    }
    console.log('');
});

console.log('='.repeat(70));
console.log('PATRONS IN WHITE\'S DIRECTORY (UNIQUE PATRONS):');
console.log('='.repeat(70) + '\n');

const uniquePatrons = db.prepare(`
    SELECT DISTINCT
        p.id,
        p.name,
        p.birth_year,
        p.profession
    FROM sheffield_people p
    JOIN sheffield_businesses b
        ON LOWER(TRIM(p.surname)) = LOWER(TRIM(b.surname))
        AND LOWER(TRIM(p.first_name)) = LOWER(TRIM(b.forename))
    WHERE p.is_patron = 1
    ORDER BY p.name
    LIMIT 20
`).all();

uniquePatrons.forEach((patron, i) => {
    const matches = db.prepare(`
        SELECT occupation, address
        FROM sheffield_businesses
        WHERE LOWER(TRIM(surname)) = LOWER(TRIM(?))
        AND LOWER(TRIM(forename)) = LOWER(TRIM(?))
    `).all(patron.name.split(' ').pop(), patron.name.split(' ')[0]);

    console.log((i + 1) + '. ' + patron.name + ' (' + patron.birth_year + ')');
    console.log('   Census: ' + patron.profession);
    console.log('   White\'s matches: ' + matches.length);

    matches.slice(0, 2).forEach((m, j) => {
        console.log('     ' + (j + 1) + '. ' + (m.occupation || 'N/A') + ' at ' + m.address);
    });
    if (matches.length > 2) {
        console.log('     ... and ' + (matches.length - 2) + ' more');
    }
    console.log('');
});

db.close();

console.log('✓ Analysis complete!');
