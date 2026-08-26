const fs = require('fs');
const { execSync } = require('child_process');

console.log('\n========================================');
console.log('BUILDING NEW SHEFFIELD1867.DB');
console.log('========================================\n');

const dbPath = './Sheffield1867.db';

// Step 1: Delete old file if exists
if (fs.existsSync(dbPath)) {
    fs.unlinkSync(dbPath);
    console.log('✓ Deleted old Sheffield1867.db\n');
}

// Step 2: Create new database with schema
console.log('Creating new database with schema...\n');
const schemaSQL = fs.readFileSync('./create_new_sheffield_db.sql', 'utf8');

// Step 3: Add club population SQL
const clubSQL = fs.readFileSync('./populate_clubs_with_reserves.sql', 'utf8');

// Combine into one SQL file
const fullSQL = schemaSQL + '\n\n' + clubSQL;
fs.writeFileSync('./build_sheffield_db_complete.sql', fullSQL);

console.log('✓ Generated complete SQL file\n');
console.log('SQL includes:');
console.log('  - Schema for all tables with postcode columns');
console.log('  - 186 main clubs');
console.log('  - 186 reserve teams');
console.log('  - Empty institutional tables\n');

console.log('To execute, you need sqlite3. Install it or use:');
console.log('  https://www.sqlite.org/download.html');
console.log('\nThen run:');
console.log(`  sqlite3 "${dbPath}" < build_sheffield_db_complete.sql\n`);
console.log('Or in PowerShell:');
console.log(`  Get-Content build_sheffield_db_complete.sql | sqlite3 "${dbPath}"\n`);
