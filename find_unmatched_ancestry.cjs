const { execSync } = require('child_process');
const path = require('path');

const sqlite3Path = path.join(__dirname, 'sqlite3.exe');
const dbPath = path.join(__dirname, 'Sheffield1867.db');

console.log('Finding ancestry records not in genealogy...\n');

// Get all ancestry records with the 4 key fields
console.log('Loading ancestry records...');
const ancestrySql = `SELECT name, census_age, census_folio, census_piece FROM unmatched_ancestry WHERE name IS NOT NULL AND census_age IS NOT NULL AND census_folio IS NOT NULL AND census_piece IS NOT NULL;`;
const ancestryResult = execSync(`"${sqlite3Path}" "${dbPath}" "${ancestrySql}"`, { encoding: 'utf8', maxBuffer: 50 * 1024 * 1024 });
const ancestryRecords = ancestryResult.trim().split('\n').map(line => {
    const [name, age, folio, piece] = line.split('|');
    return { name, age, folio, piece };
});

console.log(`Loaded ${ancestryRecords.length} ancestry records`);

// Get all genealogy records with the 4 key fields
console.log('Loading genealogy records...');
const genealogySql = `SELECT name, census_age, census_folio, census_piece FROM unmatched_genealogy WHERE name IS NOT NULL AND census_age IS NOT NULL AND census_folio IS NOT NULL AND census_piece IS NOT NULL;`;
const genealogyResult = execSync(`"${sqlite3Path}" "${dbPath}" "${genealogySql}"`, { encoding: 'utf8', maxBuffer: 50 * 1024 * 1024 });
const genealogyRecords = genealogyResult.trim().split('\n').map(line => {
    const [name, age, folio, piece] = line.split('|');
    return { name, age, folio, piece };
});

console.log(`Loaded ${genealogyRecords.length} genealogy records\n`);

// Create a Set of genealogy keys for fast lookup
const genealogyKeys = new Set();
for (const rec of genealogyRecords) {
    const key = `${rec.name}|${rec.age}|${rec.folio}|${rec.piece}`;
    genealogyKeys.add(key);
}

console.log(`Created lookup set with ${genealogyKeys.size} unique keys\n`);

// Find ancestry records not in genealogy
const notInGenealogy = [];
for (const rec of ancestryRecords) {
    const key = `${rec.name}|${rec.age}|${rec.folio}|${rec.piece}`;
    if (!genealogyKeys.has(key)) {
        notInGenealogy.push(rec);
    }
}

console.log(`=== RESULTS ===`);
console.log(`Total ancestry records: ${ancestryRecords.length}`);
console.log(`Total genealogy records: ${genealogyRecords.length}`);
console.log(`Ancestry records NOT in genealogy: ${notInGenealogy.length}`);
console.log(`Match rate: ${((ancestryRecords.length - notInGenealogy.length) / ancestryRecords.length * 100).toFixed(2)}%\n`);

if (notInGenealogy.length > 0) {
    console.log('First 20 examples of ancestry records not in genealogy:');
    for (let i = 0; i < Math.min(20, notInGenealogy.length); i++) {
        const rec = notInGenealogy[i];
        console.log(`  ${rec.name} | Age ${rec.age} | Folio ${rec.folio} | Piece ${rec.piece}`);
    }
}
