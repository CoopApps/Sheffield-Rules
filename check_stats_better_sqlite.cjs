const Database = require('better-sqlite3');

const db = new Database('D:/projects/Saturday at Three/Sheffield1867.db', { readonly: true });

// Check total footballers and those with stats
const stats = db.prepare(`
  SELECT
    COUNT(*) as total_footballers,
    COUNT(pace) as players_with_pace,
    COUNT(position) as players_with_position,
    COUNT(current_ability) as players_with_ca
  FROM sheffield_footballers
`).get();

console.log('========================================');
console.log('SHEFFIELD FOOTBALLERS STATS CHECK');
console.log('========================================');
console.log(`Total footballers: ${stats.total_footballers}`);
console.log(`Players with pace stat: ${stats.players_with_pace}`);
console.log(`Players with position: ${stats.players_with_position}`);
console.log(`Players with current_ability: ${stats.players_with_ca}`);
console.log('========================================');

// Show a sample player with stats
const player = db.prepare(`
  SELECT first_name, surname, position, pace, acceleration, current_ability, person_id
  FROM sheffield_footballers
  WHERE pace IS NOT NULL
  LIMIT 1
`).get();

if (player) {
  console.log('\nSample player with stats:');
  console.log(`${player.first_name} ${player.surname} (person_id: ${player.person_id})`);
  console.log(`Position: ${player.position}`);
  console.log(`Pace: ${player.pace}, Acceleration: ${player.acceleration}`);
  console.log(`Current Ability: ${player.current_ability}`);
} else {
  console.log('\nNo players found with stats.');
}

db.close();
