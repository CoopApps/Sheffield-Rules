const sqlite3 = require('better-sqlite3');
const db = new sqlite3('./sheffield1867.db');

console.log('=== CLEARING TABLES ===\n');

// Check current counts
const playerCount = db.prepare('SELECT COUNT(*) as count FROM sheffield_players').get();
const peopleCount = db.prepare('SELECT COUNT(*) as count FROM sheffield_people').get();

console.log('Current counts:');
console.log(`  sheffield_players: ${playerCount.count}`);
console.log(`  sheffield_people: ${peopleCount.count}`);
console.log();

// Confirm before deletion
console.log('Clearing tables...');

// Disable foreign key constraints temporarily
db.pragma('foreign_keys = OFF');

// Delete all records
db.prepare('DELETE FROM sheffield_people').run();
db.prepare('DELETE FROM sheffield_players').run();

// Re-enable foreign key constraints
db.pragma('foreign_keys = ON');

console.log('Tables cleared!\n');

// Verify
const newPlayerCount = db.prepare('SELECT COUNT(*) as count FROM sheffield_players').get();
const newPeopleCount = db.prepare('SELECT COUNT(*) as count FROM sheffield_people').get();

console.log('New counts:');
console.log(`  sheffield_players: ${newPlayerCount.count}`);
console.log(`  sheffield_people: ${newPeopleCount.count}`);

db.close();
console.log('\nDone!');
