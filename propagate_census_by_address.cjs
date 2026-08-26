const Database = require("better-sqlite3");
const db = new Database("Sheffield1867.db");

console.log("Propagating census data by address/postcode...\n");

// Get distinct addresses with census data
const addresses = db.prepare("SELECT DISTINCT address, ecclesiastical_parish, folio, household_schedule_number, page_number, piece, registration_district FROM sheffield_people WHERE address IS NOT NULL AND ecclesiastical_parish IS NOT NULL").all();

console.log("Found", addresses.length, "addresses with census data");

// Update all people at each address
let updated = 0;
for (const addr of addresses) {
  const result = db.prepare("UPDATE sheffield_people SET ecclesiastical_parish = COALESCE(ecclesiastical_parish, ?), folio = COALESCE(folio, ?), household_schedule_number = COALESCE(household_schedule_number, ?), page_number = COALESCE(page_number, ?), piece = COALESCE(piece, ?), registration_district = COALESCE(registration_district, ?) WHERE address = ?").run(addr.ecclesiastical_parish, addr.folio, addr.household_schedule_number, addr.page_number, addr.piece, addr.registration_district, addr.address);
  updated += result.changes;
}

console.log("Updated", updated, "people by address");

// Get distinct postcodes with parish data
const postcodes = db.prepare("SELECT DISTINCT postcode, ecclesiastical_parish, registration_district FROM sheffield_people WHERE postcode IS NOT NULL AND ecclesiastical_parish IS NOT NULL").all();

console.log("Found", postcodes.length, "postcodes with parish data");

// Update all people at each postcode
let postcodeUpdated = 0;
for (const pc of postcodes) {
  const result = db.prepare("UPDATE sheffield_people SET ecclesiastical_parish = COALESCE(ecclesiastical_parish, ?), registration_district = COALESCE(registration_district, ?) WHERE postcode = ?").run(pc.ecclesiastical_parish, pc.registration_district, pc.postcode);
  postcodeUpdated += result.changes;
}

console.log("Updated", postcodeUpdated, "people by postcode");
console.log("Total updated:", updated + postcodeUpdated);
db.close();
