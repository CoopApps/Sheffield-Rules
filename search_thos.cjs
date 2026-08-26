const sqlite3 = require('better-sqlite3');
const db = new sqlite3('Sheffield1867.db');

try {
    const players = db.prepare(`
        SELECT id, name, first_name, middle_name, surname, birth_year, position, club_id
        FROM sheffield_players
        WHERE first_name LIKE '%Thos%' OR middle_name LIKE '%Thos%'
        ORDER BY surname
    `).all();

    console.log(`\nFound ${players.length} player(s) with "Thos" in their name:\n`);

    if (players.length > 0) {
        players.forEach(player => {
            console.log(`ID: ${player.id}`);
            console.log(`Full Name: ${player.name}`);
            console.log(`Parsed Name: ${player.first_name || ''} ${player.middle_name || ''} ${player.surname || ''}`);
            console.log(`Birth Year: ${player.birth_year || 'Unknown'}`);
            console.log(`Position: ${player.position || 'Unknown'}`);
            console.log(`Club ID: ${player.club_id || 'Unknown'}`);
            console.log('---');
        });
    } else {
        console.log('No players found with "Thos" in their first or middle name.');
    }
} catch (error) {
    console.error('Error:', error.message);
} finally {
    db.close();
}
