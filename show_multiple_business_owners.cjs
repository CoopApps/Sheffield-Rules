const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('PEOPLE WITH MULTIPLE BUSINESSES');
console.log('='.repeat(70) + '\n');

const multipleBusinessOwners = db.prepare(`
    SELECT p.id, p.name, p.profession, p.street_address, COUNT(b.id) as business_count
    FROM sheffield_people p
    JOIN sheffield_businesses b ON p.id = b.person_id
    GROUP BY p.id
    HAVING COUNT(b.id) > 1
    ORDER BY business_count DESC, p.name
`).all();

console.log('Total people with multiple businesses:', multipleBusinessOwners.length.toLocaleString());
console.log('');

multipleBusinessOwners.forEach((owner, index) => {
    const businesses = db.prepare(`
        SELECT full_name, occupation, address, is_home_business, profession_match
        FROM sheffield_businesses
        WHERE person_id = ?
        ORDER BY occupation
    `).all(owner.id);

    console.log('='.repeat(70));
    console.log(`${index + 1}. ${owner.name}`);
    console.log('   Person Profession: ' + (owner.profession || '(none)'));
    console.log('   Person Address: ' + (owner.street_address || '(none)'));
    console.log('   Number of businesses: ' + owner.business_count);
    console.log('');

    businesses.forEach((b, i) => {
        const matchType = [];
        if (b.is_home_business) matchType.push('ADDRESS');
        if (b.profession_match) matchType.push('PROFESSION');

        console.log(`   Business ${i + 1}:`);
        console.log('      Occupation: ' + b.occupation);
        console.log('      Address: ' + b.address);
        console.log('      Match basis: ' + matchType.join(' + '));
    });
    console.log('');
});

db.close();
