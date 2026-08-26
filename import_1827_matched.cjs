const Database = require('better-sqlite3');
const fs = require('fs');
const { parse } = require('csv-parse/sync');
const crypto = require('crypto');

// Open the database
const db = new Database('Sheffield1867.db');

// Helper function to normalize names for matching
function normalizeName(name) {
  if (!name) return '';
  return name.toLowerCase()
    .replace(/[.,\-]/g, ' ')
    .replace(/\s+/g, ' ')
    .trim();
}

// Helper function to extract first and last names
function parseFullName(fullName) {
  if (!fullName) return { firstName: '', surname: '' };
  const parts = fullName.trim().split(/\s+/);
  if (parts.length === 0) return { firstName: '', surname: '' };
  if (parts.length === 1) return { firstName: parts[0], surname: '' };

  const firstName = parts[0];
  const surname = parts[parts.length - 1];
  const middleName = parts.slice(1, -1).join(' ');

  return { firstName, middleName, surname };
}

// Helper function to parse household members from Sheffield Census
function parseHouseholdMembers(householdStr) {
  if (!householdStr) return [];

  const members = [];
  const entries = householdStr.split('|').map(e => e.trim()).filter(e => e);

  for (const entry of entries) {
    const parts = entry.split('\t');
    if (parts.length >= 2) {
      const name = parts[0].trim();
      const age = parts[1].trim();

      // Skip header rows
      if (name.toLowerCase() === 'name' && age.toLowerCase() === 'age') continue;

      members.push({
        name: name,
        age: age
      });
    }
  }

  return members;
}

// Load Sheffield Census data
console.log('Loading Sheffield Census 1827 data...');
const sheffieldCensusPath = 'Sheffield Census/1827 - done.csv';
const sheffieldCensusContent = fs.readFileSync(sheffieldCensusPath, 'utf-8');
const sheffieldRecords = parse(sheffieldCensusContent, {
  columns: true,
  skip_empty_lines: true
});
console.log(`Loaded ${sheffieldRecords.length} Sheffield Census records`);

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
console.log(`Loaded ${genealogyRecords.length} Genealogy records`);

// Create lookup maps
const genealogyByName = new Map();
const genealogyBySpouse = new Map();

for (const record of genealogyRecords) {
  const normalizedName = normalizeName(record.Name);
  if (!genealogyByName.has(normalizedName)) {
    genealogyByName.set(normalizedName, []);
  }
  genealogyByName.get(normalizedName).push(record);

  if (record.Spouse) {
    const normalizedSpouse = normalizeName(record.Spouse);
    if (!genealogyBySpouse.has(normalizedSpouse)) {
      genealogyBySpouse.set(normalizedSpouse, []);
    }
    genealogyBySpouse.get(normalizedSpouse).push(record);
  }
}

// Matching logic
const matches = [];
const matchedSheffieldIds = new Set();
const matchedGenealogyIds = new Set();

console.log('\nMatching records...');

// Strategy 1: Direct name matches
for (const shefRecord of sheffieldRecords) {
  const shefName = normalizeName(shefRecord.NAME);

  // Try exact name match
  if (genealogyByName.has(shefName)) {
    const genCandidates = genealogyByName.get(shefName);

    for (const genRecord of genCandidates) {
      const genId = `gen_${genealogyRecords.indexOf(genRecord)}`;
      const shefId = `shef_${sheffieldRecords.indexOf(shefRecord)}`;

      if (!matchedGenealogyIds.has(genId)) {
        matches.push({
          sheffieldRecord: shefRecord,
          genealogyRecord: genRecord,
          matchType: 'direct_name'
        });
        matchedSheffieldIds.add(shefId);
        matchedGenealogyIds.add(genId);
        break;  // One-to-one matching
      }
    }
  }
}

console.log(`Found ${matches.length} direct name matches`);

// Strategy 2: Spouse-based matching
// If genealogy record lists spouse X, and Sheffield census has household head X with household member matching genealogy person
for (const genRecord of genealogyRecords) {
  const genId = `gen_${genealogyRecords.indexOf(genRecord)}`;
  if (matchedGenealogyIds.has(genId)) continue;

  if (!genRecord.Spouse) continue;

  const spouseName = normalizeName(genRecord.Spouse);
  const personName = normalizeName(genRecord.Name);

  // Find Sheffield record where head matches spouse name
  for (const shefRecord of sheffieldRecords) {
    const shefId = `shef_${sheffieldRecords.indexOf(shefRecord)}`;
    if (matchedSheffieldIds.has(shefId)) continue;

    const shefName = normalizeName(shefRecord.NAME);

    if (shefName === spouseName) {
      // Check if person is in household members
      const household = parseHouseholdMembers(shefRecord['HOUSEHOLD MEMBERS']);
      const foundInHousehold = household.some(member =>
        normalizeName(member.name) === personName
      );

      if (foundInHousehold) {
        matches.push({
          sheffieldRecord: shefRecord,
          genealogyRecord: genRecord,
          matchType: 'spouse_household',
          note: `${genRecord.Name} found as household member of spouse ${genRecord.Spouse}`
        });
        matchedGenealogyIds.add(genId);
        // Don't mark Sheffield record as matched - it's the spouse's record
        break;
      }
    }
  }
}

const spouseMatches = matches.filter(m => m.matchType === 'spouse_household').length;
console.log(`Found ${spouseMatches} spouse/household matches`);

