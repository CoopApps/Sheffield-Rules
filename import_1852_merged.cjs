const sqlite3 = require('better-sqlite3');
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const db = new sqlite3('./sheffield1867.db');

console.log('=== IMPORTING 1852 BIRTH YEAR (MERGED CENSUS + GENEALOGY) ===\n');

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

// Helper function to normalize name for matching
function normalizeName(name) {
  return name.toLowerCase().trim().replace(/\s+/g, ' ');
}

// Helper function to normalize parish for matching
function normalizeParish(parish) {
  return parish.toLowerCase().trim().replace(/\s+/g, '');
}

// Prepare insert statement - using only columns that exist from 1853 import
const insertStmt = db.prepare(`
  INSERT INTO sheffield_people (
    id, name, first_name, middle_name, surname, birth_year, gender,
    street_address, civil_parish, parish_area,
    relation, profession, spouse_name, source, created_at
  ) VALUES (
    ?, ?, ?, ?, ?, ?, ?,
    ?, ?, ?,
    ?, ?, ?, ?, datetime('now')
  )
`);

console.log('=== STEP 1: LOAD CENSUS DATA ===\n');

const censusFile = path.join('sheffield census', '1852.csv');
const censusContent = fs.readFileSync(censusFile, 'utf-8');
const censusLines = censusContent.split('\n');

const censusPeople = [];
for (let i = 1; i < censusLines.length; i++) {
  const line = censusLines[i].trim();
  if (!line) continue;

  const values = parseCSVLine(line);
  if (values.length < 22) continue;

  const age = values[0];
  const birthPlace = values[2];
  const civilParish = values[3];
  const ecclesiasticalParish = values[6];
  const birthYear = values[8];
  const folio = values[9];
  const gender = values[10];
  const householdMembers = values[11];
  const scheduleNum = values[12];
  const name = values[13]; // or values[14]
  const pageNum = values[15];
  const piece = values[16];
  const relation = values[18];
  const subRegDistrict = values[19];

  const person = {
    name,
    age: parseInt(age) || 19,
    birthYear: parseInt(birthYear) || 1852,
    birthPlace,
    civilParish,
    ecclesiasticalParish,
    gender,
    relation,
    householdMembers,
    piece,
    folio,
    pageNum,
    scheduleNum,
    parishArea: subRegDistrict,
    source: 'census'
  };

  censusPeople.push(person);
}

console.log(`Loaded ${censusPeople.length} people from census\n`);

console.log('=== STEP 2: LOAD GENEALOGY DATA ===\n');

const genealogyDir = './genealogy';
const files1852 = fs.readdirSync(genealogyDir)
  .filter(f => f.includes('1852'))
  .sort();

console.log(`Found ${files1852.length} genealogy files for 1852\n`);

const genealogyPeople = [];
files1852.forEach(filename => {
  const filePath = path.join(genealogyDir, filename);
  const content = fs.readFileSync(filePath, 'utf-8');
  const lines = content.split('\n');

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

    const person = {
      name,
      spouse,
      address,
      parish,
      area,
      age: parseInt(age) || 19,
      birthYear: parseInt(birthYear) || 1852,
      birthPlace,
      relation,
      profession,
      source: 'genealogy'
    };

    genealogyPeople.push(person);
  }
});

console.log(`Loaded ${genealogyPeople.length} people from genealogy\n`);

console.log('=== STEP 3: MATCH CENSUS + GENEALOGY ===\n');

const matched = [];
const censusOnly = [];
const genealogyOnly = [];
const usedGenealogyIndices = new Set();

// Try to match each census person with genealogy
censusPeople.forEach(censusPerson => {
  let bestMatch = null;
  let bestMatchIndex = -1;

  const censusNorm = normalizeName(censusPerson.name);
  const censusParish = normalizeParish(censusPerson.civilParish);

  // Look for matching genealogy person
  genealogyPeople.forEach((genPerson, idx) => {
    if (usedGenealogyIndices.has(idx)) return;

    const genNorm = normalizeName(genPerson.name);
    const genParish = normalizeParish(genPerson.parish);

    // Match criteria: same name + same parish (or close enough)
    if (censusNorm === genNorm && (censusParish === genParish || censusParish.includes(genParish) || genParish.includes(censusParish))) {
      bestMatch = genPerson;
      bestMatchIndex = idx;
    }
  });

  if (bestMatch) {
    // Merge the two records
    matched.push({
      census: censusPerson,
      genealogy: bestMatch
    });
    usedGenealogyIndices.add(bestMatchIndex);
  } else {
    censusOnly.push(censusPerson);
  }
});

// Remaining genealogy people (not matched)
genealogyPeople.forEach((genPerson, idx) => {
  if (!usedGenealogyIndices.has(idx)) {
    genealogyOnly.push(genPerson);
  }
});

console.log(`Matched: ${matched.length} people`);
console.log(`Census only: ${censusOnly.length} people`);
console.log(`Genealogy only: ${genealogyOnly.length} people\n`);

console.log('=== STEP 4: INSERT MATCHED RECORDS (census + genealogy) ===\n');

let inserted = 0;
const uniqueKeys = new Set();

