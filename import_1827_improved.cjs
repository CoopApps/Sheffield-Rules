const Database = require('better-sqlite3');
const fs = require('fs');
const { parse } = require('csv-parse/sync');
const crypto = require('crypto');

// Open the database
const db = new Database('Sheffield1867.db');

// Helper function to expand name abbreviations
function expandAbbreviations(name) {
  if (!name) return '';

  let expanded = name;

  // Common abbreviations - must use word boundaries to avoid partial matches
  expanded = expanded.replace(/\bWm\b/gi, 'William');
  expanded = expanded.replace(/\bWilliam\s+Hy\b/gi, 'William Henry');
  expanded = expanded.replace(/\bHy\b/gi, 'Henry');
  expanded = expanded.replace(/\bThos\b/gi, 'Thomas');
  expanded = expanded.replace(/\bJas\b/gi, 'James');
  expanded = expanded.replace(/\bJos\b/gi, 'Joseph');
  expanded = expanded.replace(/\bChas\b/gi, 'Charles');
  expanded = expanded.replace(/\bRobt\b/gi, 'Robert');
  expanded = expanded.replace(/\bFred\b/gi, 'Frederick');
  expanded = expanded.replace(/\bGeo\b/gi, 'George');
  expanded = expanded.replace(/\bEdw\b/gi, 'Edward');
  expanded = expanded.replace(/\bBenj\b/gi, 'Benjamin');
  expanded = expanded.replace(/\bSam\b/gi, 'Samuel');
  expanded = expanded.replace(/\bAlex\b/gi, 'Alexander');
  expanded = expanded.replace(/\bMath\b/gi, 'Matthew');
  expanded = expanded.replace(/\bJno\b/gi, 'John');
  expanded = expanded.replace(/\bJn\b/gi, 'John');

  // Common diminutives
  expanded = expanded.replace(/\bWillie\b/gi, 'William');
  expanded = expanded.replace(/\bBill\b/gi, 'William');
  expanded = expanded.replace(/\bTom\b/gi, 'Thomas');
  expanded = expanded.replace(/\bJim\b/gi, 'James');
  expanded = expanded.replace(/\bJimmy\b/gi, 'James');
  expanded = expanded.replace(/\bJoe\b/gi, 'Joseph');
  expanded = expanded.replace(/\bBob\b/gi, 'Robert');
  expanded = expanded.replace(/\bDick\b/gi, 'Richard');
  expanded = expanded.replace(/\bNed\b/gi, 'Edward');
  expanded = expanded.replace(/\bTed\b/gi, 'Edward');
  expanded = expanded.replace(/\bBen\b/gi, 'Benjamin');
  expanded = expanded.replace(/\bBetty\b/gi, 'Elizabeth');
  expanded = expanded.replace(/\bBess\b/gi, 'Elizabeth');
  expanded = expanded.replace(/\bNancy\b/gi, 'Ann');

  return expanded;
}

// Helper function to normalize names for matching
function normalizeName(name) {
  if (!name) return '';

  // First expand abbreviations
  let normalized = expandAbbreviations(name);

  // Then normalize
  return normalized.toLowerCase()
    .replace(/[.,\-]/g, ' ')
    .replace(/\s+/g, ' ')
    .trim();
}

// Levenshtein distance for fuzzy matching
function levenshteinDistance(str1, str2) {
  const len1 = str1.length;
  const len2 = str2.length;
  const matrix = [];

  if (len1 === 0) return len2;
  if (len2 === 0) return len1;

  for (let i = 0; i <= len1; i++) {
    matrix[i] = [i];
  }

  for (let j = 0; j <= len2; j++) {
    matrix[0][j] = j;
  }

  for (let i = 1; i <= len1; i++) {
    for (let j = 1; j <= len2; j++) {
      const cost = str1[i - 1] === str2[j - 1] ? 0 : 1;
      matrix[i][j] = Math.min(
        matrix[i - 1][j] + 1,
        matrix[i][j - 1] + 1,
        matrix[i - 1][j - 1] + cost
      );
    }
  }

  return matrix[len1][len2];
}

// Calculate similarity score (0-1, higher is more similar)
function similarityScore(str1, str2) {
  const maxLen = Math.max(str1.length, str2.length);
  if (maxLen === 0) return 1;
  const distance = levenshteinDistance(str1, str2);
  return 1 - (distance / maxLen);
}

