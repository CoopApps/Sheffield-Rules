const Database = require('better-sqlite3');
const crypto = require('crypto');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('Second Pass: Matching Remaining Unmatched Records');
console.log('='.repeat(70) + '\n');

function getSurname(fullName) {
    if (!fullName) return '';
    const parts = fullName.trim().split(/\s+/);
    return parts[parts.length - 1];
}

function splitName(fullName) {
    if (!fullName) return { first: '', middle: '', surname: '' };
    const parts = fullName.trim().split(/\s+/);

    if (parts.length === 1) {
        return { first: parts[0], middle: '', surname: '' };
    } else if (parts.length === 2) {
        return { first: parts[0], middle: '', surname: parts[1] };
    } else {
        return {
            first: parts[0],
            middle: parts.slice(1, -1).join(' '),
            surname: parts[parts.length - 1]
        };
    }
}

const sheffieldRecords = db.prepare('SELECT * FROM unmatched_sheffieldcensus').all();
const genealogyRecords = db.prepare('SELECT * FROM unmatched_genealogy').all();

console.log('Sheffield census unmatched:', sheffieldRecords.length.toLocaleString());
console.log('Genealogy unmatched:', genealogyRecords.length.toLocaleString());
console.log('');

// Create index for genealogy records
const genealogyByName = new Map();
genealogyRecords.forEach(g => {
    if (!genealogyByName.has(g.name)) {
        genealogyByName.set(g.name, []);
    }
    genealogyByName.get(g.name).push(g);
});

const insertPerson = db.prepare(`
    INSERT INTO sheffield_people (
        id, name, first_name, middle_name, surname, source,
        birth_year, gender, census_age, census_birth_place, census_county,
        census_relation, census_gender, census_household_members,
        census_piece, census_folio, census_page,
        where_born, civil_parish, ecclesiastical_parish,
        registration_district, street_address, profession,
        created_at
    ) VALUES (
        ?, ?, ?, ?, ?, ?,
        ?, ?, ?, ?, ?,
        ?, ?, ?,
        ?, ?, ?,
        ?, ?, ?,
        ?, ?, ?,
        CURRENT_TIMESTAMP
    )
`);

const matchedSheffieldIds = new Set();
const matchedGenealogyIds = new Set();

let matchCount = 0;

console.log('Matching records...\n');

sheffieldRecords.forEach((sheffield, index) => {
    if (index % 1000 === 0) {
        console.log(`Progress: ${index.toLocaleString()} / ${sheffieldRecords.length.toLocaleString()} (${matchCount.toLocaleString()} matches)`);
    }

    if (matchedSheffieldIds.has(sheffield.id)) return;

    const nameMatches = genealogyByName.get(sheffield.name) || [];
    let bestMatch = null;
    let matchReason = '';

    // Try exact name + birth year match
    for (const genealogy of nameMatches) {
        if (matchedGenealogyIds.has(genealogy.id)) continue;

        if (sheffield.birth_year && genealogy.birth_year) {
            const yearDiff = Math.abs(sheffield.birth_year - genealogy.birth_year);
            if (yearDiff <= 2) {
                bestMatch = genealogy;
                matchReason = 'second_pass_name_year';
                break;
            }
        }
    }

    if (bestMatch) {
        const nameParts = splitName(sheffield.name);

        insertPerson.run(
            crypto.randomUUID(),
            sheffield.name,
            nameParts.first,
            nameParts.middle,
            nameParts.surname,
            matchReason,
            sheffield.birth_year || bestMatch.birth_year,
            sheffield.gender,
            sheffield.age,
            sheffield.birth_place || bestMatch.birth_place,
            sheffield.county,
            sheffield.relation || bestMatch.relation,
            sheffield.gender,
            sheffield.household_members,
            sheffield.piece,
            sheffield.folio,
            sheffield.page,
            sheffield.birth_place || bestMatch.birth_place,
            sheffield.civil_parish,
            sheffield.ecclesiastical_parish,
            sheffield.registration_district,
            bestMatch.address,
            bestMatch.profession
        );

        matchedSheffieldIds.add(sheffield.id);
        matchedGenealogyIds.add(bestMatch.id);
        matchCount++;
    }
});

console.log(`\nMatching complete: ${matchCount.toLocaleString()} new matches found\n`);

// Remove matched records
const deleteSheffieldStmt = db.prepare('DELETE FROM unmatched_sheffieldcensus WHERE id = ?');
const deleteGenealogyStmt = db.prepare('DELETE FROM unmatched_genealogy WHERE id = ?');

matchedSheffieldIds.forEach(id => deleteSheffieldStmt.run(id));
matchedGenealogyIds.forEach(id => deleteGenealogyStmt.run(id));

console.log(`✓ Removed ${matchedSheffieldIds.size.toLocaleString()} from unmatched_sheffieldcensus`);
console.log(`✓ Removed ${matchedGenealogyIds.size.toLocaleString()} from unmatched_genealogy\n`);

// Final statistics
console.log('='.repeat(70));
console.log('SUMMARY');
console.log('='.repeat(70));

const remainingSheffield = db.prepare('SELECT COUNT(*) as c FROM unmatched_sheffieldcensus').get().c;
const remainingGenealogy = db.prepare('SELECT COUNT(*) as c FROM unmatched_genealogy').get().c;
const totalPeople = db.prepare('SELECT COUNT(*) as c FROM sheffield_people').get().c;

console.log(`\nNew matches: ${matchCount.toLocaleString()}`);
console.log(`\nRemaining unmatched:`);
console.log(`  Sheffield Census: ${remainingSheffield.toLocaleString()}`);
console.log(`  Genealogy: ${remainingGenealogy.toLocaleString()}`);
console.log(`\nTotal in sheffield_people: ${totalPeople.toLocaleString()}`);

db.close();
console.log('\n✓ Second pass matching complete!');
