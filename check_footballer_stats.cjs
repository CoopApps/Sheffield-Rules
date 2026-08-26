const sqlite3 = require('sqlite3').verbose();

const db = new sqlite3.Database('D:/projects/Saturday at Three/Sheffield1867.db', (err) => {
  if (err) {
    console.error('Error opening database:', err);
    process.exit(1);
  }
});

// Check total footballers and those with stats
db.get(`
  SELECT
    COUNT(*) as total_footballers,
    COUNT(pace) as players_with_pace,
    COUNT(position) as players_with_position,
    COUNT(current_ability) as players_with_ca
  FROM sheffield_footballers
`, (err, row) => {
  if (err) {
    console.error('Error querying database:', err);
    db.close();
    process.exit(1);
  }

  console.log('========================================');
  console.log('SHEFFIELD FOOTBALLERS STATS CHECK');
  console.log('========================================');
  console.log(`Total footballers: ${row.total_footballers}`);
  console.log(`Players with pace stat: ${row.players_with_pace}`);
  console.log(`Players with position: ${row.players_with_position}`);
  console.log(`Players with current_ability: ${row.players_with_ca}`);
  console.log('========================================');

  // Show a sample player with stats
  db.get(`
    SELECT first_name, surname, position, pace, acceleration, current_ability
    FROM sheffield_footballers
    WHERE pace IS NOT NULL
    LIMIT 1
  `, (err, player) => {
    if (err) {
      console.error('Error querying sample player:', err);
    } else if (player) {
      console.log('\nSample player with stats:');
      console.log(`${player.first_name} ${player.surname}`);
      console.log(`Position: ${player.position}`);
      console.log(`Pace: ${player.pace}, Acceleration: ${player.acceleration}`);
      console.log(`Current Ability: ${player.current_ability}`);
    } else {
      console.log('\nNo players found with stats.');
    }

    db.close();
  });
});
