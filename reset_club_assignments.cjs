const sqlite3 = require('sqlite3').verbose();

const db = new sqlite3.Database('./Sheffield1867.db', sqlite3.OPEN_READWRITE, (err) => {
  if (err) {
    console.error('Error opening database:', err.message);
    process.exit(1);
  }
});

console.log('Resetting all club assignments to UNASSIGNED...\n');

db.run(`UPDATE sheffield_footballers SET club_id = 'UNASSIGNED'`, function(err) {
  if (err) {
    console.error('Error resetting assignments:', err.message);
    db.close();
    process.exit(1);
  }

  console.log(`✓ Reset ${this.changes} player club assignments to UNASSIGNED`);

  // Verify the reset
  db.get(`
    SELECT
      COUNT(*) as total,
      SUM(CASE WHEN club_id != 'UNASSIGNED' AND club_id IS NOT NULL THEN 1 ELSE 0 END) as assigned,
      SUM(CASE WHEN club_id = 'UNASSIGNED' OR club_id IS NULL THEN 1 ELSE 0 END) as unassigned
    FROM sheffield_footballers
  `, (err, stats) => {
    if (err) {
      console.error('Error verifying reset:', err.message);
      db.close();
      process.exit(1);
    }

    console.log('\nVerification:');
    console.log(`Total Players: ${stats.total}`);
    console.log(`Assigned to Clubs: ${stats.assigned}`);
    console.log(`Unassigned: ${stats.unassigned}`);
    console.log('\n✓ All players have been reset to UNASSIGNED');

    db.close();
  });
});
