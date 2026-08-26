const Database = require('better-sqlite3');
const fs = require('fs');
const db = new Database('./Sheffield1867.db');

console.log('\n=== Henry Abel Household Analysis ===\n');

// Get Henry Abel
const henry = db.prepare(`
  SELECT *
  FROM sheffield_players
  WHERE first_name = 'Henry' AND surname = 'Abel' AND birth_year = 1827
`).get();

if (henry) {
  console.log('DATABASE RECORD - PLAYER:');
  console.log(`  Name: ${henry.first_name} ${henry.surname}`);
  console.log(`  Birth Year: ${henry.birth_year}`);
  console.log(`  Relation: ${henry.census_relation}`);
  console.log(`  Gender: ${henry.census_gender}`);
  console.log(`  Household Schedule: ${henry.census_household_schedule}`);
  console.log(`  Parish: ${henry.ecclesiastical_parish}`);
  console.log(`  Birth Place: ${henry.where_born || 'N/A'}`);
  console.log(`  Age (Census): ${henry.census_age}`);

  // Get his household members
  console.log('\n  Household Members:');
  const household = db.prepare(`
    SELECT name, first_name, surname, birth_year, census_relation, census_gender, census_age
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
  `).all(henry.census_household_schedule, henry.ecclesiastical_parish);

  household.forEach((m, i) => {
    const name = `${m.first_name || m.name} ${m.surname || ''}`.trim();
    const inRange = (m.birth_year >= 1827 && m.birth_year <= 1838) ? '***' : '';
    console.log(`    ${i + 1}. ${name} (b. ${m.birth_year || 'N/A'}, age ${m.census_age}), ${m.census_relation}, ${m.census_gender} ${inRange}`);
  });

  // Search for Abel in the genealogy CSV
  console.log('\n\nGENEALOGY CSV SEARCH:');
  console.log('Searching for "Abel" in genealogy files...\n');

  // Search in 1827 file (Henry's birth year)
  const csvFile1827 = './genealogy/tg_1827_A (1).csv';
  if (fs.existsSync(csvFile1827)) {
    const content = fs.readFileSync(csvFile1827, 'utf8');
    const lines = content.split('\n');
    const matches = lines.filter(line => /\bAbel\b/i.test(line));

    if (matches.length > 0) {
      console.log(`Year 1827 (Henry's birth year) - Found ${matches.length} Abel match(es):`);
      matches.forEach((match, i) => {
        console.log(`  ${i + 1}. ${match}`);
      });
      console.log();
    }
  }

  // Check if any household members are in the genealogy range
  const membersInRange = household.filter(m => m.birth_year >= 1827 && m.birth_year <= 1838);
  if (membersInRange.length > 1) { // More than just Henry himself
    console.log(`\n${membersInRange.length} household member(s) in genealogy year range (1827-1838):`);
    membersInRange.forEach(m => {
      if (m.census_gender === 'Female' || m.census_relation === 'Wife' || m.census_relation === 'Daughter') {
        console.log(`\n  Searching for: ${m.first_name || m.name} ${m.surname || 'Abel'} (b. ${m.birth_year})`);

        const surname = m.surname || 'Abel';
        const csvFile = `./genealogy/tg_${m.birth_year}_${surname.charAt(0).toUpperCase()}.csv`;

        if (fs.existsSync(csvFile)) {
          const content = fs.readFileSync(csvFile, 'utf8');
          const firstName = m.first_name || m.name.split(' ')[0];
          const pattern = new RegExp(`${firstName}.*${surname}|${surname}.*${firstName}`, 'i');
          const matches = content.split('\n').filter(line => pattern.test(line));

          if (matches.length > 0) {
            console.log(`    *** FOUND in ${csvFile}:`);
            matches.forEach(match => {
              console.log(`        ${match}`);
            });
          } else {
            console.log(`    Not found in ${csvFile}`);
          }
        } else {
          console.log(`    CSV file ${csvFile} does not exist`);
        }
      }
    });
  }
} else {
  console.log('Henry Abel not found in database.');
}

db.close();
