const Database = require('better-sqlite3');
const fs = require('fs');
const db = new Database('./Sheffield1867.db');

console.log('\n=== Finding male players with distinctive surnames (1827-1838) ===\n');

// Find players with less common surnames in the target range
const players = db.prepare(`
  SELECT p.id, p.first_name, p.surname, p.birth_year,
         p.census_household_schedule, p.ecclesiastical_parish,
         p.census_relation, p.where_born
  FROM sheffield_players p
  WHERE p.birth_year >= 1827
    AND p.birth_year <= 1838
    AND p.census_gender = 'Male'
    AND p.census_household_schedule IS NOT NULL
    AND p.surname NOT IN ('Smith', 'Jones', 'Brown', 'Wilson', 'Taylor', 'Johnson', 'Williams')
  ORDER BY p.surname, p.birth_year
  LIMIT 30
`).all();

console.log(`Found ${players.length} players with distinctive surnames:\n`);

// Display first 10
players.slice(0, 10).forEach((p, i) => {
  console.log(`${i + 1}. ${p.first_name} ${p.surname} (b. ${p.birth_year})`);
  console.log(`   Household: ${p.census_household_schedule}, Parish: ${p.ecclesiastical_parish}`);
  console.log(`   Birth Place: ${p.where_born || 'N/A'}`);

  // Check if surname exists in genealogy CSV for that year
  const csvFile = `./genealogy/tg_${p.birth_year}_${p.surname.charAt(0).toUpperCase()}.csv`;
  if (fs.existsSync(csvFile)) {
    const content = fs.readFileSync(csvFile, 'utf8');
    const surnamePattern = new RegExp(p.surname, 'i');
    const matches = content.split('\n').filter(line => surnamePattern.test(line));

    if (matches.length > 0) {
      console.log(`   *** Found in CSV: ${matches.length} match(es)`);
      // Show first match
      if (matches[0]) {
        const fields = matches[0].split(',');
        console.log(`       First match: ${fields[0]?.replace(/"/g, '')} (${fields[6]?.replace(/"/g, '')})`);
      }
    }
  }
  console.log();
});

db.close();
