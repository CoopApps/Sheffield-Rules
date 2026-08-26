const fs = require('fs');
const { parse } = require('csv-parse/sync');

function normalizeName(name) {
  if (!name) return '';
  return name.toLowerCase()
    .replace(/[.,\-]/g, ' ')
    .replace(/\s+/g, ' ')
    .trim();
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

// Find genealogy records with spouses
const withSpouses = genealogyRecords.filter(r => r.Spouse);
console.log(`Genealogy records with spouses: ${withSpouses.length}`);
console.log(`Total Sheffield records: ${sheffieldRecords.length}\n`);

// Check some examples
console.log('=== CHECKING SPOUSE MATCHES ===\n');

let checkCount = 0;
for (const genRecord of withSpouses.slice(0, 20)) {
  const spouseName = normalizeName(genRecord.Spouse);
  const personName = normalizeName(genRecord.Name);

  console.log(`${++checkCount}. ${genRecord.Name} (spouse: ${genRecord.Spouse})`);
  console.log(`   Normalized person: "${personName}"`);
  console.log(`   Normalized spouse: "${spouseName}"`);

  // Look for spouse in Sheffield census
  const shefMatch = sheffieldRecords.find(s => normalizeName(s.NAME) === spouseName);

  if (shefMatch) {
    console.log(`   ✓ Found spouse in Sheffield census: ${shefMatch.NAME}`);
    const household = parseHouseholdMembers(shefMatch['HOUSEHOLD MEMBERS']);
    console.log(`   Household members (${household.length}):`);
    household.forEach(m => {
      const normalized = normalizeName(m.name);
      const match = normalized === personName ? ' ← MATCH!' : '';
      console.log(`     - ${m.name} (${m.age}) [${normalized}]${match}`);
    });
  } else {
    console.log(`   ✗ Spouse not found in Sheffield census`);
  }
  console.log();
}
