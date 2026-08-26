const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('Fast profession linking...\n');

// Disable foreign keys
db.exec('PRAGMA foreign_keys = OFF');

// Step 1: Load all people into a Map for fast lookup
console.log('Loading all people into memory...');
const peopleByName = new Map();
const people = db.prepare('SELECT id, name FROM sheffield_people').all();
for (const person of people) {
  if (person.name) {
    peopleByName.set(person.name, person.id);
  }
}
console.log(`✓ Loaded ${peopleByName.size} people\n`);

// Tables to process
const tables = [
  'sheffield_employers',
  'sheffield_professionals',
  'sheffield_tradesmen',
  'sheffield_publicans',
  'sheffield_lodgers',
  'sheffield_german',
  'sheffield_irish',
  'sheffield_scottish',
  'sheffield_welsh'
];

// Process each table
for (const table of tables) {
  console.log(`Processing ${table}...`);

  // Get all rows from this table
  const rows = db.prepare(`SELECT ROWID, name FROM ${table}`).all();

  // Update each row with matching person_id
  const updateStmt = db.prepare(`UPDATE ${table} SET sheffield_person_id = ? WHERE ROWID = ?`);

  let matched = 0;
  const updateMany = db.transaction((rows) => {
    for (const row of rows) {
      if (row.name && peopleByName.has(row.name)) {
        const personId = peopleByName.get(row.name);
        updateStmt.run(personId, row.ROWID);
        matched++;
      }
    }
  });

  updateMany(rows);

  console.log(`  ✓ Matched ${matched}/${rows.length} records\n`);
}

console.log('=== SUMMARY ===');
for (const table of tables) {
  const result = db.prepare(`SELECT COUNT(*) as matched FROM ${table} WHERE sheffield_person_id IS NOT NULL`).get();
  const total = db.prepare(`SELECT COUNT(*) as total FROM ${table}`).get();
  console.log(`${table}: ${result.matched}/${total.total} linked`);
}

db.close();
console.log('\nDone!');
