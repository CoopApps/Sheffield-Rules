const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

console.log('\n========================================');
console.log('IMPORTING ALL DATA TO SHEFFIELD1867.DB');
console.log('========================================\n');

// ============================================
// HELPER FUNCTIONS
// ============================================

// Name abbreviation expansion
const NAME_ABBREVIATIONS = {
    'Geo.': 'George', 'Geo': 'George',
    'Wm': 'William', 'Wm.': 'William',
    'Thos': 'Thomas', 'Thos.': 'Thomas',
    'Jas': 'James', 'Jas.': 'James',
    'Chas': 'Charles', 'Chas.': 'Charles',
    'Jno': 'John', 'Jno.': 'John',
    'Robt': 'Robert', 'Robt.': 'Robert',
    'Richd': 'Richard', 'Richd.': 'Richard',
    'Edwd': 'Edward', 'Edwd.': 'Edward',
    'Benj': 'Benjamin', 'Benj.': 'Benjamin',
    'Sam': 'Samuel', 'Sam.': 'Samuel',
    'Jos': 'Joseph', 'Jos.': 'Joseph',
    'Danl': 'Daniel', 'Danl.': 'Daniel',
    'Michl': 'Michael', 'Michl.': 'Michael',
    'Saml': 'Samuel', 'Saml.': 'Samuel',
    'Fredk': 'Frederick', 'Fredk.': 'Frederick',
    'Hy': 'Henry', 'Hy.': 'Henry',
    'Elizth': 'Elizabeth', 'Elizth.': 'Elizabeth',
    'Margt': 'Margaret', 'Margt.': 'Margaret',
    'Cath': 'Catherine', 'Cath.': 'Catherine',
};

// Profession abbreviation expansion
const PROFESSION_ABBREVIATIONS = {
    'agri lab': 'agricultural labourer',
    'agric lab': 'agricultural labourer',
    'lab': 'labourer',
    'servt': 'servant',
    'dom servt': 'domestic servant',
    'gen servt': 'general servant',
    'app': 'apprentice',
};

function expandNameAbbreviations(name) {
    if (!name) return name;
    let expanded = name;
    for (const [abbrev, full] of Object.entries(NAME_ABBREVIATIONS)) {
        const regex = new RegExp(`\\b${abbrev}\\b`, 'gi');
        expanded = expanded.replace(regex, full);
    }
    return expanded;
}

function expandProfessionAbbreviations(profession) {
    if (!profession) return profession;
    let expanded = profession.toLowerCase();
    for (const [abbrev, full] of Object.entries(PROFESSION_ABBREVIATIONS)) {
        const regex = new RegExp(`\\b${abbrev}\\b`, 'gi');
        expanded = expanded.replace(regex, full);
    }
    return capitalizeWords(expanded);
}

// Capitalize words: "cemetary road, barnsley" → "Cemetary Road, Barnsley"
function capitalizeWords(str) {
    if (!str) return str;
    return str.split(' ').map(word => {
        if (word.length === 0) return word;
        return word.charAt(0).toUpperCase() + word.slice(1).toLowerCase();
    }).join(' ');
}

// Normalize address: "19,SomersetRoad" → "19 Somerset Road"
// "shortstreet,yard,65" → "65 Yard Short Street"
function normalizeAddress(address) {
    if (!address) return { full: null, number: null, subArea: null, street: null };

    // Remove ~ and extra spaces
    let cleaned = address.replace(/~/g, '').trim();

    // Split by comma
    let parts = cleaned.split(',').map(p => p.trim()).filter(p => p);

    // Try to find house number
    let houseNumber = null;
    let remaining = [];

    for (const part of parts) {
        // Check if part starts with or is a number
        const numMatch = part.match(/^(\d+[a-z]?)\b/i);
        if (numMatch && !houseNumber) {
            houseNumber = numMatch[1];
            const rest = part.slice(numMatch[0].length).trim();
            if (rest) remaining.push(rest);
        } else {
            remaining.push(part);
        }
    }

    // Add spacing to concatenated words: "SomersetRoad" → "Somerset Road"
    remaining = remaining.map(part => {
        return part.replace(/([a-z])([A-Z])/g, '$1 $2');
    });

    // Capitalize each part
    remaining = remaining.map(capitalizeWords);

    let street = null;
    let subArea = null;

    if (remaining.length === 1) {
        street = remaining[0];
    } else if (remaining.length === 2) {
        subArea = remaining[0];
        street = remaining[1];
    } else if (remaining.length > 2) {
        // Last part is street, middle parts are sub-areas
        street = remaining[remaining.length - 1];
        subArea = remaining.slice(0, -1).join(' ');
    }

    // Build final address
    const fullParts = [];
    if (houseNumber) fullParts.push(houseNumber);
    if (subArea) fullParts.push(subArea);
    if (street) fullParts.push(street);

    return {
        full: fullParts.join(' ') || null,
        number: houseNumber,
        subArea: subArea,
        street: street
    };
}

