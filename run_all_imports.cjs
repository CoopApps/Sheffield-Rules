const { execSync } = require('child_process');
const fs = require('fs');

console.log('\n========================================');
console.log('SHEFFIELD1867.DB DATA IMPORT');
console.log('========================================\n');

const dbPath = './Sheffield1867.db';

// Check if database exists
if (!fs.existsSync(dbPath)) {
    console.error(`ERROR: Database not found at ${dbPath}`);
    console.error('Please create the database first using build_sheffield_db_complete.sql');
    process.exit(1);
}

console.log(`Database found: ${dbPath}\n`);

// Import scripts in order
const importScripts = [
    { name: 'Census Data', script: 'import_census.cjs', sql: 'import_census.sql', table: 'unmatched_ancestry' },
    { name: 'Genealogy Data', script: 'import_genealogy.cjs', sql: 'import_genealogy.sql', table: 'unmatched_genealogy' },
    { name: 'Geni Data', script: 'import_geni.cjs', sql: 'import_geni.sql', table: 'unmatched_genealogy' },
    { name: 'Whites Business Data', script: 'import_whites.cjs', sql: 'import_whites.sql', table: 'sheffield_businesses' }
];

const results = [];

for (const imp of importScripts) {
    console.log(`\n========================================`);
    console.log(`PROCESSING: ${imp.name.toUpperCase()}`);
    console.log(`========================================\n`);

    try {
        // Step 1: Run the import script to generate SQL
        console.log(`[1/2] Running ${imp.script}...`);
        execSync(`node "${imp.script}"`, { stdio: 'inherit' });

        // Step 2: Execute the SQL file
        console.log(`\n[2/2] Executing ${imp.sql}...`);
        const sqlite3Path = './sqlite3.exe';

        if (!fs.existsSync(sqlite3Path)) {
            console.error(`ERROR: sqlite3.exe not found at ${sqlite3Path}`);
            console.error('Please download sqlite3.exe to the project root');
            process.exit(1);
        }

        execSync(`"${sqlite3Path}" "${dbPath}" < "${imp.sql}"`, { stdio: 'inherit' });

        // Step 3: Count records in table
        const countCmd = `"${sqlite3Path}" "${dbPath}" "SELECT COUNT(*) FROM ${imp.table};"`;
        const count = execSync(countCmd, { encoding: 'utf8' }).trim();

        results.push({
            name: imp.name,
            count: parseInt(count, 10),
            success: true
        });

        console.log(`\n✓ ${imp.name} imported successfully: ${count} records\n`);

    } catch (err) {
        console.error(`\n✗ Error importing ${imp.name}:`);
        console.error(err.message);
        results.push({
            name: imp.name,
            count: 0,
            success: false,
            error: err.message
        });
    }
}

// Final summary
console.log(`\n\n========================================`);
console.log('IMPORT COMPLETE - FINAL SUMMARY');
console.log(`========================================\n`);

let totalRecords = 0;
let successCount = 0;

for (const result of results) {
    const status = result.success ? '✓' : '✗';
    console.log(`${status} ${result.name}: ${result.count.toLocaleString()} records`);
    if (result.success) {
        totalRecords += result.count;
        successCount++;
    } else {
        console.log(`  Error: ${result.error}`);
    }
}

console.log(`\n========================================`);
console.log(`Successful imports: ${successCount}/${results.length}`);
console.log(`Total records imported: ${totalRecords.toLocaleString()}`);
console.log(`========================================\n`);

// Verify clubs table
try {
    const sqlite3Path = './sqlite3.exe';
    const clubCount = execSync(`"${sqlite3Path}" "${dbPath}" "SELECT COUNT(*) FROM sheffield_clubs;"`, { encoding: 'utf8' }).trim();
    console.log(`Sheffield clubs in database: ${clubCount} (should be 372)\n`);
} catch (err) {
    console.error('Could not verify clubs table');
}

// Sort tables
console.log('\n========================================');
console.log('SORTING TABLES');
console.log('========================================\n');

try {
    console.log('Sorting unmatched_ancestry by surname (A-Z)...');
    console.log('Sorting unmatched_genealogy by street address...\n');

    const sqlite3Path = './sqlite3.exe';
    execSync(`"${sqlite3Path}" "${dbPath}" < "sort_tables.sql"`, { stdio: 'inherit' });

    console.log('\n✓ Tables sorted successfully\n');
} catch (err) {
    console.error('✗ Error sorting tables:');
    console.error(err.message);
}

console.log('Import process complete.\n');
