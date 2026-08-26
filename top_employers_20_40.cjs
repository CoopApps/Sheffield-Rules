const sqlite3 = require('better-sqlite3');

const db = new sqlite3('Sheffield1867.db');

console.log('='.repeat(70));
console.log('TOP EMPLOYERS RANKED 20-40 BY WORKFORCE SIZE');
console.log('='.repeat(70) + '\n');

// Get all employers
const employers = db.prepare(`
    SELECT name, birth_year, profession, street_address
    FROM sheffield_people
    WHERE profession LIKE '%employ%'
    ORDER BY name
`).all();

console.log(`Found ${employers.length} total employers\n`);

// Parse employment numbers from profession text
function parseEmploymentCount(profession) {
    if (!profession) return 0;

    const text = profession.toLowerCase();
    let total = 0;

    // Match patterns like "45 men", "7 boys", "360+ men", etc.
    const patterns = [
        /(\d+)\+?\s*men/i,
        /(\d+)\+?\s*boys?/i,
        /(\d+)\+?\s*women/i,
        /(\d+)\+?\s*woman/i,
        /(\d+)\+?\s*girls?/i,
        /(\d+)\+?\s*hands?/i,
        /(\d+)\+?\s*persons?/i,
        /(\d+)\+?\s*workmen/i,
        /(\d+)\+?\s*apprentices?/i
    ];

    patterns.forEach(pattern => {
        const match = text.match(pattern);
        if (match) {
            total += parseInt(match[1]);
        }
    });

    // Also check for simpler patterns like "employing 45"
    if (total === 0) {
        const simpleMatch = text.match(/employ(?:ing)?\s+(\d+)/i);
        if (simpleMatch) {
            total = parseInt(simpleMatch[1]);
        }
    }

    return total;
}

// Create array with employment counts
const employersWithCounts = employers.map(emp => {
    const count = parseEmploymentCount(emp.profession);
    return {
        ...emp,
        employmentCount: count
    };
});

// Sort by employment count (descending)
employersWithCounts.sort((a, b) => b.employmentCount - a.employmentCount);

// Filter to only those with actual employee counts
const withEmployees = employersWithCounts.filter(e => e.employmentCount > 0);

console.log(`Employers with measurable workforce: ${withEmployees.length}\n`);

// Get ranks 20-40 (indices 19-39 in 0-indexed array)
console.log('EMPLOYERS RANKED 20-40:\n');
console.log('='.repeat(70) + '\n');

for (let i = 19; i < 40 && i < withEmployees.length; i++) {
    const emp = withEmployees[i];
    console.log(`${i + 1}. ${emp.name} (${emp.birth_year})`);
    console.log(`   Workforce: ${emp.employmentCount} people`);
    console.log(`   ${emp.profession}`);
    if (emp.street_address) {
        console.log(`   Address: ${emp.street_address}`);
    }
    console.log('');
}

db.close();

console.log('✓ Complete!');
