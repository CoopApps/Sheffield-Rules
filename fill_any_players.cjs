const sqlite3 = require('sqlite3').verbose();
const db = new sqlite3.Database('./Sheffield1867.db');

console.log('Filling remaining clubs with any available players...\n');

// Get all clubs that have fewer than 15 players
db.all(`
  SELECT
    c.id as club_id,
    c.name as club_name,
    COUNT(f.id) as current_count,
    (15 - COUNT(f.id)) as spaces_needed
  FROM sheffield_clubs c
  LEFT JOIN sheffield_footballers f ON c.id = f.club_id AND f.club_id != 'UNASSIGNED'
  GROUP BY c.id, c.name
  HAVING current_count < 15
  ORDER BY current_count ASC
`, (err, clubs) => {
  if (err) {
    console.error('Error getting clubs:', err);
    db.close();
    return;
  }

  console.log(`Found ${clubs.length} clubs with space\n`);

  let totalAssigned = 0;
  let clubIndex = 0;

  function assignToNextClub() {
    if (clubIndex >= clubs.length) {
      console.log(`\n✓ Completed! Assigned ${totalAssigned} players total`);
      db.close();
      return;
    }

    const club = clubs[clubIndex];

    // Find ANY unassigned players (ignore postcode matching)
    db.all(`
      SELECT f.id, p.surname || ', ' || p.first_name as name
      FROM sheffield_footballers f
      JOIN sheffield_people p ON f.person_id = p.unique_id
      WHERE f.club_id = 'UNASSIGNED'
      LIMIT ?
    `, [club.spaces_needed], (err2, players) => {
      if (err2) {
        console.error('Error getting players:', err2);
        db.close();
        return;
      }

      if (players.length === 0) {
        console.log(`⊘ ${club.club_name}: No available players left [${club.current_count}/15]`);
        clubIndex++;
        assignToNextClub();
        return;
      }

      // Assign these players to the club
      const playerIds = players.map(p => p.id);
      const placeholders = playerIds.map(() => '?').join(',');

      db.run(`
        UPDATE sheffield_footballers
        SET club_id = ?
        WHERE id IN (${placeholders})
      `, [club.club_id, ...playerIds], function(err3) {
        if (err3) {
          console.error('Error assigning players:', err3);
          db.close();
          return;
        }

        const assigned = this.changes;
        totalAssigned += assigned;
        const newTotal = club.current_count + assigned;

        console.log(`✓ ${club.club_name}: +${assigned} players [${newTotal}/15]`);

        clubIndex++;
        assignToNextClub();
      });
    });
  }

  assignToNextClub();
});
