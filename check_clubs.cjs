const Database = require('better-sqlite3');
const db = new Database('./Sheffield1867.db');

console.log('\n=== Checking 105th Regiment Reserves ===');
const regiment = db.prepare('SELECT * FROM sheffield_clubs WHERE id LIKE ?').all('%105th-regiment%');
console.log('Clubs table:', regiment);

const regimentLeague = db.prepare('SELECT * FROM sheffield_league_clubs WHERE club_id LIKE ?').all('%105th-regiment%');
console.log('League clubs table:', regimentLeague);

console.log('\n=== Checking Albion FC Reserves ===');
const albion = db.prepare('SELECT * FROM sheffield_clubs WHERE id LIKE ?').all('%albion-fc%');
console.log('Clubs table:', albion);

const albionLeague = db.prepare('SELECT * FROM sheffield_league_clubs WHERE club_id LIKE ?').all('%albion-fc%');
console.log('League clubs table:', albionLeague);

console.log('\n=== Total clubs in sheffield_clubs ===');
const totalClubs = db.prepare('SELECT COUNT(*) as count FROM sheffield_clubs').get();
console.log(totalClubs);

console.log('\n=== Total assignments in sheffield_league_clubs ===');
const totalAssignments = db.prepare('SELECT COUNT(*) as count FROM sheffield_league_clubs').get();
console.log(totalAssignments);

console.log('\n=== Reserve teams in league ===');
const reserveTeams = db.prepare('SELECT club_id, division_id, is_reserve_team FROM sheffield_league_clubs WHERE is_reserve_team = 1').all();
console.log(`Found ${reserveTeams.length} reserve teams`);
reserveTeams.slice(0, 5).forEach(team => console.log(team));

db.close();
