const Database = require('better-sqlite3');
const XLSX = require('xlsx');

console.log('\n========================================');
console.log('ADDING POSTCODES TO GENEALOGY RECORDS');
console.log('========================================\n');

// Open database
const db = new Database('Sheffield1867.db');

// Step 1: Clear existing postcodes
console.log('Step 1: Clearing existing postcodes...');
db.exec('UPDATE unmatched_genealogy SET postcode = NULL');
console.log('✓ Postcodes cleared\n');

// Step 2: Load postcodes from Excel
console.log('Step 2: Loading postcodes from postcodes.xlsx...');
const workbook = XLSX.readFile('postcodes.xlsx');
const sheet = workbook.Sheets[workbook.SheetNames[0]];
const postcodeData = XLSX.utils.sheet_to_json(sheet);

console.log(`✓ Loaded ${postcodeData.length.toLocaleString()} postcode records\n`);

// Step 3: Build street name to postcode map, checking for uniqueness
console.log('Step 3: Building street name lookup map (unique streets only)...');
const streetToPostcode = new Map();
const streetCounts = new Map();

// First pass: count how many times each street appears
for (const row of postcodeData) {
    const streetName = row['Street Name'] || row['street name'] || row['STREET NAME'];

    if (streetName) {
        const normalized = streetName.trim().toLowerCase();
        streetCounts.set(normalized, (streetCounts.get(normalized) || 0) + 1);
    }
}

// Second pass: only add streets that appear exactly once
for (const row of postcodeData) {
    const streetName = row['Street Name'] || row['street name'] || row['STREET NAME'];
    const postcode = row['Postcode'] || row['postcode'] || row['POSTCODE'];

    if (streetName && postcode) {
        const normalized = streetName.trim().toLowerCase();

        // Only add if this street is unique (appears exactly once)
        if (streetCounts.get(normalized) === 1) {
            streetToPostcode.set(normalized, postcode);
        }
    }
}

const duplicateStreets = Array.from(streetCounts.entries())
    .filter(([_, count]) => count > 1)
    .length;

console.log(`✓ Total unique streets: ${streetToPostcode.size.toLocaleString()}`);
console.log(`✓ Skipped ${duplicateStreets.toLocaleString()} duplicate street names\n`);

// Step 4: Get genealogy records with addresses
console.log('Step 4: Loading genealogy records...');
const genealogyRecords = db.prepare(`
    SELECT rowid, address
    FROM unmatched_genealogy
    WHERE address IS NOT NULL AND address != ''
`).all();

console.log(`✓ Found ${genealogyRecords.length.toLocaleString()} records with addresses\n`);

// Step 5: Extract street names from addresses and match postcodes (EXACT match only)
console.log('Step 5: Matching EXACT street names to postcodes...\n');

const updateStmt = db.prepare('UPDATE unmatched_genealogy SET postcode = ? WHERE rowid = ?');

let matched = 0;
let notMatched = 0;
let samples = 0;

console.log('Sample matches:');

for (const record of genealogyRecords) {
    const address = record.address;

    // Extract street name - typically the text after the house number
    // Examples: "28 Clarke Street" -> "clarke street"
    //           "356 Back house" -> "back house"

    let streetName = null;

    // Try to extract street after number pattern
    const numberMatch = address.match(/^\d+\s+(.+)$/);
    if (numberMatch) {
        streetName = numberMatch[1].trim().toLowerCase();
    } else {
        // No number at start, use whole address but remove trailing numbers
        streetName = address.replace(/\d+\s*$/, '').trim().toLowerCase();
    }

    // Look up postcode - ONLY if exact match exists
    const postcode = streetToPostcode.get(streetName);

    if (postcode) {
        updateStmt.run(postcode, record.rowid);
        matched++;

        // Show first 10 examples
        if (samples < 10) {
            console.log(`  "${address}" -> street: "${streetName}" -> ${postcode}`);
            samples++;
        }

        if (matched % 10000 === 0) {
            console.log(`\nMatched ${matched.toLocaleString()} postcodes...`);
        }
    } else {
        notMatched++;
    }
}

console.log('\n========================================');
console.log('COMPLETE');
console.log('========================================');
console.log(`Total addresses: ${genealogyRecords.length.toLocaleString()}`);
console.log(`Matched postcodes: ${matched.toLocaleString()}`);
console.log(`Not matched: ${notMatched.toLocaleString()}`);
console.log(`Match rate: ${(matched / genealogyRecords.length * 100).toFixed(1)}%`);
console.log('========================================\n');

db.close();
