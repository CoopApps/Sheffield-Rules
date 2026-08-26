const sqlite3 = require('better-sqlite3');
const db = new sqlite3('Sheffield1867.db');

try {
    const tables = db.prepare(`
        SELECT name FROM sqlite_master WHERE type='table' ORDER BY name
    `).all();

    console.log('\nTables in Sheffield1867.db:');
    tables.forEach(t => console.log('  -', t.name));
} catch (error) {
    console.error('Error:', error.message);
} finally {
    db.close();
}
