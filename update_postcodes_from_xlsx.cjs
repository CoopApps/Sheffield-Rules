const Database = require('better-sqlite3');
const XLSX = require('xlsx');

const db = new Database('./Sheffield1867.db');
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

// Show sample
console.log('\nSample mappings:');
let count = 0;
for (const [street, postcode] of streetMap) {
    console.log(`  "${street}" -> "${postcode}"`);
    if (++count >= 10) break;
}

// Begin transaction
db.exec('BEGIN TRANSACTION');

let genealogyUpdated = 0;
let businessesUpdated = 0;
let ancestryUpdated = 0;

// Update each table
for (const [street, postcode] of streetMap) {
    const escapedStreet = street.replace(/'/g, "''");

    // Update unmatched_genealogy
    const result1 = db.prepare(`
        UPDATE unmatched_genealogy
        SET postcode = ?
        WHERE street_address = ?
        AND (postcode IS NULL OR postcode = '')
    `).run(postcode, street);
    genealogyUpdated += result1.changes;

    // Update sheffield_businesses
    const result2 = db.prepare(`
        UPDATE sheffield_businesses
        SET postcode = ?
        WHERE street_address = ?
        AND (postcode IS NULL OR postcode = '')
    `).run(postcode, street);
    businessesUpdated += result2.changes;

    // Update unmatched_ancestry
    const result3 = db.prepare(`
        UPDATE unmatched_ancestry
        SET postcode = ?
        WHERE street_address = ?
        AND (postcode IS NULL OR postcode = '')
    `).run(postcode, street);
    ancestryUpdated += result3.changes;
}

db.exec('COMMIT');

console.log('\n=== Update Complete ===');
console.log(`unmatched_genealogy: ${genealogyUpdated} records updated`);
console.log(`sheffield_businesses: ${businessesUpdated} records updated`);
console.log(`unmatched_ancestry: ${ancestryUpdated} records updated`);
console.log(`Total: ${genealogyUpdated + businessesUpdated + ancestryUpdated} records updated`);

db.close();
