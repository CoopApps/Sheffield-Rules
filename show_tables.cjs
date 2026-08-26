const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

const tables = db.prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name").all();

console.log('Tables in Sheffield1867.db:');
console.log('='.repeat(50));

tables.forEach(t => {
    const count = db.prepare(`SELECT COUNT(*) as c FROM ${t.name}`).get().c;
    console.log(`${t.name.padEnd(30)} ${count.toLocaleString().padStart(10)} rows`);
});

db.close();
