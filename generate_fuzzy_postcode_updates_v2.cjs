const XLSX = require('xlsx');
const fs = require('fs');
const { execSync } = require('child_process');

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

console.log(`Parsed ${streetPostcodeMap.size} unique street-postcode mappings`);

// Helper function to extract street name from address
function extractStreetName(address) {
    if (!address) return null;

    let cleaned = address.trim();

    // Remove leading house numbers (e.g., "19 Somerset Road" -> "Somerset Road")
    cleaned = cleaned.replace(/^\d+\s+/, '');

    // Remove "Court" prefix (e.g., "Court Trinity Street" -> "Trinity Street")
    cleaned = cleaned.replace(/^Court\s+/i, '');

    // Remove yard references (e.g., "Wilsons Yard Duke Street" -> "Duke Street")
    cleaned = cleaned.replace(/^.+?\s+Yard\s+/i, '');

    // Remove trailing slashes and extra info
    cleaned = cleaned.replace(/\/.*$/, '');
    cleaned = cleaned.replace(/\s+(Sheffield|Halifax|Redcar|Sculcoates)$/i, '');

    // Handle workhouse addresses - try to extract street if present
    if (cleaned.match(/Union Workhouse/i)) {
        const streetMatch = cleaned.match(/Workhouse\s+(.+?)(?:\s+Sheffield)?$/i);
        if (streetMatch) {
            cleaned = streetMatch[1];
        }
    }

    return cleaned.trim();
}

// Query database using sqlite3 command
function getUniqueAddresses(table) {
    const path = require('path');
    const sqlite3Path = path.join(__dirname, 'sqlite3.exe');
    const dbPath = path.join(__dirname, 'Sheffield1867.db');
    const sql = `SELECT DISTINCT street_address FROM ${table} WHERE (postcode IS NULL OR postcode = '') AND street_address IS NOT NULL AND street_address <> '';`;
    const result = execSync(`"${sqlite3Path}" "${dbPath}" "${sql}"`, { encoding: 'utf8' });
    return result.trim().split('\n').filter(a => a);
}

console.log('\nFinding addresses to update...');

const tables = ['unmatched_genealogy', 'sheffield_businesses', 'unmatched_ancestry'];
const updates = new Map();

for (const table of tables) {
    updates.set(table, new Map());

    console.log(`\nQuerying ${table}...`);
    const addresses = getUniqueAddresses(table);
    console.log(`  ${addresses.length} unique addresses without postcodes`);

    let matched = 0;
    for (const address of addresses) {
        const streetName = extractStreetName(address);

        if (streetName) {
            const postcode = streetPostcodeMap.get(streetName.toLowerCase());
            if (postcode) {
                updates.get(table).set(address, postcode);
                matched++;
            }
        }
    }

    console.log(`  Matched: ${matched} addresses to postcodes`);
}

// Generate SQL
let sql = '-- Fuzzy postcode matching (extract street names from full addresses)\n';
sql += 'BEGIN TRANSACTION;\n\n';

let totalUpdates = 0;
for (const [table, addressMap] of updates) {
    sql += `-- Updates for ${table}\n`;
    for (const [address, postcode] of addressMap) {
        const escapedAddress = address.replace(/'/g, "''");
        const escapedPostcode = postcode.replace(/'/g, "''");
        sql += `UPDATE ${table} SET postcode = '${escapedPostcode}' WHERE street_address = '${escapedAddress}' AND (postcode IS NULL OR postcode = '');\n`;
        totalUpdates++;
    }
    sql += '\n';
}

sql += 'COMMIT;\n\n';
sql += '-- Show results\n';
sql += 'SELECT \'unmatched_genealogy postcodes:\', COUNT(*) FROM unmatched_genealogy WHERE postcode IS NOT NULL AND postcode <> \'\';\n';
sql += 'SELECT \'sheffield_businesses postcodes:\', COUNT(*) FROM sheffield_businesses WHERE postcode IS NOT NULL AND postcode <> \'\';\n';
sql += 'SELECT \'unmatched_ancestry postcodes:\', COUNT(*) FROM unmatched_ancestry WHERE postcode IS NOT NULL AND postcode <> \'\';\n';

fs.writeFileSync('./update_postcodes_fuzzy.sql', sql);
console.log(`\n=== Generated update_postcodes_fuzzy.sql ===`);
console.log(`Total updates: ${totalUpdates}`);
