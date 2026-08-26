const sqlite3 = require('sqlite3').verbose();
const dbPath = 'D:/projects/Saturday at Three/Sheffield1867.db';

const db = new sqlite3.Database(dbPath, (err) => {
  if (err) {
    console.error('Error:', err);
    process.exit(1);
  }
  console.log('Connected to database\n');
});

// Add name component fields
const statements = [
  "ALTER TABLE sheffield_players ADD COLUMN first_name TEXT",
  "ALTER TABLE sheffield_players ADD COLUMN middle_name TEXT",
  "ALTER TABLE sheffield_players ADD COLUMN surname TEXT"
];

let completed = 0;

console.log('Adding name fields...\n');

statements.forEach((sql, index) => {
  db.run(sql, (err) => {
    if (err && !err.message.includes('duplicate column')) {
      console.error(`✗ Error adding column ${index + 1}:`, err.message);
    } else if (err) {
      console.log(`✓ Column ${index + 1}: Already exists (skipped)`);
    } else {
      console.log(`✓ Column ${index + 1}: Added successfully`);
    }

    completed++;
    if (completed === statements.length) {
      // Now populate the fields from existing name data
      console.log('\nPopulating name fields from existing data...\n');

      db.all("SELECT id, name FROM sheffield_players WHERE first_name IS NULL", (err, rows) => {
        if (err) {
          console.error('✗ Error reading players:', err.message);
          db.close();
          return;
        }

        let updated = 0;
        if (rows.length === 0) {
          console.log('✓ All players already have name fields populated');
          db.close();
          return;
        }

        rows.forEach(row => {
          const name = row.name || '';
          const parts = name.trim().split(/\s+/);

          let firstName = '';
          let middleName = '';
          let surname = '';

          if (parts.length === 1) {
            firstName = parts[0];
          } else if (parts.length === 2) {
            firstName = parts[0];
            surname = parts[1];
          } else if (parts.length >= 3) {
            firstName = parts[0];
            surname = parts[parts.length - 1];
            middleName = parts.slice(1, -1).join(' ');
          }

          db.run(
            "UPDATE sheffield_players SET first_name = ?, middle_name = ?, surname = ? WHERE id = ?",
            [firstName, middleName, surname, row.id],
            (err) => {
              if (err) {
                console.error(`✗ Error updating ${row.name}:`, err.message);
              } else {
                console.log(`✓ Updated: ${row.name} → [${firstName}] [${middleName}] [${surname}]`);
              }

              updated++;
              if (updated === rows.length) {
                console.log(`\n✓ Successfully updated ${updated} players`);

                // Verify
                db.all("PRAGMA table_info(sheffield_players)", (err, cols) => {
                  if (!err) {
                    const columns = cols.map(c => c.name);
                    console.log('\n✓ first_name present:', columns.includes('first_name'));
                    console.log('✓ middle_name present:', columns.includes('middle_name'));
                    console.log('✓ surname present:', columns.includes('surname'));
                  }
                  db.close();
                });
              }
            }
          );
        });
      });
    }
  });
});
