const Database = require('better-sqlite3');
const db = new Database('./Sheffield1867.db');

console.log('\n=== SHEFFIELD_PLAYERS TABLE SCHEMA ===\n');
const schema = db.prepare("PRAGMA table_info(sheffield_players)").all();
schema.forEach(col => {
  console.log(`${col.cid}. ${col.name.padEnd(30)} ${col.type.padEnd(15)} ${col.notnull ? 'NOT NULL' : ''} ${col.pk ? 'PRIMARY KEY' : ''}`);
});

console.log('\n=== SEARCHING FOR HOUSEHOLD-RELATED COLUMNS ===\n');
const householdCols = schema.filter(col =>
  col.name.toLowerCase().includes('household') ||
  col.name.toLowerCase().includes('family') ||
  col.name.toLowerCase().includes('member')
);

if (householdCols.length > 0) {
  console.log('Found household-related columns:');
  householdCols.forEach(col => {
    console.log(`  - ${col.name} (${col.type})`);
  });
} else {
  console.log('No household-related columns found.');
}

console.log('\n=== SAMPLE DATA FROM CENSUS_HOUSEHOLD_SCHEDULE ===\n');
const samplePlayers = db.prepare(`
  SELECT
    id,
    first_name,
    surname,
    birth_year,
    census_relation,
    census_household_schedule,
    ecclesiastical_parish
  FROM sheffield_players
  WHERE census_household_schedule IS NOT NULL
    AND census_household_schedule != ''
  LIMIT 5
`).all();

console.log('Sample players with household schedule numbers:\n');
samplePlayers.forEach((player, i) => {
  console.log(`${i + 1}. ${player.first_name} ${player.surname} (b. ${player.birth_year})`);
  console.log(`   ID: ${player.id}`);
  console.log(`   Household Schedule: ${player.census_household_schedule}`);
  console.log(`   Parish: ${player.ecclesiastical_parish}`);
  console.log(`   Relation: ${player.census_relation}`);
  console.log('');
});

console.log('\n=== CHECKING FOR ACTUAL HOUSEHOLD MEMBERS DATA ===\n');

// Check if there's a column with actual household member names/data
const checkColumns = ['household_members', 'household_data', 'family_members', 'census_household_members'];
let foundDataColumn = null;

for (const colName of checkColumns) {
  const exists = schema.find(col => col.name.toLowerCase() === colName.toLowerCase());
  if (exists) {
    foundDataColumn = exists.name;
    console.log(`✓ Found column: ${foundDataColumn}`);

    // Sample data from this column
    const sampleData = db.prepare(`
      SELECT id, first_name, surname, ${foundDataColumn}
      FROM sheffield_players
      WHERE ${foundDataColumn} IS NOT NULL AND ${foundDataColumn} != ''
      LIMIT 3
    `).all();

    console.log(`\nSample data from ${foundDataColumn}:\n`);
    sampleData.forEach((row, i) => {
      console.log(`${i + 1}. ${row.first_name} ${row.surname}`);
      console.log(`   ${foundDataColumn}: ${row[foundDataColumn].substring(0, 200)}...`);
      console.log('');
    });
    break;
  }
}

if (!foundDataColumn) {
  console.log('⚠ No column found with actual household member data.');
  console.log('\nHowever, the census CSV files contain a "HOUSEHOLD MEMBERS" column.');
  console.log('Looking at BULK_CENSUS_IMPORT.md, the census CSV format includes:');
  console.log('  Column 12: "HOUSEHOLD MEMBERS" - Contains: "Name Age | Mary Martin 68 | ..."');
  console.log('\nThis data appears to be stored in the CSV but may not be imported to the database.');
  console.log('\nTo get household members, you need to:');
  console.log('  1. Query all players with the same census_household_schedule + ecclesiastical_parish');
  console.log('  2. This gives you all family members in that household');
}

console.log('\n=== EXAMPLE: Getting Household Members for First Player ===\n');
if (samplePlayers.length > 0) {
  const firstPlayer = samplePlayers[0];
  console.log(`Finding household members for: ${firstPlayer.first_name} ${firstPlayer.surname}`);
  console.log(`Household Schedule: ${firstPlayer.census_household_schedule}`);
  console.log(`Parish: ${firstPlayer.ecclesiastical_parish}\n`);

  const household = db.prepare(`
    SELECT
      first_name,
      surname,
      birth_year,
      census_age,
      census_relation,
      census_gender
    FROM sheffield_players
    WHERE census_household_schedule = ?
      AND ecclesiastical_parish = ?
    ORDER BY
      CASE census_relation
        WHEN 'Head' THEN 1
        WHEN 'Wife' THEN 2
        WHEN 'Son' THEN 3
        WHEN 'Daughter' THEN 4
        ELSE 5
      END,
      census_age DESC
  `).all(firstPlayer.census_household_schedule, firstPlayer.ecclesiastical_parish);

  console.log(`Household Members (${household.length} total):\n`);
  household.forEach((member, i) => {
    console.log(`  ${i + 1}. ${member.first_name || ''} ${member.surname || ''} (b. ${member.birth_year || 'N/A'}, age ${member.census_age})`);
    console.log(`     ${member.census_relation}, ${member.census_gender}`);
  });
}

db.close();
