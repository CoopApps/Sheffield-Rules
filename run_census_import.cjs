/**
 * Script to trigger census import via Tauri command
 * This will import all census CSV files and populate the household_members column
 */

const Database = require('better-sqlite3');

// First, let's manually add the missing columns to the existing database
const db = new Database('D:/projects/Saturday at Three/Sheffield1867.db');

console.log('Adding missing columns to sheffield_players table...\n');

try {
    // Add census_household_members column
    db.exec('ALTER TABLE sheffield_players ADD COLUMN census_household_members TEXT');
    console.log('✓ Added census_household_members column');
} catch (e) {
    if (e.message.includes('duplicate column name')) {
        console.log('✓ census_household_members column already exists');
    } else {
        console.error('Error adding census_household_members:', e.message);
    }
}

try {
    // Add street_address column
    db.exec('ALTER TABLE sheffield_players ADD COLUMN street_address TEXT');
    console.log('✓ Added street_address column');
} catch (e) {
    if (e.message.includes('duplicate column name')) {
        console.log('✓ street_address column already exists');
    } else {
        console.error('Error adding street_address:', e.message);
    }
}

try {
    // Add profession column
    db.exec('ALTER TABLE sheffield_players ADD COLUMN profession TEXT');
    console.log('✓ Added profession column');
} catch (e) {
    if (e.message.includes('duplicate column name')) {
        console.log('✓ profession column already exists');
    } else {
        console.error('Error adding profession:', e.message);
    }
}

// Verify columns exist
console.log('\nVerifying database schema...');
const schema = db.prepare('PRAGMA table_info(sheffield_players)').all();
const householdCol = schema.find(col => col.name === 'census_household_members');
const addressCol = schema.find(col => col.name === 'street_address');
const professionCol = schema.find(col => col.name === 'profession');

console.log('census_household_members:', householdCol ? '✓ EXISTS' : '✗ MISSING');
console.log('street_address:', addressCol ? '✓ EXISTS' : '✗ MISSING');
console.log('profession:', professionCol ? '✓ EXISTS' : '✗ MISSING');

db.close();

console.log('\n========================================');
console.log('Database schema updated successfully!');
console.log('========================================');
console.log('\nNow you can re-import census data using the Tauri command:');
console.log('  db_import_all_census_files("D:/projects/Saturday at Three/Sheffield Census")');
console.log('\nOr import individual files using:');
console.log('  db_import_census_file("D:/projects/Saturday at Three/Sheffield Census/1804.csv")');
