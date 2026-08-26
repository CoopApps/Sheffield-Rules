const sqlite3 = require('better-sqlite3');
const fs = require('fs');
const path = require('path');

const db = new sqlite3('Sheffield1867.db');
const censusDir = 'Sheffield Census';

// Simple CSV parser function
function parseCSV(content) {
    const lines = content.split('\n');
    if (lines.length < 2) return [];

    const headers = lines[0].split('","').map(h => h.replace(/^"|"$/g, ''));
    const rows = [];

    for (let i = 1; i < lines.length; i++) {
        const line = lines[i].trim();
        if (!line) continue;

        const values = line.split('","').map(v => v.replace(/^"|"$/g, ''));
        const row = {};
        headers.forEach((header, index) => {
            row[header] = values[index] || '';
        });
        rows.push(row);
    }

    return rows;
}

// First, add the missing columns to the database
console.log('Adding missing census columns to database...\n');

const columnsToAdd = [
    'census_birth_date',
    'census_birth_place',
    'census_county',
    'census_household_members'
];

for (const column of columnsToAdd) {
    try {
        db.prepare(`ALTER TABLE sheffield_players ADD COLUMN ${column} TEXT`).run();
        console.log(`✓ Added column: ${column}`);
    } catch (error) {
        if (error.message.includes('duplicate column name')) {
            console.log(`○ Column already exists: ${column}`);
        } else {
            console.error(`✗ Error adding ${column}:`, error.message);
        }
    }
}

console.log('\n' + '='.repeat(60) + '\n');
console.log('Starting census data import...\n');

// Get all CSV files
const csvFiles = fs.readdirSync(censusDir)
    .filter(f => f.toLowerCase().endsWith('.csv'))
    .sort();

console.log(`Found ${csvFiles.length} census CSV files\n`);

let totalFiles = csvFiles.length;
let processedFiles = 0;
let totalUpdated = 0;
let totalInserted = 0;
let errors = [];

// Process each CSV file
function processFile(filename) {
    const filePath = path.join(censusDir, filename);
    const content = fs.readFileSync(filePath, 'utf-8');
    return parseCSV(content);
}

// Process all files
for (const filename of csvFiles) {
    try {
        console.log(`Processing ${filename}...`);

        const players = processFile(filename);
            let updated = 0;
            let inserted = 0;

            // Begin transaction for this file
            const transaction = db.transaction((playerList) => {
                for (const player of playerList) {
                    // Get the player data
                    const name = player.Name || player.NAME || '';
                    const birthYear = parseInt(player['ESTIMATED BIRTH YEAR'] || player['Birth Date'] || '0');

                    if (!name || !birthYear || birthYear < 1700 || birthYear > 1900) {
                        continue;
                    }

                    // Check if player exists
                    const existing = db.prepare(
                        'SELECT id FROM sheffield_players WHERE name = ? AND birth_year = ? LIMIT 1'
                    ).get(name, birthYear);

                    if (existing) {
                        // Update existing player
                        db.prepare(`
                            UPDATE sheffield_players SET
                                census_birth_date = COALESCE(?, census_birth_date),
                                census_birth_place = COALESCE(?, census_birth_place),
                                census_county = COALESCE(?, census_county),
                                census_household_members = COALESCE(?, census_household_members),
                                census_age = COALESCE(?, census_age),
                                census_relation = COALESCE(?, census_relation),
                                census_gender = COALESCE(?, census_gender),
                                census_ed = COALESCE(?, census_ed),
                                census_household_schedule = COALESCE(?, census_household_schedule),
                                census_piece = COALESCE(?, census_piece),
                                census_folio = COALESCE(?, census_folio),
                                census_page = COALESCE(?, census_page),
                                where_born = COALESCE(?, where_born),
                                birth_town = COALESCE(?, birth_town),
                                birth_county = COALESCE(?, birth_county),
                                birth_country = COALESCE(?, birth_country),
                                civil_parish = COALESCE(?, civil_parish),
                                ecclesiastical_parish = COALESCE(?, ecclesiastical_parish),
                                registration_district = COALESCE(?, registration_district),
                                sub_registration_district = COALESCE(?, sub_registration_district)
                            WHERE id = ?
                        `).run(
                            player['Birth Date'] || null,
                            player['Birth Place'] || null,
                            player['COUNTY/ISLAND'] || null,
                            player['HOUSEHOLD MEMBERS'] || null,
                            player.AGE || null,
                            player.RELATION || null,
                            player.GENDER || null,
                            player['ED, INSTITUTION, OR VESSEL'] || null,
                            player['HOUSEHOLD SCHEDULE NUMBER'] || null,
                            player.PIECE || null,
                            player.FOLIO || null,
                            player['PAGE NUMBER'] || null,
                            player['WHERE BORN'] || null,
                            player.TOWN || null,
                            player['COUNTY/ISLAND'] || null,
                            player.COUNTRY || null,
                            player['CIVIL PARISH'] || null,
                            player['ECCLESIASTICAL PARISH'] || null,
                            player['REGISTRATION DISTRICT'] || null,
                            player['SUB-REGISTRATION DISTRICT'] || null,
                            existing.id
                        );
                        updated++;
                    } else {
                        // Player doesn't exist - could insert new, but for safety we'll skip
                        // since you said you don't want duplicates and have 28k players already
                        // If you want to insert new players, uncomment below:
                        /*
                        const { v4: uuidv4 } = require('uuid');
                        db.prepare(`INSERT INTO sheffield_players (...) VALUES (...)`).run(...);
                        inserted++;
                        */
                    }
                }
            });

            transaction(players);

            processedFiles++;
            totalUpdated += updated;
            totalInserted += inserted;

            console.log(`  ✓ Updated ${updated} players`);

    } catch (error) {
        console.error(`  ✗ Error processing ${filename}:`, error.message);
        errors.push(`${filename}: ${error.message}`);
    }
}

console.log('\n' + '='.repeat(60) + '\n');
console.log('Import Complete!\n');
console.log(`Files processed: ${processedFiles}/${totalFiles}`);
console.log(`Players updated: ${totalUpdated}`);
console.log(`Players inserted: ${totalInserted}`);

if (errors.length > 0) {
    console.log(`\nErrors: ${errors.length}`);
    errors.forEach(err => console.log(`  - ${err}`));
}

db.close();
console.log('\nDatabase connection closed.');
