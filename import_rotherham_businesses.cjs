const fs = require('fs');
const path = require('path');
const {
    expandNameAbbreviations,
    expandProfessionAbbreviations,
    capitalizeWords,
    normalizeAddress,
    splitName,
    sqlEscape
} = require('./import_helpers.cjs');

console.log('\n========================================');
console.log('IMPORTING ROTHERHAM BUSINESSES ONLY');
console.log('========================================\n');

const whitesDir = './Whites';
const outputSQL = './import_rotherham.sql';

if (!fs.existsSync(whitesDir)) {
    console.error(`ERROR: Whites directory not found: ${whitesDir}`);
    process.exit(1);
}

const files = fs.readdirSync(whitesDir).filter(f => f.startsWith('sheffield_1871_') && f.endsWith('.csv'));
console.log(`Found ${files.length} Whites files\n`);

const sqlStatements = [];
let totalRecords = 0;
let errorCount = 0;

for (const file of files) {
    const filePath = path.join(whitesDir, file);
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
            // Parse CSV - fields may contain commas, so we need proper parsing
            const values = [];
            let current = '';
            let inQuotes = false;

            for (let j = 0; j < line.length; j++) {
                const char = line[j];
                if (char === '"') {
                    inQuotes = !inQuotes;
                } else if (char === ',' && !inQuotes) {
                    values.push(current.trim());
                    current = '';
                } else {
                    current += char;
                }
            }
            values.push(current.trim());

            const record = {};
            headers.forEach((h, idx) => {
                record[h] = values[idx] || null;
            });

            // Normalize address
            const rawAddress = record['Address'] || '';

            // ONLY PROCESS ROTHERHAM BUSINESSES
            if (!rawAddress.toLowerCase().includes('rotherham')) {
                continue;
            }

            // Combine surname and forename
            const surname = record['Surname'] || '';
            const forename = record['Forename'] || '';
            const fullName = `${forename} ${surname}`.trim();

            // Expand and split name
            const expandedName = expandNameAbbreviations(fullName);
            const nameParts = splitName(expandedName);

            const addr = normalizeAddress(rawAddress);

            // Expand profession/occupation
            const rawOccupation = record['Occupation'] || '';
            const occupation = expandProfessionAbbreviations(rawOccupation);

            sqlStatements.push(`INSERT INTO sheffield_businesses (name, first_name, middle_name, surname, street_address, house_number, sub_area, street_name, profession, occupation_expanded, business_type) VALUES (${sqlEscape(expandedName)}, ${sqlEscape(nameParts.first)}, ${sqlEscape(nameParts.middle)}, ${sqlEscape(nameParts.surname)}, ${sqlEscape(addr.full)}, ${sqlEscape(addr.number)}, ${sqlEscape(addr.subArea)}, ${sqlEscape(addr.street)}, ${sqlEscape(rawOccupation)}, ${sqlEscape(occupation)}, 'business directory');`);

            fileRecords++;
            totalRecords++;
        } catch (err) {
            errorCount++;
            console.error(`  Error parsing line ${i} in ${file}: ${err.message}`);
        }
    }

    if (fileRecords > 0) {
        console.log(`✓ ${file}: ${fileRecords} Rotherham records`);
    }
}

console.log(`\n========================================`);
console.log(`ROTHERHAM IMPORT SUMMARY`);
console.log(`========================================`);
console.log(`Files processed: ${files.length}`);
console.log(`Total Rotherham records: ${totalRecords}`);
console.log(`Errors: ${errorCount}`);
console.log(`========================================\n`);

// Write SQL file with transaction wrapper
const sqlContent = `BEGIN TRANSACTION;\n${sqlStatements.join('\n')}\nCOMMIT;`;
fs.writeFileSync(outputSQL, sqlContent);
console.log(`SQL written to: ${outputSQL}\n`);
console.log(`To execute: sqlite3 Sheffield1867.db < ${outputSQL}\n`);
