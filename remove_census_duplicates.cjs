const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('\n========================================');
console.log('REMOVING DUPLICATE CENSUS RECORDS');
console.log('========================================\n');

// Get current count
const beforeCount = db.prepare('SELECT COUNT(*) as c FROM unmatched_sheffieldcensus').get().c;
console.log(`Current census records: ${beforeCount.toLocaleString()}`);

// Find duplicates based on name + age + registration_district + sub_district + piece_number + folio_number
// (same person with same census location identifiers)
console.log('\nFinding duplicates (matching name + age + registration_district + sub_district + piece_number + folio_number)...');

const duplicates = db.prepare(`
    SELECT name, age, registration_district, sub_district, piece_number, folio_number,
           COUNT(*) as count, GROUP_CONCAT(rowid) as rowids
    FROM unmatched_sheffieldcensus
    WHERE name IS NOT NULL
    GROUP BY name, age, registration_district, sub_district, piece_number, folio_number
    HAVING COUNT(*) > 1
`).all();

console.log(`Found ${duplicates.length.toLocaleString()} duplicate groups\n`);

if (duplicates.length === 0) {
    console.log('No duplicates found!');
    db.close();
    process.exit(0);
}

// Show some examples
console.log('Sample duplicates:');
duplicates.slice(0, 5).forEach(dup => {
    console.log(`  "${dup.name}" age ${dup.age}, RG10/${dup.piece_number} folio ${dup.folio_number} - ${dup.count} copies`);
});
console.log('');

// Delete duplicates, keeping only the first occurrence
let totalDeleted = 0;

for (const dup of duplicates) {
    const rowids = dup.rowids.split(',').map(id => parseInt(id));
    // Keep the first rowid, delete the rest
    const toDelete = rowids.slice(1);

    for (const rowid of toDelete) {
        db.prepare('DELETE FROM unmatched_sheffieldcensus WHERE rowid = ?').run(rowid);
        totalDeleted++;
    }

    if (totalDeleted % 1000 === 0) {
        console.log(`Deleted ${totalDeleted.toLocaleString()} duplicates...`);
    }
}

const afterCount = db.prepare('SELECT COUNT(*) as c FROM unmatched_sheffieldcensus').get().c;

console.log('\n========================================');
console.log('COMPLETE');
console.log('========================================');
console.log(`Before: ${beforeCount.toLocaleString()} records`);
console.log(`Deleted: ${totalDeleted.toLocaleString()} duplicates`);
console.log(`After: ${afterCount.toLocaleString()} records`);
console.log(`Kept: ${duplicates.length.toLocaleString()} unique groups (one from each duplicate set)`);
console.log('========================================\n');

db.close();
