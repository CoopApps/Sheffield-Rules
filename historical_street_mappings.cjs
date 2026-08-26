const XLSX = require('xlsx');
const fs = require('fs');
const { execSync } = require('child_process');
const path = require('path');

// Historical street name mappings from the Sheffield street names document
const historicalMappings = {
    // Old name -> Modern name
    'Pudding Lane': 'King Street',
    'Bull Stake': 'Haymarket',
    'Townfield Street': 'Trinity Street',
    'Prior Row': 'High Street',
    'Prior Gate': 'High Street',
    'Under-the-water': 'Bridge Street',
    'Workhouse Lane': 'Paradise Street',
    'Coalpit Lane': 'Cambridge Street',
    'Pinson Lane': 'Pinstone Street',
    'Pepper Alley': 'Norfolk Row',
    'South Street': 'The Moor',
    'Sheffield Moor': 'The Moor',  // Even older name for The Moor
    'Balm Green': 'Barker\'s Pool',
    'Le Balne': 'Barker\'s Pool',
    'Pincher Croft Lane': 'Pinstone Street',
    'Pea Croft': 'Solly Street',
    'Norfolk Lane': 'Norfolk Street',
    'Townhead Street': 'Church Street',
    'Bow Street': 'West Street',
    'Orchard Street': 'Leopold Street',  // Partial - Leopold is part of Orchard
    'Truelove\'s Gutter': 'Castle Street',
    'Charlotte Street': 'Mappin Street',
    'St. George\'s Square': 'Mappin Street',  // Even older name
    'Pot Square': 'Paradise Square',
    'Handsworth Hill': 'Main Road',  // Darnall
    'Bole Hill Lane': 'Cobnar Road',  // Woodseats - note: different from Bole Hill Lane in Crookes
    'Cutlers Gate': 'Derek Dooley Way',  // Renamed 2008
};

// Read postcodes from xlsx
const wb = XLSX.readFile('./postcodes.xlsx');
const ws = wb.Sheets[wb.SheetNames[0]];
const range = XLSX.utils.decode_range(ws['!ref']);

console.log(`Reading ${range.e.r + 1} street entries from postcodes.xlsx...`);

const streetPostcodeMap = new Map();

// Parse each row to extract street name and postcode
for (let R = 0; R <= range.e.r; ++R) {
    const cell = ws[XLSX.utils.encode_cell({r: R, c: 0})];
    if (!cell || !cell.v) continue;

    const value = cell.v.trim();
    const match = value.match(/^(.+?)\s{2,}([A-Z]\d+\s\d[A-Z]{2})$/);

    if (match) {
        const street = match[1].trim();
        const postcode = match[2].trim();
        streetPostcodeMap.set(street.toLowerCase(), postcode);
    }
}

console.log(`Parsed ${streetPostcodeMap.size} unique street-postcode mappings from xlsx\n`);

// Check which historical names have modern equivalents in postcodes.xlsx
console.log('=== Historical Street Name Matches ===\n');

const foundMappings = [];

for (const [oldName, modernName] of Object.entries(historicalMappings)) {
    const postcode = streetPostcodeMap.get(modernName.toLowerCase());
    if (postcode) {
        console.log(`✓ ${oldName} → ${modernName} : ${postcode}`);
        foundMappings.push({ oldName, modernName, postcode });
    } else {
        console.log(`✗ ${oldName} → ${modernName} : NOT IN POSTCODES.XLSX`);
    }
}

console.log(`\n${foundMappings.length} historical names have modern equivalents in postcodes.xlsx\n`);

// Now check database for records with old street names
console.log('=== Checking Database for Historical Street Names ===\n');

const sqlite3Path = path.join(__dirname, 'sqlite3.exe');
const dbPath = path.join(__dirname, 'Sheffield1867.db');

const updates = [];

for (const mapping of foundMappings) {
    // Check if any records have the old street name
    const sql = `SELECT COUNT(*) FROM unmatched_genealogy WHERE street_address LIKE '%${mapping.oldName.replace(/'/g, "''")}%' AND (postcode IS NULL OR postcode = '');`;

    try {
        const result = execSync(`"${sqlite3Path}" "${dbPath}" "${sql}"`, { encoding: 'utf8' }).trim();
        const count = parseInt(result);

        if (count > 0) {
            console.log(`Found ${count} records with "${mapping.oldName}"`);
            updates.push(mapping);
        }
    } catch (err) {
        console.error(`Error checking ${mapping.oldName}:`, err.message);
    }
}

console.log(`\n${updates.length} historical street names found in database that can be updated\n`);

// Generate SQL for updates
if (updates.length > 0) {
    let sql = '-- Update postcodes based on historical street name mappings\n';
    sql += 'BEGIN TRANSACTION;\n\n';

    for (const mapping of updates) {
        const escapedOld = mapping.oldName.replace(/'/g, "''");
        const escapedPostcode = mapping.postcode.replace(/'/g, "''");

        sql += `-- ${mapping.oldName} → ${mapping.modernName} : ${mapping.postcode}\n`;
        sql += `UPDATE unmatched_genealogy SET postcode = '${escapedPostcode}' WHERE street_address LIKE '%${escapedOld}%' AND (postcode IS NULL OR postcode = '');\n`;
        sql += `UPDATE sheffield_businesses SET postcode = '${escapedPostcode}' WHERE street_address LIKE '%${escapedOld}%' AND (postcode IS NULL OR postcode = '');\n`;
        sql += `UPDATE unmatched_ancestry SET postcode = '${escapedPostcode}' WHERE street_address LIKE '%${escapedOld}%' AND (postcode IS NULL OR postcode = '');\n\n`;
    }

    sql += 'COMMIT;\n\n';
    sql += '-- Show results\n';
    sql += 'SELECT \'unmatched_genealogy postcodes:\', COUNT(*) FROM unmatched_genealogy WHERE postcode IS NOT NULL AND postcode <> \'\';\n';
    sql += 'SELECT \'sheffield_businesses postcodes:\', COUNT(*) FROM sheffield_businesses WHERE postcode IS NOT NULL AND postcode <> \'\';\n';
    sql += 'SELECT \'unmatched_ancestry postcodes:\', COUNT(*) FROM unmatched_ancestry WHERE postcode IS NOT NULL AND postcode <> \'\';\n';

    fs.writeFileSync('./update_postcodes_historical.sql', sql);
    console.log('Generated update_postcodes_historical.sql');
}