matched.forEach(match => {
  const census = match.census;
  const gen = match.genealogy;

  // Create unique key to prevent duplicates
  const uniqueKey = `${census.name}|${census.birthYear}|${gen.address}|${census.civilParish}|${census.relation}`;

  if (uniqueKeys.has(uniqueKey)) {
    return; // Skip duplicate
  }
  uniqueKeys.add(uniqueKey);

  const nameParts = parseName(census.name);

  // Infer gender if not set
  let gender = census.gender;
  if (!gender || gender === 'Unknown') {
    if (gen.relation === 'Wife' || gen.relation === 'Daughter' || gen.relation === 'Sister' || gen.relation === 'Mother') {
      gender = 'Female';
    } else if (gen.relation === 'Head' || gen.relation === 'Son' || gen.relation === 'Brother' || gen.relation === 'Father') {
      gender = 'Male';
    }
  }

  try {
    insertStmt.run(
      generateId(),
      census.name,
      nameParts.first_name,
      nameParts.middle_name,
      nameParts.surname,
      census.birthYear,
      gender,
      gen.address,  // Use genealogy address
      census.civilParish,
      gen.area || census.parishArea,
      gen.relation || census.relation,  // Prefer genealogy relation
      gen.profession,  // Use genealogy profession
      gen.spouse,
      'census+genealogy'
    );
    inserted++;
  } catch (err) {
    console.log(`  ERROR: ${census.name} - ${err.message}`);
  }
});

console.log(`Inserted ${inserted} merged records\n`);

console.log('=== STEP 5: INSERT CENSUS-ONLY RECORDS ===\n');

let censusOnlyInserted = 0;

censusOnly.forEach(census => {
  const uniqueKey = `${census.name}|${census.birthYear}|${census.civilParish}|${census.scheduleNum}|${census.relation}`;

  if (uniqueKeys.has(uniqueKey)) {
    return;
  }
  uniqueKeys.add(uniqueKey);

  const nameParts = parseName(census.name);

  try {
    insertStmt.run(
      generateId(),
      census.name,
      nameParts.first_name,
      nameParts.middle_name,
      nameParts.surname,
      census.birthYear,
      census.gender,
      null,  // No address from genealogy
      census.civilParish,
      census.parishArea,
      census.relation,
      null,  // No profession from genealogy
      null,  // No spouse
      'census'
    );
    censusOnlyInserted++;
  } catch (err) {
    console.log(`  ERROR: ${census.name} - ${err.message}`);
  }
});

console.log(`Inserted ${censusOnlyInserted} census-only records\n`);

console.log('=== STEP 6: INSERT GENEALOGY-ONLY RECORDS ===\n');

let genealogyOnlyInserted = 0;

genealogyOnly.forEach(gen => {
  const uniqueKey = `${gen.name}|${gen.birthYear}|${gen.address}|${gen.parish}|${gen.relation}`;

  if (uniqueKeys.has(uniqueKey)) {
    return;
  }
  uniqueKeys.add(uniqueKey);

  const nameParts = parseName(gen.name);

  // Infer gender
  let gender = null;
  if (gen.relation === 'Wife' || gen.relation === 'Daughter' || gen.relation === 'Sister' || gen.relation === 'Mother') {
    gender = 'Female';
  } else if (gen.relation === 'Head' || gen.relation === 'Son' || gen.relation === 'Brother' || gen.relation === 'Father') {
    gender = 'Male';
  }

  try {
    insertStmt.run(
      generateId(),
      gen.name,
      nameParts.first_name,
      nameParts.middle_name,
      nameParts.surname,
      gen.birthYear,
      gender,
      gen.address,
      gen.parish,
      gen.area,
      gen.relation,
      gen.profession,
      gen.spouse,
      'genealogy'
    );
    genealogyOnlyInserted++;
  } catch (err) {
    console.log(`  ERROR: ${gen.name} - ${err.message}`);
  }
});

console.log(`Inserted ${genealogyOnlyInserted} genealogy-only records\n`);

console.log('\n=== IMPORT COMPLETE ===');
console.log(`Total inserted: ${inserted + censusOnlyInserted + genealogyOnlyInserted}`);
console.log(`  - Merged (census+genealogy): ${inserted}`);
console.log(`  - Census only: ${censusOnlyInserted}`);
console.log(`  - Genealogy only: ${genealogyOnlyInserted}`);

// Verify
const count = db.prepare('SELECT COUNT(*) as count FROM sheffield_people WHERE birth_year = 1852').get();
console.log(`\nVerification: ${count.count} people in database with birth_year = 1852`);

// Show breakdown by source
console.log('\n=== SOURCE BREAKDOWN ===');
const sources = db.prepare(`
  SELECT source, COUNT(*) as count
  FROM sheffield_people
  WHERE birth_year = 1852
  GROUP BY source
`).all();

sources.forEach(s => {
  console.log(`  ${s.source}: ${s.count}`);
});

// Show sample merged records
console.log('\n=== SAMPLE MERGED RECORDS (census+genealogy) ===');
const samples = db.prepare(`
  SELECT name, profession, street_address, civil_parish, relation
  FROM sheffield_people
  WHERE birth_year = 1852 AND source = 'census+genealogy'
  LIMIT 10
`).all();

samples.forEach((p, i) => {
  console.log(`${i + 1}. ${p.name} - ${p.profession || 'no profession'} (${p.relation})`);
  console.log(`   Address: ${p.street_address}`);
  console.log(`   Parish: ${p.civil_parish}`);
  console.log();
});

db.close();
console.log('Done!');
