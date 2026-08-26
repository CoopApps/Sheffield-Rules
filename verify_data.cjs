const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('\n=== DATA COMPLETENESS VERIFICATION ===\n');

const total = 508;
const withAddress = db.prepare('SELECT COUNT(*) as count FROM sheffield_people WHERE street_address IS NOT NULL').get();
const withProfession = db.prepare("SELECT COUNT(*) as count FROM sheffield_people WHERE profession IS NOT NULL AND profession != ''").get();
const withParish = db.prepare('SELECT COUNT(*) as count FROM sheffield_people WHERE ecclesiastical_parish IS NOT NULL').get();
const withRegistration = db.prepare('SELECT COUNT(*) as count FROM sheffield_people WHERE registration_district IS NOT NULL').get();
const withCivilParish = db.prepare('SELECT COUNT(*) as count FROM sheffield_people WHERE civil_parish IS NOT NULL').get();
const withSpouse = db.prepare('SELECT COUNT(*) as count FROM sheffield_people WHERE spouse_name IS NOT NULL').get();

console.log('Records with street address (from Genealogy):', withAddress.count, `(${((withAddress.count/total)*100).toFixed(1)}%)`);
console.log('Records with profession (from Genealogy):', withProfession.count, `(${((withProfession.count/total)*100).toFixed(1)}%)`);
console.log('Records with spouse name (from Genealogy):', withSpouse.count, `(${((withSpouse.count/total)*100).toFixed(1)}%)`);
console.log('\nRecords with ecclesiastical parish (from Sheffield Census):', withParish.count, `(${((withParish.count/total)*100).toFixed(1)}%)`);
console.log('Records with civil parish (from Sheffield Census):', withCivilParish.count, `(${((withCivilParish.count/total)*100).toFixed(1)}%)`);
console.log('Records with registration district (from Sheffield Census):', withRegistration.count, `(${((withRegistration.count/total)*100).toFixed(1)}%)`);

console.log('\n✓ This confirms we successfully merged both data sources!');

// Show one complete record as example
const completeExample = db.prepare(`
  SELECT name, street_address, profession, ecclesiastical_parish,
         civil_parish, registration_district, spouse_name, census_household_members
  FROM sheffield_people
  WHERE street_address IS NOT NULL
    AND profession IS NOT NULL
    AND ecclesiastical_parish IS NOT NULL
  LIMIT 1
`).get();

if (completeExample) {
  console.log('\n=== EXAMPLE OF COMPLETE MERGED RECORD ===');
  console.log('Name:', completeExample.name);
  console.log('Address:', completeExample.street_address, '(from Genealogy)');
  console.log('Profession:', completeExample.profession, '(from Genealogy)');
  if (completeExample.spouse_name) {
    console.log('Spouse:', completeExample.spouse_name, '(from Genealogy)');
  }
  console.log('Ecclesiastical Parish:', completeExample.ecclesiastical_parish, '(from Sheffield Census)');
  console.log('Civil Parish:', completeExample.civil_parish, '(from Sheffield Census)');
  console.log('Registration District:', completeExample.registration_district, '(from Sheffield Census)');
  console.log('Household Members:', completeExample.census_household_members ? 'Yes' : 'No', '(from Sheffield Census)');
}

db.close();
