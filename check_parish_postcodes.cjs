const Database = require('better-sqlite3');
const path = require('path');

const dbPath = path.join(__dirname, 'Sheffield1867.db');
const db = new Database(dbPath);

console.log('Checking postcodes assigned to parishes...\n');

try {
  // Check All Saints parish
  const allSaintsPlayers = db.prepare(`
    SELECT where_born, COUNT(*) as count
    FROM sheffield_players
    WHERE ecclesiastical_parish = 'All Saints'
    GROUP BY where_born
    ORDER BY count DESC
  `).all();

  console.log('All Saints parish - where_born values:');
  allSaintsPlayers.forEach(row => {
    console.log(`  ${row.where_born || 'NULL'}: ${row.count} players`);
  });

  // Check a sample of players from All Saints
  console.log('\nSample players from All Saints:');
  const samples = db.prepare(`
    SELECT id, name, ecclesiastical_parish, where_born
    FROM sheffield_players
    WHERE ecclesiastical_parish = 'All Saints'
    LIMIT 10
  `).all();

  samples.forEach(p => {
    console.log(`  ${p.name}: where_born="${p.where_born || 'NULL'}"`);
  });

  db.close();
  console.log('\nDone!');
} catch (err) {
  console.error('Error:', err);
  db.close();
  process.exit(1);
}
