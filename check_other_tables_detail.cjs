const sqlite3 = require('better-sqlite3');
const db = new sqlite3('./sheffield1867.db');

console.log('=== CHECKING OTHER TABLES ===\n');

// Check workhouse
console.log('=== SHEFFIELD_WORKHOUSE (27 records) ===');
const workhouseSchema = db.prepare('PRAGMA table_info(sheffield_workhouse)').all();
console.log('Columns:', workhouseSchema.map(c => c.name).join(', '));

const workhouseSample = db.prepare('SELECT * FROM sheffield_workhouse LIMIT 3').all();
console.log('\nSample records:');
workhouseSample.forEach((rec, i) => {
  console.log(`\n${i + 1}. ${rec.name} (age ${rec.census_age}, born ${rec.birth_year})`);
  console.log(`   Address: ${rec.street_address || 'N/A'}`);
  console.log(`   Profession: ${rec.profession || 'N/A'}`);
});

// Check asylum
console.log('\n\n=== SHEFFIELD_ASYLUM (13 records) ===');
const asylumSchema = db.prepare('PRAGMA table_info(sheffield_asylum)').all();
console.log('Columns:', asylumSchema.map(c => c.name).join(', '));

const asylumSample = db.prepare('SELECT * FROM sheffield_asylum LIMIT 3').all();
console.log('\nSample records:');
asylumSample.forEach((rec, i) => {
  console.log(`\n${i + 1}. ${rec.name} (age ${rec.census_age}, born ${rec.birth_year})`);
  console.log(`   Asylum: ${rec.asylum_name || 'N/A'}`);
  console.log(`   Profession: ${rec.profession || 'N/A'}`);
});

// Check businesses
console.log('\n\n=== SHEFFIELD_BUSINESSES (23,282 records) ===');
const businessSchema = db.prepare('PRAGMA table_info(sheffield_businesses)').all();
console.log('Columns:', businessSchema.map(c => c.name).join(', '));

const businessSample = db.prepare('SELECT * FROM sheffield_businesses LIMIT 5').all();
console.log('\nSample records:');
businessSample.forEach((rec, i) => {
  console.log(`\n${i + 1}. ${rec.forename} ${rec.surname}`);
  console.log(`   Occupation: ${rec.occupation || 'N/A'}`);
  console.log(`   Address: ${rec.address || 'N/A'}`);
  console.log(`   Year: ${rec.year}`);
  console.log(`   Source: ${rec.source}`);
});

console.log('\n\n=== ANALYSIS ===');
console.log('These are SEPARATE data sources:');
console.log('  - sheffield_workhouse: People from Ecclesall Bierlow Union Workhouse');
console.log('  - sheffield_asylum: People from West Yorkshire County Lunatic Asylum');
console.log('  - sheffield_businesses: Business directory entries (1871)');
console.log('\nThey should NOT be cleared - they contain supplementary data.');

db.close();