// Split name into first, middle, surname
function splitName(fullName) {
    if (!fullName) return { first: null, middle: null, surname: null };

    const parts = fullName.trim().split(/\s+/);

    if (parts.length === 1) {
        return { first: parts[0], middle: null, surname: null };
    } else if (parts.length === 2) {
        return { first: parts[0], middle: null, surname: parts[1] };
    } else {
        return {
            first: parts[0],
            middle: parts.slice(1, -1).join(' '),
            surname: parts[parts.length - 1]
        };
    }
}

// Escape SQL strings
function sqlEscape(str) {
    if (str === null || str === undefined) return 'NULL';
    return `'${String(str).replace(/'/g, "''")}'`;
}

// ============================================
// CENSUS IMPORT
// ============================================

function importCensusData() {
    console.log('\n=== IMPORTING CENSUS DATA ===\n');

    const censusDir = './sheffield census';
    const files = fs.readdirSync(censusDir).filter(f => f.endsWith('.csv'));

    console.log(`Found ${files.length} census files\n`);

    const sqlStatements = [];
    let totalRecords = 0;

    for (const file of files) {
        const filePath = path.join(censusDir, file);
        const content = fs.readFileSync(filePath, 'utf8');
        const lines = content.split('\n');

        if (lines.length < 2) continue;

        const headers = lines[0].split('","').map(h => h.replace(/"/g, ''));

        for (let i = 1; i < lines.length; i++) {
            const line = lines[i].trim();
            if (!line) continue;

            // Parse CSV line
            const values = line.split('","').map(v => v.replace(/^"|"$/g, ''));
            const record = {};
            headers.forEach((h, idx) => {
                record[h] = values[idx] || null;
            });

            // Expand and split name
            const expandedName = expandNameAbbreviations(record['NAME'] || record['Name']);
            const nameParts = splitName(expandedName);

            // Normalize address (from WHERE BORN or other fields)
            const addr = normalizeAddress(record['WHERE BORN']);

            sqlStatements.push(`
                INSERT INTO unmatched_ancestry (
                    name, first_name, middle_name, surname,
                    census_age, census_relation, census_gender,
                    census_ed, census_household_schedule, census_piece, census_folio, census_page,
                    civil_parish, ecclesiastical_parish, registration_district, sub_registration_district,
                    street_address, house_number, sub_area, street_name,
                    birth_year, birth_town, birth_county, birth_country, where_born,
                    profession
                ) VALUES (
                    ${sqlEscape(expandedName)}, ${sqlEscape(nameParts.first)}, ${sqlEscape(nameParts.middle)}, ${sqlEscape(nameParts.surname)},
                    ${sqlEscape(record['AGE'])}, ${sqlEscape(record['RELATION'])}, ${sqlEscape(record['GENDER'])},
                    ${sqlEscape(record['ED, INSTITUTION, OR VESSEL'])}, ${sqlEscape(record['HOUSEHOLD SCHEDULE NUMBER'])},
                    ${sqlEscape(record['PIECE'])}, ${sqlEscape(record['FOLIO'])}, ${sqlEscape(record['PAGE NUMBER'])},
                    ${sqlEscape(record['CIVIL PARISH'])}, NULL, ${sqlEscape(record['REGISTRATION DISTRICT'])}, ${sqlEscape(record['SUB-REGISTRATION DISTRICT'])},
                    ${sqlEscape(addr.full)}, ${sqlEscape(addr.number)}, ${sqlEscape(addr.subArea)}, ${sqlEscape(addr.street)},
                    ${sqlEscape(record['ESTIMATED BIRTH YEAR'])}, ${sqlEscape(record['TOWN'])}, ${sqlEscape(record['COUNTY/ISLAND'])}, ${sqlEscape(record['COUNTRY'])}, ${sqlEscape(record['WHERE BORN'])},
                    NULL
                );
            `);

            totalRecords++;
        }

        console.log(`✓ Processed ${file}`);
    }

    console.log(`\nTotal census records to import: ${totalRecords}\n`);

    return sqlStatements;
}

// Save this for now - script is getting long
// I'll continue in the next file

console.log('Script loaded. Ready to import data.');
console.log('This is part 1 - building helper functions');
