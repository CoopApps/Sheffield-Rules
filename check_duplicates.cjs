const sqlite3 = require('better-sqlite3');
const db = new sqlite3('./sheffield1867.db');

console.log('=== CHECKING FOR DUPLICATES ===\n');

// Count total people
const total = db.prepare('SELECT COUNT(*) as count FROM sheffield_people').get();
console.log(`Total people in database: ${total.count}\n`);

// Check for exact name duplicates
const nameDupes = db.prepare(`
  SELECT name, birth_year, COUNT(*) as count
  FROM sheffield_people
  GROUP BY name, birth_year
  HAVING count > 1
  ORDER BY count DESC
  LIMIT 20
`).all();

console.log('Top 20 name+birth_year duplicates:');
nameDupes.forEach(d => {
  console.log(`  ${d.name} (${d.birth_year}): ${d.count} records`);
});

// Check one specific duplicate
if (nameDupes.length > 0) {
  const first = nameDupes[0];
  console.log(`\n=== DETAILED LOOK AT: ${first.name} (${first.birth_year}) ===`);

  const records = db.prepare(`
    SELECT name, street_address, civil_parish, profession, source, relation
    FROM sheffield_people
    WHERE name = ? AND birth_year = ?
  `).all(first.name, first.birth_year);

  records.forEach((r, i) => {
    console.log(`\n${i + 1}. ${r.name}`);
    console.log(`   Address: ${r.street_address || 'none'}`);
    console.log(`   Parish: ${r.civil_parish}`);
    console.log(`   Profession: ${r.profession || 'none'}`);
    console.log(`   Relation: ${r.relation}`);
    console.log(`   Source: ${r.source}`);
  });
}

db.close();
console.log('\nDone!');
