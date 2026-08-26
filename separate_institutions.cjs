const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('Separating Institutional and Pauper Records');
console.log('='.repeat(70) + '\n');

// Identify asylum records
const asylumRecords = db.prepare(`
    SELECT * FROM sheffield_people
    WHERE street_address LIKE '%asylum%'
    OR street_address LIKE '%lunatic%'
    OR civil_parish LIKE '%asylum%'
    OR civil_parish LIKE '%lunatic%'
`).all();

console.log('Asylum records found:', asylumRecords.length.toLocaleString());

// Identify workhouse records
const workhouseRecords = db.prepare(`
    SELECT * FROM sheffield_people
    WHERE street_address LIKE '%workhouse%'
    OR street_address LIKE '%union%'
    OR civil_parish LIKE '%workhouse%'
    OR civil_parish LIKE '%union%'
`).all();

console.log('Workhouse records found:', workhouseRecords.length.toLocaleString());

// Identify pauper records (not already in asylum/workhouse)
const asylumIds = new Set(asylumRecords.map(r => r.id));
const workhouseIds = new Set(workhouseRecords.map(r => r.id));

const pauperRecords = db.prepare(`
    SELECT * FROM sheffield_people
    WHERE census_relation LIKE '%pauper%'
`).all().filter(r => !asylumIds.has(r.id) && !workhouseIds.has(r.id));

console.log('Pauper records found (excluding those already in asylum/workhouse):', pauperRecords.length.toLocaleString());
console.log('');

// Show sample records
console.log('Sample asylum records:');
asylumRecords.slice(0, 3).forEach(r => {
    console.log('  ' + r.name + ' - ' + r.street_address);
});

console.log('\nSample workhouse records:');
workhouseRecords.slice(0, 3).forEach(r => {
    console.log('  ' + r.name + ' - ' + r.street_address);
});

console.log('\nSample pauper records:');
pauperRecords.slice(0, 3).forEach(r => {
    console.log('  ' + r.name + ' - Relation: ' + r.census_relation);
});

// Prepare insert statements for each institutional table
const insertAsylum = db.prepare(`
    INSERT INTO sheffield_asylum (
        id, name, first_name, middle_name, surname, birth_year, gender,
        census_age, census_birth_date, census_birth_place, census_county,
        census_relation, census_gender, census_ed, census_household_schedule,
        census_household_members, census_piece, census_folio, census_page,
        where_born, birth_town, birth_county, birth_country,
        civil_parish, ecclesiastical_parish, registration_district,
        sub_registration_district, street_address, profession, asylum_name
    ) VALUES (
        ?, ?, ?, ?, ?, ?, ?,
        ?, ?, ?, ?,
        ?, ?, ?, ?,
        ?, ?, ?, ?,
        ?, ?, ?, ?,
        ?, ?, ?,
        ?, ?, ?, ?
    )
`);

const insertWorkhouse = db.prepare(`
    INSERT INTO sheffield_workhouse (
        id, name, first_name, middle_name, surname, birth_year, gender,
        census_age, census_birth_date, census_birth_place, census_county,
        census_relation, census_gender, census_ed, census_household_schedule,
        census_household_members, census_piece, census_folio, census_page,
        where_born, birth_town, birth_county, birth_country,
        civil_parish, ecclesiastical_parish, registration_district,
        sub_registration_district, street_address, profession, workhouse_name
    ) VALUES (
        ?, ?, ?, ?, ?, ?, ?,
        ?, ?, ?, ?,
        ?, ?, ?, ?,
        ?, ?, ?, ?,
        ?, ?, ?, ?,
        ?, ?, ?,
        ?, ?, ?, ?
    )
`);

const insertPauper = db.prepare(`
    INSERT INTO sheffield_paupers (
        id, name, first_name, middle_name, surname, birth_year, gender,
        census_age, census_birth_date, census_birth_place, census_county,
        census_relation, census_gender, census_ed, census_household_schedule,
        census_household_members, census_piece, census_folio, census_page,
        where_born, birth_town, birth_county, birth_country,
        civil_parish, ecclesiastical_parish, registration_district,
        sub_registration_district, street_address, profession, pauper_status
    ) VALUES (
        ?, ?, ?, ?, ?, ?, ?,
        ?, ?, ?, ?,
        ?, ?, ?, ?,
        ?, ?, ?, ?,
        ?, ?, ?, ?,
        ?, ?, ?,
        ?, ?, ?, ?
    )
`);

