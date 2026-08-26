const { execSync } = require('child_process');
const path = require('path');
const fs = require('fs');

const sqlite3Path = path.join(__dirname, 'sqlite3.exe');
const dbPath = path.join(__dirname, 'Sheffield1867.db');

console.log('Assigning postcodes based on census folio/piece matches...\n');

// Step 1: Load all unique folio/piece → postcode mappings
console.log('Loading folio/piece → postcode mappings...');
const mappingResult = execSync(`"${sqlite3Path}" "${dbPath}" < get_folio_postcode_mappings.sql`, {
    encoding: 'utf8',
    maxBuffer: 50 * 1024 * 1024,
    cwd: __dirname
});

const mappings = new Map();
const lines = mappingResult.trim().split('\n').filter(l => l.trim());

for (const line of lines) {
    const parts = line.split('|');
    if (parts.length >= 3) {
        const folio = parts[0].trim();
        const piece = parts[1].trim();
        const postcode = parts[2].trim();
        const key = `${folio}|${piece}`;

        // Store first postcode found for each folio/piece combo
        if (!mappings.has(key) && postcode) {
            mappings.set(key, postcode);
        }
    }
}

console.log(`Loaded ${mappings.size} unique folio/piece combinations with postcodes\n`);

// Step 2: Load all records that need postcodes
console.log('Loading records without postcodes...');
const needResult = execSync(`"${sqlite3Path}" "${dbPath}" < get_records_needing_postcodes.sql`, {
    encoding: 'utf8',
    maxBuffer: 50 * 1024 * 1024,
    cwd: __dirname
});

const needPostcode = needResult.trim().split('\n').filter(l => l.trim()).map(line => {
    const parts = line.split('|');
    if (parts.length >= 3) {
        return {
            rowid: parts[0].trim(),
            folio: parts[1].trim(),
            piece: parts[2].trim()
        };
    }
    return null;
}).filter(r => r !== null);

console.log(`Found ${needPostcode.length} records without postcodes\n`);

// Step 3: Generate UPDATE statements in batches
console.log('Generating and executing UPDATE statements in batches...');
let updateCount = 0;
let skippedCount = 0;
const batchSize = 500;
let currentBatch = [];

for (const record of needPostcode) {
    const key = `${record.folio}|${record.piece}`;
    const postcode = mappings.get(key);

    if (postcode) {
        // Escape single quotes in postcode
        const escapedPostcode = postcode.replace(/'/g, "''");
        currentBatch.push(`UPDATE unmatched_genealogy SET postcode = '${escapedPostcode}' WHERE rowid = ${record.rowid};`);
        updateCount++;

        // Execute batch when full
        if (currentBatch.length >= batchSize) {
            const batchSql = currentBatch.join('\n');
            const tempFile = path.join(__dirname, 'temp_update_batch.sql');
            fs.writeFileSync(tempFile, batchSql, 'utf8');
            execSync(`"${sqlite3Path}" "${dbPath}" < temp_update_batch.sql`, { encoding: 'utf8', cwd: __dirname });
            fs.unlinkSync(tempFile);
            process.stdout.write(`\rUpdated ${updateCount} records...`);
            currentBatch = [];
        }
    } else {
        skippedCount++;
    }
}

// Execute remaining batch
if (currentBatch.length > 0) {
    const batchSql = currentBatch.join('\n');
    const tempFile = path.join(__dirname, 'temp_update_batch.sql');
    fs.writeFileSync(tempFile, batchSql, 'utf8');
    execSync(`"${sqlite3Path}" "${dbPath}" < temp_update_batch.sql`, { encoding: 'utf8', cwd: __dirname });
    fs.unlinkSync(tempFile);
}

console.log('\n\n=== RESULTS ===');
console.log(`Records updated with postcodes: ${updateCount}`);
console.log(`Records skipped (no matching folio/piece): ${skippedCount}`);
console.log(`Total processed: ${needPostcode.length}`);

// Final statistics
const statsResult = execSync(`"${sqlite3Path}" "${dbPath}" < check_postcode_stats.sql`, {
    encoding: 'utf8',
    cwd: __dirname
});

console.log('\n=== FINAL STATISTICS ===');
console.log(statsResult);
