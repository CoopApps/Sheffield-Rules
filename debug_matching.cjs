const sqlite3 = require('better-sqlite3');
const fs = require('fs');
const path = require('path');

const db = new sqlite3('./sheffield1867.db');

console.log('=== DEBUG MATCHING ===\n');

// Get sample people
const people = db.prepare(`
  SELECT name, first_name, surname, street_address, birth_year
  FROM sheffield_people
  WHERE surname = 'Smith'
  LIMIT 5
`).all();

console.log('Sample people from database:');
people.forEach((p, i) => {
  console.log(`${i + 1}. Name: "${p.name}" | First: "${p.first_name}" | Surname: "${p.surname}"`);
  console.log(`   Address: "${p.street_address}" | Birth Year: ${p.birth_year}`);
});

// Get sample businesses
console.log('\n=== Sample businesses from whites/ ===\n');

function parseCSVLine(line) {
  const result = [];
  let current = '';
  let inQuotes = false;

  for (let i = 0; i < line.length; i++) {
    const char = line[i];

    if (char === '"') {
      inQuotes = !inQuotes;
    } else if (char === ',' && !inQuotes) {
      result.push(current.trim());
      current = '';
    } else {
      current += char;
    }
  }
  result.push(current.trim());

  return result;
}

const testFile = path.join('./whites', 'sheffield_1871_S.csv');
const content = fs.readFileSync(testFile, 'utf-8');
const lines = content.split('\n');

console.log('First 5 businesses from S file:');
for (let i = 1; i <= 5 && i < lines.length; i++) {
  const line = lines[i].trim();
  if (!line) continue;

  const values = parseCSVLine(line);
  console.log(`\n${i}. Raw values:`, values);
  console.log(`   Surname: "${values[0]}" | Forename: "${values[1]}"`);
  console.log(`   Occupation: "${values[3]}" | Address: "${values[4]}"`);
}

// Check if people are actually 18 years old (born 1853)
console.log('\n=== BIRTH YEAR CHECK ===');
const birthYears = db.prepare(`
  SELECT birth_year, COUNT(*) as count
  FROM sheffield_people
  GROUP BY birth_year
  ORDER BY birth_year DESC
  LIMIT 10
`).all();

console.log('Birth year distribution:');
birthYears.forEach(b => {
  console.log(`  ${b.birth_year}: ${b.count} people`);
});

// The issue: businesses are from 1871, but people are born in 1853
// In 1871, these people would be 18 years old - too young to own businesses!
console.log('\n=== KEY INSIGHT ===');
console.log('Business directory is from 1871.');
console.log('Current people in database were born in 1853.');
console.log('In 1871, these people would be only 18 years old.');
console.log('They would NOT be business owners yet!');
console.log('\nWe need to import OLDER people (born earlier years) to match businesses.');

db.close();
console.log('\nDone!');