const deleteFromPeople = db.prepare('DELETE FROM sheffield_people WHERE id = ?');

console.log('\n' + '='.repeat(70));
console.log('Moving records to institutional tables...');
console.log('='.repeat(70) + '\n');

// Move asylum records
console.log('Moving asylum records...');
asylumRecords.forEach(r => {
    insertAsylum.run(
        r.id, r.name, r.first_name, r.middle_name, r.surname, r.birth_year, r.gender,
        r.census_age, r.census_birth_date, r.census_birth_place, r.census_county,
        r.census_relation, r.census_gender, r.census_ed, r.census_household_schedule,
        r.census_household_members, r.census_piece, r.census_folio, r.census_page,
        r.where_born, r.birth_town, r.birth_county, r.birth_country,
        r.civil_parish, r.ecclesiastical_parish, r.registration_district,
        r.sub_registration_district, r.street_address, r.profession, r.street_address
    );
    deleteFromPeople.run(r.id);
});
console.log('✓ Moved', asylumRecords.length, 'to sheffield_asylum');

// Move workhouse records
console.log('Moving workhouse records...');
workhouseRecords.forEach(r => {
    insertWorkhouse.run(
        r.id, r.name, r.first_name, r.middle_name, r.surname, r.birth_year, r.gender,
        r.census_age, r.census_birth_date, r.census_birth_place, r.census_county,
        r.census_relation, r.census_gender, r.census_ed, r.census_household_schedule,
        r.census_household_members, r.census_piece, r.census_folio, r.census_page,
        r.where_born, r.birth_town, r.birth_county, r.birth_country,
        r.civil_parish, r.ecclesiastical_parish, r.registration_district,
        r.sub_registration_district, r.street_address, r.profession, r.street_address
    );
    deleteFromPeople.run(r.id);
});
console.log('✓ Moved', workhouseRecords.length, 'to sheffield_workhouse');

// Move pauper records
console.log('Moving pauper records...');
pauperRecords.forEach(r => {
    insertPauper.run(
        r.id, r.name, r.first_name, r.middle_name, r.surname, r.birth_year, r.gender,
        r.census_age, r.census_birth_date, r.census_birth_place, r.census_county,
        r.census_relation, r.census_gender, r.census_ed, r.census_household_schedule,
        r.census_household_members, r.census_piece, r.census_folio, r.census_page,
        r.where_born, r.birth_town, r.birth_county, r.birth_country,
        r.civil_parish, r.ecclesiastical_parish, r.registration_district,
        r.sub_registration_district, r.street_address, r.profession, r.census_relation
    );
    deleteFromPeople.run(r.id);
});
console.log('✓ Moved', pauperRecords.length, 'to sheffield_paupers');

// Final counts
console.log('\n' + '='.repeat(70));
console.log('SUMMARY');
console.log('='.repeat(70));

const peopleCount = db.prepare('SELECT COUNT(*) as c FROM sheffield_people').get().c;
const asylumCount = db.prepare('SELECT COUNT(*) as c FROM sheffield_asylum').get().c;
const workhouseCount = db.prepare('SELECT COUNT(*) as c FROM sheffield_workhouse').get().c;
const pauperCount = db.prepare('SELECT COUNT(*) as c FROM sheffield_paupers').get().c;

console.log('\nFinal counts:');
console.log('  sheffield_people:', peopleCount.toLocaleString());
console.log('  sheffield_asylum:', asylumCount.toLocaleString());
console.log('  sheffield_workhouse:', workhouseCount.toLocaleString());
console.log('  sheffield_paupers:', pauperCount.toLocaleString());
console.log('  Total:', (peopleCount + asylumCount + workhouseCount + pauperCount).toLocaleString());

db.close();
console.log('\n✓ Separation complete!');
