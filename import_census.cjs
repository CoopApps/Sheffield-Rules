const fs = require('fs');
const path = require('path');
const {
    expandNameAbbreviations,
    capitalizeWords,
    normalizeAddress,
    splitName,
    sqlEscape
} = require('./import_helpers.cjs');

console.log('\n========================================');
console.log('IMPORTING CENSUS DATA');
console.log('========================================\n');

const censusDir = './sheffield census';
const outputSQL = './import_census.sql';

if (!fs.existsSync(censusDir)) {
    console.error(`ERROR: Census directory not found: ${censusDir}`);
    process.exit(1);
}

const files = fs.readdirSync(censusDir).filter(f => f.endsWith('.csv'));
console.log(`Found ${files.length} census files\n`);

const sqlStatements = [];
let totalRecords = 0;
let errorCount = 0;

for (const file of files) {
    const filePath = path.join(censusDir, file);
    const content = fs.readFileSync(filePath, 'utf8');
    const lines = content.split('\n');

    if (lines.length < 2) {
        console.log(`⚠ Skipping ${file} - no data rows`);
        continue;
    }

    // Parse headers
    const headers = lines[0].split('","').map(h => h.replace(/^"|"$/g, ''));
    let fileRecords = 0;

    for (let i = 1; i < lines.length; i++) {
        const line = lines[i].trim();
        if (!line) continue;

        try {
            // Parse CSV line
            const values = line.split('","').map(v => v.replace(/^"|"$/g, ''));
            const record = {};
            headers.forEach((h, idx) => {
                record[h] = values[idx] || null;
            });

            // Expand and split name
            const rawName = record['NAME'] || record['Name'] || '';
            const expandedName = expandNameAbbreviations(rawName);
            const nameParts = splitName(expandedName);

            // Normalize address from WHERE BORN field
            const whereBorn = record['WHERE BORN'] || '';
            const addr = normalizeAddress(whereBorn);

            // Capitalize location fields
            const birthTown = capitalizeWords(record['TOWN']);
            const birthCounty = capitalizeWords(record['COUNTY/ISLAND']);
            const birthCountry = capitalizeWords(record['COUNTRY']);
            const civilParish = capitalizeWords(record['CIVIL PARISH']);
            const ecclesiasticalParish = capitalizeWords(record['ECCLESIASTICAL PARISH']);
            const registrationDistrict = capitalizeWords(record['REGISTRATION DISTRICT']);
            const subRegistrationDistrict = capitalizeWords(record['SUB-REGISTRATION DISTRICT']);

            sqlStatements.push(`INSERT INTO unmatched_ancestry (name, first_name, middle_name, surname, census_age, census_relation, census_gender, census_ed, census_household_schedule, census_piece, census_folio, census_page, civil_parish, ecclesiastical_parish, registration_district, sub_registration_district, street_address, house_number, sub_area, street_name, birth_year, birth_town, birth_county, birth_country, where_born) VALUES (${sqlEscape(expandedName)}, ${sqlEscape(nameParts.first)}, ${sqlEscape(nameParts.middle)}, ${sqlEscape(nameParts.surname)}, ${sqlEscape(record['AGE'])}, ${sqlEscape(record['RELATION'])}, ${sqlEscape(record['GENDER'])}, ${sqlEscape(record['ED, INSTITUTION, OR VESSEL'])}, ${sqlEscape(record['HOUSEHOLD SCHEDULE NUMBER'])}, ${sqlEscape(record['PIECE'])}, ${sqlEscape(record['FOLIO'])}, ${sqlEscape(record['PAGE NUMBER'])}, ${sqlEscape(civilParish)}, ${sqlEscape(ecclesiasticalParish)}, ${sqlEscape(registrationDistrict)}, ${sqlEscape(subRegistrationDistrict)}, ${sqlEscape(addr.full)}, ${sqlEscape(addr.number)}, ${sqlEscape(addr.subArea)}, ${sqlEscape(addr.street)}, ${sqlEscape(record['ESTIMATED BIRTH YEAR'])}, ${sqlEscape(birthTown)}, ${sqlEscape(birthCounty)}, ${sqlEscape(birthCountry)}, ${sqlEscape(whereBorn)});`);

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
console.log(`CENSUS IMPORT SUMMARY`);
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
