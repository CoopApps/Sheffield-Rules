const Database = require('better-sqlite3');
const fs = require('fs');
const { parse } = require('csv-parse/sync');
const crypto = require('crypto');

// Years to process (all except 1827 which is already done)
const YEARS_TO_PROCESS = [1800, 1820, 1840]; // TEST
const YEARS_TO_PROCESS_FULL = [
  1772, 1773, 1776, 1778, 1780, 1782, 1783, 1784, 1785, 1786, 1787, 1788, 1789,
  1790, 1791, 1792, 1794, 1795, 1796, 1797, 1798, 1799, 1800, 1801, 1802, 1803,
  1804, 1805, 1806, 1807, 1808, 1809, 1810, 1811, 1812, 1813, 1814, 1815, 1816,
  1817, 1818, 1819, 1820, 1821, 1822, 1823, 1824, 1825, 1826, 1828, 1829, 1830,
  1831, 1832, 1833, 1834, 1835, 1836, 1837, 1838, 1839, 1840, 1841, 1842, 1843,
  1844, 1845, 1846, 1847, 1848, 1849, 1850, 1851, 1852
];

// Helper functions
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

function parseHouseholdMembers(householdStr) {
  if (!householdStr) return [];
  const members = [];
  const entries = householdStr.split('|').map(e => e.trim()).filter(e => e);

  for (const entry of entries) {
    const parts = entry.split('\t');
    if (parts.length >= 2) {
      const name = parts[0].trim();
      const age = parts[1].trim();
      if (name.toLowerCase() === 'name' && age.toLowerCase() === 'age') continue;
      members.push({ name, age });
    }
  }
  return members;
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

function isSimilarName(name1, name2, threshold = 0.85) {
  const normalized1 = normalizeName(name1);
  const normalized2 = normalizeName(name2);
  if (normalized1 === normalized2) return true;
  const score = similarityScore(normalized1, normalized2);
  return score >= threshold;
}

// Process a single year
function processYear(year, db) {
  console.log(`\n${'='.repeat(60)}`);
  console.log(`Processing year ${year}...`);
  console.log('='.repeat(60));

  // Check if Sheffield Census file exists
  const sheffieldPath = `Sheffield Census/${year}.csv`;
  const sheffieldPathDone = `Sheffield Census/${year} - done.csv`;

  let sheffieldFile = null;
  if (fs.existsSync(sheffieldPath)) {
    sheffieldFile = sheffieldPath;
  } else if (fs.existsSync(sheffieldPathDone)) {
    sheffieldFile = sheffieldPathDone;
  }

  if (!sheffieldFile) {
    console.log(`  ⚠ Sheffield Census file not found, skipping`);
    return { year, status: 'skipped', reason: 'No Sheffield Census file' };
  }

  // Load Sheffield Census
  const sheffieldContent = fs.readFileSync(sheffieldFile, 'utf-8');
  const sheffieldRecords = parse(sheffieldContent, { columns: true, skip_empty_lines: true });

  // Load Genealogy files
  const genealogyRecords = [];
  const genealogyDir = 'Genealogy';
  const genealogyFiles = fs.readdirSync(genealogyDir)
    .filter(f => f.includes(`tg_${year}_`) && f.endsWith('.csv'));

  if (genealogyFiles.length === 0) {
    console.log(`  ⚠ No Genealogy files found, skipping`);
    return { year, status: 'skipped', reason: 'No Genealogy files' };
  }

  for (const file of genealogyFiles) {
    const content = fs.readFileSync(`${genealogyDir}/${file}`, 'utf-8');
    const records = parse(content, { columns: true, skip_empty_lines: true });
    genealogyRecords.push(...records);
  }

  console.log(`  Sheffield Census: ${sheffieldRecords.length} records`);
  console.log(`  Genealogy: ${genealogyRecords.length} records (${genealogyFiles.length} files)`);

  // Create lookup maps
  const genealogyByName = new Map();
  const sheffieldByName = new Map();

  for (const record of genealogyRecords) {
    const normalizedName = normalizeName(record.Name);
    if (!genealogyByName.has(normalizedName)) {
      genealogyByName.set(normalizedName, []);
    }
    genealogyByName.get(normalizedName).push(record);
  }

  for (const record of sheffieldRecords) {
    const normalizedName = normalizeName(record.NAME);
    if (normalizedName && !sheffieldByName.has(normalizedName)) {
      sheffieldByName.set(normalizedName, record);
    }
  }

  const matches = [];
  const matchedSheffieldIds = new Set();
  const matchedGenealogyIds = new Set();

  // Strategy 1: Direct name matches
  for (const shefRecord of sheffieldRecords) {
    const shefName = normalizeName(shefRecord.NAME);
    if (!shefName) continue;

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
          break;
        }
      }
    }
  }

  const directMatches = matches.length;
  console.log(`  ✓ Direct name matches: ${directMatches}`);

  // Strategy 2: Wife matches via spouse
  let wifeMatches = 0;
  for (const genRecord of genealogyRecords) {
    const genId = `gen_${genealogyRecords.indexOf(genRecord)}`;
    if (matchedGenealogyIds.has(genId)) continue;

    if (genRecord.Relation && genRecord.Relation.toLowerCase().includes('wife') && genRecord.Spouse) {
      const spouseName = normalizeName(genRecord.Spouse);
      if (sheffieldByName.has(spouseName)) {
        const shefRecord = sheffieldByName.get(spouseName);
        matches.push({
          sheffieldRecord: shefRecord,
          genealogyRecord: genRecord,
          matchType: 'wife_via_spouse'
        });
        matchedGenealogyIds.add(genId);
        wifeMatches++;
      }
    }
  }

  console.log(`  ✓ Wife matches via spouse: ${wifeMatches}`);

  // Strategy 3: Household member matches
  let householdMatches = 0;
  for (const shefRecord of sheffieldRecords) {
    const household = parseHouseholdMembers(shefRecord['HOUSEHOLD MEMBERS']);
    for (const member of household) {
      const memberName = normalizeName(member.name);
      if (genealogyByName.has(memberName)) {
        const genCandidates = genealogyByName.get(memberName);
        for (const genRecord of genCandidates) {
          const genId = `gen_${genealogyRecords.indexOf(genRecord)}`;
          if (!matchedGenealogyIds.has(genId)) {
            matches.push({
              sheffieldRecord: shefRecord,
              genealogyRecord: genRecord,
              matchType: 'household_member'
            });
            matchedGenealogyIds.add(genId);
            householdMatches++;
            break;
          }
        }
      }
    }
  }

  console.log(`  ✓ Household member matches: ${householdMatches}`);

  // Strategy 4: Fuzzy name matching
  let fuzzyMatches = 0;
  for (const shefRecord of sheffieldRecords) {
    const shefId = `shef_${sheffieldRecords.indexOf(shefRecord)}`;
    if (matchedSheffieldIds.has(shefId)) continue;

    const shefName = normalizeName(shefRecord.NAME);
    if (!shefName) continue;

    for (const genRecord of genealogyRecords) {
      const genId = `gen_${genealogyRecords.indexOf(genRecord)}`;
      if (matchedGenealogyIds.has(genId)) continue;

      const genName = normalizeName(genRecord.Name);
      if (isSimilarName(shefName, genName, 0.85)) {
        matches.push({
          sheffieldRecord: shefRecord,
          genealogyRecord: genRecord,
          matchType: 'fuzzy_name'
        });
        matchedSheffieldIds.add(shefId);
        matchedGenealogyIds.add(genId);
        fuzzyMatches++;
        break;
      }
    }
  }

  console.log(`  ✓ Fuzzy name matches: ${fuzzyMatches}`);
  console.log(`  📊 Total matches: ${matches.length}`);

  return {
    year,
    status: 'processed',
    sheffieldCount: sheffieldRecords.length,
    genealogyCount: genealogyRecords.length,
    matches: matches.length,
    directMatches,
    wifeMatches,
    householdMatches,
    fuzzyMatches,
    matchData: matches
  };
}

