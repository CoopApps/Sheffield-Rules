// Simple script to populate the Sheffield1867.db database
const fs = require('fs');
const path = require('path');

// Try to load better-sqlite3, fallback to sqlite3 if needed
let Database;
try {
  Database = require('better-sqlite3');
  console.log('Using better-sqlite3');
} catch (e) {
  console.error('better-sqlite3 not available:', e.message);
  console.log('Please install sqlite3 command line tool or rebuild better-sqlite3');
  process.exit(1);
}

const dbPath = path.join(__dirname, 'Sheffield1867.db');
const sqlPath = path.join(__dirname, 'populate_clubs_1857_1875.sql');

try {
  const db = new Database(dbPath);
  const sql = fs.readFileSync(sqlPath, 'utf8');

  console.log('Executing SQL...');
  db.exec(sql);

  // Verify the results
  const count = db.prepare('SELECT COUNT(*) as count FROM sheffield_clubs').get();
  console.log(`Successfully populated database with ${count.count} clubs`);

  // Show a few examples
  const examples = db.prepare('SELECT name, founded_year, ground_name FROM sheffield_clubs ORDER BY founded_year LIMIT 5').all();
  console.log('\nFirst 5 clubs:');
  examples.forEach(club => {
    console.log(`  ${club.name} (${club.founded_year}) - ${club.ground_name}`);
  });

  db.close();
  console.log('\nDatabase closed successfully');
} catch (error) {
  console.error('Error:', error.message);
  process.exit(1);
}
