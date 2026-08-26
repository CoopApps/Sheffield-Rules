const Database = require('better-sqlite3');
const path = require('path');

const dbPath = path.join(__dirname, 'Sheffield1867.db');
const db = new Database(dbPath);

console.log('Copying region data to where_from column...');

try {
  // Copy region to where_from
  const result = db.prepare("UPDATE sheffield_clubs SET where_from = region WHERE region IS NOT NULL AND region != ''").run();

  console.log(`Updated ${result.changes} clubs`);

  // Show some sample data
  const samples = db.prepare("SELECT name, region, where_from FROM sheffield_clubs WHERE where_from IS NOT NULL LIMIT 10").all();

  console.log('\nSample clubs with postcodes:');
  samples.forEach(club => {
    console.log(`  ${club.name}: region="${club.region}" -> where_from="${club.where_from}"`);
  });

  db.close();
  console.log('\nDone!');
} catch (err) {
  console.error('Error:', err);
  db.close();
  process.exit(1);
}
