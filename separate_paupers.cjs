const sqlite3 = require('better-sqlite3');

const db = new sqlite3('Sheffield1867.db', { timeout: 30000 });

console.log('='.repeat(70));
console.log('Separating Paupers from Database');
console.log('='.repeat(70) + '\n');

// Drop existing table if it exists
console.log('Dropping existing pauper table if it exists...');
db.exec('DROP TABLE IF EXISTS sheffield_paupers');
console.log('✓ Dropped existing table\n');

// Create paupers table
console.log('Creating sheffield_paupers table...');
db.exec(`
    CREATE TABLE sheffield_paupers (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        first_name TEXT,
        middle_name TEXT,
        surname TEXT,
        birth_year INTEGER,
        gender TEXT,
        census_age INTEGER,
        census_birth_date TEXT,
        census_birth_place TEXT,
        census_county TEXT,
        census_relation TEXT,
        census_gender TEXT,
        census_ed TEXT,
        census_household_schedule TEXT,
        census_household_members TEXT,
        census_piece TEXT,
        census_folio TEXT,
        census_page TEXT,
        where_born TEXT,
        birth_town TEXT,
        birth_county TEXT,
        birth_country TEXT,
        civil_parish TEXT,
        ecclesiastical_parish TEXT,
        registration_district TEXT,
        sub_registration_district TEXT,
        street_address TEXT,
        profession TEXT,
        pauper_status TEXT,
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP
    )
`);
console.log('✓ sheffield_paupers table created\n');

// Find paupers in sheffield_people
console.log('Finding paupers in sheffield_people...');
const paupersFromPeople = db.prepare(`
    SELECT * FROM sheffield_people
    WHERE street_address LIKE '%pauper%'
    OR street_address LIKE '%Pauper%'
    OR street_address LIKE '%poor house%'
    OR street_address LIKE '%Poor House%'
    OR street_address LIKE '%poorhouse%'
    OR street_address LIKE '%Poorhouse%'
    OR street_address LIKE '%poor law%'
    OR street_address LIKE '%Poor Law%'
    OR street_address LIKE '%union house%'
    OR street_address LIKE '%Union House%'
    OR profession LIKE '%pauper%'
    OR profession LIKE '%Pauper%'
`).all();

console.log(`Found ${paupersFromPeople.length} paupers in sheffield_people\n`);

// Show samples
if (paupersFromPeople.length > 0) {
    console.log('Sample paupers from sheffield_people:');
    paupersFromPeople.slice(0, 5).forEach(p => {
        console.log(`  ${p.name} (${p.birth_year}) - ${p.street_address || p.profession}`);
    });
    console.log('');
}

// Extract pauper status from address or profession
function extractPauperStatus(address, profession) {
    const text = `${address || ''} ${profession || ''}`.toLowerCase();

    if (text.includes('poor house') || text.includes('poorhouse')) return 'Poor House Resident';
    if (text.includes('poor law')) return 'Poor Law Relief';
    if (text.includes('union house')) return 'Union House Resident';
    if (text.includes('pauper')) return 'Pauper';

    return 'Pauper';
}

// Move paupers from sheffield_people
console.log('Moving paupers from sheffield_people...');
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
        ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
    )
`);

let paupersMoved = 0;
paupersFromPeople.forEach(person => {
    const pauperStatus = extractPauperStatus(person.street_address, person.profession);

    insertPauper.run(
        person.id, person.name, person.first_name, person.middle_name,
        person.surname, person.birth_year, person.gender,
        person.census_age, person.census_birth_date, person.census_birth_place,
        person.census_county, person.census_relation, person.census_gender,
        person.census_ed, person.census_household_schedule,
        person.census_household_members, person.census_piece, person.census_folio,
        person.census_page, person.where_born, person.birth_town,
        person.birth_county, person.birth_country, person.civil_parish,
        person.ecclesiastical_parish, person.registration_district,
        person.sub_registration_district, person.street_address,
        person.profession, pauperStatus
    );

    paupersMoved++;
});

console.log(`✓ Moved ${paupersMoved} people to sheffield_paupers\n`);

// Get player IDs that need to be removed
console.log('Finding pauper players...');
const pauperPlayerIds = db.prepare(`
    SELECT DISTINCT player_id FROM sheffield_people
    WHERE player_id IS NOT NULL
    AND (
        street_address LIKE '%pauper%'
        OR street_address LIKE '%Pauper%'
        OR street_address LIKE '%poor house%'
        OR street_address LIKE '%poorhouse%'
        OR street_address LIKE '%poor law%'
        OR street_address LIKE '%union house%'
        OR profession LIKE '%pauper%'
        OR profession LIKE '%Pauper%'
    )
`).all().map(r => r.player_id);

console.log(`Found ${pauperPlayerIds.length} pauper players\n`);

// Remove from sheffield_people first
console.log('Removing paupers from sheffield_people...');
const deletedPaupers = db.prepare(`
    DELETE FROM sheffield_people
    WHERE street_address LIKE '%pauper%'
    OR street_address LIKE '%Pauper%'
    OR street_address LIKE '%poor house%'
    OR street_address LIKE '%Poor House%'
    OR street_address LIKE '%poorhouse%'
    OR street_address LIKE '%Poorhouse%'
    OR street_address LIKE '%poor law%'
    OR street_address LIKE '%Poor Law%'
    OR street_address LIKE '%union house%'
    OR street_address LIKE '%Union House%'
    OR profession LIKE '%pauper%'
    OR profession LIKE '%Pauper%'
`).run();

console.log(`✓ Removed ${deletedPaupers.changes} paupers from sheffield_people\n`);

// Then remove from sheffield_players
if (pauperPlayerIds.length > 0) {
    console.log('Removing paupers from sheffield_players...');
    const placeholders = pauperPlayerIds.map(() => '?').join(',');
    const removed = db.prepare(`
        DELETE FROM sheffield_players
        WHERE id IN (${placeholders})
    `).run(...pauperPlayerIds);

    console.log(`✓ Removed ${removed.changes} players from sheffield_players\n`);
}

// Final statistics
const peopleCount = db.prepare('SELECT COUNT(*) as count FROM sheffield_people').get();
const playersCount = db.prepare('SELECT COUNT(*) as count FROM sheffield_players').get();
const paupersCount = db.prepare('SELECT COUNT(*) as count FROM sheffield_paupers').get();
const workhouseCount = db.prepare('SELECT COUNT(*) as count FROM sheffield_workhouse').get();
const asylumCount = db.prepare('SELECT COUNT(*) as count FROM sheffield_asylum').get();

console.log('='.repeat(70));
console.log('Final Statistics:');
console.log('='.repeat(70));
console.log(`  sheffield_people: ${peopleCount.count.toLocaleString()}`);
console.log(`  sheffield_players: ${playersCount.count.toLocaleString()}`);
console.log(`  sheffield_paupers: ${paupersCount.count.toLocaleString()}`);
console.log(`  sheffield_workhouse: ${workhouseCount.count.toLocaleString()}`);
console.log(`  sheffield_asylum: ${asylumCount.count.toLocaleString()}`);

// Show pauper breakdown
const pauperBreakdown = db.prepare(`
    SELECT pauper_status, COUNT(*) as count
    FROM sheffield_paupers
    GROUP BY pauper_status
    ORDER BY count DESC
`).all();

console.log('\nPauper breakdown:');
pauperBreakdown.forEach(p => {
    console.log(`  ${p.pauper_status}: ${p.count} people`);
});

db.close();
console.log('\n✓ Pauper separation complete!');
