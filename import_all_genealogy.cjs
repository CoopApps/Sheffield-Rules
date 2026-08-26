const Database = require('better-sqlite3');
const fs = require('fs');
const { parse } = require('csv-parse/sync');
const crypto = require('crypto');

const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('Importing ALL Genealogy Records');
console.log('='.repeat(70) + '\n');

const insertGenealogy = db.prepare(`
    INSERT INTO unmatched_genealogy (
        id, year, name, age, birth_year, birth_place, address,
        profession, spouse, relation, parish, area
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
`);

// Track all records for deduplication
const recordKeys = new Set();

// Years to process
const years = [];
for (let y = 1772; y <= 1852; y++) {
    years.push(y);
}

let totalInserted = 0;
let totalSkipped = 0;

const genealogyDir = 'Genealogy';

for (const year of years) {
    // Find all genealogy files for this year
    const genealogyFiles = fs.existsSync(genealogyDir)
        ? fs.readdirSync(genealogyDir).filter(f =>
            f.includes(`tg_${year}_`) && f.endsWith('.csv'))
        : [];

    if (genealogyFiles.length === 0) continue;

    let inserted = 0;
    let skipped = 0;

    for (const file of genealogyFiles) {
        const content = fs.readFileSync(`${genealogyDir}/${file}`, 'utf-8');
        const records = parse(content, { columns: true, skip_empty_lines: true });

        for (const record of records) {
            if (!record.Name || !record.Name.trim()) {
                skipped++;
                continue;
            }

            // Create unique key for deduplication
            const key = [
                year,
                record.Name,
                record.Age,
                record['Born Approx'],
                record['Birth Place'],
                record.Address,
                record.Profession,
                record.Spouse,
                record.Relation,
                record.Parish,
                record.Area
            ].join('|');

            if (recordKeys.has(key)) {
                skipped++;
                continue;
            }

            recordKeys.add(key);

            insertGenealogy.run(
                crypto.randomUUID(),
                year,
                record.Name,
                record.Age ? parseInt(record.Age) : null,
                record['Born Approx'] ? parseInt(record['Born Approx']) : null,
                record['Birth Place'],
                record.Address,
                record.Profession,
                record.Spouse,
                record.Relation,
                record.Parish,
                record.Area
            );
            inserted++;
        }
    }

    if (inserted > 0 || skipped > 0) {
        console.log(`${year}: ${inserted.toLocaleString()} inserted, ${skipped.toLocaleString()} skipped`);
    }

    totalInserted += inserted;
    totalSkipped += skipped;
}

console.log('\n' + '='.repeat(70));
console.log('SUMMARY');
console.log('='.repeat(70));
console.log(`Total inserted: ${totalInserted.toLocaleString()}`);
console.log(`Total skipped (duplicates/blank): ${totalSkipped.toLocaleString()}`);

const finalCount = db.prepare('SELECT COUNT(*) as c FROM unmatched_genealogy').get().c;
console.log(`Final count in database: ${finalCount.toLocaleString()}`);

db.close();
console.log('\n✓ Import complete!');