// Check if two names are similar enough to be considered a match
function isSimilarName(name1, name2, threshold = 0.85) {
  const normalized1 = normalizeName(name1);
  const normalized2 = normalizeName(name2);

  // Exact match
  if (normalized1 === normalized2) return true;

  // Fuzzy match
  const score = similarityScore(normalized1, normalized2);
  return score >= threshold;
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
const sheffieldByName = new Map();

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

for (const record of sheffieldRecords) {
  const normalizedName = normalizeName(record.NAME);
  if (normalizedName && !sheffieldByName.has(normalizedName)) {
    sheffieldByName.set(normalizedName, record);
  }
}

// Matching logic
const matches = [];
const matchedSheffieldIds = new Set();
const matchedGenealogyIds = new Set();

console.log('\nMatching records...');

// Strategy 1: Direct name matches (with abbreviation expansion)
for (const shefRecord of sheffieldRecords) {
  const shefName = normalizeName(shefRecord.NAME);
  if (!shefName) continue; // Skip empty names

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

// Strategy 2: Wife matches via spouse lookup
// If genealogy has Wife with Spouse X, find X in Sheffield Census and use his record
let wifeMatches = 0;
for (const genRecord of genealogyRecords) {
  const genId = `gen_${genealogyRecords.indexOf(genRecord)}`;
  if (matchedGenealogyIds.has(genId)) continue;

  // Check if this is a wife with a spouse listed
  if (genRecord.Relation && genRecord.Relation.toLowerCase().includes('wife') && genRecord.Spouse) {
    const spouseName = normalizeName(genRecord.Spouse);

    // Find spouse in Sheffield Census
    if (sheffieldByName.has(spouseName)) {
      const shefRecord = sheffieldByName.get(spouseName);

      matches.push({
        sheffieldRecord: shefRecord,
        genealogyRecord: genRecord,
        matchType: 'wife_via_spouse',
        note: `${genRecord.Name} matched via spouse ${genRecord.Spouse}`
      });
      matchedGenealogyIds.add(genId);
      wifeMatches++;
      // Don't mark Sheffield record as matched - the husband might match separately
    }
  }
}

console.log(`Found ${wifeMatches} wife matches via spouse lookup`);

// Strategy 3: Household member to genealogy direct matching
// Find genealogy records that appear as household members in Sheffield census
let householdMatches = 0;
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
          householdMatches++;
          break;
        }
      }
    }
  }
}

console.log(`Found ${householdMatches} household member matches`);

// Strategy 4: Fuzzy name matching for unmatched records
// This catches spelling mistakes and OCR errors
console.log('\nPerforming fuzzy name matching...');
let fuzzyMatches = 0;

for (const shefRecord of sheffieldRecords) {
  const shefId = `shef_${sheffieldRecords.indexOf(shefRecord)}`;
  if (matchedSheffieldIds.has(shefId)) continue;

  const shefName = normalizeName(shefRecord.NAME);
  if (!shefName) continue;

  // Try fuzzy matching against all genealogy records
  for (const genRecord of genealogyRecords) {
    const genId = `gen_${genealogyRecords.indexOf(genRecord)}`;
    if (matchedGenealogyIds.has(genId)) continue;

    const genName = normalizeName(genRecord.Name);

    // Use similarity threshold of 0.85 (85% similar)
    if (isSimilarName(shefName, genName, 0.85)) {
      matches.push({
        sheffieldRecord: shefRecord,
        genealogyRecord: genRecord,
        matchType: 'fuzzy_name',
        note: `Fuzzy match: "${shefRecord.NAME}" ≈ "${genRecord.Name}" (similarity: ${(similarityScore(shefName, genName) * 100).toFixed(1)}%)`
      });
      matchedSheffieldIds.add(shefId);
      matchedGenealogyIds.add(genId);
      fuzzyMatches++;
      break; // One-to-one matching
    }
  }
}

console.log(`Found ${fuzzyMatches} fuzzy name matches`);
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
console.log(`  - Wife matches via spouse: ${matches.filter(m => m.matchType === 'wife_via_spouse').length}`);
console.log(`  - Household member matches: ${matches.filter(m => m.matchType === 'household_member').length}`);
console.log(`  - Fuzzy name matches: ${matches.filter(m => m.matchType === 'fuzzy_name').length}`);
console.log(`Match rate: ${((matches.length / genealogyRecords.length) * 100).toFixed(1)}% of genealogy records`);

// Show some example matches
console.log('\n=== EXAMPLE MATCHES ===');
const examplesByType = {};
for (const match of matches) {
  if (!examplesByType[match.matchType]) {
    examplesByType[match.matchType] = [];
  }
  if (examplesByType[match.matchType].length < 3) {
    examplesByType[match.matchType].push(match);
  }
}

let exampleNum = 1;
for (const [type, examples] of Object.entries(examplesByType)) {
  console.log(`\n${type.toUpperCase()} examples:`);
  for (const match of examples) {
    console.log(`${exampleNum}. ${match.genealogyRecord.Name}`);
    console.log(`   Address: ${match.genealogyRecord.Address}`);
    console.log(`   Profession: ${match.genealogyRecord.Profession || '(none)'}`);
    console.log(`   Parish: ${match.sheffieldRecord['ECCLESIASTICAL PARISH']}`);
    if (match.note) console.log(`   Note: ${match.note}`);
    exampleNum++;
  }
}

db.close();
console.log('\nDone!');