// Strategy 3: Household member to genealogy direct matching
// Find genealogy records that appear as household members in Sheffield census
for (const shefRecord of sheffieldRecords) {
  const household = parseHouseholdMembers(shefRecord['HOUSEHOLD MEMBERS']);

  for (const member of household) {
    const memberName = normalizeName(member.name);

    // Look for this household member in genealogy records
    if (genealogyByName.has(memberName)) {
      const genCandidates = genealogyByName.get(memberName);

      for (const genRecord of genCandidates) {
        const genId = `gen_${genealogyRecords.indexOf(genRecord)}`;

        if (!matchedGenealogyIds.has(genId)) {
          // This genealogy person appears as a household member
          // Use the head's Sheffield record for parish data
          matches.push({
            sheffieldRecord: shefRecord,
            genealogyRecord: genRecord,
            matchType: 'household_member',
            note: `${genRecord.Name} found as household member of ${shefRecord.NAME}`
          });
          matchedGenealogyIds.add(genId);
          break;
        }
      }
    }
  }
}

const householdMemberMatches = matches.filter(m => m.matchType === 'household_member').length;
console.log(`Found ${householdMemberMatches} household member matches`);
console.log(`Total matches: ${matches.length}`);

// Insert matched records into database
console.log('\nInserting matched records into sheffield_people...');

const insertStmt = db.prepare(`
  INSERT INTO sheffield_people (
    id, name, first_name, middle_name, surname,
    birth_year, gender,
    census_age, census_birth_date, census_birth_place, census_county,
    census_relation, census_gender, census_ed, census_household_schedule,
    census_household_members, census_piece, census_folio, census_page,
    where_born, birth_town, birth_county, birth_country,
    civil_parish, ecclesiastical_parish, registration_district, sub_registration_district,
    street_address, profession, spouse_name, relation, parish_area, source
  ) VALUES (
    ?, ?, ?, ?, ?,
    ?, ?,
    ?, ?, ?, ?,
    ?, ?, ?, ?,
    ?, ?, ?, ?,
    ?, ?, ?, ?,
    ?, ?, ?, ?,
    ?, ?, ?, ?, ?, ?
  )
`);

let insertCount = 0;
const insertTransaction = db.transaction((matches) => {
  for (const match of matches) {
    const shef = match.sheffieldRecord;
    const gen = match.genealogyRecord;

    // Use genealogy name as primary (it's who we're inserting)
    const fullName = gen.Name || shef.NAME;
    const { firstName, middleName, surname } = parseFullName(fullName);

    const id = crypto.randomUUID();
    const birthYear = gen['Born Approx'] || shef['ESTIMATED BIRTH YEAR'] || null;

    insertStmt.run(
      id,
      fullName,
      firstName,
      middleName || null,
      surname,
      birthYear,
      shef.GENDER || null,

      // Census data from Sheffield
      shef.AGE || null,
      shef['Birth Date'] || null,
      shef['Birth Place'] || null,
      shef['COUNTY/ISLAND'] || null,
      shef.RELATION || null,
      shef.GENDER || null,
      shef['ED, INSTITUTION, OR VESSEL'] || null,
      shef['HOUSEHOLD SCHEDULE NUMBER'] || null,
      shef['HOUSEHOLD MEMBERS'] || null,
      shef.PIECE || null,
      shef.FOLIO || null,
      shef['PAGE NUMBER'] || null,

      // Birth location
      shef['WHERE BORN'] || gen['Birth Place'] || null,
      shef.TOWN || null,
      shef['COUNTY/ISLAND'] || null,
      shef.COUNTRY || null,

      // Parish data from Sheffield
      shef['CIVIL PARISH'] || null,
      shef['ECCLESIASTICAL PARISH'] || null,
      shef['REGISTRATION DISTRICT'] || null,
      shef['SUB-REGISTRATION DISTRICT'] || null,

      // Address and profession from Genealogy
      gen.Address || null,
      gen.Profession || null,
      gen.Spouse || null,
      gen.Relation || shef.RELATION || null,
      gen.Area || gen.Parish || null,
      `1827_matched_${match.matchType}`
    );

    insertCount++;
  }
});

insertTransaction(matches);

console.log(`\nSuccessfully inserted ${insertCount} matched records into sheffield_people`);

// Generate report
console.log('\n=== MATCHING REPORT ===');
console.log(`Sheffield Census records: ${sheffieldRecords.length}`);
console.log(`Genealogy records: ${genealogyRecords.length}`);
console.log(`Total matches: ${matches.length}`);
console.log(`  - Direct name matches: ${matches.filter(m => m.matchType === 'direct_name').length}`);
console.log(`  - Spouse/household matches: ${matches.filter(m => m.matchType === 'spouse_household').length}`);
console.log(`  - Household member matches: ${matches.filter(m => m.matchType === 'household_member').length}`);
console.log(`Match rate: ${((matches.length / genealogyRecords.length) * 100).toFixed(1)}% of genealogy records`);

// Show some example matches
console.log('\n=== EXAMPLE MATCHES ===');
for (let i = 0; i < Math.min(5, matches.length); i++) {
  const match = matches[i];
  console.log(`\n${i + 1}. ${match.genealogyRecord.Name} (${match.matchType})`);
  console.log(`   Address: ${match.genealogyRecord.Address}`);
  console.log(`   Profession: ${match.genealogyRecord.Profession}`);
  console.log(`   Parish: ${match.sheffieldRecord['ECCLESIASTICAL PARISH']}`);
  if (match.note) console.log(`   Note: ${match.note}`);
}

db.close();
console.log('\nDone!');
