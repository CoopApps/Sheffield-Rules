const Database = require('better-sqlite3');
const crypto = require('crypto');

const db = new Database('Sheffield1867.db');

console.log('Fixing genealogy record IDs...\n');

// Get all records with empty IDs
const emptyIdRecords = db.prepare(`
    SELECT rowid, id, name, address, profession
    FROM unmatched_genealogy
    WHERE id IS NULL OR id = ''
`).all();

console.log(`Found ${emptyIdRecords.length} records with empty IDs`);

if (emptyIdRecords.length === 0) {
    console.log('All records already have IDs!');
    db.close();
    process.exit(0);
}

// Update each record with a unique UUID-style ID
const updateStmt = db.prepare('UPDATE unmatched_genealogy SET id = ? WHERE rowid = ?');

let updated = 0;
for (const record of emptyIdRecords) {
    // Generate a unique ID based on the record data + rowid to ensure uniqueness
    const uniqueString = `${record.rowid}-${record.name}-${record.address || ''}-${record.profession || ''}`;
    const hash = crypto.createHash('sha256').update(uniqueString).digest('hex').substring(0, 16);

    updateStmt.run(hash, record.rowid);
    updated++;

    if (updated % 1000 === 0) {
        console.log(`Updated ${updated} records...`);
    }
}

console.log(`\n✓ Successfully updated ${updated} genealogy records with unique IDs`);

// Verify
const remaining = db.prepare(`
    SELECT COUNT(*) as count
    FROM unmatched_genealogy
    WHERE id IS NULL OR id = ''
`).get();

console.log(`Remaining records with empty IDs: ${remaining.count}`);

db.close();
console.log('\nDone!');
