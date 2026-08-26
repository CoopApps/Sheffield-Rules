const sqlite3 = require('sqlite3').verbose();
const db = new sqlite3.Database('./Sheffield1867.db');

console.log('Starting player unassignment...\n');

// First, check how many players are currently assigned
db.get(`
  SELECT COUNT(*) as assigned_count
  FROM sheffield_footballers
  WHERE club_id != 'UNASSIGNED' AND club_id IS NOT NULL
`, (err, row) => {
  if (err) {
    console.error('Error checking assigned players:', err);
    db.close();
    return;
  }

  const assignedCount = row.assigned_count;
  console.log(`Found ${assignedCount} players currently assigned to clubs`);

  if (assignedCount === 0) {
    console.log('No players to unassign. All players are already unassigned.');
    db.close();
    return;
  }

  // Unassign all players
  db.run(`
    UPDATE sheffield_footballers
    SET club_id = 'UNASSIGNED'
    WHERE club_id != 'UNASSIGNED' AND club_id IS NOT NULL
  `, function(err) {
    if (err) {
      console.error('Error unassigning players:', err);
      db.close();
      return;
    }

    console.log(`\n✓ Successfully unassigned ${this.changes} players`);
    console.log('All players are now set to UNASSIGNED\n');

    db.close();
  });
});
