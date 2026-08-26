const Database = require('better-sqlite3');
const crypto = require('crypto');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('Matching Sheffield Census with Genealogy Records');
console.log('='.repeat(70) + '\n');

// Helper function to split full name into parts
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

// Helper function to extract surname
function getSurname(fullName) {
    if (!fullName) return '';
    const parts = fullName.trim().split(/\s+/);
    return parts[parts.length - 1];
}

// Helper function to check if person is in household
function isInHousehold(name, householdMembers) {
    if (!householdMembers || !name) return false;
    const members = householdMembers.split(',').map(n => n.trim());
    return members.some(m => m === name);
}

// Prepare insert statement
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

console.log('Loading records...\n');

// Get all Sheffield Census records
const sheffieldRecords = db.prepare('SELECT * FROM unmatched_sheffieldcensus').all();
console.log(`Sheffield Census records: ${sheffieldRecords.length.toLocaleString()}`);

// Get all Genealogy records
const genealogyRecords = db.prepare('SELECT * FROM unmatched_genealogy').all();
console.log(`Genealogy records: ${genealogyRecords.length.toLocaleString()}\n`);

// Create indexes for faster lookup
const genealogyByName = new Map();
const genealogyBySurname = new Map();

genealogyRecords.forEach(g => {
    // Index by full name
    if (!genealogyByName.has(g.name)) {
        genealogyByName.set(g.name, []);
    }
    genealogyByName.get(g.name).push(g);

    // Index by surname
    const surname = getSurname(g.name);
    if (!genealogyBySurname.has(surname)) {
        genealogyBySurname.set(surname, []);
    }
    genealogyBySurname.get(surname).push(g);
});

// Track matched IDs
const matchedSheffieldIds = new Set();
const matchedGenealogyIds = new Set();

let matchCount = 0;
const totalRecords = sheffieldRecords.length;

console.log('Matching records...\n');

