const sqlite3 = require('sqlite3').verbose();

const db = new sqlite3.Database('./Sheffield1867.db', sqlite3.OPEN_READONLY, (err) => {
  if (err) {
    console.error('Error opening database:', err.message);
    process.exit(1);
  }
});

// Count total, assigned, and unassigned players
db.get(`
  SELECT
    COUNT(*) as total,
    SUM(CASE WHEN club_id != 'UNASSIGNED' AND club_id IS NOT NULL THEN 1 ELSE 0 END) as assigned,
    SUM(CASE WHEN club_id = 'UNASSIGNED' OR club_id IS NULL THEN 1 ELSE 0 END) as unassigned
  FROM sheffield_footballers
`, (err, stats) => {
  if (err) {
    console.error('Error querying stats:', err.message);
    db.close();
    process.exit(1);
  }

  console.log('\n=== CLUB ASSIGNMENT STATISTICS ===\n');
  console.log(`Total Players: ${stats.total}`);
  console.log(`Assigned to Clubs: ${stats.assigned}`);
  console.log(`Unassigned: ${stats.unassigned}`);
  console.log('');

  // Get top clubs by player count
  db.all(`
    SELECT c.id, c.name, COUNT(f.id) as player_count
    FROM sheffield_clubs c
    LEFT JOIN sheffield_footballers f ON c.id = f.club_id
    GROUP BY c.id, c.name
    HAVING player_count > 0
    ORDER BY player_count DESC
    LIMIT 20
  `, (err, clubs) => {
    if (err) {
      console.error('Error querying clubs:', err.message);
      db.close();
      process.exit(1);
    }

    if (clubs.length > 0) {
      console.log('=== TOP CLUBS BY PLAYER COUNT ===\n');
      clubs.forEach((club, idx) => {
        console.log(`${idx + 1}. ${club.name} (${club.id}): ${club.player_count} players`);
      });
    } else {
      console.log('No clubs have any players assigned.');
    }

    db.close();
  });
});
