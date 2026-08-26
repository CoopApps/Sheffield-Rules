const sqlite3 = require('better-sqlite3');
const db = new sqlite3('./sheffield1867.db');

console.log('=== ADDING MISSING COLUMNS TO sheffield_people ===\n');

// Add columns for genealogy data and household tracking
const columns = [
  { name: 'spouse_name', type: 'TEXT', description: 'Name of spouse (from genealogy)' },
  { name: 'spouse_id', type: 'TEXT', description: 'Foreign key to spouse person record' },
  { name: 'household_id', type: 'TEXT', description: 'Groups family members together' },
  { name: 'relation', type: 'TEXT', description: 'Head, Wife, Son, Daughter, Lodger, etc.' },
  { name: 'parish_area', type: 'TEXT', description: 'Area/sub-district from genealogy' },
  { name: 'source', type: 'TEXT', description: 'census, genealogy, or both' }
];

columns.forEach(col => {
  try {
    const sql = `ALTER TABLE sheffield_people ADD COLUMN ${col.name} ${col.type}`;
    db.prepare(sql).run();
    console.log(`✓ Added column: ${col.name} (${col.type}) - ${col.description}`);
  } catch (err) {
    if (err.message.includes('duplicate column name')) {
      console.log(`  Column ${col.name} already exists, skipping`);
    } else {
      console.log(`✗ Error adding ${col.name}: ${err.message}`);
    }
  }
});

console.log('\n=== UPDATED SCHEMA ===');
const schema = db.prepare("SELECT sql FROM sqlite_master WHERE name='sheffield_people'").get();
console.log(schema.sql);

db.close();
console.log('\nDone!');