// Match each Sheffield record
sheffieldRecords.forEach((sheffield, index) => {
    // Progress indicator every 1000 records
    if (index % 1000 === 0) {
        console.log(`Progress: ${index.toLocaleString()} / ${totalRecords.toLocaleString()} (${matchCount.toLocaleString()} matches found)`);
    }

    if (matchedSheffieldIds.has(sheffield.id)) return;

    const sheffieldSurname = getSurname(sheffield.name);
    let bestMatch = null;
    let matchReason = '';

    // Strategy 1: Exact name + birth year match (±2 years)
    const nameMatches = genealogyByName.get(sheffield.name) || [];
    for (const genealogy of nameMatches) {
        if (matchedGenealogyIds.has(genealogy.id)) continue;

        if (sheffield.birth_year && genealogy.birth_year) {
            const yearDiff = Math.abs(sheffield.birth_year - genealogy.birth_year);
            if (yearDiff <= 2) {
                bestMatch = genealogy;
                matchReason = 'exact_name_year';
                break;
            }
        }
    }

    // Strategy 2: Spouse cross-reference
    // Example: Thomas Stevens (Sheffield) has Holly Stevens in household
    //          Holly Stevens (Genealogy) has Thomas Stevens as spouse
    if (!bestMatch && sheffield.household_members) {
        const householdNames = sheffield.household_members.split(',').map(n => n.trim());

        for (const householdName of householdNames) {
            const potentialSpouses = genealogyByName.get(householdName) || [];

            for (const genealogy of potentialSpouses) {
                if (matchedGenealogyIds.has(genealogy.id)) continue;

                // Check if this genealogy record has Sheffield person as spouse
                if (genealogy.spouse && genealogy.spouse === sheffield.name) {
                    // Verify birth years are reasonable (within 10 years for spouses)
                    if (sheffield.birth_year && genealogy.birth_year) {
                        const yearDiff = Math.abs(sheffield.birth_year - genealogy.birth_year);
                        if (yearDiff <= 15) {  // Spouses can have larger age gaps
                            bestMatch = genealogy;
                            matchReason = 'spouse_cross_ref';
                            break;
                        }
                    } else {
                        // If no birth years, still match on spouse
                        bestMatch = genealogy;
                        matchReason = 'spouse_cross_ref';
                        break;
                    }
                }
            }
            if (bestMatch) break;
        }
    }

    // Strategy 3: Same surname + birth year (±2 years) + same location
    if (!bestMatch) {
        const surnameMatches = genealogyBySurname.get(sheffieldSurname) || [];

        for (const genealogy of surnameMatches) {
            if (matchedGenealogyIds.has(genealogy.id)) continue;

            // Check birth year
            if (sheffield.birth_year && genealogy.birth_year) {
                const yearDiff = Math.abs(sheffield.birth_year - genealogy.birth_year);
                if (yearDiff <= 2) {
                    // Check location
                    let locationMatch = false;

                    if (sheffield.civil_parish && genealogy.parish) {
                        if (sheffield.civil_parish === genealogy.parish ||
                            sheffield.civil_parish.includes(genealogy.parish) ||
                            genealogy.parish.includes(sheffield.civil_parish)) {
                            locationMatch = true;
                        }
                    }

                    if (sheffield.civil_parish && genealogy.area) {
                        if (sheffield.civil_parish.includes(genealogy.area)) {
                            locationMatch = true;
                        }
                    }

                    if (locationMatch) {
                        bestMatch = genealogy;
                        matchReason = 'surname_year_location';
                        break;
                    }
                }
            }
        }
    }

    // Strategy 4: Household member appears in genealogy with matching details
    if (!bestMatch && sheffield.household_members) {
        const householdNames = sheffield.household_members.split(',').map(n => n.trim());
        const sheffieldSurnameLC = sheffieldSurname.toLowerCase();

        for (const householdName of householdNames) {
            const householdSurname = getSurname(householdName);

            // Only check if different surname (not family member)
            if (householdSurname.toLowerCase() !== sheffieldSurnameLC) continue;

            const matches = genealogyByName.get(householdName) || [];

            for (const genealogy of matches) {
                if (matchedGenealogyIds.has(genealogy.id)) continue;

                // Check if this genealogy record mentions Sheffield person as relation
                if (genealogy.spouse === sheffield.name) {
                    bestMatch = genealogy;
                    matchReason = 'household_relation';
                    break;
                }
            }
            if (bestMatch) break;
        }
    }

    // If we found a match, insert it
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

console.log(`\nMatching complete: ${matchCount.toLocaleString()} matches found\n`);

// Remove matched records from unmatched tables
console.log('Removing matched records from unmatched tables...\n');

const deleteSheffieldStmt = db.prepare('DELETE FROM unmatched_sheffieldcensus WHERE id = ?');
const deleteGenealogyStmt = db.prepare('DELETE FROM unmatched_genealogy WHERE id = ?');

matchedSheffieldIds.forEach(id => deleteSheffieldStmt.run(id));
matchedGenealogyIds.forEach(id => deleteGenealogyStmt.run(id));

console.log(`✓ Removed ${matchedSheffieldIds.size.toLocaleString()} from unmatched_sheffieldcensus`);
console.log(`✓ Removed ${matchedGenealogyIds.size.toLocaleString()} from unmatched_genealogy\n`);

// Final statistics
const remainingSheffield = db.prepare('SELECT COUNT(*) as c FROM unmatched_sheffieldcensus').get().c;
const remainingGenealogy = db.prepare('SELECT COUNT(*) as c FROM unmatched_genealogy').get().c;
const totalPeople = db.prepare('SELECT COUNT(*) as c FROM sheffield_people').get().c;

// Count by match reason
console.log('='.repeat(70));
console.log('MATCH BREAKDOWN');
console.log('='.repeat(70));
const byReason = db.prepare(`
    SELECT source, COUNT(*) as count
    FROM sheffield_people
    WHERE source != 'census_only'
    GROUP BY source
`).all();

byReason.forEach(r => {
    console.log(`  ${r.source}: ${r.count.toLocaleString()}`);
});

console.log('\n' + '='.repeat(70));
console.log('SUMMARY');
console.log('='.repeat(70));
console.log(`\nTotal matched: ${matchCount.toLocaleString()}`);
console.log(`\nRemaining unmatched:`);
console.log(`  Sheffield Census: ${remainingSheffield.toLocaleString()}`);
console.log(`  Genealogy: ${remainingGenealogy.toLocaleString()}`);
console.log(`\nTotal in sheffield_people: ${totalPeople.toLocaleString()}`);

db.close();
console.log('\n✓ Matching complete!');
