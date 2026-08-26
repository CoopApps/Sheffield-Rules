const sqlite3 = require('better-sqlite3');

const db = new sqlite3('Sheffield1867.db');

console.log('='.repeat(70));
console.log('IDENTIFYING PATRONS (EMPLOYERS & INFLUENTIAL PEOPLE)');
console.log('='.repeat(70) + '\n');

// Add patron column to both tables if they don't exist
console.log('Adding patron columns...');

try {
    db.exec('ALTER TABLE sheffield_people ADD COLUMN is_patron INTEGER DEFAULT 0');
    console.log('✓ Added is_patron to sheffield_people');
} catch (e) {
    if (e.message.includes('duplicate column')) {
        console.log('  Column is_patron already exists in sheffield_people');
    } else {
        throw e;
    }
}

try {
    db.exec('ALTER TABLE sheffield_players ADD COLUMN is_patron INTEGER DEFAULT 0');
    console.log('✓ Added is_patron to sheffield_players');
} catch (e) {
    if (e.message.includes('duplicate column')) {
        console.log('  Column is_patron already exists in sheffield_players');
    } else {
        throw e;
    }
}

console.log('\n');

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
        /(\d+)\+?\s*apprentices?/i,
        /(\d+)\+?\s*females?/i
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

// Mark employers as patrons in sheffield_people
console.log('Identifying employers in sheffield_people...');

const employers = db.prepare(`
    SELECT id, name, profession
    FROM sheffield_people
    WHERE profession LIKE '%employ%'
`).all();

console.log(`Found ${employers.length} people with employer professions\n`);

// Categorize by employment size
let majorPatrons = 0;  // 20+ employees
let significantPatrons = 0;  // 10-19 employees
let minorPatrons = 0;  // 1-9 employees

const updatePatron = db.prepare(`
    UPDATE sheffield_people
    SET is_patron = 1
    WHERE id = ?
`);

employers.forEach(emp => {
    const employeeCount = parseEmploymentCount(emp.profession);

    updatePatron.run(emp.id);

    if (employeeCount >= 20) {
        majorPatrons++;
    } else if (employeeCount >= 10) {
        significantPatrons++;
    } else if (employeeCount > 0) {
        minorPatrons++;
    }
});

console.log('Patron categories:');
console.log(`  Major patrons (20+ employees): ${majorPatrons}`);
console.log(`  Significant patrons (10-19 employees): ${significantPatrons}`);
console.log(`  Minor patrons (1-9 employees): ${minorPatrons}`);
console.log(`  Total marked as patrons: ${employers.length}\n`);

// Copy patron status to sheffield_players
console.log('Copying patron status to sheffield_players...');

const copiedPatrons = db.prepare(`
    UPDATE sheffield_players
    SET is_patron = 1
    WHERE id IN (
        SELECT player_id
        FROM sheffield_people
        WHERE player_id IS NOT NULL
        AND is_patron = 1
    )
`).run();

console.log(`  ${copiedPatrons.changes} players marked as patrons\n`);

// Show top 20 patrons
console.log('='.repeat(70));
console.log('TOP 20 PATRONS BY WORKFORCE SIZE:');
console.log('='.repeat(70) + '\n');

const patronsWithCounts = employers.map(emp => ({
    ...emp,
    employeeCount: parseEmploymentCount(emp.profession)
}));

patronsWithCounts.sort((a, b) => b.employeeCount - a.employeeCount);

for (let i = 0; i < 20 && i < patronsWithCounts.length; i++) {
    const patron = patronsWithCounts[i];
    console.log(`${i + 1}. ${patron.name}`);
    console.log(`   Employees: ${patron.employeeCount}`);
    console.log(`   ${patron.profession}`);
    console.log('');
}

// Final statistics
const peopleStats = db.prepare(`
    SELECT
        COUNT(*) as total,
        COUNT(CASE WHEN is_patron = 1 THEN 1 END) as patrons
    FROM sheffield_people
`).get();

const playerStats = db.prepare(`
    SELECT
        COUNT(*) as total,
        COUNT(CASE WHEN is_patron = 1 THEN 1 END) as patrons
    FROM sheffield_players
`).get();

console.log('='.repeat(70));
console.log('FINAL STATISTICS:');
console.log('='.repeat(70));
console.log('\nsheffield_people:');
console.log(`  Total: ${peopleStats.total.toLocaleString()}`);
console.log(`  Patrons: ${peopleStats.patrons.toLocaleString()} (${(peopleStats.patrons / peopleStats.total * 100).toFixed(2)}%)`);

console.log('\nsheffield_players:');
console.log(`  Total: ${playerStats.total.toLocaleString()}`);
console.log(`  Patrons: ${playerStats.patrons.toLocaleString()} (${(playerStats.patrons / playerStats.total * 100).toFixed(2)}%)`);

db.close();

console.log('\n✓ Patron identification complete!');
