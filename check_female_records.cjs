const Database = require('better-sqlite3');
const db = new Database('./Sheffield1867.db');

console.log('\n=== Checking for Female records in database ===\n');

// Check how many female records exist
const femaleCount = db.prepare(`
  SELECT COUNT(*) as count
  FROM sheffield_players
  WHERE census_gender = 'Female'
`).get();

console.log(`Total female records: ${femaleCount.count}`);

// Check for actual Wife/Daughter records with details
const femaleDetails = db.prepare(`
  SELECT name, first_name, surname, birth_year, census_relation, census_gender,
         census_household_schedule, ecclesiastical_parish
  FROM sheffield_players
  WHERE census_gender = 'Female'
  LIMIT 20
`).all();

if (femaleDetails.length > 0) {
  console.log(`\nFound ${femaleDetails.length} female records:\n`);
  femaleDetails.forEach((f, i) => {
    console.log(`${i + 1}. ${f.first_name || f.name} ${f.surname || ''} (b. ${f.birth_year || 'N/A'})`);
    console.log(`   Relation: ${f.census_relation}, Household: ${f.census_household_schedule}, Parish: ${f.ecclesiastical_parish}`);
  });
} else {
  console.log('\nNo female records found in database.');
  console.log('\nThis suggests that the database only contains male players,');
  console.log('and household member information (wives, daughters) is NOT stored');
  console.log('as separate records in the database.');
  console.log('\nThe census_household_schedule field likely references external census data,');
  console.log('not other records within this database.');
}

// Let's check one more thing - look at the Samuel Jubb "Daughter" record we found earlier
console.log('\n\n=== Examining the "Daughter" record (Samuel Jubb) ===\n');
const samuelJubb = db.prepare(`
  SELECT *
  FROM sheffield_players
  WHERE name LIKE '%Jubb%' AND census_relation = 'Daughter'
`).get();

if (samuelJubb) {
  console.log('Name:', samuelJubb.name);
  console.log('First Name:', samuelJubb.first_name);
  console.log('Surname:', samuelJubb.surname);
  console.log('Birth Year:', samuelJubb.birth_year);
  console.log('Gender:', samuelJubb.census_gender);
  console.log('Relation:', samuelJubb.census_relation);
  console.log('\nNote: This appears to be a data error - a male player');
  console.log('incorrectly marked as "Daughter".');
}

db.close();
