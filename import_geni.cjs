const fs = require('fs');
const path = require('path');
const {
    expandNameAbbreviations,
    capitalizeWords,
    splitName,
    sqlEscape
} = require('./import_helpers.cjs');

console.log('\n========================================');
console.log('IMPORTING GENI DATA');
console.log('========================================\n');

const geniDir = './Geni';
const outputSQL = './import_geni.sql';

if (!fs.existsSync(geniDir)) {
    console.error(`ERROR: Geni directory not found: ${geniDir}`);
    process.exit(1);
}

const files = fs.readdirSync(geniDir).filter(f => f.startsWith('Sheffield_1871_') && f.endsWith('.csv'));
console.log(`Found ${files.length} Geni files\n`);

const sqlStatements = [];
let totalRecords = 0;
let errorCount = 0;

for (const file of files) {
    const filePath = path.join(geniDir, file);
    const content = fs.readFileSync(filePath, 'utf8');
    const lines = content.split('\n');

    if (lines.length < 2) {
        console.log(`⚠ Skipping ${file} - no data rows`);
        continue;
    }

    // Parse headers
    const headers = lines[0].split(',').map(h => h.trim());
    let fileRecords = 0;

    for (let i = 1; i < lines.length; i++) {
        const line = lines[i].trim();
        if (!line) continue;

        try {
            // Simple CSV parsing (no quoted fields in Geni data)
            const values = line.split(',').map(v => v.trim());
            const record = {};
            headers.forEach((h, idx) => {
                record[h] = values[idx] || null;
            });

            // Combine surname and forename
            const surname = record['Surname'] || '';
            const forename = record['Fore Name'] || '';
            const fullName = `${forename} ${surname}`.trim();

            // Expand and split name
            const expandedName = expandNameAbbreviations(fullName);
            const nameParts = splitName(expandedName);

            // Capitalize location fields
            const registrationDistrict = capitalizeWords(record['Registration District']);
            const subDistrict = capitalizeWords(record['Sub District']);

            sqlStatements.push(`INSERT INTO unmatched_genealogy (name, first_name, middle_name, surname, census_age, census_piece, census_folio, registration_district, sub_registration_district, genealogy_source) VALUES (${sqlEscape(expandedName)}, ${sqlEscape(nameParts.first)}, ${sqlEscape(nameParts.middle)}, ${sqlEscape(nameParts.surname)}, ${sqlEscape(record['Age'])}, ${sqlEscape(record['Piece Number RG10'])}, ${sqlEscape(record['Folio Number'])}, ${sqlEscape(registrationDistrict)}, ${sqlEscape(subDistrict)}, 'geni');`);

            fileRecords++;
            totalRecords++;
        } catch (err) {
            errorCount++;
            console.error(`  Error parsing line ${i} in ${file}: ${err.message}`);
        }
    }

    console.log(`✓ ${file}: ${fileRecords} records`);
}

console.log(`\n========================================`);
console.log(`GENI IMPORT SUMMARY`);
console.log(`========================================`);
console.log(`Files processed: ${files.length}`);
console.log(`Total records: ${totalRecords}`);
console.log(`Errors: ${errorCount}`);
console.log(`========================================\n`);

// Write SQL file with transaction wrapper
const sqlContent = `BEGIN TRANSACTION;\n${sqlStatements.join('\n')}\nCOMMIT;`;
fs.writeFileSync(outputSQL, sqlContent);
console.log(`SQL written to: ${outputSQL}\n`);
console.log(`To execute: sqlite3 Sheffield1867.db < ${outputSQL}\n`);
