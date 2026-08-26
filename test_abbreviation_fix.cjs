const sqlite3 = require('better-sqlite3');
const db = new sqlite3('Sheffield1867.db');

try {
    // First, let's see what "Thos" entries exist before any changes
    console.log('\n=== BEFORE: Players with "Thos" ===');
    const before = db.prepare(`
        SELECT id, name, first_name, middle_name, surname
        FROM sheffield_players
        WHERE first_name LIKE '%Thos%' OR middle_name LIKE '%Thos%'
        ORDER BY surname
        LIMIT 5
    `).all();

    console.log(`Found ${before.length} players (showing first 5):`);
    before.forEach(p => {
        console.log(`  ${p.name} (first: "${p.first_name}", middle: "${p.middle_name}", surname: "${p.surname}")`);
    });

    // Now let's test what the current UPDATE would do
    console.log('\n=== Testing SQLite REPLACE behavior ===');

    const testCases = [
        { input: "Thos Smith", pattern: "Thos ", replacement: "Thomas " },
        { input: "Thos", pattern: "Thos", replacement: "Thomas" },
        { input: " Thos", pattern: " Thos", replacement: " Thomas" },
        { input: "John Thos", pattern: " Thos", replacement: " Thomas" },
    ];

    testCases.forEach(({ input, pattern, replacement }) => {
        const result = db.prepare(`SELECT REPLACE(?, ?, ?) as result`).get(input, pattern, replacement);
        console.log(`  REPLACE("${input}", "${pattern}", "${replacement}") = "${result.result}"`);
    });

} catch (error) {
    console.error('Error:', error.message);
} finally {
    db.close();
}
