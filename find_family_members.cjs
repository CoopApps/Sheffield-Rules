const Database = require('better-sqlite3');
const db = new Database('./Sheffield1867.db');

console.log('\n=== Finding players with family member relations (Wife, Daughter, etc.) ===\n');

// First, let's see what relations exist
const relations = db.prepare(`
  SELECT DISTINCT census_relation, COUNT(*) as count
  FROM sheffield_players
  WHERE census_relation IS NOT NULL AND census_relation != ''
  GROUP BY census_relation
  ORDER BY count DESC
`).all();

console.log('All census relations found:');
relations.forEach(r => {
  console.log(`  ${r.census_relation}: ${r.count}`);
});

// Now find a household with a wife or daughter
console.log('\n=== Finding households with female family members ===\n');
const householdsWithWomen = db.prepare(`
  SELECT DISTINCT census_household_schedule, ecclesiastical_parish
  FROM sheffield_players
  WHERE census_relation IN ('Wife', 'Daughter', 'wife', 'daughter')
    AND census_household_schedule IS NOT NULL
    AND census_household_schedule != ''
  LIMIT 5
`).all();

if (householdsWithWomen.length > 0) {
  console.log(`Found ${householdsWithWomen.length} households with female family members\n`);

  householdsWithWomen.forEach((h, idx) => {
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
      console.log(`  ${i + 1}. ${fullName} (b. ${m.birth_year || 'N/A'})`);
      console.log(`     Relation: ${m.census_relation || 'N/A'}, Gender: ${m.census_gender || 'N/A'}, Age: ${m.census_age || 'N/A'}`);
    });
  });
} else {
  console.log('No households with female family members found.');
}

db.close();
