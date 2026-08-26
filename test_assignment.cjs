const sqlite3 = require('sqlite3').verbose();
const db = new sqlite3.Database('./Sheffield1867.db', sqlite3.OPEN_READONLY);

db.all(`
  SELECT COUNT(*) as unassigned 
  FROM sheffield_footballers f 
  JOIN sheffield_people p ON f.person_id = p.unique_id 
  WHERE p.ecclesiastical_parish = 'All Saints' AND f.club_id = 'UNASSIGNED'
`, (err, rows) => {
  if (err) {
    console.error(err);
    db.close();
    return;
  }
  console.log('All Saints - Unassigned players:', rows[0].unassigned);
  db.close();
});
