const Database = require('better-sqlite3');
const db = new Database('D:/projects/Saturday at Three/Sheffield1867.db', { readonly: true });

console.log('\n=== Tables in Sheffield1867.db ===\n');

const tables = db.prepare(`
    SELECT name FROM sqlite_master
    WHERE type='table'
    ORDER BY name
`).all();

console.log('Tables found:');
tables.forEach(table => {
    console.log(`  - ${table.name}`);
});

// Check for any tables with 'news' or 'event' in the name
console.log('\n=== Tables with "news" or "event" in name ===');
const eventTables = tables.filter(t =>
    t.name.toLowerCase().includes('news') ||
    t.name.toLowerCase().includes('event')
);

if (eventTables.length > 0) {
    eventTables.forEach(table => {
        console.log(`\n--- Table: ${table.name} ---`);

        // Get row count
        const count = db.prepare(`SELECT COUNT(*) as count FROM ${table.name}`).get();
        console.log(`Row count: ${count.count}`);

        // Get schema
        const schema = db.prepare(`PRAGMA table_info(${table.name})`).all();
        console.log('Columns:');
        schema.forEach(col => {
            console.log(`  ${col.name} (${col.type})`);
        });

        // Show a few sample rows
        if (count.count > 0) {
            console.log('\nSample rows (first 3):');
            const samples = db.prepare(`SELECT * FROM ${table.name} LIMIT 3`).all();
            samples.forEach(row => {
                console.log(JSON.stringify(row, null, 2));
            });
        }
    });
} else {
    console.log('No tables found with "news" or "event" in name');
}

db.close();
