const Database = require('better-sqlite3');
const db = new Database('D:/projects/Saturday at Three/Sheffield1867_temp.db', { readonly: true });

console.log('\n=== Cup Competitions in Sheffield1867_temp.db ===\n');

try {
    const comps = db.prepare('SELECT * FROM sheffield_competitions').all();
    console.log(`Found ${comps.length} competitions:`);
    comps.forEach(comp => {
        console.log('\n' + JSON.stringify(comp, null, 2));
    });
} catch (e) {
    console.log('Error:', e.message);
}

db.close();
