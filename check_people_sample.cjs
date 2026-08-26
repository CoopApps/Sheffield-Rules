const sqlite3 = require('better-sqlite3');
const db = new sqlite3('./sheffield1867.db');

console.log('=== SHEFFIELD_PEOPLE SAMPLE ===\n');

// Check current count
const count = db.prepare('SELECT COUNT(*) as count FROM sheffield_people').get();
console.log(`Total people in database: ${count.count}\n`);

// Get sample of people with professions
console.log('Sample of people with street addresses and professions:');
const sample = db.prepare(`
  SELECT name, first_name, surname, street_address, civil_parish, profession, source
  FROM sheffield_people
  WHERE profession IS NOT NULL AND street_address IS NOT NULL
  LIMIT 10
`).all();

sample.forEach((p, i) => {
  console.log(`${i + 1}. ${p.name} - ${p.profession}`);
  console.log(`   Address: ${p.street_address}, ${p.civil_parish}`);
  console.log(`   Source: ${p.source}`);
  console.log();
});

// Check for "Smith" surnames
console.log('=== SMITH SURNAMES ===');
const smiths = db.prepare(`
  SELECT name, first_name, surname, street_address, profession
  FROM sheffield_people
  WHERE surname = 'Smith'
  LIMIT 5
`).all();

smiths.forEach((p, i) => {
  console.log(`${i + 1}. ${p.name} (${p.first_name} ${p.surname})`);
  console.log(`   Address: ${p.street_address}, Profession: ${p.profession || 'none'}`);
});

db.close();
console.log('\nDone!');
