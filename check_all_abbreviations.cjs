const sqlite3 = require('better-sqlite3');
const db = new sqlite3('Sheffield1867.db');

try {
    // List of abbreviations from the Rust code
    const abbreviations = [
        { abbrev: "Thos", full: "Thomas" },
        { abbrev: "Wm", full: "William" },
        { abbrev: "Geo", full: "George" },
        { abbrev: "Chas", full: "Charles" },
        { abbrev: "Jas", full: "James" },
        { abbrev: "Jno", full: "John" },
        { abbrev: "Robt", full: "Robert" },
        { abbrev: "Saml", full: "Samuel" },
        { abbrev: "Benj", full: "Benjamin" },
        { abbrev: "Edw", full: "Edward" },
        { abbrev: "Richd", full: "Richard" },
        { abbrev: "Jos", full: "Joseph" },
        { abbrev: "Fredk", full: "Frederick" },
        { abbrev: "Fred", full: "Frederick" },
        { abbrev: "Hy", full: "Henry" },
    ];

    console.log('=== Checking for abbreviations in player database ===\n');

    let totalFound = 0;

    for (const { abbrev, full } of abbreviations) {
        const count = db.prepare(`
            SELECT COUNT(*) as count
            FROM sheffield_players
            WHERE first_name LIKE ? OR middle_name LIKE ? OR surname LIKE ?
        `).get(`%${abbrev}%`, `%${abbrev}%`, `%${abbrev}%`);

        if (count.count > 0) {
            console.log(`${abbrev} → ${full}: ${count.count} players`);
            totalFound += count.count;

            // Show a few examples
            const examples = db.prepare(`
                SELECT name, first_name, middle_name, surname
                FROM sheffield_players
                WHERE first_name LIKE ? OR middle_name LIKE ? OR surname LIKE ?
                LIMIT 3
            `).all(`%${abbrev}%`, `%${abbrev}%`, `%${abbrev}%`);

            examples.forEach(p => {
                console.log(`  - ${p.name} (first: "${p.first_name || ''}", middle: "${p.middle_name || ''}", surname: "${p.surname || ''}")`);
            });
            console.log('');
        }
    }

    console.log(`\nTotal players with abbreviations: ${totalFound}`);

    // Check for patterns that might be missed
    console.log('\n=== Checking for other potential abbreviations ===');
    const shortNames = db.prepare(`
        SELECT DISTINCT first_name
        FROM sheffield_players
        WHERE LENGTH(first_name) <= 4 AND first_name != ''
        ORDER BY first_name
    `).all();

    console.log('Short first names (4 chars or less):');
    shortNames.forEach(n => console.log(`  - ${n.first_name}`));

} catch (error) {
    console.error('Error:', error.message);
} finally {
    db.close();
}
