const sqlite3 = require('better-sqlite3');
const fs = require('fs');
const path = require('path');

const db = new sqlite3('./sheffield1867.db');

console.log('=== IMPORTING 1853 BIRTH YEAR (Age 18 in 1871) ===\n');

// First, let's count how many files we have
const genealogyDir = './genealogy';
const files1853 = fs.readdirSync(genealogyDir).filter(f => f.includes('1853'));
console.log(`Found ${files1853.length} files for 1853:`);
files1853.forEach(f => console.log(`  - ${f}`));
console.log();

// Count total lines across all files
let totalLines = 0;
files1853.forEach(file => {
  const content = fs.readFileSync(path.join(genealogyDir, file), 'utf-8');
  const lines = content.split('\n').length - 2; // subtract header and empty line
  totalLines += lines;
  console.log(`  ${file}: ~${lines} people`);
});
console.log(`\nEstimated total: ~${totalLines} people born in 1853\n`);

// Now let's parse ONE file as a test
console.log('=== PARSING TEST FILE: tg_1853_J.csv ===\n');

const testFile = path.join(genealogyDir, 'tg_1853_J.csv');
const content = fs.readFileSync(testFile, 'utf-8');
const lines = content.split('\n');
const headers = lines[0].split(',').map(h => h.replace(/"/g, ''));

console.log('Headers:', headers.join(' | '));
console.log();

// Parse the CSV manually (simple approach)
const people = [];
for (let i = 1; i < lines.length; i++) {
  const line = lines[i].trim();
  if (!line) continue;

  // Simple CSV parsing (doesn't handle commas within quotes perfectly, but good enough for test)
  const values = line.match(/(".*?"|[^",]+)(?=\s*,|\s*$)/g).map(v => v.replace(/^"|"$/g, ''));

  if (values.length >= 10) {
    const person = {
      name: values[0],
      spouse: values[1],
      address: values[2],
      parish: values[3],
      area: values[4],
      age: values[5],
      birthYear: values[6],
      birthPlace: values[7],
      relation: values[8],
      profession: values[9]
    };
    people.push(person);
  }
}

console.log(`Parsed ${people.length} people from tg_1853_J.csv\n`);

// Show first 5
console.log('First 5 people:');
people.slice(0, 5).forEach((p, i) => {
  console.log(`\n${i + 1}. ${p.name}`);
  console.log(`   Age: ${p.age}, Birth Year: ${p.birthYear}`);
  console.log(`   Relation: ${p.relation}, Profession: ${p.profession || 'N/A'}`);
  console.log(`   Address: ${p.address}`);
  console.log(`   Parish: ${p.parish}, Area: ${p.area || 'N/A'}`);
  console.log(`   Spouse: ${p.spouse || 'N/A'}`);
});

// Check for potential duplicates in this one file
const nameCount = {};
people.forEach(p => {
  nameCount[p.name] = (nameCount[p.name] || 0) + 1;
});
const dupes = Object.entries(nameCount).filter(([name, count]) => count > 1);
console.log(`\nDuplicates in this file: ${dupes.length}`);
if (dupes.length > 0) {
  console.log('Duplicate names:');
  dupes.forEach(([name, count]) => console.log(`  ${name}: ${count}`));
}

db.close();
console.log('\nTest complete!');
