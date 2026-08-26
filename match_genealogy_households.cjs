const sqlite3 = require('better-sqlite3');
const fs = require('fs');
const path = require('path');

const db = new sqlite3('Sheffield1867.db', { timeout: 30000 });
const genealogyDir = 'Genealogy';

// Simple CSV parser
function parseCSV(content) {
    const lines = content.split('\n');
    if (lines.length < 2) return [];

    const headers = lines[0].split(',').map(h => h.trim());
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
                values.push(current.trim());
                current = '';
            } else {
                current += char;
            }
        }
        values.push(current.trim());

        const row = {};
        headers.forEach((header, index) => {
            row[header] = values[index] || '';
        });
        rows.push(row);
    }

    return rows;
}

// Normalize text
function normalize(text) {
    if (!text) return '';
    return text.toLowerCase().replace(/[^a-z\s]/g, '').replace(/\s+/g, ' ').trim();
}

// Check if a person name appears in household members string
function isInHousehold(personName, householdMembers) {
    if (!householdMembers || !personName) return false;

    const normalizedPerson = normalize(personName);
    const normalizedHousehold = normalize(householdMembers);

    // Check if the full name appears in household
    return normalizedHousehold.includes(normalizedPerson);
}

console.log('='.repeat(60));
console.log('Household-based genealogy matching');
console.log('='.repeat(60) + '\n');

const genFiles = fs.readdirSync(genealogyDir)
    .filter(f => f.toLowerCase().endsWith('.csv'))
    .sort();

console.log(`Processing ${genFiles.length} genealogy files\n`);

let totalMatches = 0;
let directMatches = 0;
let householdMatches = 0;

// Prepare update statement
const updatePerson = db.prepare(`
    UPDATE sheffield_people
    SET street_address = COALESCE(?, street_address),
        profession = COALESCE(?, profession)
    WHERE id = ?
`);

// Process each genealogy file
for (const filename of genFiles) {
    const filePath = path.join(genealogyDir, filename);
    const content = fs.readFileSync(filePath, 'utf-8');
    const genRecords = parseCSV(content);

    // Group records by address to find households
    const addressGroups = {};
    for (const record of genRecords) {
        const address = record.Address?.trim();
        if (address) {
            if (!addressGroups[address]) {
                addressGroups[address] = [];
            }
            addressGroups[address].push(record);
        }
    }

    let fileDirectMatches = 0;
    let fileHouseholdMatches = 0;

    // Process each address group (household)
    for (const [address, people] of Object.entries(addressGroups)) {
        // For each person at this address in genealogy
        for (const genPerson of people) {
            const genName = genPerson.Name?.trim();
            const genBirthYear = parseInt(genPerson['Born Approx']);
            const genRelation = genPerson.Relation?.trim();
            const genProfession = genPerson.Profession?.trim();

            if (!genName || !genBirthYear || genBirthYear < 1700 || genBirthYear > 1900) {
                continue;
            }

            // STRATEGY 1: Direct name + birth year match
            const directMatch = db.prepare(`
                SELECT id, name, census_household_members
                FROM sheffield_people
                WHERE name = ? AND birth_year = ?
                LIMIT 1
            `).get(genName, genBirthYear);

            if (directMatch) {
                // Direct match found - update with address and profession
                updatePerson.run(address, genProfession, directMatch.id);
                fileDirectMatches++;

                if (fileDirectMatches <= 2) {
                    console.log(`  ✓ Direct: ${genName} (${genBirthYear})`);
                    console.log(`    Address: ${address}`);
                    if (genProfession) console.log(`    Profession: ${genProfession}`);
                }

                // STRATEGY 2: Check if others at same genealogy address are in this person's household
                // This helps match family members
                const householdMembers = directMatch.census_household_members;

                if (householdMembers) {
                    for (const otherGenPerson of people) {
                        if (otherGenPerson === genPerson) continue; // Skip self

                        const otherName = otherGenPerson.Name?.trim();
                        const otherBirthYear = parseInt(otherGenPerson['Born Approx']);
                        const otherProfession = otherGenPerson.Profession?.trim();

                        if (!otherName) continue;

                        // Check if this other person is in the household members list
                        if (isInHousehold(otherName, householdMembers)) {
                            // Find this person in database
                            const householdMember = db.prepare(`
                                SELECT id FROM sheffield_people
                                WHERE name = ? AND birth_year BETWEEN ? AND ?
                                LIMIT 1
                            `).get(otherName, otherBirthYear - 1, otherBirthYear + 1);

                            if (householdMember) {
                                // Update household member with same address
                                updatePerson.run(address, otherProfession, householdMember.id);
                                fileHouseholdMatches++;

                                if (fileHouseholdMatches <= 2) {
                                    console.log(`  ✓ Household: ${otherName} (via ${genName}'s household)`);
                                    console.log(`    Address: ${address}`);
                                    if (otherProfession) console.log(`    Profession: ${otherProfession}`);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if (fileDirectMatches > 0 || fileHouseholdMatches > 0) {
        console.log(`  ${filename}: ${fileDirectMatches} direct + ${fileHouseholdMatches} household = ${fileDirectMatches + fileHouseholdMatches} total\n`);
    }

    directMatches += fileDirectMatches;
    householdMatches += fileHouseholdMatches;
    totalMatches += fileDirectMatches + fileHouseholdMatches;
}

console.log('='.repeat(60));
console.log(`Total matches: ${totalMatches}`);
console.log(`  Direct matches: ${directMatches}`);
console.log(`  Household matches: ${householdMatches}`);

const final = db.prepare(`
    SELECT
        COUNT(*) as total,
        COUNT(street_address) as with_address,
        COUNT(profession) as with_profession,
        COUNT(CASE WHEN census_gender = 'Male' THEN street_address END) as males_with_address,
        COUNT(CASE WHEN census_gender = 'Female' THEN street_address END) as females_with_address
    FROM sheffield_people
`).get();

console.log('\nFinal Statistics:');
console.log(`  Total people: ${final.total}`);
console.log(`  With addresses: ${final.with_address}`);
console.log(`    Males: ${final.males_with_address}`);
console.log(`    Females: ${final.females_with_address}`);
console.log(`  With professions: ${final.with_profession}`);

db.close();
console.log('\n✓ Household matching complete!');
