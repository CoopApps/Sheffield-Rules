// Script to populate Sheffield clubs 1857-1875
const { exec } = require('child_process');

console.log('\n=== Populating Sheffield Clubs 1857-1875 ===\n');

const dbPath = 'D:/projects/Saturday at Three/Sheffield1867.db';
const sqlFile = 'populate_clubs_1857_1875.sql';

// Try to find sqlite3
exec('where sqlite3', (err, stdout) => {
    if (err || !stdout.trim()) {
        console.log('Note: sqlite3 not found in PATH');
        console.log(`\nGenerated SQL file: ${sqlFile}`);
        console.log(`\nTo populate, run: sqlite3 "${dbPath}" < ${sqlFile}\n`);
        console.log(`Or use: type ${sqlFile} | sqlite3 "${dbPath}"\n`);
    } else {
        console.log('Found sqlite3, executing SQL...\n');
        exec(`type ${sqlFile} | sqlite3 "${dbPath}"`, (err, stdout, stderr) => {
            if (err) {
                console.error('Error executing SQL:', err);
                console.log('\nTry manually: type ' + sqlFile + ' | sqlite3 "' + dbPath + '"\n');
            } else {
                console.log('✓ Clubs populated successfully!');
                if (stdout) console.log(stdout);
                if (stderr) console.error(stderr);

                // Verify the data
                console.log('\nVerifying clubs...\n');
                exec(`echo "SELECT COUNT(*) as total_clubs FROM sheffield_clubs;" | sqlite3 "${dbPath}"`, (err, stdout) => {
                    if (!err && stdout) {
                        console.log('Total clubs in database:', stdout.trim());
                    }
                });
            }
        });
    }
});
