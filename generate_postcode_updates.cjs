const XLSX = require('xlsx');
const fs = require('fs');

const wb = XLSX.readFile('./postcodes.xlsx');
const ws = wb.Sheets[wb.SheetNames[0]];
const range = XLSX.utils.decode_range(ws['!ref']);

console.log(`Reading ${range.e.r + 1} street entries from postcodes.xlsx...`);

const streetMap = new Map();

// Parse each row to extract street name and postcode
for (let R = 0; R <= range.e.r; ++R) {
    const cell = ws[XLSX.utils.encode_cell({r: R, c: 0})];
    if (!cell || !cell.v) continue;

    const value = cell.v.trim();
    // Pattern: "Street Name     POSTCODE"
    // Postcodes are format like S8 7UQ (letter+number space number+letters)
    const match = value.match(/^(.+?)\s{2,}([A-Z]\d+\s\d[A-Z]{2})$/);

    if (match) {
        const street = match[1].trim();
        const postcode = match[2].trim();
        streetMap.set(street, postcode);
    }
}

console.log(`Parsed ${streetMap.size} unique street-postcode mappings`);

// Generate SQL
let sql = '-- Update postcodes from postcodes.xlsx exact street matches\n';
sql += 'BEGIN TRANSACTION;\n\n';

for (const [street, postcode] of streetMap) {
    const escapedStreet = street.replace(/'/g, "''");
    const escapedPostcode = postcode.replace(/'/g, "''");

    sql += `-- ${street} -> ${postcode}\n`;
    sql += `UPDATE unmatched_genealogy SET postcode = '${escapedPostcode}' WHERE street_address = '${escapedStreet}' AND (postcode IS NULL OR postcode = '');\n`;
    sql += `UPDATE sheffield_businesses SET postcode = '${escapedPostcode}' WHERE street_address = '${escapedStreet}' AND (postcode IS NULL OR postcode = '');\n`;
    sql += `UPDATE unmatched_ancestry SET postcode = '${escapedPostcode}' WHERE street_address = '${escapedStreet}' AND (postcode IS NULL OR postcode = '');\n\n`;
}

sql += 'COMMIT;\n\n';
sql += '-- Show results\n';
sql += 'SELECT \'unmatched_genealogy postcodes:\', COUNT(*) FROM unmatched_genealogy WHERE postcode IS NOT NULL AND postcode <> \'\';\n';
sql += 'SELECT \'sheffield_businesses postcodes:\', COUNT(*) FROM sheffield_businesses WHERE postcode IS NOT NULL AND postcode <> \'\';\n';
sql += 'SELECT \'unmatched_ancestry postcodes:\', COUNT(*) FROM unmatched_ancestry WHERE postcode IS NOT NULL AND postcode <> \'\';\n';

fs.writeFileSync('./update_postcodes_from_streets.sql', sql);
console.log('\nGenerated update_postcodes_from_streets.sql');
console.log(`SQL file contains ${streetMap.size} street updates`);
