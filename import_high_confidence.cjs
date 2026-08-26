const Database = require('better-sqlite3');
const fs = require('fs');
const { parse } = require('csv-parse/sync');
const crypto = require('crypto');

// Helper function to expand name abbreviations
function expandAbbreviations(name) {
  if (!name) return '';
  let expanded = name;
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
  return expanded;
}

function normalizeName(name) {
  if (!name) return '';
  let normalized = expandAbbreviations(name);
  return normalized.toLowerCase()
    .replace(/[.,\-]/g, ' ')
    .replace(/\s+/g, ' ')
    .trim();
}

function parseNameComponents(fullName) {
  if (!fullName) return { parts: [], firstName: '', middleNames: [], surname: '' };
  const normalized = normalizeName(fullName);
  const parts = normalized.split(/\s+/).filter(p => p);
  if (parts.length === 0) return { parts: [], firstName: '', middleNames: [], surname: '' };
  if (parts.length === 1) return { parts, firstName: parts[0], middleNames: [], surname: '' };

  const firstName = parts[0];
  const surname = parts[parts.length - 1];
  const middleNames = parts.slice(1, -1);

  return { parts, firstName, middleNames, surname };
}

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

function levenshteinDistance(str1, str2) {
  const len1 = str1.length;
  const len2 = str2.length;
  const matrix = [];

  if (len1 === 0) return len2;
  if (len2 === 0) return len1;

  for (let i = 0; i <= len1; i++) matrix[i] = [i];
  for (let j = 0; j <= len2; j++) matrix[0][j] = j;

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

function similarityScore(str1, str2) {
  const maxLen = Math.max(str1.length, str2.length);
  if (maxLen === 0) return 1;
  const distance = levenshteinDistance(str1, str2);
  return 1 - (distance / maxLen);
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

// Filter to unmatched only
const unmatchedSheffield = sheffieldRecords.filter(r => !matchedNames.has(normalizeName(r.NAME)));
const unmatchedGenealogy = genealogyRecords.filter(r => !matchedNames.has(normalizeName(r.Name)));

console.log(`Unmatched Sheffield Census: ${unmatchedSheffield.length}`);
console.log(`Unmatched Genealogy: ${unmatchedGenealogy.length}\n`);

// Find high-confidence matches (same logic as analyze script)
const highConfidenceMatches = [];

for (const shef of unmatchedSheffield) {
  const shefComponents = parseNameComponents(shef.NAME);
  if (!shefComponents.firstName || !shefComponents.surname) continue;

  for (const gen of unmatchedGenealogy) {
    const genComponents = parseNameComponents(gen.Name);
    if (!genComponents.firstName || !genComponents.surname) continue;

    if (shefComponents.firstName !== genComponents.firstName) continue;

    const surnameSimilarity = similarityScore(shefComponents.surname, genComponents.surname);

    const middleNameMatch = shefComponents.middleNames.length > 0 &&
                           genComponents.middleNames.length > 0 &&
                           shefComponents.middleNames.some(m1 =>
                             genComponents.middleNames.some(m2 => m1 === m2)
                           );

    let confidence = 0;
    let reason = '';

    if (surnameSimilarity === 1.0) {
      confidence = 'EXACT';
      reason = 'First name + exact surname match';
    } else if (surnameSimilarity >= 0.85) {
      confidence = 'HIGH';
      reason = `Very similar surname (${(surnameSimilarity * 100).toFixed(0)}%)`;
      if (middleNameMatch) {
        confidence = 'VERY HIGH';
        reason += ' + middle name match';
      }
    } else if (surnameSimilarity >= 0.70 && middleNameMatch) {
      confidence = 'MEDIUM-HIGH';
      reason = `Similar surname (${(surnameSimilarity * 100).toFixed(0)}%) + middle name match`;
    }

    if (confidence) {
      // Store unique matches (avoid duplicates from the genealogy data)
      const matchKey = `${normalizeName(shef.NAME)}_${normalizeName(gen.Name)}`;

      highConfidenceMatches.push({
        matchKey,
        sheffieldRecord: shef,
        genealogyRecord: gen,
        confidence,
        reason
      });
    }
  }
}

// Remove duplicates by matchKey
const uniqueMatches = [];
const seenKeys = new Set();

for (const match of highConfidenceMatches) {
  if (!seenKeys.has(match.matchKey)) {
    seenKeys.add(match.matchKey);
    uniqueMatches.push(match);
  }
}

console.log(`Found ${uniqueMatches.length} unique high-confidence matches\n`);

// Insert into database
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
      `1827_matched_high_confidence_${match.confidence.toLowerCase().replace(/\s/g, '_')}`
    );

    insertCount++;

    console.log(`Imported: ${gen.Name} (${match.confidence})`);
    console.log(`  Sheffield: "${shef.NAME}" | Parish: ${shef['ECCLESIASTICAL PARISH']}`);
    console.log(`  Genealogy: ${gen.Address} | ${gen.Profession || 'no profession'}`);
    console.log(`  Reason: ${match.reason}\n`);
  }
});

insertTransaction(uniqueMatches);

console.log(`\n=== IMPORT COMPLETE ===`);
console.log(`Successfully imported ${insertCount} high-confidence matches`);

// Show updated totals
const totalCount = db.prepare('SELECT COUNT(*) as count FROM sheffield_people').get();
console.log(`\nTotal records in sheffield_people: ${totalCount.count}`);

db.close();
console.log('\nDone!');
