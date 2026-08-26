// Simple script to run database migration
const sqlite3 = require('sqlite3').verbose();
const fs = require('fs');
const path = require('path');

// Database path
const dbPath = 'D:/projects/Saturday at Three/Sheffield1867.db';

// Read migration file
const migrationPath = path.join(__dirname, 'src-tauri/src/database/migrations/add_player_geographic_fields.sql');
const sql = fs.readFileSync(migrationPath, 'utf8');

// Connect to database
const db = new sqlite3.Database(dbPath, (err) => {
  if (err) {
    console.error('Error opening database:', err);
    process.exit(1);
  }
  console.log('Connected to database:', dbPath);
});

// Split SQL into individual statements (by semicolon)
const statements = sql
  .split(';')
  .map(s => s.trim())
  .filter(s => s.length > 0 && !s.startsWith('--'));

console.log(`\nRunning ${statements.length} migration statements...\n`);

// Run each statement
let completed = 0;
let errors = 0;

statements.forEach((statement, index) => {
  db.run(statement, (err) => {
    completed++;

    if (err) {
      // Ignore "duplicate column" errors (column already exists)
      if (err.message.includes('duplicate column')) {
        console.log(`✓ Statement ${index + 1}: Column already exists (skipped)`);
      } else {
        console.error(`✗ Statement ${index + 1} ERROR:`, err.message);
        errors++;
      }
    } else {
      console.log(`✓ Statement ${index + 1}: Success`);
    }

    // If all statements processed
    if (completed === statements.length) {
      console.log(`\n${'='.repeat(50)}`);
      console.log(`Migration complete!`);
      console.log(`Success: ${completed - errors} | Errors: ${errors}`);
      console.log(`${'='.repeat(50)}\n`);

      // Verify columns
      db.all("PRAGMA table_info(sheffield_players)", (err, rows) => {
        if (!err) {
          const columnNames = rows.map(r => r.name);
          console.log('\nColumns in sheffield_players table:');
          console.log(columnNames.join(', '));

          // Check for our new columns
          const newColumns = ['where_born', 'birth_town', 'birth_county', 'has_stats'];
          const missing = newColumns.filter(col => !columnNames.includes(col));

          if (missing.length === 0) {
            console.log('\n✓ All required columns present!');
          } else {
            console.log('\n✗ Missing columns:', missing.join(', '));
          }
        }

        db.close();
      });
    }
  });
});
