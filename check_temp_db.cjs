const sqlite3 = require('better-sqlite3');

try {
    const db = new sqlite3('Sheffield1867_temp.db', { readonly: true });

    // Get all tables
    const tables = db.prepare(`SELECT name FROM sqlite_master WHERE type='table' ORDER BY name`).all();
    console.log('\n=== TABLES IN Sheffield1867_temp.db ===');
    tables.forEach(t => console.log(' -', t.name));

    // Check if sheffield_players exists and has data
    if (tables.some(t => t.name === 'sheffield_players')) {
        const count = db.prepare('SELECT COUNT(*) as count FROM sheffield_players').get();
        console.log('\n=== SHEFFIELD PLAYERS ===');
        console.log('Total players:', count.count);

        // Check if it has the new columns
        const columns = db.prepare('PRAGMA table_info(sheffield_players)').all();
        const hasFirstName = columns.some(c => c.name === 'first_name');
        const hasMiddleName = columns.some(c => c.name === 'middle_name');
        const hasSurname = columns.some(c => c.name === 'surname');

        console.log('\nColumns:');
        console.log('  - has first_name:', hasFirstName);
        console.log('  - has middle_name:', hasMiddleName);
        console.log('  - has surname:', hasSurname);

        // Sample some data
        const sample = db.prepare('SELECT * FROM sheffield_players LIMIT 3').all();
        console.log('\nSample records:');
        sample.forEach((p, i) => {
            console.log(`\n  Player ${i + 1}:`);
            console.log('    Name:', p.name);
            if (hasFirstName) console.log('    First:', p.first_name);
            if (hasSurname) console.log('    Surname:', p.surname);
            console.log('    Birth Year:', p.birth_year);
            console.log('    Club:', p.club_id);
        });
    }

    db.close();
} catch (err) {
    console.error('Error:', err.message);
}
