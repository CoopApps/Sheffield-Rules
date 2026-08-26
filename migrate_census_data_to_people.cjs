const Database = require("better-sqlite3");
const db = new Database("Sheffield1867.db");

console.log("Migrating census data...");

// Simple approach: match on exact name and age where both are unique
const censusRecords = db.prepare("SELECT * FROM unmatched_sheffieldcensus WHERE name IS NOT NULL AND age IS NOT NULL").all();

let matched = 0;

for (const census of censusRecords) {
  // Check if this name+age is unique in census
  const censusCount = db.prepare("SELECT COUNT(*) as c FROM unmatched_sheffieldcensus WHERE name = ? AND age = ?").get(census.name, census.age).c;
  if (censusCount !== 1) continue;

  // Check if this name exists uniquely in sheffield_people  
  const peopleCount = db.prepare("SELECT COUNT(*) as c FROM sheffield_people WHERE name = ?").get(census.name).c;
  if (peopleCount !== 1) continue;

  // Update the person with census data
  db.prepare("UPDATE sheffield_people SET ecclesiastical_parish = ?, ed_institution_vessel = ?, estimated_birth_year = ?, folio = ?, household_schedule_number = ?, page_number = ?, piece = ?, registration_district = ? WHERE name = ?").run(
    census.parish, census.relationship, census.birth_year, census.folio_number,
    census.household_id, census.page_number, census.piece_number, census.registration_district, census.name
  );
  matched++;
  
  if (matched % 100 === 0) console.log("Matched:", matched);
}

console.log("Total matched:", matched);
db.close();
