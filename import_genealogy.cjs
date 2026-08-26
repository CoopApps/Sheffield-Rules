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
console.log('IMPORTING GENEALOGY DATA');
console.log('========================================\n');

const genealogyDir = './Genealogy';
const outputSQL = './import_genealogy.sql';

if (!fs.existsSync(genealogyDir)) {
    console.error(`ERROR: Genealogy directory not found: ${genealogyDir}`);
    process.exit(1);
}

const files = fs.readdirSync(genealogyDir).filter(f => f.startsWith('tg_') && f.endsWith('.csv'));
console.log(`Found ${files.length} genealogy files\n`);

const sqlStatements = [];
let totalRecords = 0;
let errorCount = 0;

for (const file of files) {
    const filePath = path.join(genealogyDir, file);
    const content = fs.readFileSync(filePath, 'utf8');
    const lines = content.split('\n');

    if (lines.length < 2) {
        console.log(`⚠ Skipping ${file} - no data rows`);
        continue;
    }

    // Parse headers
    const headers = lines[0].split(',').map(h => h.replace(/^"|"$/g, ''));
    let fileRecords = 0;

    for (let i = 1; i < lines.length; i++) {
        const line = lines[i].trim();
        if (!line) continue;

        try {
            // Parse CSV line (simple split, values are quoted)
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

            // Expand and split name
            const rawName = record['Name'] || '';
            const expandedName = expandNameAbbreviations(rawName);
            const nameParts = splitName(expandedName);

            // Normalize address
            const rawAddress = record['Address'] || '';
            const addr = normalizeAddress(rawAddress);

            // Expand profession
            const rawProfession = record['Profession'] || '';
            const profession = expandProfessionAbbreviations(rawProfession);

            // Capitalize location fields
            const parish = capitalizeWords(record['Parish']);
            const area = capitalizeWords(record['Area']);
            const birthPlace = capitalizeWords(record['Birth Place']);

            // Parse birth place into components
            const birthParts = birthPlace ? birthPlace.split(',').map(p => p.trim()) : [];
            const birthTown = birthParts[0] || null;
            const birthCounty = birthParts[1] || null;

            sqlStatements.push(`INSERT INTO unmatched_genealogy (name, first_name, middle_name, surname, census_age, census_relation, civil_parish, street_address, house_number, sub_area, street_name, birth_year, birth_town, birth_county, where_born, profession, occupation_expanded, genealogy_source) VALUES (${sqlEscape(expandedName)}, ${sqlEscape(nameParts.first)}, ${sqlEscape(nameParts.middle)}, ${sqlEscape(nameParts.surname)}, ${sqlEscape(record['Age'])}, ${sqlEscape(record['Relation'])}, ${sqlEscape(parish)}, ${sqlEscape(addr.full)}, ${sqlEscape(addr.number)}, ${sqlEscape(addr.subArea)}, ${sqlEscape(addr.street)}, ${sqlEscape(record['Born Approx'])}, ${sqlEscape(birthTown)}, ${sqlEscape(birthCounty)}, ${sqlEscape(birthPlace)}, ${sqlEscape(rawProfession)}, ${sqlEscape(profession)}, 'genealogy');`);

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
console.log(`GENEALOGY IMPORT SUMMARY`);
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
