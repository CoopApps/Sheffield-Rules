const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('\n========================================');
console.log('ADDING CENSUS COLUMNS TO SHEFFIELD_PEOPLE');
console.log('========================================\n');

const columns = [
    'ecclesiastical_parish',
    'ed_institution_vessel',
    'estimated_birth_year',
    'folio',
    'gender',
    'household_members',
    'household_schedule_number',
    'page_number',
    'piece',
    'registration_district'
];

let added = 0;
let existing = 0;

columns.forEach(col => {
    try {
        db.exec(`ALTER TABLE sheffield_people ADD COLUMN ${col} TEXT`);
        console.log('✓ Added: ' + col);
        added++;
    } catch(e) {
        if (e.message.includes('duplicate column')) {
            console.log('- Already exists: ' + col);
            existing++;
        } else {
            console.log('✗ Error adding ' + col + ': ' + e.message);
        }
    }
});

console.log('\n========================================');
console.log('COMPLETE');
console.log('========================================');
console.log('  Added: ' + added);
console.log('  Already existed: ' + existing);
console.log('  Total columns: ' + columns.length);
console.log('========================================\n');

console.log('Current sheffield_people columns:');
const tableInfo = db.prepare('PRAGMA table_info(sheffield_people)').all();
tableInfo.forEach(col => {
    console.log('  ' + col.name + ' (' + col.type + ')');
});

db.close();
