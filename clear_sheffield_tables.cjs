const Database = require('better-sqlite3');

// Open the database
const db = new Database('Sheffield1867.db');

try {
  // Clear the tables
  const deletePlayers = db.prepare('DELETE FROM sheffield_players');
  const deletePeople = db.prepare('DELETE FROM sheffield_people');

  const playersResult = deletePlayers.run();
  const peopleResult = deletePeople.run();

  console.log(`Cleared ${playersResult.changes} rows from sheffield_players`);
  console.log(`Cleared ${peopleResult.changes} rows from sheffield_people`);
  console.log('Tables cleared successfully!');
} catch (error) {
  console.error('Error clearing tables:', error.message);
} finally {
  db.close();
}
