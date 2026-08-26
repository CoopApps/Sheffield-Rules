const fs = require('fs');
const { parse } = require('csv-parse/sync');
const Database = require('better-sqlite3');

function normalizeName(name) {
  if (!name) return '';
  return name.toLowerCase()
    .replace(/[.,\-]/g, ' ')
    .replace(/\s+/g, ' ')
    .trim();
}

// Load data
console.log('Loading data...\n');

const sheffieldContent = fs.readFileSync('Sheffield Census/1827 - done.csv', 'utf-8');
const sheffieldRecords = parse(sheffieldContent, { columns: true, skip_empty_lines: true });

const genealogyRecords = [];
const genealogyDir = 'Genealogy';
const genealogyFiles = fs.readdirSync(genealogyDir)
  .filter(f => f.includes('tg_1827_') && f.endsWith('.csv'));

for (const file of genealogyFiles) {
  const content = fs.readFileSync(`${genealogyDir}/${file}`, 'utf-8');
  const records = parse(content, { columns: true, skip_empty_lines: true });
  genealogyRecords.push(...records);
}

// Get matched names from database
const db = new Database('Sheffield1867.db');
const matchedRecords = db.prepare("SELECT name FROM sheffield_people WHERE source LIKE '1827_matched_%'").all();
const matchedNames = new Set(matchedRecords.map(r => normalizeName(r.name)));
db.close();

// Filter to unmatched only
const unmatchedSheffield = sheffieldRecords.filter(r => {
  const normalized = normalizeName(r.NAME);
  return normalized && !matchedNames.has(normalized);
});

const unmatchedGenealogy = genealogyRecords.filter(r => {
  const normalized = normalizeName(r.Name);
  return normalized && !matchedNames.has(normalized);
});

console.log(`Unmatched Sheffield Census: ${unmatchedSheffield.length}`);
console.log(`Unmatched Genealogy: ${unmatchedGenealogy.length}`);

// Create JSON file
const data = {
  sheffield: unmatchedSheffield,
  genealogy: unmatchedGenealogy,
  generatedAt: new Date().toISOString()
};

fs.writeFileSync('unmatched_data_1827.json', JSON.stringify(data, null, 2));

console.log('\n✓ Generated unmatched_data_1827.json');
console.log('\nTo review unmatched records:');
console.log('1. Open review_unmatched.html in your web browser');
console.log('2. Search and filter records');
console.log('3. Click on records from each side to match them');
console.log('\nNote: The web interface requires a local web server to save matches.');
console.log('You can use: npx http-server -p 8080');