// Main execution
console.log('🚀 Multi-Year Census Import Script');
console.log(`Processing ${YEARS_TO_PROCESS.length} years (excluding 1827)`);
console.log('');

const db = new Database('Sheffield1867.db');

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

const results = [];
let totalInserted = 0;

for (const year of YEARS_TO_PROCESS) {
  const result = processYear(year, db);
  results.push(result);

  if (result.status === 'processed' && result.matchData) {
    // Insert matches for this year
    const insertTransaction = db.transaction((matches) => {
      for (const match of matches) {
        const shef = match.sheffieldRecord;
        const gen = match.genealogyRecord;

        const fullName = gen.Name || shef.NAME;
        const { firstName, middleName, surname } = parseFullName(fullName);
        const id = crypto.randomUUID();
        const birthYear = gen['Born Approx'] || shef['ESTIMATED BIRTH YEAR'] || null;

        insertStmt.run(
          id, fullName, firstName, middleName || null, surname,
          birthYear, shef.GENDER || null,
          shef.AGE || null, shef['Birth Date'] || null, shef['Birth Place'] || null, shef['COUNTY/ISLAND'] || null,
          shef.RELATION || null, shef.GENDER || null, shef['ED, INSTITUTION, OR VESSEL'] || null,
          shef['HOUSEHOLD SCHEDULE NUMBER'] || null, shef['HOUSEHOLD MEMBERS'] || null,
          shef.PIECE || null, shef.FOLIO || null, shef['PAGE NUMBER'] || null,
          shef['WHERE BORN'] || gen['Birth Place'] || null, shef.TOWN || null,
          shef['COUNTY/ISLAND'] || null, shef.COUNTRY || null,
          shef['CIVIL PARISH'] || null, shef['ECCLESIASTICAL PARISH'] || null,
          shef['REGISTRATION DISTRICT'] || null, shef['SUB-REGISTRATION DISTRICT'] || null,
          gen.Address || null, gen.Profession || null, gen.Spouse || null,
          gen.Relation || shef.RELATION || null, gen.Area || gen.Parish || null,
          `${year}_matched_${match.matchType}`
        );

        totalInserted++;
      }
    });

    insertTransaction(result.matchData);
    console.log(`  ✅ Inserted ${result.matches} records`);
  }
}

