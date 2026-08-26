const Database = require('better-sqlite3');
const db = new Database('./Sheffield1867.db');

console.log('\n=== Finding households with members born 1827-1838 (covered by genealogy CSVs) ===\n');

// Find households that have at least one person born 1827-1838
const householdsInRange = db.prepare(`
  SELECT DISTINCT p1.census_household_schedule, p1.ecclesiastical_parish
  FROM sheffield_players p1
  WHERE p1.census_household_schedule IS NOT NULL
    AND p1.census_household_schedule != ''
    AND p1.birth_year >= 1827
    AND p1.birth_year <= 1838
  LIMIT 10
`).all();

console.log(`Found ${householdsInRange.length} households with members in birth year range 1827-1838\n`);

householdsInRange.forEach((h, idx) => {
  console.log(`\n--- Household ${idx + 1}: Schedule ${h.census_household_schedule}, Parish: ${h.ecclesiastical_parish} ---`);

  const members = db.prepare(`
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
  `).all(h.census_household_schedule, h.ecclesiastical_parish);

  members.forEach((m, i) => {
    const fullName = `${m.first_name || ''} ${m.surname || m.name}`.trim();
    const inRange = (m.birth_year >= 1827 && m.birth_year <= 1838) ? '***' : '';
    console.log(`  ${i + 1}. ${fullName} (b. ${m.birth_year || 'N/A'}) ${inRange}`);
    console.log(`     Relation: ${m.census_relation || 'N/A'}, Gender: ${m.census_gender || 'N/A'}, Age: ${m.census_age || 'N/A'}`);
  });
});

// Let's specifically look for one with a distinctive surname and Wife/Daughter
console.log('\n\n=== Looking for households with Wife/Daughter in birth year range ===\n');

const familyHouseholds = db.prepare(`
  SELECT p1.census_household_schedule, p1.ecclesiastical_parish,
         p1.first_name as head_first, p1.surname as head_surname, p1.birth_year as head_birth,
         p2.first_name as wife_first, p2.surname as wife_surname, p2.birth_year as wife_birth,
         p2.census_relation as wife_relation
  FROM sheffield_players p1
  INNER JOIN sheffield_players p2
    ON p1.census_household_schedule = p2.census_household_schedule
    AND p1.ecclesiastical_parish = p2.ecclesiastical_parish
  WHERE p1.census_relation = 'Head'
    AND p2.census_relation IN ('Wife', 'Daughter')
    AND p2.birth_year >= 1827
    AND p2.birth_year <= 1838
    AND p1.census_household_schedule IS NOT NULL
    AND p1.census_household_schedule != ''
  LIMIT 5
`).all();

if (familyHouseholds.length > 0) {
  console.log(`Found ${familyHouseholds.length} households with female family members in range:\n`);
  familyHouseholds.forEach((f, i) => {
    console.log(`${i + 1}. Household ${f.census_household_schedule} (${f.ecclesiastical_parish})`);
    console.log(`   Head: ${f.head_first} ${f.head_surname} (b. ${f.head_birth})`);
    console.log(`   ${f.wife_relation}: ${f.wife_first} ${f.wife_surname} (b. ${f.wife_birth}) ***`);
  });
} else {
  console.log('No matching households found.');
}

db.close();
