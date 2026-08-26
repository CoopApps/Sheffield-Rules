const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

// Find Alfred Latham records
console.log('Looking for Alfred Latham matches...\n');

const matched = db.prepare(`
    SELECT id, name, address, profession,
           business_forename, business_surname, business_address, business_occupation
    FROM unmatched_genealogy
    WHERE name LIKE '%Alfred Latham%' AND business_surname IS NOT NULL
`).all();

if (matched.length === 0) {
    console.log('No matched Alfred Latham records found.');
    db.close();
    process.exit(0);
}

console.log(`Found ${matched.length} matched Alfred Latham record(s):\n`);
matched.forEach((rec, i) => {
    console.log(`${i + 1}. Genealogy: ${rec.name} at ${rec.address}`);
    console.log(`   Matched to: ${rec.business_forename} ${rec.business_surname} at ${rec.business_address}`);
    console.log(`   Occupation: ${rec.business_occupation}\n`);
});

// Clear the matches
console.log('Clearing matches...');
const result = db.prepare(`
    UPDATE unmatched_genealogy
    SET business_forename = NULL,
        business_surname = NULL,
        business_address = NULL,
        business_occupation = NULL,
        business_year = NULL,
        business_source = NULL
    WHERE name LIKE '%Alfred Latham%' AND business_surname IS NOT NULL
`).run();

console.log(`✓ Cleared ${result.changes} Alfred Latham match(es)`);

db.close();
