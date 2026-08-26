const sqlite3 = require('sqlite3').verbose();
const dbPath = 'D:/projects/Saturday at Three/Sheffield1867.db';

const db = new sqlite3.Database(dbPath, (err) => {
  if (err) {
    console.error('Error:', err);
    process.exit(1);
  }
});

db.all(
  "SELECT DISTINCT ecclesiastical_parish FROM sheffield_players WHERE ecclesiastical_parish IS NOT NULL AND ecclesiastical_parish != '' ORDER BY ecclesiastical_parish",
  [],
  (err, rows) => {
    if (err) {
      console.error('Query error:', err);
      process.exit(1);
    }

    console.log('\nEcclesiastical Parishes in Database:\n');
    console.log('=====================================');
    rows.forEach((row, index) => {
      console.log(`${index + 1}. ${row.ecclesiastical_parish}`);
    });
    console.log(`\nTotal: ${rows.length} parishes\n`);

    db.close();
  }
);
