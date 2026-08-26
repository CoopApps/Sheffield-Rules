const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('COMPANY PATTERN BUSINESSES (& Co, & Son, etc)');
console.log('='.repeat(70) + '\n');

// Find businesses with company patterns but no obvious person name
const companyPatterns = db.prepare(`
    SELECT * FROM sheffield_businesses
    WHERE (
        LOWER(full_name) LIKE '%& co%'
        OR LOWER(full_name) LIKE '%& son%'
        OR LOWER(full_name) LIKE '%& sons%'
        OR LOWER(full_name) LIKE '%& brothers%'
        OR LOWER(full_name) LIKE '%& bros%'
        OR LOWER(full_name) LIKE '%brothers%'
        OR LOWER(full_name) LIKE 'sheffield%'
        OR LOWER(full_name) LIKE '%coal co%'
        OR LOWER(full_name) LIKE '%bank%'
        OR full_name LIKE '-%'
        OR full_name LIKE '*%'
    )
`).all();

const total = companyPatterns.length;
const linked = companyPatterns.filter(b => b.person_id !== null).length;
const unlinked = total - linked;

console.log('Total company pattern businesses:', total.toLocaleString());
console.log('  - Linked to a person:', linked.toLocaleString());
console.log('  - NOT linked (no owner):', unlinked.toLocaleString());
console.log('Percentage unlinked:', ((unlinked/total)*100).toFixed(1) + '%');

console.log('\n' + '='.repeat(70));
console.log('SAMPLE LINKED COMPANY BUSINESSES (First 25)');
console.log('='.repeat(70) + '\n');

const linkedSamples = companyPatterns.filter(b => b.person_id !== null).slice(0, 25);
linkedSamples.forEach((b, i) => {
    const person = db.prepare('SELECT * FROM sheffield_people WHERE id = ?').get(b.person_id);
    console.log(`${i+1}. "${b.full_name}"`);
    console.log(`   Business: ${b.occupation || '(no occupation)'} at ${b.address}`);
    if (person) {
        console.log(`   Owner: ${person.name} - ${person.profession || '(no profession)'}`);
    }
    console.log('');
});

console.log('='.repeat(70));
console.log('SAMPLE UNLINKED COMPANY BUSINESSES (First 50)');
console.log('='.repeat(70) + '\n');

const unlinkedSamples = companyPatterns.filter(b => b.person_id === null).slice(0, 50);
unlinkedSamples.forEach((b, i) => {
    console.log(`${i+1}. "${b.full_name}"`);
    console.log(`   ${b.occupation || '(no occupation)'} at ${b.address}`);
    console.log('');
});

if (unlinked > 50) {
    console.log(`... and ${(unlinked - 50).toLocaleString()} more unlinked company businesses\n`);
}

db.close();
