const Database = require('better-sqlite3');
const path = require('path');

const dbPath = path.join(__dirname, 'Sheffield1867.db');
const db = new Database(dbPath);

console.log('Starting ID assignment process...\n');

// First, let's check the current state
const countBefore = db.prepare('SELECT COUNT(*) as count FROM sheffield_people WHERE id IS NOT NULL').get();
console.log(`People with IDs before: ${countBefore.count}`);

const totalPeople = db.prepare('SELECT COUNT(*) as count FROM sheffield_people').get();
console.log(`Total people in table: ${totalPeople.count}\n`);

// Start a transaction for safety
db.exec('BEGIN TRANSACTION');

try {
  console.log('Assigning sequential IDs to all people...');

  // Create a temporary table with ROWIDs and assign sequential IDs
  const result = db.prepare(`
    UPDATE sheffield_people
    SET id = (
      SELECT COUNT(*)
      FROM sheffield_people AS p2
      WHERE p2.ROWID <= sheffield_people.ROWID
    )
  `).run();

  console.log(`Updated ${result.changes} rows\n`);

  // Verify the assignment
  const countAfter = db.prepare('SELECT COUNT(*) as count FROM sheffield_people WHERE id IS NOT NULL').get();
  const minId = db.prepare('SELECT MIN(id) as min FROM sheffield_people').get();
  const maxId = db.prepare('SELECT MAX(id) as max FROM sheffield_people').get();
  const uniqueIds = db.prepare('SELECT COUNT(DISTINCT id) as count FROM sheffield_people').get();

  console.log('=== VERIFICATION ===');
  console.log(`People with IDs after: ${countAfter.count}`);
  console.log(`ID range: ${minId.min} to ${maxId.max}`);
  console.log(`Unique IDs: ${uniqueIds.count}`);
  console.log(`Total people: ${totalPeople.count}`);

  if (uniqueIds.count === totalPeople.count && countAfter.count === totalPeople.count) {
    console.log('\n✓ Success! All people have unique IDs.');
    db.exec('COMMIT');

    // Show some examples
    console.log('\n=== SAMPLE DATA ===');
    const samples = db.prepare('SELECT id, name, first_name, surname, census_age FROM sheffield_people LIMIT 10').all();
    console.table(samples);
  } else {
    console.log('\n✗ Error: ID assignment verification failed!');
    console.log('Rolling back changes...');
    db.exec('ROLLBACK');
  }

} catch (error) {
  console.error('Error during ID assignment:', error.message);
  console.log('Rolling back changes...');
  db.exec('ROLLBACK');
}

db.close();
console.log('\nDone!');
