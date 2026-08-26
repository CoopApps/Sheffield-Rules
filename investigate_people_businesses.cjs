const sqlite3 = require('better-sqlite3');
const db = new sqlite3('./sheffield1867.db');

console.log('=== INVESTIGATING SHEFFIELD_PEOPLE AND BUSINESS LINKS ===\n');

// Check if there's a direct link between people and businesses
const peopleSchema = db.prepare('PRAGMA table_info(sheffield_people)').all();
console.log('sheffield_people columns:');
peopleSchema.forEach(col => {
  console.log(`  - ${col.name} (${col.type})`);
});

// Check for duplicate names in sheffield_people
console.log('\n=== PEOPLE WITH DUPLICATE NAMES ===');
const duplicateNames = db.prepare(`
  SELECT name, COUNT(*) as count
  FROM sheffield_people
  GROUP BY name
  HAVING count > 1
  ORDER BY count DESC
  LIMIT 20
`).all();

console.log('Top 20 duplicate names in sheffield_people:');
duplicateNames.forEach(row => {
  console.log(`  ${row.name}: ${row.count} records`);
});

// Sample a specific duplicate to see the issue
if (duplicateNames.length > 0) {
  const exampleName = duplicateNames[0].name;
  console.log(`\n=== EXAMPLE: All records for "${exampleName}" ===`);
  const examples = db.prepare(`
    SELECT id, name, profession, street_address, birth_year, census_age, is_player, player_id
    FROM sheffield_people
    WHERE name = ?
    LIMIT 10
  `).all(exampleName);
  console.log(JSON.stringify(examples, null, 2));
}

// Check if profession field might contain multiple businesses
console.log('\n=== PROFESSION FIELD ANALYSIS ===');
const longProfessions = db.prepare(`
  SELECT name, profession, LENGTH(profession) as len
  FROM sheffield_people
  WHERE profession IS NOT NULL
  ORDER BY len DESC
  LIMIT 10
`).all();
console.log('People with longest profession strings:');
longProfessions.forEach(row => {
  console.log(`  ${row.name}: "${row.profession}" (${row.len} chars)`);
});

// Check businesses table for duplicate names
console.log('\n=== BUSINESSES TABLE - DUPLICATE NAMES ===');
const businessDupes = db.prepare(`
  SELECT surname || ' ' || forename as full_name, COUNT(*) as count
  FROM sheffield_businesses
  GROUP BY full_name
  HAVING count > 1
  ORDER BY count DESC
  LIMIT 20
`).all();
console.log('Top 20 names with multiple business entries:');
businessDupes.forEach(row => {
  console.log(`  ${row.full_name}: ${row.count} businesses`);
});

// Sample one person with multiple businesses
if (businessDupes.length > 0) {
  const parts = businessDupes[0].full_name.split(' ');
  const surname = parts[0];
  const forename = parts.slice(1).join(' ');

  console.log(`\n=== EXAMPLE: All businesses for "${businessDupes[0].full_name}" ===`);
  const businessExamples = db.prepare(`
    SELECT id, surname, forename, occupation, address, year
    FROM sheffield_businesses
    WHERE surname = ? AND forename = ?
    LIMIT 15
  `).all(surname, forename);
  console.log(JSON.stringify(businessExamples, null, 2));
}

// Check if there's a linking table
console.log('\n=== CHECKING FOR LINKING TABLES ===');
const allTables = db.prepare(`
  SELECT name FROM sqlite_master WHERE type='table' AND name LIKE '%business%' OR name LIKE '%person%' OR name LIKE '%people%'
`).all();
console.log('Tables related to businesses/people:');
allTables.forEach(t => console.log(`  - ${t.name}`));

db.close();
