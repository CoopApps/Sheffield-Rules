const fs = require('fs');
const { parse } = require('csv-parse/sync');

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

  return expanded;
}

// Helper function to normalize names
function normalizeName(name) {
  if (!name) return '';
  let normalized = expandAbbreviations(name);
  return normalized.toLowerCase()
    .replace(/[.,\-]/g, ' ')
    .replace(/\s+/g, ' ')
    .trim();
}

// Parse name into components
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

// Load unmatched records
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

// Find partial matches
const partialMatches = [];

for (const shef of unmatchedSheffield) {
  const shefComponents = parseNameComponents(shef.NAME);
  if (!shefComponents.firstName) continue;

  for (const gen of unmatchedGenealogy) {
    const genComponents = parseNameComponents(gen.Name);
    if (!genComponents.firstName) continue;

    let matchScore = 0;
    let matchReasons = [];

    // Check first name match
    const firstNameMatch = shefComponents.firstName === genComponents.firstName;
    if (firstNameMatch) {
      matchScore++;
      matchReasons.push('first name');
    }

    // Check surname match
    const surnameMatch = shefComponents.surname && genComponents.surname &&
                        shefComponents.surname === genComponents.surname;
    if (surnameMatch) {
      matchScore++;
      matchReasons.push('surname');
    }

    // Check first 2 letters of surname
    const surnamePrefix = shefComponents.surname && genComponents.surname &&
                         shefComponents.surname.length >= 2 && genComponents.surname.length >= 2 &&
                         shefComponents.surname.substring(0, 2) === genComponents.surname.substring(0, 2);

    if (!surnameMatch && surnamePrefix) {
      matchScore += 0.5;
      matchReasons.push(`surname prefix (${shefComponents.surname.substring(0, 2)}*)`);
    }

    // Check middle name matches
    if (shefComponents.middleNames.length > 0 && genComponents.middleNames.length > 0) {
      const middleMatch = shefComponents.middleNames.some(m1 =>
        genComponents.middleNames.some(m2 => m1 === m2)
      );
      if (middleMatch) {
        matchScore += 0.5;
        matchReasons.push('middle name');
      }
    }

    // Report if we have decent match
    // Criteria 1: First name + surname match (2 points)
    // Criteria 2: First name + first 2 letters of surname (1.5 points)
    if ((firstNameMatch && surnameMatch) ||
        (firstNameMatch && surnamePrefix && matchScore >= 1.5)) {

      partialMatches.push({
        sheffieldName: shef.NAME,
        genealogyName: gen.Name,
        sheffieldParish: shef['ECCLESIASTICAL PARISH'],
        genealogyAddress: gen.Address,
        genealogyProfession: gen.Profession,
        genealogySpouse: gen.Spouse,
        genealogyRelation: gen.Relation,
        matchScore,
        matchReasons: matchReasons.join(', ')
      });
    }
  }
}

// Sort by match score (descending)
partialMatches.sort((a, b) => b.matchScore - a.matchScore);

console.log(`=== POTENTIAL PARTIAL MATCHES: ${partialMatches.length} ===\n`);

// Group by match type
const fullMatches = partialMatches.filter(m => m.matchScore >= 2);
const partialSurnameMatches = partialMatches.filter(m => m.matchScore >= 1.5 && m.matchScore < 2);

console.log(`Full matches (first + surname): ${fullMatches.length}`);
console.log(`Partial matches (first + surname prefix): ${partialSurnameMatches.length}\n`);

console.log('=== FIRST 30 POTENTIAL MATCHES ===\n');

for (let i = 0; i < Math.min(30, partialMatches.length); i++) {
  const match = partialMatches[i];
  console.log(`${i + 1}. Sheffield: "${match.sheffieldName}" vs Genealogy: "${match.genealogyName}"`);
  console.log(`   Match: ${match.matchReasons} (score: ${match.matchScore})`);
  console.log(`   Sheffield Parish: ${match.sheffieldParish}`);
  console.log(`   Genealogy: ${match.genealogyAddress} | ${match.genealogyProfession || 'no profession'}`);
  if (match.genealogySpouse) {
    console.log(`   Spouse: ${match.genealogySpouse} | Relation: ${match.genealogyRelation}`);
  }
  console.log();
}

// Export to CSV for review
const csvHeader = 'Sheffield Name,Genealogy Name,Match Score,Match Reasons,Sheffield Parish,Genealogy Address,Genealogy Profession,Genealogy Spouse,Genealogy Relation\n';
const csvRows = partialMatches.map(m =>
  `"${m.sheffieldName}","${m.genealogyName}",${m.matchScore},"${m.matchReasons}","${m.sheffieldParish}","${m.genealogyAddress}","${m.genealogyProfession || ''}","${m.genealogySpouse || ''}","${m.genealogyRelation}"`
);

fs.writeFileSync('potential_matches_1827.csv', csvHeader + csvRows.join('\n'));
console.log(`\nExported ${partialMatches.length} potential matches to potential_matches_1827.csv`);
console.log('\nDone!');