db.close();

// Generate summary report
console.log('\n' + '='.repeat(60));
console.log('FINAL SUMMARY');
console.log('='.repeat(60));

const processed = results.filter(r => r.status === 'processed');
const skipped = results.filter(r => r.status === 'skipped');

console.log(`\nYears processed: ${processed.length}`);
console.log(`Years skipped: ${skipped.length}`);
console.log(`Total records inserted: ${totalInserted}`);

if (processed.length > 0) {
  const totalSheffield = processed.reduce((sum, r) => sum + r.sheffieldCount, 0);
  const totalGenealogy = processed.reduce((sum, r) => sum + r.genealogyCount, 0);
  const totalMatches = processed.reduce((sum, r) => sum + r.matches, 0);

  console.log(`\nTotal source records:`);
  console.log(`  Sheffield Census: ${totalSheffield.toLocaleString()}`);
  console.log(`  Genealogy: ${totalGenealogy.toLocaleString()}`);
  console.log(`  Matched: ${totalMatches.toLocaleString()}`);
  console.log(`  Match rate: ${((totalMatches / totalGenealogy) * 100).toFixed(1)}%`);

  console.log(`\nTop 10 years by matches:`);
  processed
    .sort((a, b) => b.matches - a.matches)
    .slice(0, 10)
    .forEach((r, i) => {
      console.log(`  ${i + 1}. ${r.year}: ${r.matches.toLocaleString()} matches`);
    });
}

console.log('\n✅ Import complete!');
