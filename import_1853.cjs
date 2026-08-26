const sqlite3 = require('better-sqlite3');
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const db = new sqlite3('./sheffield1867.db');

console.log('=== IMPORTING 1853 BIRTH YEAR ===\n');

// Helper function to generate UUID
function generateId() {
  return crypto.randomUUID();
}

// Helper function to parse name into parts
function parseName(fullName) {
  const parts = fullName.trim().split(' ');
  if (parts.length === 1) {
    return { first_name: parts[0], middle_name: '', surname: parts[0] };
  } else if (parts.length === 2) {
    return { first_name: parts[0], middle_name: '', surname: parts[1] };
  } else {
    return {
      first_name: parts[0],
      middle_name: parts.slice(1, -1).join(' '),
      surname: parts[parts.length - 1]
    };
  }
}

// Helper function to parse CSV line (handles quoted fields with commas)
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

// Prepare insert statement
const insertStmt = db.prepare(`
  INSERT INTO sheffield_people (
    id, name, first_name, middle_name, surname, birth_year, gender,
    street_address, civil_parish, parish_area, census_age, where_born,
    relation, profession, spouse_name, source, created_at
  ) VALUES (
    ?, ?, ?, ?, ?, ?, ?,
    ?, ?, ?, ?, ?,
    ?, ?, ?, ?, datetime('now')
  )
`);

// Get all 1853 files
const genealogyDir = './genealogy';
const files1853 = fs.readdirSync(genealogyDir)
  .filter(f => f.includes('1853'))
  .sort();

console.log(`Found ${files1853.length} files for 1853\n`);

let totalImported = 0;
let totalDuplicates = 0;
const uniqueKeys = new Set();

// Process each file
files1853.forEach(filename => {
  console.log(`Processing ${filename}...`);

  const filePath = path.join(genealogyDir, filename);
  const content = fs.readFileSync(filePath, 'utf-8');
  const lines = content.split('\n');

  let imported = 0;
  let skipped = 0;

  // Skip header
  for (let i = 1; i < lines.length; i++) {
    const line = lines[i].trim();
    if (!line) continue;

    const values = parseCSVLine(line);
    if (values.length < 10) continue;

    const name = values[0];
    const spouse = values[1];
    const address = values[2];
    const parish = values[3];
    const area = values[4];
    const age = values[5];
    const birthYear = values[6];
    const birthPlace = values[7];
    const relation = values[8];
    const profession = values[9];

    // Create unique key: name + birth_year + address + parish + relation
    const uniqueKey = `${name}|${birthYear}|${address}|${parish}|${relation}`;

    if (uniqueKeys.has(uniqueKey)) {
      console.log(`  DUPLICATE: ${name} at ${address} - SKIPPING`);
      skipped++;
      totalDuplicates++;
      continue;
    }

    uniqueKeys.add(uniqueKey);

    // Parse name
    const nameParts = parseName(name);

    // Infer gender from relation and name
    let gender = null;
    if (relation === 'Wife' || relation === 'Daughter' || relation === 'Sister' || relation === 'Mother') {
      gender = 'Female';
    } else if (relation === 'Head' || relation === 'Son' || relation === 'Brother' || relation === 'Father' || relation === 'Lodger') {
      // Check first name for common patterns
      const firstName = nameParts.first_name.toLowerCase();
      if (firstName.startsWith('john') || firstName.startsWith('william') || firstName.startsWith('thomas') ||
          firstName.startsWith('james') || firstName.startsWith('joseph') || firstName.startsWith('henry')) {
        gender = 'Male';
      } else if (firstName.startsWith('mary') || firstName.startsWith('elizabeth') || firstName.startsWith('sarah') ||
                 firstName.startsWith('jane') || firstName.startsWith('ann')) {
        gender = 'Female';
      }
    }

    // Insert into database
    try {
      insertStmt.run(
        generateId(),
        name,
        nameParts.first_name,
        nameParts.middle_name,
        nameParts.surname,
        parseInt(birthYear) || 1853,
        gender,
        address,
        parish,
        area || null,
        parseInt(age) || 18,
        birthPlace,
        relation,
        profession || null,
        spouse || null,
        'genealogy'
      );
      imported++;
      totalImported++;
    } catch (err) {
      console.log(`  ERROR: ${name} - ${err.message}`);
      skipped++;
    }
  }

  console.log(`  Imported: ${imported}, Skipped: ${skipped}`);
});

console.log(`\n=== IMPORT COMPLETE ===`);
console.log(`Total imported: ${totalImported}`);
console.log(`Total duplicates skipped: ${totalDuplicates}`);

// Verify
const count = db.prepare('SELECT COUNT(*) as count FROM sheffield_people WHERE birth_year = 1853').get();
console.log(`\nVerification: ${count.count} people in database with birth_year = 1853`);

// Show some examples
console.log('\n=== SAMPLE RECORDS ===');
const samples = db.prepare(`
  SELECT name, profession, street_address, civil_parish, relation
  FROM sheffield_people
  WHERE birth_year = 1853
  LIMIT 10
`).all();

samples.forEach((p, i) => {
  console.log(`${i + 1}. ${p.name} (${p.profession || 'no profession'})`);
  console.log(`   ${p.relation} at ${p.street_address}, ${p.civil_parish}`);
});

// Check for John Smiths
console.log('\n=== ALL "JOHN SMITH" RECORDS (should be separate) ===');
const johnSmiths = db.prepare(`
  SELECT name, profession, street_address, civil_parish, relation
  FROM sheffield_people
  WHERE name = 'John Smith' AND birth_year = 1853
`).all();

johnSmiths.forEach((p, i) => {
  console.log(`${i + 1}. ${p.name} - ${p.profession || 'no profession'} (${p.relation})`);
  console.log(`   Address: ${p.street_address}, ${p.civil_parish}`);
});

db.close();
console.log('\nDone!');
