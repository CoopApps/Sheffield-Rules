const Database = require('better-sqlite3');
const fs = require('fs');
const { parse } = require('csv-parse/sync');
const crypto = require('crypto');

const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('Importing White\'s Directory Business Records');
console.log('='.repeat(70) + '\n');

// Create businesses table
console.log('Creating sheffield_businesses table...');
db.exec(`
    CREATE TABLE IF NOT EXISTS sheffield_businesses (
        id TEXT PRIMARY KEY,
        surname TEXT,
        forename TEXT,
        full_name TEXT,
        title TEXT,
        occupation TEXT,
        address TEXT,
        year INTEGER,
        source TEXT,
        person_id TEXT,
        is_home_business BOOLEAN,
        profession_match BOOLEAN,
        employs_people BOOLEAN,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        FOREIGN KEY (person_id) REFERENCES sheffield_people(id)
    )
`);
console.log('✓ sheffield_businesses table created\n');

// Prepare insert statement
const insertBusiness = db.prepare(`
    INSERT INTO sheffield_businesses (
        id, surname, forename, full_name, title, occupation, address,
        year, source, person_id, is_home_business, profession_match, employs_people
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
`);

const whitesDir = 'whites';
const files = fs.readdirSync(whitesDir).filter(f => f.endsWith('.csv'));

console.log(`Found ${files.length} CSV files to process\n`);

let totalInserted = 0;
let totalSkipped = 0;

for (const file of files) {
    const filePath = `${whitesDir}/${file}`;
    const content = fs.readFileSync(filePath, 'utf-8');
    const records = parse(content, { columns: true, skip_empty_lines: true });

    let inserted = 0;
    let skipped = 0;

    for (const record of records) {
        if (!record.Surname || !record.Surname.trim()) {
            skipped++;
            continue;
        }

        const fullName = [record.Forename, record.Surname].filter(Boolean).join(' ').trim();

        insertBusiness.run(
            crypto.randomUUID(),
            record.Surname,
            record.Forename,
            fullName,
            record.Title,
            record.Occupation,
            record.Address,
            parseInt(record.Year) || 1871,
            record.Source,
            null, // person_id - will be filled in matching step
            null, // is_home_business
            null, // profession_match
            null  // employs_people
        );
        inserted++;
    }

    if (inserted > 0 || skipped > 0) {
        console.log(`${file}: ${inserted.toLocaleString()} inserted, ${skipped.toLocaleString()} skipped`);
    }

    totalInserted += inserted;
    totalSkipped += skipped;
}

console.log('\n' + '='.repeat(70));
console.log('IMPORT SUMMARY');
console.log('='.repeat(70));
console.log(`Total inserted: ${totalInserted.toLocaleString()}`);
console.log(`Total skipped: ${totalSkipped.toLocaleString()}`);

const finalCount = db.prepare('SELECT COUNT(*) as c FROM sheffield_businesses').get().c;
console.log(`Final count in database: ${finalCount.toLocaleString()}`);

// Create indexes
console.log('\nCreating indexes...');
db.exec('CREATE INDEX IF NOT EXISTS idx_businesses_name ON sheffield_businesses(full_name)');
db.exec('CREATE INDEX IF NOT EXISTS idx_businesses_surname ON sheffield_businesses(surname)');
db.exec('CREATE INDEX IF NOT EXISTS idx_businesses_address ON sheffield_businesses(address)');
console.log('✓ Indexes created');

db.close();
console.log('\n✓ Import complete!');
