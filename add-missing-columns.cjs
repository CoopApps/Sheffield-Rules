const sqlite3 = require('sqlite3').verbose();
const dbPath = 'D:/projects/Saturday at Three/Sheffield1867.db';

const db = new sqlite3.Database(dbPath, (err) => {
  if (err) {
    console.error('Error:', err);
    process.exit(1);
  }
  console.log('Connected to database\n');
});

// Add missing columns
const statements = [
  "ALTER TABLE sheffield_players ADD COLUMN where_born TEXT",
  "ALTER TABLE sheffield_players ADD COLUMN has_stats BOOLEAN DEFAULT 0"
];

let completed = 0;

statements.forEach((sql, index) => {
  db.run(sql, (err) => {
    if (err && !err.message.includes('duplicate column')) {
      console.error(`✗ Error:`, err.message);
    } else if (err) {
      console.log(`✓ Column ${index + 1}: Already exists (skipped)`);
    } else {
      console.log(`✓ Column ${index + 1}: Added successfully`);
    }

    completed++;
    if (completed === statements.length) {
      // Verify
      db.all("PRAGMA table_info(sheffield_players)", (err, rows) => {
        if (!err) {
          const columns = rows.map(r => r.name);
          console.log('\n✓ where_born present:', columns.includes('where_born'));
          console.log('✓ has_stats present:', columns.includes('has_stats'));
        }
        db.close();
      });
    }
  });
});
