const Database = require('better-sqlite3');
const fs = require('fs');
const db = new Database('./Sheffield1867.db');

console.log('\n=== Elizabeth Brook Analysis ===\n');

// Get Elizabeth Brook from database
const elizabeth = db.prepare(`
  SELECT *
  FROM sheffield_players
  WHERE first_name = 'Elizabeth' AND surname = 'Brook'
`).get();

if (elizabeth) {
  console.log('DATABASE RECORD:');
  console.log(`  Name: ${elizabeth.first_name} ${elizabeth.surname}`);
  console.log(`  Birth Year: ${elizabeth.birth_year}`);
  console.log(`  Relation: ${elizabeth.census_relation}`);
  console.log(`  Gender: ${elizabeth.census_gender}`);
  console.log(`  Household Schedule: ${elizabeth.census_household_schedule}`);
  console.log(`  Parish: ${elizabeth.ecclesiastical_parish}`);
  console.log(`  Birth Place: ${elizabeth.where_born || 'N/A'}`);

  // Get her household members
  console.log('\n  Household Members:');
  const household = db.prepare(`
    SELECT name, first_name, surname, birth_year, census_relation, census_gender
    FROM sheffield_players
    WHERE census_household_schedule = ?
      AND (ecclesiastical_parish = ? OR (ecclesiastical_parish IS NULL AND ? IS NULL))
    ORDER BY
      CASE census_relation
        WHEN 'Head' THEN 1
        WHEN 'Wife' THEN 2
        WHEN 'Son' THEN 3
        WHEN 'Daughter' THEN 4
        ELSE 5
      END,
      birth_year
  `).all(elizabeth.census_household_schedule, elizabeth.ecclesiastical_parish, elizabeth.ecclesiastical_parish);

  household.forEach((m, i) => {
    const name = `${m.first_name || m.name} ${m.surname || ''}`.trim();
    console.log(`    ${i + 1}. ${name} (b. ${m.birth_year || 'N/A'}), ${m.census_relation}, ${m.census_gender}`);
  });
} else {
  console.log('Elizabeth Brook not found in database.');
}

// Now search CSV files for Elizabeth Brook
console.log('\n\nGENEALOGY CSV MATCHES:');
console.log('Searching for "Elizabeth Brook" in genealogy/tg_1831_E.csv...\n');

const csvPath = './genealogy/tg_1831_E.csv';
if (fs.existsSync(csvPath)) {
  const csvContent = fs.readFileSync(csvPath, 'utf8');
  const lines = csvContent.split('\n');

  const matches = lines.filter(line =>
    line.toLowerCase().includes('elizabeth') &&
    (line.toLowerCase().includes('brook') || line.toLowerCase().includes('brooke'))
  );

  if (matches.length > 0) {
    console.log(`Found ${matches.length} match(es):\n`);
    matches.forEach((match, i) => {
      console.log(`  ${i + 1}. ${match}`);
    });
  } else {
    console.log('  No matches found in this file.');
  }
} else {
  console.log('  CSV file not found.');
}

// Let's also check 1830 and 1832 in case of +/-1 year error
console.log('\n\nSearching adjacent years (1830, 1832) for Elizabeth Brook...\n');

['1830', '1832'].forEach(year => {
  const path = `./genealogy/tg_${year}_E.csv`;
  if (fs.existsSync(path)) {
    const content = fs.readFileSync(path, 'utf8');
    const lines = content.split('\n');
    const matches = lines.filter(line =>
      line.toLowerCase().includes('elizabeth') &&
      (line.toLowerCase().includes('brook') || line.toLowerCase().includes('brooke'))
    );

    if (matches.length > 0) {
      console.log(`Year ${year}:`);
      matches.forEach(match => {
        console.log(`  ${match}`);
      });
      console.log();
    }
  }
});

db.close();
