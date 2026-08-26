const sqlite3 = require('sqlite3').verbose();
const db = new sqlite3.Database('./Sheffield1867.db', sqlite3.OPEN_READONLY);

db.get(`
  SELECT COUNT(*) as unassigned
  FROM sheffield_footballers
  WHERE club_id = 'UNASSIGNED' OR club_id IS NULL
`, (err, row) => {
  if (err) {
    console.error('Error:', err);
    db.close();
    return;
  }

  console.log('Unassigned players:', row.unassigned);
  db.close();
});
