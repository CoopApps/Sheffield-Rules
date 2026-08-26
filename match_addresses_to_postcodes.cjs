const sqlite3 = require('better-sqlite3');
const XLSX = require('xlsx');

const db = new sqlite3('Sheffield1867.db', { timeout: 30000 });

console.log('='.repeat(60));
console.log('Matching historical addresses to modern postcodes');
console.log('='.repeat(60) + '\n');

// Read Excel file
console.log('Reading postcodes.xlsx...');
const workbook = XLSX.readFile('postcodes.xlsx');
const sheet = workbook.Sheets[workbook.SheetNames[0]];
const range = XLSX.utils.decode_range(sheet['!ref']);

// Parse postcode data: "Street Name     POSTCODE"
const postcodeMap = new Map();

console.log('Parsing postcode data...');
for (let row = range.s.r; row <= range.e.r; row++) {
    const cellAddress = XLSX.utils.encode_cell({ r: row, c: 0 });
    const cell = sheet[cellAddress];

    if (cell && cell.v) {
        const text = cell.v.toString().trim();

        // Extract postcode (last part matching S[0-9]+ pattern)
        const postcodeMatch = text.match(/\b(S\d+\s+\d[A-Z]{2})\s*$/);

        if (postcodeMatch) {
            const postcode = postcodeMatch[1];
            // Street name is everything before the postcode
            const streetName = text.substring(0, postcodeMatch.index).trim();

            // Normalize street name for matching
            const normalizedStreet = streetName.toLowerCase()
                .replace(/\s+/g, ' ')
                .replace(/street/gi, 'st')
                .replace(/road/gi, 'rd')
                .replace(/lane/gi, 'ln');

            postcodeMap.set(normalizedStreet, postcode);

            // Also store original
            postcodeMap.set(streetName.toLowerCase(), postcode);
        }
    }
}

console.log(`Loaded ${postcodeMap.size} street-postcode mappings\n`);

// Normalize address for matching
function normalizeAddress(address) {
    if (!address) return '';

    return address.toLowerCase()
        .replace(/,/g, ' ')
        .replace(/\s+/g, ' ')
        .replace(/street/gi, 'st')
        .replace(/road/gi, 'rd')
        .replace(/lane/gi, 'ln')
        .trim();
}

// Extract main street name from historical address
function extractStreetName(address) {
    if (!address) return null;

    // Remove house numbers at start: "123,StreetName" -> "StreetName"
    let cleaned = address
        .replace(/^\d+[,\s]+/, '')  // Remove leading numbers
        .replace(/^Court\d+[,\s]+/i, '')  // Remove "Court1,"
        .replace(/^Back[,\s]+/i, '')  // Remove "Back,"
        .replace(/^Yard[,\s]+/i, '');  // Remove "Yard,"

    // Extract the main street part (before first comma if multiple parts)
    const parts = cleaned.split(',');
    let mainStreet = parts[parts.length - 1].trim();  // Often the street is last part

    // If that didn't work, try the first substantial part
    if (mainStreet.length < 3) {
        mainStreet = parts.find(p => p.trim().length > 3) || parts[0];
    }

    return mainStreet.trim();
}

// Get all people with addresses
const peopleWithAddresses = db.prepare(`
    SELECT id, name, street_address
    FROM sheffield_people
    WHERE street_address IS NOT NULL
`).all();

console.log(`Matching ${peopleWithAddresses.length} addresses to postcodes...\n`);

let matched = 0;
let unmatched = 0;
const updatePostcode = db.prepare('UPDATE sheffield_people SET postcode = ? WHERE id = ?');

for (const person of peopleWithAddresses) {
    const address = person.street_address;
    const streetName = extractStreetName(address);

    if (!streetName) {
        unmatched++;
        continue;
    }

    const normalized = normalizeAddress(streetName);

    // Try exact match
    let postcode = postcodeMap.get(normalized);

    // Try partial matches if no exact match
    if (!postcode) {
        // Check if any postcode street contains this street name
        for (const [street, pc] of postcodeMap.entries()) {
            if (street.includes(normalized) || normalized.includes(street)) {
                postcode = pc;
                break;
            }
        }
    }

    if (postcode) {
        updatePostcode.run(postcode, person.id);
        matched++;

        if (matched <= 10) {
            console.log(`✓ ${person.name}: "${address}" → ${streetName} → ${postcode}`);
        }
    } else {
        unmatched++;
    }
}

console.log('\n' + '='.repeat(60));
console.log(`Matching complete!`);
console.log(`  Matched: ${matched}`);
console.log(`  Unmatched: ${unmatched}`);
console.log(`  Success rate: ${(matched / peopleWithAddresses.length * 100).toFixed(1)}%`);

// Copy postcodes to sheffield_players for people who are players
console.log('\nCopying postcodes to sheffield_players...');
const copied = db.prepare(`
    UPDATE sheffield_players
    SET postcode = (
        SELECT postcode FROM sheffield_people
        WHERE sheffield_people.player_id = sheffield_players.id
        AND sheffield_people.postcode IS NOT NULL
        LIMIT 1
    )
    WHERE id IN (
        SELECT player_id FROM sheffield_people
        WHERE player_id IS NOT NULL
        AND postcode IS NOT NULL
    )
`).run();

console.log(`✓ Copied postcodes to ${copied.changes} players`);

// Final stats
const stats = db.prepare(`
    SELECT
        COUNT(*) as total,
        COUNT(postcode) as with_postcode
    FROM sheffield_people
    WHERE street_address IS NOT NULL
`).get();

const playerStats = db.prepare(`
    SELECT
        COUNT(*) as total,
        COUNT(postcode) as with_postcode
    FROM sheffield_players
`).get();

console.log('\nFinal Statistics:');
console.log(`  sheffield_people:`);
console.log(`    Total with addresses: ${stats.total}`);
console.log(`    With postcodes: ${stats.with_postcode}`);
console.log(`  sheffield_players:`);
console.log(`    With postcodes: ${playerStats.with_postcode}`);

db.close();
console.log('\n✓ Postcode matching complete!');
