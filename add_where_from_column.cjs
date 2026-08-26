const Database = require('better-sqlite3');
const path = require('path');

const dbPath = path.join(__dirname, 'Sheffield1867.db');
const db = new Database(dbPath);

console.log('Adding where_from column to sheffield_clubs table...');

try {
  // Check if column already exists
  const tableInfo = db.prepare("PRAGMA table_info(sheffield_clubs)").all();
  const hasWhereFrom = tableInfo.some(col => col.name === 'where_from');

  if (hasWhereFrom) {
    console.log('Column where_from already exists!');
  } else {
    // Add the column
    db.prepare("ALTER TABLE sheffield_clubs ADD COLUMN where_from TEXT").run();
    console.log('Successfully added where_from column!');
  }

  // Show current columns
  console.log('\nCurrent columns in sheffield_clubs:');
  tableInfo.forEach(col => {
    console.log(`  - ${col.name} (${col.type})`);
  });

  db.close();
  console.log('\nDone!');
} catch (err) {
  console.error('Error:', err);
  db.close();
  process.exit(1);
}
