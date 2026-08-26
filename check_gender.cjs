const Database = require('better-sqlite3');
const db = new Database('D:/projects/Saturday at Three/Sheffield1867.db');

// Check gender distribution in sheffield_footballers
const genderDist = db.prepare(`
  SELECT p.census_gender, COUNT(*) as count
  FROM sheffield_people p
  JOIN sheffield_footballers f ON p.unique_id = f.person_id
  GROUP BY p.census_gender
`).all();

console.log('Gender distribution in sheffield_footballers:');
genderDist.forEach(row => {
  console.log(`  ${row.census_gender || 'NULL'}: ${row.count}`);
});

// Get total count
const total = db.prepare(`
  SELECT COUNT(*) as count FROM sheffield_footballers
`).get();
console.log(`\nTotal footballers: ${total.count}`);

// Show some female examples
const females = db.prepare(`
  SELECT f.id, p.first_name, p.middle_name, p.surname, p.census_gender
  FROM sheffield_footballers f
  JOIN sheffield_people p ON f.person_id = p.unique_id
  WHERE p.census_gender = 'F'
  LIMIT 10
`).all();

console.log('\nExample female footballers:');
females.forEach(p => {
  const name = [p.first_name, p.middle_name, p.surname].filter(Boolean).join(' ');
  console.log(`  ${p.id}: ${name}`);
});

db.close();
