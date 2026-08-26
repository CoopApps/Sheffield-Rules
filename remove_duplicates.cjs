const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('=== Removing duplicate records ===\n');

// Get initial counts
const initialSheffield = db.prepare('SELECT COUNT(*) as count FROM unmatched_sheffield').get().count;
const initialGenealogy = db.prepare('SELECT COUNT(*) as count FROM unmatched_genealogy').get().count;

console.log('Initial counts:');
console.log(`  unmatched_sheffield: ${initialSheffield.toLocaleString()}`);
console.log(`  unmatched_genealogy: ${initialGenealogy.toLocaleString()}\n`);

// Remove duplicates from unmatched_sheffield
console.log('Removing duplicates from unmatched_sheffield...');
db.exec(`
    DELETE FROM unmatched_sheffield
    WHERE id NOT IN (
        SELECT MIN(id)
        FROM unmatched_sheffield
        GROUP BY year, name, age, birth_year, birth_place, relation,
                 civil_parish, piece, folio, page
    )
`);

// Remove duplicates from unmatched_genealogy
console.log('Removing duplicates from unmatched_genealogy...');
db.exec(`
    DELETE FROM unmatched_genealogy
    WHERE id NOT IN (
        SELECT MIN(id)
        FROM unmatched_genealogy
        GROUP BY year, name, age, birth_year, birth_place, address,
                 profession, spouse, relation, parish, area
    )
`);

// Get final counts
const finalSheffield = db.prepare('SELECT COUNT(*) as count FROM unmatched_sheffield').get().count;
const finalGenealogy = db.prepare('SELECT COUNT(*) as count FROM unmatched_genealogy').get().count;

console.log('\nFinal counts:');
console.log(`  unmatched_sheffield: ${finalSheffield.toLocaleString()}`);
console.log(`  unmatched_genealogy: ${finalGenealogy.toLocaleString()}\n`);

console.log('Removed:');
console.log(`  unmatched_sheffield: ${(initialSheffield - finalSheffield).toLocaleString()} duplicates`);
console.log(`  unmatched_genealogy: ${(initialGenealogy - finalGenealogy).toLocaleString()} duplicates\n`);

// Verify Benjamin Harrop
console.log('Verifying Benjamin Harrop in 1792:');
const benjamins = db.prepare(`
    SELECT COUNT(*) as count FROM unmatched_sheffield
    WHERE name LIKE '%Benjamin Harrop%' AND year = 1792
`).get().count;
console.log(`  Found: ${benjamins} records (should be 2 - the CSV has genuine duplicates)\n`);

db.close();
console.log('✓ Duplicate removal complete!');
