const Database = require('better-sqlite3');
const fs = require('fs');
const { parse } = require('csv-parse/sync');

// Helper function to normalize names for matching
function normalizeName(name) {
  if (!name) return '';
  return name.toLowerCase()
    .replace(/[.,\-]/g, ' ')
    .replace(/\s+/g, ' ')
    .trim();
}

// Load Sheffield Census data
console.log('Loading Sheffield Census 1827 data...');
const sheffieldCensusPath = 'Sheffield Census/1827 - done.csv';
const sheffieldCensusContent = fs.readFileSync(sheffieldCensusPath, 'utf-8');
const sheffieldRecords = parse(sheffieldCensusContent, {
  columns: true,
  skip_empty_lines: true
});

// Load all Genealogy files for 1827
console.log('Loading Genealogy 1827 data...');
const genealogyRecords = [];
const genealogyDir = 'Genealogy';
const genealogyFiles = fs.readdirSync(genealogyDir)
  .filter(f => f.includes('tg_1827_') && f.endsWith('.csv'));

for (const file of genealogyFiles) {
  const content = fs.readFileSync(`${genealogyDir}/${file}`, 'utf-8');
  const records = parse(content, {
    columns: true,
    skip_empty_lines: true
  });
  genealogyRecords.push(...records);
}

// Load matched records from database
const db = new Database('Sheffield1867.db');
const matchedRecords = db.prepare("SELECT name FROM sheffield_people WHERE source LIKE '1827_matched_%'").all();
const matchedNames = new Set(matchedRecords.map(r => normalizeName(r.name)));
db.close();

console.log(`\nTotal Sheffield Census records: ${sheffieldRecords.length}`);
console.log(`Total Genealogy records: ${genealogyRecords.length}`);
console.log(`Total matched records in database: ${matchedNames.size}\n`);

// Find unmatched Sheffield Census records
const unmatchedSheffield = sheffieldRecords.filter(r => !matchedNames.has(normalizeName(r.NAME)));
console.log(`=== UNMATCHED SHEFFIELD CENSUS RECORDS: ${unmatchedSheffield.length} ===`);

// Show some examples
console.log('\nFirst 20 unmatched Sheffield Census records:');
for (let i = 0; i < Math.min(20, unmatchedSheffield.length); i++) {
  const record = unmatchedSheffield[i];
  console.log(`${i + 1}. ${record.NAME}`);
  console.log(`   Parish: ${record['ECCLESIASTICAL PARISH']}, Civil: ${record['CIVIL PARISH']}`);
  console.log(`   Age: ${record.AGE}, Gender: ${record.GENDER}, Relation: ${record.RELATION}`);
}

// Find unmatched Genealogy records
const unmatchedGenealogy = genealogyRecords.filter(r => !matchedNames.has(normalizeName(r.Name)));
console.log(`\n=== UNMATCHED GENEALOGY RECORDS: ${unmatchedGenealogy.length} ===`);

// Show some examples
console.log('\nFirst 20 unmatched Genealogy records:');
for (let i = 0; i < Math.min(20, unmatchedGenealogy.length); i++) {
  const record = unmatchedGenealogy[i];
  console.log(`${i + 1}. ${record.Name}`);
  console.log(`   Address: ${record.Address}`);
  console.log(`   Profession: ${record.Profession || '(none)'}`);
  console.log(`   Spouse: ${record.Spouse || '(none)'}`);
  console.log(`   Relation: ${record.Relation}`);
}

// Analyze patterns in unmatched records
console.log('\n=== ANALYSIS OF UNMATCHED GENEALOGY RECORDS ===');

const byRelation = {};
unmatchedGenealogy.forEach(r => {
  const rel = r.Relation || 'Unknown';
  byRelation[rel] = (byRelation[rel] || 0) + 1;
});

console.log('\nUnmatched Genealogy records by Relation:');
Object.entries(byRelation)
  .sort((a, b) => b[1] - a[1])
  .slice(0, 10)
  .forEach(([rel, count]) => {
    console.log(`  ${rel}: ${count} (${((count/unmatchedGenealogy.length)*100).toFixed(1)}%)`);
  });

// Check if unmatched genealogy records have spouses
const unmatchedWithSpouses = unmatchedGenealogy.filter(r => r.Spouse);
console.log(`\nUnmatched Genealogy records with spouses: ${unmatchedWithSpouses.length} (${((unmatchedWithSpouses.length/unmatchedGenealogy.length)*100).toFixed(1)}%)`);

// Analyze unmatched Sheffield records
console.log('\n=== ANALYSIS OF UNMATCHED SHEFFIELD CENSUS RECORDS ===');

const shefByRelation = {};
unmatchedSheffield.forEach(r => {
  const rel = r.RELATION || 'Unknown';
  shefByRelation[rel] = (shefByRelation[rel] || 0) + 1;
});

console.log('\nUnmatched Sheffield Census records by Relation:');
Object.entries(shefByRelation)
  .sort((a, b) => b[1] - a[1])
  .slice(0, 10)
  .forEach(([rel, count]) => {
    console.log(`  ${rel}: ${count} (${((count/unmatchedSheffield.length)*100).toFixed(1)}%)`);
  });

console.log('\n=== SUMMARY ===');
console.log(`Sheffield Census: ${sheffieldRecords.length} records`);
console.log(`Genealogy: ${genealogyRecords.length} records`);
console.log(`Matched and imported: ${matchedNames.size} records`);
console.log(`\nUnmatched from Sheffield Census: ${unmatchedSheffield.length} (${((unmatchedSheffield.length/sheffieldRecords.length)*100).toFixed(1)}%)`);
console.log(`Unmatched from Genealogy: ${unmatchedGenealogy.length} (${((unmatchedGenealogy.length/genealogyRecords.length)*100).toFixed(1)}%)`);

console.log('\nDone!');
