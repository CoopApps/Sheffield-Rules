const sqlite3 = require('sqlite3').verbose();
const db = new sqlite3.Database('./Sheffield1867.db', sqlite3.OPEN_READONLY);

console.log('Checking All Saints player assignments...\n');

// Get total players in All Saints
db.get(`
  SELECT COUNT(*) as total
  FROM sheffield_footballers f
  JOIN sheffield_people p ON f.person_id = p.unique_id
  WHERE p.ecclesiastical_parish = 'All Saints'
`, (err, row) => {
  if (err) {
    console.error('Error:', err);
    db.close();
    return;
  }

  console.log(`Total players in All Saints parish: ${row.total}`);

  // Get assigned players in All Saints
  db.get(`
    SELECT COUNT(*) as assigned
    FROM sheffield_footballers f
    JOIN sheffield_people p ON f.person_id = p.unique_id
    WHERE p.ecclesiastical_parish = 'All Saints'
    AND f.club_id != 'UNASSIGNED'
    AND f.club_id IS NOT NULL
  `, (err2, row2) => {
    if (err2) {
      console.error('Error:', err2);
      db.close();
      return;
    }

    console.log(`Assigned players: ${row2.assigned}`);
    console.log(`Unassigned players: ${row.total - row2.assigned}`);

    // Get breakdown by club
    db.all(`
      SELECT f.club_id, c.name as club_name, COUNT(*) as count
      FROM sheffield_footballers f
      JOIN sheffield_people p ON f.person_id = p.unique_id
      LEFT JOIN sheffield_clubs c ON f.club_id = c.id
      WHERE p.ecclesiastical_parish = 'All Saints'
      AND f.club_id != 'UNASSIGNED'
      AND f.club_id IS NOT NULL
      GROUP BY f.club_id, c.name
      ORDER BY count DESC
    `, (err3, rows) => {
      if (err3) {
        console.error('Error:', err3);
        db.close();
        return;
      }

      if (rows.length > 0) {
        console.log('\nAssignments by club:');
        rows.forEach(r => {
          console.log(`  ${r.club_name || r.club_id}: ${r.count} players`);
        });
      } else {
        console.log('\nNo players are assigned to any clubs.');
      }

      // Check if players have postcodes
      db.get(`
        SELECT COUNT(*) as with_postcode
        FROM sheffield_footballers f
        JOIN sheffield_people p ON f.person_id = p.unique_id
        WHERE p.ecclesiastical_parish = 'All Saints'
        AND p.postcode IS NOT NULL
        AND p.postcode != ''
      `, (err4, row4) => {
        if (err4) {
          console.error('Error:', err4);
          db.close();
          return;
        }

        console.log(`\nPlayers with postcodes: ${row4.with_postcode}`);
        console.log(`Players without postcodes: ${row.total - row4.with_postcode}`);

        db.close();
      });
    });
  });
});
