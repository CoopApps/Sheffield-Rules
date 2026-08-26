const { execSync } = require('child_process');
const path = require('path');

const sqlite3Path = path.join(__dirname, 'sqlite3.exe');
const dbPath = path.join(__dirname, 'Sheffield1867.db');

console.log('Assigning postcodes based on census folio/piece matches...\n');

// Step 1: Load all unique folio/piece → postcode mappings
console.log('Loading folio/piece → postcode mappings...');
const mappingSql = `
SELECT DISTINCT census_folio, census_piece, postcode
FROM unmatched_genealogy
WHERE census_folio IS NOT NULL
  AND census_piece IS NOT NULL
  AND postcode IS NOT NULL
  AND postcode != '';
`;

const mappingResult = execSync(`"${sqlite3Path}" "${dbPath}" "${mappingSql}"`, {
    encoding: 'utf8',
    maxBuffer: 50 * 1024 * 1024
});

const mappings = new Map();
const lines = mappingResult.trim().split('\n');

for (const line of lines) {
    const [folio, piece, postcode] = line.split('|');
    const key = `${folio}|${piece}`;

    // Store first postcode found for each folio/piece combo
    if (!mappings.has(key)) {
        mappings.set(key, postcode);
    }
}

console.log(`Loaded ${mappings.size} unique folio/piece combinations with postcodes\n`);

// Step 2: Load all records that need postcodes
console.log('Loading records without postcodes...');
const needPostcodeSql = `
SELECT rowid, census_folio, census_piece
FROM unmatched_genealogy
WHERE (postcode IS NULL OR postcode = '')
  AND census_folio IS NOT NULL
  AND census_piece IS NOT NULL;
`;

const needResult = execSync(`"${sqlite3Path}" "${dbPath}" "${needPostcodeSql}"`, {
    encoding: 'utf8',
    maxBuffer: 50 * 1024 * 1024
});

const needPostcode = needResult.trim().split('\n').map(line => {
    const [rowid, folio, piece] = line.split('|');
    return { rowid, folio, piece };
});

console.log(`Found ${needPostcode.length} records without postcodes\n`);

// Step 3: Generate UPDATE statements in batches
console.log('Generating UPDATE statements...');
let updateCount = 0;
const batchSize = 500;
let currentBatch = [];

for (const record of needPostcode) {
    const key = `${record.folio}|${record.piece}`;
    const postcode = mappings.get(key);

    if (postcode) {
        currentBatch.push(`UPDATE unmatched_genealogy SET postcode = '${postcode}' WHERE rowid = ${record.rowid};`);
        updateCount++;

        // Execute batch when full
        if (currentBatch.length >= batchSize) {
            const batchSql = currentBatch.join('\n');
            execSync(`"${sqlite3Path}" "${dbPath}" "${batchSql}"`, { encoding: 'utf8' });
            process.stdout.write(`\rUpdated ${updateCount} records...`);
            currentBatch = [];
        }
    }
}

// Execute remaining batch
if (currentBatch.length > 0) {
    const batchSql = currentBatch.join('\n');
    execSync(`"${sqlite3Path}" "${dbPath}" "${batchSql}"`, { encoding: 'utf8' });
    process.stdout.write(`\rUpdated ${updateCount} records...`);
}

console.log('\n\n=== RESULTS ===');
console.log(`Records updated with postcodes: ${updateCount}`);
console.log(`Records still without postcodes: ${needPostcode.length - updateCount}`);

// Final statistics
const statsSql = `
SELECT
    'With postcodes:' as label, COUNT(*) as count
FROM unmatched_genealogy
WHERE postcode IS NOT NULL AND postcode != ''
UNION ALL
SELECT
    'Without postcodes:' as label, COUNT(*) as count
FROM unmatched_genealogy
WHERE postcode IS NULL OR postcode = ''
UNION ALL
SELECT
    'Total records:' as label, COUNT(*) as count
FROM unmatched_genealogy;
`;

const statsResult = execSync(`"${sqlite3Path}" "${dbPath}" "${statsSql}"`, { encoding: 'utf8' });
console.log('\n' + statsResult);
