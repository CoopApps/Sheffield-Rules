const Database = require('better-sqlite3');
const db = new Database('./Sheffield1867.db');

// Let's look at George Booth in detail since "Booth" is a distinctive surname
const playerId = '512fe790-10e7-4452-ae89-416959f1ed3b';

console.log('\n=== Detailed Player Information for George Booth ===\n');
const player = db.prepare(`
  SELECT id, name, first_name, middle_name, surname, birth_year,
         birth_town, birth_county, birth_country, where_born,
         civil_parish, ecclesiastical_parish, registration_district, sub_registration_district,
         census_age, census_relation, census_gender, census_ed,
         census_household_schedule, census_piece, census_folio, census_page
  FROM sheffield_players
  WHERE id = ?
`).get(playerId);

if (player) {
  console.log('Personal Information:');
  console.log(`  Full Name: ${player.first_name || ''} ${player.middle_name || ''} ${player.surname || player.name}`);
  console.log(`  Birth Year: ${player.birth_year}`);
  console.log(`  Birth Location: ${player.where_born || 'N/A'}`);
  console.log(`  Town: ${player.birth_town || 'N/A'}, County: ${player.birth_county || 'N/A'}, Country: ${player.birth_country || 'N/A'}`);

  console.log('\nParish Information:');
  console.log(`  Civil Parish: ${player.civil_parish || 'N/A'}`);
  console.log(`  Ecclesiastical Parish: ${player.ecclesiastical_parish || 'N/A'}`);
  console.log(`  Registration District: ${player.registration_district || 'N/A'}`);
  console.log(`  Sub-registration District: ${player.sub_registration_district || 'N/A'}`);

  console.log('\nCensus Information:');
  console.log(`  Age (Census): ${player.census_age || 'N/A'}`);
  console.log(`  Relation to Head: ${player.census_relation || 'N/A'}`);
  console.log(`  Gender: ${player.census_gender || 'N/A'}`);
  console.log(`  ED Number: ${player.census_ed || 'N/A'}`);
  console.log(`  Household Schedule: ${player.census_household_schedule || 'N/A'}`);
  console.log(`  Piece: ${player.census_piece || 'N/A'}`);
  console.log(`  Folio: ${player.census_folio || 'N/A'}`);
  console.log(`  Page: ${player.census_page || 'N/A'}`);

  // Now search for other people with same household schedule (potential family members)
  console.log('\n=== Searching for other players in same household ===\n');
  const householdMembers = db.prepare(`
    SELECT id, name, first_name, surname, birth_year, census_relation, census_gender, census_age
    FROM sheffield_players
    WHERE census_household_schedule = ?
      AND ecclesiastical_parish = ?
    ORDER BY census_relation, census_age DESC
  `).all(player.census_household_schedule, player.ecclesiastical_parish);

  if (householdMembers.length > 0) {
    console.log(`Found ${householdMembers.length} player(s) in household ${player.census_household_schedule}:`);
    householdMembers.forEach((member, i) => {
      console.log(`  ${i + 1}. ${member.first_name || member.name} ${member.surname} (b. ${member.birth_year})`);
      console.log(`     Relation: ${member.census_relation || 'N/A'}, Gender: ${member.census_gender || 'N/A'}, Age: ${member.census_age || 'N/A'}`);
    });
  }
} else {
  console.log('Player not found!');
}

db.close();
