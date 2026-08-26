const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('BUSINESSES WITHOUT PERSON NAMES');
console.log('='.repeat(70) + '\n');

// Common first names to check against
const firstNames = [
    'abraham', 'albert', 'alexander', 'alfred', 'andrew', 'arthur', 'benjamin',
    'charles', 'daniel', 'david', 'edward', 'edwin', 'eli', 'elijah', 'ernest',
    'francis', 'frank', 'frederick', 'george', 'henry', 'herbert', 'isaac',
    'james', 'john', 'joseph', 'joshua', 'mark', 'matthew', 'michael',
    'nathaniel', 'patrick', 'peter', 'richard', 'robert', 'samuel', 'stephen',
    'thomas', 'walter', 'william',
    // Abbreviated forms
    'jas', 'jno', 'wm', 'thos', 'geo', 'robt', 'chas', 'jos', 'benj', 'saml',
    'edwd', 'richd', 'fred', 'hy', 'jas', 'hy', 'thos', 'edwd'
];

// Common surnames to check against
const surnames = [
    'smith', 'jones', 'taylor', 'brown', 'wilson', 'johnson', 'white', 'hall',
    'wood', 'walker', 'wright', 'robinson', 'thompson', 'evans', 'green',
    'harris', 'clark', 'lewis', 'lee', 'martin', 'jackson', 'allen', 'hill'
];

function hasPersonName(name) {
    if (!name) return false;

    const lower = name.toLowerCase();

    // Check for first names
    for (const firstName of firstNames) {
        if (lower.includes(firstName)) return true;
    }

    // Check for common surnames
    for (const surname of surnames) {
        // Only match whole words to avoid false positives
        const regex = new RegExp('\\b' + surname + '\\b', 'i');
        if (regex.test(lower)) return true;
    }

    return false;
}

// Get all businesses
const businesses = db.prepare('SELECT * FROM sheffield_businesses').all();

const withoutPersonNames = [];
const companyPatterns = [];
const placeNames = [];
const descriptive = [];

businesses.forEach(business => {
    const name = business.full_name;
    const lower = name.toLowerCase();

    // Skip if it has person name
    if (hasPersonName(name)) return;

    // Skip company patterns that might have person names removed
    if (lower.includes('& co') || lower.includes('& son') || lower.includes('& brothers')) {
        companyPatterns.push(business);
        return;
    }

    // Check if it's a place-based name
    if (lower.includes('sheffield') || lower.includes('york') || lower.includes('britannia') ||
        lower.includes('royal') || lower.includes('victoria') || lower.includes('albert hall') ||
        lower.includes('crown') || lower.includes('globe') || lower.includes('star')) {
        placeNames.push(business);
    } else {
        descriptive.push(business);
    }

    withoutPersonNames.push(business);
});

console.log('Total businesses:', businesses.length.toLocaleString());
console.log('Businesses without person names:', withoutPersonNames.length.toLocaleString());
console.log('  - Company patterns (& Co, & Son, etc):', companyPatterns.length.toLocaleString());
console.log('  - Place/symbolic names:', placeNames.length.toLocaleString());
console.log('  - Descriptive names:', descriptive.length.toLocaleString());
console.log('Percentage without person names:', ((withoutPersonNames.length / businesses.length) * 100).toFixed(1) + '%');

console.log('\n' + '='.repeat(70));
console.log('COMPANY PATTERNS (& Co, & Son, etc) - First 50');
console.log('='.repeat(70) + '\n');

companyPatterns.slice(0, 50).forEach((b, i) => {
    console.log(`${i + 1}. "${b.full_name}"`);
    console.log(`   ${b.occupation || '(no occupation)'} at ${b.address}`);
    console.log('');
});

if (companyPatterns.length > 50) {
    console.log(`... and ${(companyPatterns.length - 50).toLocaleString()} more\n`);
}

console.log('='.repeat(70));
console.log('PLACE/SYMBOLIC NAMES - First 50');
console.log('='.repeat(70) + '\n');

placeNames.slice(0, 50).forEach((b, i) => {
    console.log(`${i + 1}. "${b.full_name}"`);
    console.log(`   ${b.occupation || '(no occupation)'} at ${b.address}`);
    console.log('');
});

if (placeNames.length > 50) {
    console.log(`... and ${(placeNames.length - 50).toLocaleString()} more\n`);
}

console.log('='.repeat(70));
console.log('DESCRIPTIVE NAMES (No person, place, or company pattern) - First 100');
console.log('='.repeat(70) + '\n');

descriptive.slice(0, 100).forEach((b, i) => {
    console.log(`${i + 1}. "${b.full_name}"`);
    console.log(`   ${b.occupation || '(no occupation)'} at ${b.address}`);
    console.log('');
});

if (descriptive.length > 100) {
    console.log(`... and ${(descriptive.length - 100).toLocaleString()} more\n`);
}

db.close();
