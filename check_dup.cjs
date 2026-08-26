const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('=== Checking Benjamin Harrop in 1792 ===\n');

const unmatched = db.prepare(`
    SELECT * FROM unmatched_sheffield
    WHERE name LIKE '%Benjamin Harrop%' AND year = 1792
`).all();

console.log('Found in unmatched_sheffield:', unmatched.length);
unmatched.forEach((r, i) => {
    console.log('\nRecord', i+1);
    console.log('  Name:', r.name);
    console.log('  Age:', r.age, 'Birth Year:', r.birth_year);
    console.log('  Birth Place:', r.birth_place);
    console.log('  Relation:', r.relation);
    console.log('  Civil Parish:', r.civil_parish);
    console.log('  Piece/Folio/Page:', r.piece, r.folio, r.page);
});

console.log('\n\n=== Top duplicate names in 1792 ===\n');
const duplicates = db.prepare(`
    SELECT name, COUNT(*) as count
    FROM unmatched_sheffield
    WHERE year = 1792
    GROUP BY name
    HAVING count > 1
    ORDER BY count DESC
    LIMIT 10
`).all();

duplicates.forEach(d => {
    console.log(`  ${d.name}: ${d.count} occurrences`);
});

db.close();
