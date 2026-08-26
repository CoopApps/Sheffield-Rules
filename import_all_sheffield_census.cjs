const Database = require('better-sqlite3');
const fs = require('fs');
const { parse } = require('csv-parse/sync');
const crypto = require('crypto');

const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('Importing ALL Sheffield Census Records');
console.log('='.repeat(70) + '\n');

const insertSheffield = db.prepare(`
    INSERT INTO unmatched_sheffieldcensus (
        id, year, name, age, gender, birth_year, birth_place, relation,
        household_members, civil_parish, ecclesiastical_parish,
        registration_district, county, piece, folio, page
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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

for (const year of years) {
    const csvPath1 = `Sheffield Census/${year}.csv`;
    const csvPath2 = `Sheffield Census/${year} - done.csv`;

    let csvPath = null;
    if (fs.existsSync(csvPath1)) {
        csvPath = csvPath1;
    } else if (fs.existsSync(csvPath2)) {
        csvPath = csvPath2;
    }

    if (csvPath) {
        const content = fs.readFileSync(csvPath, 'utf-8');
        const records = parse(content, { columns: true, skip_empty_lines: true });

        let inserted = 0;
        let skipped = 0;

        for (const record of records) {
            if (!record.NAME || !record.NAME.trim()) {
                skipped++;
                continue;
            }

            // Create unique key for deduplication
            const key = [
                year,
                record.NAME,
                record.AGE,
                record['ESTIMATED BIRTH YEAR'],
                record['Birth Place'],
                record.RELATION,
                record['CIVIL PARISH'],
                record.PIECE,
                record.FOLIO,
                record['PAGE NUMBER']
            ].join('|');

            if (recordKeys.has(key)) {
                skipped++;
                continue;
            }

            recordKeys.add(key);

            insertSheffield.run(
                crypto.randomUUID(),
                year,
                record.NAME,
                record.AGE ? parseInt(record.AGE) : null,
                record.GENDER,
                record['ESTIMATED BIRTH YEAR'] ? parseInt(record['ESTIMATED BIRTH YEAR']) : null,
                record['Birth Place'],
                record.RELATION,
                record['HOUSEHOLD MEMBERS'],
                record['CIVIL PARISH'],
                record['ECCLESIASTICAL PARISH'],
                record['REGISTRATION DISTRICT'],
                record['COUNTY/ISLAND'],
                record.PIECE,
                record.FOLIO,
                record['PAGE NUMBER']
            );
            inserted++;
        }

        if (inserted > 0 || skipped > 0) {
            console.log(`${year}: ${inserted.toLocaleString()} inserted, ${skipped.toLocaleString()} skipped`);
        }

        totalInserted += inserted;
        totalSkipped += skipped;
    }
}

console.log('\n' + '='.repeat(70));
console.log('SUMMARY');
console.log('='.repeat(70));
console.log(`Total inserted: ${totalInserted.toLocaleString()}`);
console.log(`Total skipped (duplicates/blank): ${totalSkipped.toLocaleString()}`);

const finalCount = db.prepare('SELECT COUNT(*) as c FROM unmatched_sheffieldcensus').get().c;
console.log(`Final count in database: ${finalCount.toLocaleString()}`);

// Recreate indexes with new table name
console.log('\nRecreating indexes...');
db.exec('DROP INDEX IF EXISTS idx_unmatched_sheffield_name');
db.exec('DROP INDEX IF EXISTS idx_unmatched_sheffield_year');
db.exec('CREATE INDEX idx_unmatched_sheffieldcensus_name ON unmatched_sheffieldcensus(name)');
db.exec('CREATE INDEX idx_unmatched_sheffieldcensus_year ON unmatched_sheffieldcensus(year)');
console.log('✓ Indexes created');

db.close();
console.log('\n✓ Import complete!');
