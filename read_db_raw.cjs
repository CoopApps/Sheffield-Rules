// Read SQLite database using raw file parsing
const fs = require('fs');

function readSQLiteCount(dbPath, tableName) {
    const buffer = fs.readFileSync(dbPath);

    // SQLite file format check
    const header = buffer.toString('ascii', 0, 16);
    if (!header.startsWith('SQLite format 3')) {
        throw new Error('Not a valid SQLite database');
    }

    // For a simple count, we'll use sqlite3 module if available,
    // otherwise just check if table exists in the file
    const dbContent = buffer.toString('binary');
    const tableMatch = dbContent.match(new RegExp(`CREATE TABLE[^(]*${tableName}`, 'i'));

    if (tableMatch) {
        console.log(`✓ Table '${tableName}' exists in database`);

        // Try to estimate records by counting PRIMARY KEY occurrences
        // This is a rough estimate
        const matches = dbContent.match(/[a-z0-9]{8}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{4}-[a-z0-9]{12}/gi);
        if (matches) {
            console.log(`Estimated records (by UUID pattern): ~${matches.length}`);
        }
        return true;
    } else {
        console.log(`✗ Table '${tableName}' not found`);
        return false;
    }
}

// Check clubs table
console.log('\n=== Analyzing saturday_at_three.db ===\n');
readSQLiteCount('./saturday_at_three.db', 'clubs');

// Also list file size
const stats = fs.statSync('./saturday_at_three.db');
console.log(`\nDatabase size: ${(stats.size / 1024).toFixed(2)} KB`);
