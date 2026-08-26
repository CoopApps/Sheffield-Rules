const Database = require('better-sqlite3');

try {
    const db = new Database('./saturday_at_three.db', { readonly: true });

    // Get table schema
    console.log('\n=== CLUBS TABLE SCHEMA ===');
    const schema = db.prepare('PRAGMA table_info(clubs)').all();
    schema.forEach(col => {
        console.log(`${col.name} - ${col.type}${col.notnull ? ' NOT NULL' : ''}${col.dflt_value ? ' DEFAULT ' + col.dflt_value : ''}`);
    });

    // Get sample data
    console.log('\n=== SAMPLE CLUBS DATA ===');
    const clubs = db.prepare('SELECT * FROM clubs LIMIT 3').all();
    clubs.forEach((club, i) => {
        console.log(`\nClub ${i + 1}:`);
        Object.entries(club).forEach(([key, value]) => {
            if (value !== null) {
                console.log(`  ${key}: ${value}`);
            }
        });
    });

    // Get total count
    const count = db.prepare('SELECT COUNT(*) as count FROM clubs').get();
    console.log(`\n=== TOTAL CLUBS: ${count.count} ===`);

    db.close();
} catch (err) {
    console.error('Error:', err.message);
}
