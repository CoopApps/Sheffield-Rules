const Database = require('better-sqlite3');
const fs = require('fs');

const db = new Database('./Sheffield1867.db');
const sql = fs.readFileSync('./populate_clubs.sql', 'utf8');

console.log('\n=== Executing SQL ===\n');

// Split by semicolon and execute each statement
const statements = sql.split(';').filter(s => {
    const trimmed = s.trim();
    return trimmed && !trimmed.startsWith('--') && trimmed !== 'SELECT COUNT(*) as total_clubs FROM sheffield_clubs';
});

console.log(`Executing ${statements.length} INSERT statements...\n`);

db.exec('BEGIN TRANSACTION');

for (const stmt of statements) {
    if (stmt.trim()) {
        db.exec(stmt);
    }
}

db.exec('COMMIT');

const count = db.prepare('SELECT COUNT(*) as c FROM sheffield_clubs').get();
console.log(`✓ Clubs populated successfully!`);
console.log(`Total clubs in database: ${count.c}\n`);

db.close();
