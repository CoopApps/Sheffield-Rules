const sqlite3 = require('better-sqlite3');
const db = new sqlite3('Sheffield1867.db');

try {
    // Let's manually test what the UPDATE would do to a "Thos" first_name
    console.log('=== Testing the actual UPDATE logic ===\n');

    const testValue = "Thos";

    // This mimics the logic from the Rust code for "Thos" -> "Thomas"
    const result = db.prepare(`
        SELECT
            ? as original,
            REPLACE(REPLACE(REPLACE(TRIM(?), 'Thos.', 'Thomas.'), 'Thos ', 'Thomas '), ' Thos', ' Thomas') as result1,
            TRIM(?) as trimmed
    `).get(testValue, testValue, testValue);

    console.log(`Original: "${result.original}"`);
    console.log(`Trimmed: "${result.trimmed}"`);
    console.log(`After REPLACE chain: "${result.result1}"`);

    console.log('\n=== The problem ===');
    console.log('When first_name = "Thos" (no spaces, no dots):');
    console.log('  - REPLACE("Thos", "Thos.", "Thomas.") = "Thos" (no match)');
    console.log('  - REPLACE("Thos", "Thos ", "Thomas ") = "Thos" (no match)');
    console.log('  - REPLACE("Thos", " Thos", " Thomas") = "Thos" (no match)');
    console.log('\nNone of the patterns match standalone "Thos"!');

    console.log('\n=== What we need ===');
    console.log('We need to also handle exact word boundaries:');

    // Test a better approach
    const better = db.prepare(`
        SELECT
            CASE
                WHEN TRIM(?) = 'Thos' THEN 'Thomas'
                ELSE REPLACE(REPLACE(REPLACE(TRIM(?), 'Thos.', 'Thomas.'), 'Thos ', 'Thomas '), ' Thos', ' Thomas')
            END as result
    `).get(testValue, testValue);

    console.log(`Better result: "${better.result}"`);

} catch (error) {
    console.error('Error:', error.message);
} finally {
    db.close();
}
