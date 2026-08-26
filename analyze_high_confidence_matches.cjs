const fs = require('fs');
const { parse } = require('csv-parse/sync');

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

// Calculate Levenshtein distance
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
const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');
const matchedRecords = db.prepare("SELECT name FROM sheffield_people WHERE source LIKE '1827_matched_%'").all();
const matchedNames = new Set(matchedRecords.map(r => normalizeName(r.name)));
db.close();

// Filter to unmatched only
const unmatchedSheffield = sheffieldRecords.filter(r => !matchedNames.has(normalizeName(r.NAME)));
const unmatchedGenealogy = genealogyRecords.filter(r => !matchedNames.has(normalizeName(r.Name)));

console.log(`Unmatched Sheffield Census: ${unmatchedSheffield.length}`);
console.log(`Unmatched Genealogy: ${unmatchedGenealogy.length}\n`);

// Find high-confidence matches
const highConfidenceMatches = [];

for (const shef of unmatchedSheffield) {
  const shefComponents = parseNameComponents(shef.NAME);
  if (!shefComponents.firstName || !shefComponents.surname) continue;

  for (const gen of unmatchedGenealogy) {
    const genComponents = parseNameComponents(gen.Name);
    if (!genComponents.firstName || !genComponents.surname) continue;

    // Must have first name match
    if (shefComponents.firstName !== genComponents.firstName) continue;

    // Calculate surname similarity
    const surnameSimilarity = similarityScore(shefComponents.surname, genComponents.surname);

    // Check if middle names match
    const middleNameMatch = shefComponents.middleNames.length > 0 &&
                           genComponents.middleNames.length > 0 &&
                           shefComponents.middleNames.some(m1 =>
                             genComponents.middleNames.some(m2 => m1 === m2)
                           );

    // High confidence criteria:
    // 1. Exact surname match (100%)
    // 2. Very similar surname (>=85%)
    // 3. Somewhat similar surname (>=70%) + middle name match

    let confidence = 0;
    let reason = '';

    if (surnameSimilarity === 1.0) {
      confidence = 'EXACT';
      reason = 'First name + exact surname match';
    } else if (surnameSimilarity >= 0.85) {
      confidence = 'HIGH';
      reason = `First name + very similar surname (${(surnameSimilarity * 100).toFixed(0)}% similar: "${shefComponents.surname}" vs "${genComponents.surname}")`;
      if (middleNameMatch) {
        confidence = 'VERY HIGH';
        reason += ' + middle name match';
      }
    } else if (surnameSimilarity >= 0.70 && middleNameMatch) {
      confidence = 'MEDIUM-HIGH';
      reason = `First name + similar surname (${(surnameSimilarity * 100).toFixed(0)}% similar: "${shefComponents.surname}" vs "${genComponents.surname}") + middle name match`;
    }

    if (confidence) {
      highConfidenceMatches.push({
        sheffieldName: shef.NAME,
        genealogyName: gen.Name,
        confidence,
        reason,
        surnameSimilarity: (surnameSimilarity * 100).toFixed(1),
        sheffieldParish: shef['ECCLESIASTICAL PARISH'] || shef['CIVIL PARISH'],
        sheffieldRelation: shef.RELATION,
        genealogyAddress: gen.Address,
        genealogyProfession: gen.Profession,
        genealogySpouse: gen.Spouse,
        genealogyRelation: gen.Relation,
        sheffieldRecord: shef,
        genealogyRecord: gen
      });
    }
  }
}

// Sort by confidence then similarity
const confidenceOrder = { 'VERY HIGH': 1, 'EXACT': 2, 'HIGH': 3, 'MEDIUM-HIGH': 4 };
highConfidenceMatches.sort((a, b) => {
  const confDiff = confidenceOrder[a.confidence] - confidenceOrder[b.confidence];
  if (confDiff !== 0) return confDiff;
  return b.surnameSimilarity - a.surnameSimilarity;
});

console.log(`=== HIGH CONFIDENCE MATCHES: ${highConfidenceMatches.length} ===\n`);

const byConfidence = {};
highConfidenceMatches.forEach(m => {
  byConfidence[m.confidence] = (byConfidence[m.confidence] || 0) + 1;
});

console.log('Breakdown by confidence:');
Object.entries(byConfidence)
  .sort((a, b) => confidenceOrder[a[0]] - confidenceOrder[b[0]])
  .forEach(([conf, count]) => {
    console.log(`  ${conf}: ${count}`);
  });

console.log('\n=== TOP 40 HIGH CONFIDENCE MATCHES ===\n');

for (let i = 0; i < Math.min(40, highConfidenceMatches.length); i++) {
  const match = highConfidenceMatches[i];
  console.log(`${i + 1}. [${match.confidence}] "${match.sheffieldName}" vs "${match.genealogyName}"`);
  console.log(`   ${match.reason}`);
  console.log(`   Sheffield: ${match.sheffieldParish} | ${match.sheffieldRelation}`);
  console.log(`   Genealogy: ${match.genealogyAddress} | ${match.genealogyProfession || 'no profession'} | ${match.genealogyRelation}`);
  if (match.genealogySpouse) {
    console.log(`   Spouse: ${match.genealogySpouse}`);
  }
  console.log();
}

// Export to CSV
const csvHeader = 'Confidence,Sheffield Name,Genealogy Name,Surname Similarity %,Reason,Sheffield Parish,Sheffield Relation,Genealogy Address,Genealogy Profession,Genealogy Spouse,Genealogy Relation\n';
const csvRows = highConfidenceMatches.map(m =>
  `"${m.confidence}","${m.sheffieldName}","${m.genealogyName}",${m.surnameSimilarity},"${m.reason}","${m.sheffieldParish}","${m.sheffieldRelation}","${m.genealogyAddress}","${m.genealogyProfession || ''}","${m.genealogySpouse || ''}","${m.genealogyRelation}"`
);

fs.writeFileSync('high_confidence_matches_1827.csv', csvHeader + csvRows.join('\n'));
console.log(`Exported ${highConfidenceMatches.length} high-confidence matches to high_confidence_matches_1827.csv`);
console.log('\nDone!');
