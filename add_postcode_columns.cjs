const sqlite3 = require('better-sqlite3');

const db = new sqlite3('Sheffield1867.db', { timeout: 30000 });

console.log('Adding postcode columns to database...\n');

// Add postcode column to sheffield_people
try {
    db.prepare('ALTER TABLE sheffield_people ADD COLUMN postcode TEXT').run();
    console.log('✓ Added postcode column to sheffield_people');
} catch (error) {
    if (error.message.includes('duplicate column name')) {
        console.log('○ postcode column already exists in sheffield_people');
    } else {
        console.error('✗ Error adding postcode to sheffield_people:', error.message);
    }
}

// Add postcode column to sheffield_players
try {
    db.prepare('ALTER TABLE sheffield_players ADD COLUMN postcode TEXT').run();
    console.log('✓ Added postcode column to sheffield_players');
} catch (error) {
    if (error.message.includes('duplicate column name')) {
        console.log('○ postcode column already exists in sheffield_players');
    } else {
        console.error('✗ Error adding postcode to sheffield_players:', error.message);
    }
}

// Create index for faster postcode lookups
db.exec('CREATE INDEX IF NOT EXISTS idx_people_postcode ON sheffield_people(postcode)');
db.exec('CREATE INDEX IF NOT EXISTS idx_players_postcode ON sheffield_players(postcode)');
console.log('✓ Created postcode indexes');

console.log('\n✓ Database schema updated!');

db.close();
