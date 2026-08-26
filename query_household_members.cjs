const Database = require('better-sqlite3');
const db = new Database('./Sheffield1867.db');

console.log('\n=== Checking sheffield_players table schema ===');
const schema = db.prepare("SELECT sql FROM sqlite_master WHERE type='table' AND name='sheffield_players'").get();
console.log(schema.sql);

console.log('\n=== All table names ===');
const tables = db.prepare("SELECT name FROM sqlite_master WHERE type='table'").all();
tables.forEach(t => console.log(t.name));

console.log('\n=== Finding players with household data ===');
const playersWithHousehold = db.prepare(`
  SELECT id, name, surname, birth_year, ecclesiastical_parish, census_household_schedule
  FROM sheffield_players
  WHERE census_household_schedule IS NOT NULL AND census_household_schedule != ''
  LIMIT 20
`).all();

console.log(`\nFound ${playersWithHousehold.length} players with household data:`);
playersWithHousehold.forEach((player, i) => {
  console.log(`\n${i + 1}. ${player.name} ${player.surname} (b. ${player.birth_year})`);
  console.log(`   ID: ${player.id}`);
  console.log(`   Parish: ${player.ecclesiastical_parish || 'N/A'}`);
  console.log(`   Household: ${player.census_household_schedule ? player.census_household_schedule.substring(0, 100) + '...' : 'N/A'}`);
});

db.close();
