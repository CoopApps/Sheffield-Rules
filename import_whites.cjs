const Database = require('better-sqlite3');
const fs = require('fs');
const path = require('path');

const db = new Database('Sheffield1867_rescued.db');

console.log('=== IMPORTING BUSINESSES FROM WHITES ===\n');

db.exec(`DROP TABLE IF EXISTS sheffield_businesses`);
db.exec(`CREATE TABLE sheffield_businesses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    surname TEXT,
    forename TEXT,
    title TEXT,
    occupation TEXT,
    address TEXT,
    year INTEGER,
    source TEXT,
    sheffield_person_id INTEGER
)`);

const insert = db.prepare(`INSERT INTO sheffield_businesses (surname, forename, title, occupation, address, year, source) VALUES (?, ?, ?, ?, ?, ?, ?)`);

const whitesDir = './Whites';
const files = fs.readdirSync(whitesDir).filter(f => f.endsWith('.csv')).sort();

console.log(`Found ${files.length} CSV files`);

let total = 0;
const insertMany = db.transaction((rows) => {
    for (const row of rows) insert.run(...row);
});

for (const file of files) {
    const content = fs.readFileSync(path.join(whitesDir, file), 'utf8');
    const lines = content.split('\n').slice(1).filter(l => l.trim());

    const rows = [];
    for (const line of lines) {
        const fields = parseCSV(line);
        if (fields.length >= 7) {
            rows.push([fields[0], fields[1], fields[2], fields[3], fields[4], parseInt(fields[5]) || 1871, fields[6]]);
        }
    }

    insertMany(rows);
    console.log(`${file}: ${rows.length} rows`);
    total += rows.length;
}

const count = db.prepare('SELECT COUNT(*) as c FROM sheffield_businesses').get().c;
console.log(`\nTotal: ${count} businesses imported`);

db.close();

function parseCSV(line) {
    const fields = [];
    let current = '';
    let inQuotes = false;
    for (const c of line) {
        if (c === '"') inQuotes = !inQuotes;
        else if (c === ',' && !inQuotes) { fields.push(current.trim()); current = ''; }
        else current += c;
    }
    fields.push(current.trim());
    return fields;
}
