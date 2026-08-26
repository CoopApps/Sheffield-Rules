const sqlite3 = require('better-sqlite3');
const db = new sqlite3('Sheffield1867.db');

try {
    console.log('Adding current_ability, potential_ability, and current_reputation columns...\n');

    // Add the three new columns
    db.exec(`
        ALTER TABLE sheffield_players ADD COLUMN current_ability INTEGER DEFAULT 50;
    `);
    console.log('✓ Added current_ability column');

    db.exec(`
        ALTER TABLE sheffield_players ADD COLUMN potential_ability INTEGER DEFAULT 100;
    `);
    console.log('✓ Added potential_ability column');

    db.exec(`
        ALTER TABLE sheffield_players ADD COLUMN current_reputation INTEGER DEFAULT 10;
    `);
    console.log('✓ Added current_reputation column');

    console.log('\n✓ Migration completed successfully!');

} catch (error) {
    console.error('Error:', error.message);
} finally {
    db.close();
}
