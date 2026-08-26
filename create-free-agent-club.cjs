const sqlite3 = require('sqlite3').verbose();
const dbPath = 'D:/projects/Saturday at Three/Sheffield1867.db';

const db = new sqlite3.Database(dbPath, (err) => {
  if (err) {
    console.error('Error:', err);
    process.exit(1);
  }
  console.log('Connected to database\n');
});

// Create a special "Unassigned" club for players not yet assigned to a team
const unassignedId = 'UNASSIGNED';

db.run(
  `INSERT OR IGNORE INTO sheffield_clubs (id, name, founded_year)
   VALUES (?, 'Unassigned', 1858)`,
  [unassignedId],
  (err) => {
    if (err) {
      console.error('✗ Error creating Unassigned club:', err.message);
    } else {
      console.log('✓ Unassigned club created successfully');
    }

    // Verify
    db.get(
      'SELECT id, name FROM sheffield_clubs WHERE id = ?',
      [unassignedId],
      (err, row) => {
        if (!err && row) {
          console.log('✓ Verification: Unassigned club exists');
          console.log('  ID:', row.id);
          console.log('  Name:', row.name);
        }
        db.close();
      }
    );
  }
);
