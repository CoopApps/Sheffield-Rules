const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('=== Analyzing Hannah Hall duplicates ===\n');

const records = db.prepare(`
    SELECT id, source FROM sheffield_people
    WHERE name = ? AND birth_year = ?
`).all('Hannah Hall', 1820);

console.log('Total Hannah Hall (1820) records:', records.length);
console.log('');

// Group by source
const bySources = {};
records.forEach(r => {
    if (!bySources[r.source]) bySources[r.source] = [];
    bySources[r.source].push(r.id);
});

console.log('Breakdown by source:');
Object.keys(bySources).forEach(source => {
    console.log(`  ${source}: ${bySources[source].length} times`);
});

console.log('\n=== Summary of all duplicates in sheffield_people ===\n');

// Check if import_all_years ran multiple times
const sources = db.prepare(`
    SELECT source, COUNT(*) as count
    FROM sheffield_people
    GROUP BY source
    ORDER BY source
`).all();

console.log('Records by source:');
sources.forEach(s => {
    console.log(`  ${s.source}: ${s.count.toLocaleString()}`);
});

db.close();
