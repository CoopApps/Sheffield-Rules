const sqlite3 = require('better-sqlite3');
const fs = require('fs');

const db = new sqlite3('Sheffield1867.db', { timeout: 30000 });

console.log('='.repeat(70));
console.log('STEP 1: REMOVING DUPLICATES FROM YEARS 1772-1810');
console.log('='.repeat(70) + '\n');

// Count duplicates before
const beforeCount = db.prepare(`
    SELECT COUNT(*) as count
    FROM sheffield_people
    WHERE birth_year BETWEEN 1772 AND 1810
`).get();

console.log('Records before cleanup: ' + beforeCount.count.toLocaleString());

// Delete duplicates, keeping only the one with the lowest ID (first inserted)
const deleteDuplicates = db.prepare(`
    DELETE FROM sheffield_people
    WHERE id NOT IN (
        SELECT MIN(id)
        FROM sheffield_people
        WHERE birth_year BETWEEN 1772 AND 1810
        GROUP BY name, birth_year, census_piece, census_folio, census_household_schedule
    )
    AND birth_year BETWEEN 1772 AND 1810
`);

const deleted = deleteDuplicates.run();
console.log('Deleted duplicate records: ' + deleted.changes.toLocaleString());

// Count after
const afterCount = db.prepare(`
    SELECT COUNT(*) as count
    FROM sheffield_people
    WHERE birth_year BETWEEN 1772 AND 1810
`).get();

console.log('Records after cleanup: ' + afterCount.count.toLocaleString() + '\n');

console.log('='.repeat(70));
console.log('STEP 2: IMPORTING MISSING YEARS (1849, 1850, 1851, 1852)');
console.log('='.repeat(70) + '\n');

// Parse CSV function
function parseCSV(content) {
    const lines = content.split('\n');
    if (lines.length < 2) return [];

    const headers = lines[0].split(',').map(h => h.trim().replace(/^"|"$/g, ''));
    const rows = [];

    for (let i = 1; i < lines.length; i++) {
        const line = lines[i].trim();
        if (!line) continue;

        const values = [];
        let current = '';
        let inQuotes = false;

        for (let j = 0; j < line.length; j++) {
            const char = line[j];
            if (char === '"') {
                inQuotes = !inQuotes;
            } else if (char === ',' && !inQuotes) {
                values.push(current.trim().replace(/^"|"$/g, ''));
                current = '';
            } else {
                current += char;
            }
        }
        values.push(current.trim().replace(/^"|"$/g, ''));

        const row = {};
        headers.forEach((h, idx) => {
            row[h] = values[idx] || '';
        });
        rows.push(row);
    }

    return rows;
}

// Insert statement
const insertPerson = db.prepare(`
    INSERT INTO sheffield_people (
        name, first_name, surname, birth_year,
        census_gender, census_relation, census_piece, census_folio,
        census_household_schedule, census_household_members,
        street_address, profession,
        ecclesiastical_parish, civil_parish
    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
`);

const years = [1849, 1850, 1851, 1852];
let totalImported = 0;

years.forEach(year => {
    const filename = `Sheffield Census/${year}.csv`;
    console.log(`Importing ${year}.csv...`);

    const content = fs.readFileSync(filename, 'utf-8');
    const records = parseCSV(content);

    let imported = 0;
    records.forEach(record => {
        const name = record.NAME || record.Name || '';
        if (!name) return;

        const nameParts = name.trim().split(/\s+/);
        const firstName = nameParts[0] || '';
        const surname = nameParts[nameParts.length - 1] || '';

        const birthYear = parseInt(record['ESTIMATED BIRTH YEAR'] || record['Birth Date'] || year);
        const gender = record.GENDER || record.Gender || '';
        const relation = record.RELATION || record.Relation || '';
        const piece = record.PIECE || record.Piece || '';
        const folio = record.FOLIO || record.Folio || '';
        const schedule = record['HOUSEHOLD SCHEDULE NUMBER'] || record['Household Schedule Number'] || '';
        const household = record['HOUSEHOLD MEMBERS'] || record['Household Members'] || '';
        const address = record['WHERE BORN'] || record['Birth Place'] || '';
        const profession = record.PROFESSION || record.Profession || '';
        const eccParish = record['ECCLESIASTICAL PARISH'] || record['Ecclesiastical Parish'] || '';
        const civParish = record['CIVIL PARISH'] || record['Civil Parish'] || '';

        try {
            insertPerson.run(
                name, firstName, surname, birthYear,
                gender, relation, piece, folio,
                schedule, household,
                address, profession,
                eccParish, civParish
            );
            imported++;
        } catch (e) {
            // Skip duplicates
        }
    });

    console.log(`  Imported ${imported.toLocaleString()} records from ${year}`);
    totalImported += imported;
});

console.log('\nTotal new records imported: ' + totalImported.toLocaleString());

// Final verification
console.log('\n' + '='.repeat(70));
console.log('VERIFICATION: Checking all years now');
console.log('='.repeat(70) + '\n');

const censusFiles = fs.readdirSync('Sheffield Census').filter(f => f.match(/^\d{4}\.csv$/)).sort();

console.log('Year | CSV Lines | DB Count | Difference');
console.log('-'.repeat(50));

const problemYears = [];
censusFiles.forEach(file => {
    const year = parseInt(file.replace('.csv', ''));
    const csvPath = 'Sheffield Census/' + file;
    const content = fs.readFileSync(csvPath, 'utf-8');
    const lines = content.split('\n').filter(l => l.trim()).length - 1;
    const dbCount = db.prepare('SELECT COUNT(*) as count FROM sheffield_people WHERE birth_year = ?').get(year);
    const diff = lines - dbCount.count;

    if (Math.abs(diff) > 50) {
        console.log(year + ' | ' + lines.toString().padStart(9) + ' | ' + dbCount.count.toString().padStart(8) + ' | ' + diff.toString().padStart(10));
        problemYears.push(year);
    }
});

if (problemYears.length === 0) {
    console.log('✓ All years are now within acceptable range!');
} else {
    console.log('\nYears with significant differences: ' + problemYears.join(', '));
}

db.close();

console.log('\n✓ Complete!');
