const sqlite3 = require('better-sqlite3');
const db = new sqlite3('Sheffield1867.db');

try {
    console.log('=== Finding ACTUAL abbreviations (standalone or with punctuation) ===\n');

    // Check for common Victorian abbreviations
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
        { abbrev: "Edwd", full: "Edward" },
        { abbrev: "Richd", full: "Richard" },
        { abbrev: "Jos", full: "Joseph" },
        { abbrev: "Fredk", full: "Frederick" },
        { abbrev: "Fred", full: "Frederick" },
        { abbrev: "Hy", full: "Henry" },
        { abbrev: "Albt", full: "Albert" },
        { abbrev: "Alfd", full: "Alfred" },
        { abbrev: "Jame", full: "James" },
        { abbrev: "Geor", full: "George" },
        { abbrev: "Will", full: "William" },
        { abbrev: "Isac", full: "Isaac" },
    ];

    for (const { abbrev, full } of abbreviations) {
        // Look for exact matches or matches with dots/spaces
        const count = db.prepare(`
            SELECT COUNT(*) as count
            FROM sheffield_players
            WHERE first_name = ?
               OR first_name = ?
               OR middle_name = ?
               OR middle_name = ?
        `).get(abbrev, abbrev + '.', abbrev, abbrev + '.');

        if (count.count > 0) {
            console.log(`\n${abbrev} → ${full}: ${count.count} players`);

            // Show examples
            const examples = db.prepare(`
                SELECT name, first_name, middle_name
                FROM sheffield_players
                WHERE first_name = ?
                   OR first_name = ?
                   OR middle_name = ?
                   OR middle_name = ?
                LIMIT 5
            `).all(abbrev, abbrev + '.', abbrev, abbrev + '.');

            examples.forEach(p => {
                console.log(`  - ${p.name}`);
                console.log(`    first: "${p.first_name || ''}", middle: "${p.middle_name || ''}"`);
            });
        }
    }

    // Check for single-letter first names (likely initials that should be expanded)
    console.log('\n\n=== Single letter first names (likely initials) ===');
    const singleLetters = db.prepare(`
        SELECT first_name, COUNT(*) as count, GROUP_CONCAT(name, '; ') as examples
        FROM sheffield_players
        WHERE LENGTH(first_name) = 1
        GROUP BY first_name
        ORDER BY count DESC
    `).all();

    singleLetters.forEach(({ first_name, count, examples }) => {
        const exampleList = examples.split('; ').slice(0, 3).join('; ');
        console.log(`  ${first_name}: ${count} players (e.g., ${exampleList})`);
    });

} catch (error) {
    console.error('Error:', error.message);
} finally {
    db.close();
}
