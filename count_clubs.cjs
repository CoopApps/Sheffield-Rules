const Database = require('better-sqlite3');

try {
    const db = new Database('./saturday_at_three.db', { readonly: true });
    const count = db.prepare('SELECT COUNT(*) as count FROM clubs').get();
    console.log(`Total clubs in saturday_at_three.db: ${count.count}`);
    db.close();
} catch (err) {
    console.error('Error:', err.message);
}
