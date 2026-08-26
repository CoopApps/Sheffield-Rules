const sqlite3 = require('better-sqlite3');
const fs = require('fs');
const path = require('path');

const db = new sqlite3('Sheffield1867.db', { timeout: 30000 });
const genealogyDir = 'Genealogy';

// CSV parser
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
        headers.forEach((h, i) => row[h] = values[i] || '');
        rows.push(row);
    }

    return rows;
}

// Normalize text
function normalize(text) {
    if (!text) return '';
    return text.toLowerCase().replace(/[^a-z\s]/g, '').replace(/\s+/g, ' ').trim();
}

// Check if person name is in household members string
function isInHousehold(personName, householdMembers) {
    if (!householdMembers || !personName) return false;
    const normalizedPerson = normalize(personName);
    const normalizedHousehold = normalize(householdMembers);
    return normalizedHousehold.includes(normalizedPerson);
}

console.log('='.repeat(60));
console.log('Cross-verified household matching');
console.log('='.repeat(60) + '\n');

const genFiles = fs.readdirSync(genealogyDir)
    .filter(f => f.toLowerCase().endsWith('.csv'))
    .sort();

let totalMatches = 0;
let verifiedMatches = 0;
let unverifiedMatches = 0;

const updatePerson = db.prepare(`
    UPDATE sheffield_people
    SET street_address = COALESCE(?, street_address),
        profession = COALESCE(?, profession)
    WHERE id = ?
`);

for (const filename of genFiles) {
    const filePath = path.join(genealogyDir, filename);
    const content = fs.readFileSync(filePath, 'utf-8');
    const genRecords = parseCSV(content);

    // Group by address to find potential households
    const addressGroups = {};
    for (const record of genRecords) {
        const address = record.Address?.trim();
        if (address) {
            if (!addressGroups[address]) addressGroups[address] = [];
            addressGroups[address].push(record);
        }
    }

    let fileVerified = 0;
    let fileUnverified = 0;

    // Process each address group
    for (const [address, genPeople] of Object.entries(addressGroups)) {
        // Match each person at this address to census data
        const censusMatches = [];

        for (const genPerson of genPeople) {
            const genName = genPerson.Name?.trim();
            const genBirthYear = parseInt(genPerson['Born Approx']);
            const genProfession = genPerson.Profession?.trim();

            if (!genName || !genBirthYear || genBirthYear < 1700 || genBirthYear > 1900) {
                continue;
            }

            // Find potential census match
            const censusPerson = db.prepare(`
                SELECT id, name, birth_year, census_household_members
                FROM sheffield_people
                WHERE name = ? AND birth_year BETWEEN ? AND ?
                LIMIT 1
            `).get(genName, genBirthYear - 1, genBirthYear + 1);

            if (censusPerson) {
                censusMatches.push({
                    genPerson,
                    censusPerson,
                    address,
                    profession: genProfession
                });
            }
        }

        // Now cross-verify: check if people at same address are in each other's households
        for (const match of censusMatches) {
            let verificationScore = 0;
            const reasons = [];

            // Check if OTHER people at this address appear in THIS person's household members
            for (const otherMatch of censusMatches) {
                if (otherMatch === match) continue;

                const otherName = otherMatch.genPerson.Name;
                if (isInHousehold(otherName, match.censusPerson.census_household_members)) {
                    verificationScore++;
                    reasons.push(`${otherName} in household`);
                }
            }

            // If we found household cross-references, this is VERIFIED
            const isVerified = verificationScore > 0;

            // Update the person
            updatePerson.run(match.address, match.profession, match.censusPerson.id);

            if (isVerified) {
                fileVerified++;
                if (fileVerified <= 2) {
                    console.log(`  ✓ VERIFIED: ${match.censusPerson.name} (${match.censusPerson.birth_year})`);
                    console.log(`    Address: ${match.address}`);
                    if (match.profession) console.log(`    Profession: ${match.profession}`);
                    console.log(`    Verification: ${reasons.join(', ')}`);
                }
            } else {
                fileUnverified++;
            }
        }
    }

    if (fileVerified > 0 || fileUnverified > 0) {
        console.log(`  ${filename}: ${fileVerified} verified + ${fileUnverified} unverified = ${fileVerified + fileUnverified} total\n`);
    }

    verifiedMatches += fileVerified;
    unverifiedMatches += fileUnverified;
    totalMatches += fileVerified + fileUnverified;
}

console.log('='.repeat(60));
console.log(`Total matches: ${totalMatches}`);
console.log(`  Verified (household cross-reference): ${verifiedMatches}`);
console.log(`  Unverified (single person): ${unverifiedMatches}`);

const final = db.prepare(`
    SELECT
        COUNT(*) as total,
        COUNT(street_address) as with_address,
        COUNT(profession) as with_profession,
        COUNT(CASE WHEN census_gender = 'Male' AND street_address IS NOT NULL THEN 1 END) as males_with_address,
        COUNT(CASE WHEN census_gender = 'Female' AND street_address IS NOT NULL THEN 1 END) as females_with_address,
        COUNT(CASE WHEN census_relation = 'Head' AND street_address IS NOT NULL THEN 1 END) as heads_with_address,
        COUNT(CASE WHEN census_relation = 'Wife' AND street_address IS NOT NULL THEN 1 END) as wives_with_address
    FROM sheffield_people
`).get();

console.log('\nFinal Statistics:');
console.log(`  Total people: ${final.total}`);
console.log(`  With addresses: ${final.with_address}`);
console.log(`    Males: ${final.males_with_address}`);
console.log(`    Females: ${final.females_with_address}`);
console.log(`    Heads of household: ${final.heads_with_address}`);
console.log(`    Wives: ${final.wives_with_address}`);
console.log(`  With professions: ${final.with_profession}`);

db.close();
console.log('\n✓ Cross-verified matching complete!');
