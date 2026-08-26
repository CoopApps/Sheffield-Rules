const sqlite3 = require('better-sqlite3');
const fs = require('fs');
const path = require('path');

const db = new sqlite3('Sheffield1867.db', { timeout: 30000 });
const censusDir = 'Sheffield Census';

// Simple CSV parser
function parseCSV(content) {
    const lines = content.split('\n');
    if (lines.length < 2) return [];

    const headers = lines[0].split('","').map(h => h.replace(/^"|"$/g, '').trim());
    const rows = [];

    for (let i = 1; i < lines.length; i++) {
        const line = lines[i].trim();
        if (!line) continue;

        const values = line.split('","').map(v => v.replace(/^"|"$/g, '').trim());
        const row = {};
        headers.forEach((header, index) => {
            row[header] = values[index] || '';
        });
        rows.push(row);
    }

    return rows;
}

// Parse name
function parseName(fullName) {
    const parts = fullName.trim().split(/\s+/);
    if (parts.length === 0) return { first: 'Unknown', middle: null, surname: 'Unknown' };
    if (parts.length === 1) return { first: parts[0], middle: null, surname: parts[0] };
    if (parts.length === 2) return { first: parts[0], middle: null, surname: parts[1] };

    const first = parts[0];
    const surname = parts[parts.length - 1];
    const middle = parts.slice(1, -1).join(' ');
    return { first, middle, surname };
}

// UUID generator
function generateUUID() {
    return 'xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
        const r = Math.random() * 16 | 0;
        const v = c === 'x' ? r : (r & 0x3 | 0x8);
        return v.toString(16);
    });
}

console.log('Importing remaining people from census files...\n');

const csvFiles = fs.readdirSync(censusDir)
    .filter(f => f.toLowerCase().endsWith('.csv'))
    .sort();

let totalImported = 0;
let totalSkipped = 0;

// Prepare statements
const checkExisting = db.prepare('SELECT id FROM sheffield_people WHERE name = ? AND birth_year = ? LIMIT 1');
const checkPlayer = db.prepare('SELECT id FROM sheffield_players WHERE name = ? AND birth_year = ? LIMIT 1');
const insertPerson = db.prepare(`
    INSERT INTO sheffield_people (
        id, name, first_name, middle_name, surname,
        birth_year, gender,
        census_age, census_birth_date, census_birth_place,
        census_county, census_relation, census_gender,
        census_ed, census_household_schedule, census_household_members,
        census_piece, census_folio, census_page,
        where_born, birth_town, birth_county, birth_country,
        civil_parish, ecclesiastical_parish,
        registration_district, sub_registration_district,
        player_id, is_player
    ) VALUES (
        ?, ?, ?, ?, ?,
        ?, ?,
        ?, ?, ?,
        ?, ?, ?,
        ?, ?, ?,
        ?, ?, ?,
        ?, ?, ?, ?,
        ?, ?,
        ?, ?,
        ?, ?
    )
`);

for (const filename of csvFiles) {
    const filePath = path.join(censusDir, filename);
    const content = fs.readFileSync(filePath, 'utf-8');
    const records = parseCSV(content);

    let fileImported = 0;
    let fileSkipped = 0;

    for (const record of records) {
        const name = record.Name || record.NAME || '';
        const birthYear = parseInt(record['ESTIMATED BIRTH YEAR'] || record['Birth Date'] || '0');

        if (!name || !birthYear || birthYear < 1700 || birthYear > 1900) {
            continue;
        }

        // Check if already exists
        if (checkExisting.get(name, birthYear)) {
            fileSkipped++;
            continue;
        }

        const { first, middle, surname } = parseName(name);

        // Check if player
        const matchingPlayer = checkPlayer.get(name, birthYear);
        const playerId = matchingPlayer ? matchingPlayer.id : null;
        const isPlayer = matchingPlayer ? 1 : 0;

        try {
            insertPerson.run(
                generateUUID(), name, first, middle, surname,
                birthYear, record.GENDER || null,
                parseInt(record.AGE) || null,
                record['Birth Date'] || null,
                record['Birth Place'] || null,
                record['COUNTY/ISLAND'] || null,
                record.RELATION || null,
                record.GENDER || null,
                record['ED, INSTITUTION, OR VESSEL'] || null,
                record['HOUSEHOLD SCHEDULE NUMBER'] || null,
                record['HOUSEHOLD MEMBERS'] || null,
                record.PIECE || null,
                record.FOLIO || null,
                record['PAGE NUMBER'] || null,
                record['WHERE BORN'] || null,
                record.TOWN || null,
                record['COUNTY/ISLAND'] || null,
                record.COUNTRY || null,
                record['CIVIL PARISH'] || null,
                record['ECCLESIASTICAL PARISH'] || null,
                record['REGISTRATION DISTRICT'] || null,
                record['SUB-REGISTRATION DISTRICT'] || null,
                playerId,
                isPlayer
            );
            fileImported++;
        } catch (error) {
            console.error(`Error inserting ${name}: ${error.message}`);
        }
    }

    if (fileImported > 0 || fileSkipped > 0) {
        console.log(`${filename}: +${fileImported} imported, ${fileSkipped} skipped`);
    }

    totalImported += fileImported;
    totalSkipped += fileSkipped;
}

console.log('\n' + '='.repeat(60));
console.log(`Total imported: ${totalImported}`);
console.log(`Total skipped: ${totalSkipped}`);

const final = db.prepare('SELECT COUNT(*) as total, COUNT(CASE WHEN is_player = 1 THEN 1 END) as players, COUNT(CASE WHEN is_player = 0 THEN 1 END) as non_players FROM sheffield_people').get();
console.log('\nFinal Stats:');
console.log(`  Total people: ${final.total}`);
console.log(`  Players: ${final.players}`);
console.log(`  Non-players: ${final.non_players}`);

db.close();
console.log('\n✓ Import complete!');
