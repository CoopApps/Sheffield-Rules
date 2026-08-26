const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('Normalizing genealogy addresses...\n');

// Get all genealogy records with addresses
const records = db.prepare(`
    SELECT rowid, address
    FROM unmatched_genealogy
    WHERE address IS NOT NULL AND address != ''
`).all();

console.log(`Found ${records.length.toLocaleString()} records with addresses\n`);

function normalizeAddress(address) {
    if (!address) return address;

    let normalized = address;

    // Replace comma with space (e.g., "28,ClarkeStreet" -> "28 ClarkeStreet")
    normalized = normalized.replace(/,/g, ' ');

    // Handle "back" prefix specifically - must come before general camelCase splitting
    // "backhouse,356" -> "back house 356"
    normalized = normalized.replace(/\bback([a-z])/gi, 'Back $1');

    // Handle other common directional/descriptive prefixes
    normalized = normalized.replace(/\bnew([A-Z])/g, 'New $1');
    normalized = normalized.replace(/\bold([A-Z])/g, 'Old $1');
    normalized = normalized.replace(/\bwest([A-Z])/g, 'West $1');
    normalized = normalized.replace(/\beast([A-Z])/g, 'East $1');
    normalized = normalized.replace(/\bnorth([A-Z])/g, 'North $1');
    normalized = normalized.replace(/\bsouth([A-Z])/g, 'South $1');

    // Add space before capital letters that follow lowercase letters
    // This handles cases like "ClarkeStreet" -> "Clarke Street"
    normalized = normalized.replace(/([a-z])([A-Z])/g, '$1 $2');

    // Collapse multiple spaces into one
    normalized = normalized.replace(/\s+/g, ' ');

    // Trim whitespace
    normalized = normalized.trim();

    return normalized;
}

// Update addresses
const updateStmt = db.prepare('UPDATE unmatched_genealogy SET address = ? WHERE rowid = ?');

let updated = 0;
let unchanged = 0;

console.log('Sample transformations:');
let samples = 0;

for (const record of records) {
    const normalized = normalizeAddress(record.address);

    if (normalized !== record.address) {
        updateStmt.run(normalized, record.rowid);
        updated++;

        // Show first 10 examples
        if (samples < 10) {
            console.log(`  "${record.address}" -> "${normalized}"`);
            samples++;
        }

        if (updated % 10000 === 0) {
            console.log(`\nUpdated ${updated.toLocaleString()} addresses...`);
        }
    } else {
        unchanged++;
    }
}

console.log(`\n========================================`);
console.log(`COMPLETE`);
console.log(`========================================`);
console.log(`Updated: ${updated.toLocaleString()}`);
console.log(`Unchanged: ${unchanged.toLocaleString()}`);
console.log(`Total: ${records.length.toLocaleString()}`);
console.log(`========================================\n`);

db.close();
