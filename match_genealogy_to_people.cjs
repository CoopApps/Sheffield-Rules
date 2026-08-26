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

        // Parse CSV properly handling quoted fields
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

// Normalize text for comparison
function normalize(text) {
    if (!text) return '';
    return text.toLowerCase().replace(/[^a-z\s]/g, '').replace(/\s+/g, ' ').trim();
}

// Extract surnames from household members
function extractHouseholdSurnames(householdMembers) {
    if (!householdMembers) return [];

    const members = householdMembers.split('|').map(m => m.trim());
    const surnames = new Set();

    for (const member of members) {
        const parts = member.split(/\s+/);
        if (parts.length >= 2) {
            // Last word before numbers is likely surname
            for (let i = parts.length - 1; i >= 0; i--) {
                if (!/^\d+$/.test(parts[i]) && parts[i].length > 1) {
                    surnames.add(normalize(parts[i]));
                    break;
                }
            }
        }
    }

    return Array.from(surnames);
}

// Calculate comprehensive match confidence
function calculateMatchScore(person, genRecord) {
    let score = 0;
    const reasons = [];

    // 1. NAME MATCH (40 points max)
    const personName = normalize(person.name);
    const genName = normalize(genRecord.Name);

    if (personName === genName) {
        score += 40;
        reasons.push('exact name');
    } else if (personName.includes(genName) || genName.includes(personName)) {
        score += 25;
        reasons.push('partial name');
    } else {
        // No match - not the same person
        return { score: 0, reasons: ['name mismatch'] };
    }

    // 2. BIRTH YEAR MATCH (30 points max)
    const personBirthYear = person.birth_year;
    const genBirthYear = parseInt(genRecord['Born Approx']);

    if (personBirthYear === genBirthYear) {
        score += 30;
        reasons.push('exact birth year');
    } else if (Math.abs(personBirthYear - genBirthYear) <= 1) {
        score += 20;
        reasons.push('birth year ±1');
    } else if (Math.abs(personBirthYear - genBirthYear) <= 2) {
        score += 10;
        reasons.push('birth year ±2');
    } else {
        // Birth year too far off
        return { score: 0, reasons: ['birth year mismatch'] };
    }

    // 3. HOUSEHOLD SURNAME MATCH (15 points)
    const householdSurnames = extractHouseholdSurnames(person.census_household_members);
    const genSurname = normalize(genRecord.Name.split(/\s+/).pop());

    if (householdSurnames.includes(genSurname)) {
        score += 15;
        reasons.push('household surname match');
    }

    // 4. PARISH MATCH (10 points)
    const personParish = normalize(person.ecclesiastical_parish || person.civil_parish || '');
    const genParish = normalize(genRecord.Parish || '');

    if (personParish && genParish) {
        if (personParish === genParish) {
            score += 10;
            reasons.push('parish exact');
        } else if (personParish.includes(genParish) || genParish.includes(personParish)) {
            score += 5;
            reasons.push('parish partial');
        }
    }

    // 5. RELATION MATCH (3 points)
    const personRelation = normalize(person.census_relation || '');
    const genRelation = normalize(genRecord.Relation || '');

    if (personRelation && genRelation && personRelation === genRelation) {
        score += 3;
        reasons.push('relation match');
    }

    // 6. GENDER CONSISTENCY (2 points)
    const personGender = normalize(person.census_gender || '');
    const genGender = genRecord.Name.toLowerCase().includes(' mrs ') ||
                       genRecord.Name.toLowerCase().includes(' miss ') ? 'female' : '';

    if (personGender && genGender && personGender.startsWith(genGender.charAt(0))) {
        score += 2;
        reasons.push('gender match');
    }

    return { score, reasons };
}

console.log('='.repeat(60));
console.log('Matching genealogy data to people...');
console.log('='.repeat(60) + '\n');

const genFiles = fs.readdirSync(genealogyDir)
    .filter(f => f.toLowerCase().endsWith('.csv'))
    .sort();

console.log(`Found ${genFiles.length} genealogy CSV files\n`);

let totalMatches = 0;
let highConfidenceMatches = 0;
let mediumConfidenceMatches = 0;

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

    let fileMatches = 0;
    let fileHigh = 0;
    let fileMedium = 0;

    for (const genRecord of genRecords) {
        const name = genRecord.Name;
        const birthYear = parseInt(genRecord['Born Approx']);

        if (!name || !birthYear || birthYear < 1700 || birthYear > 1900) {
            continue;
        }

        // Get potential matches from same birth year
        const potentialMatches = db.prepare(`
            SELECT id, name, birth_year, census_household_members,
                   ecclesiastical_parish, civil_parish,
                   census_relation, census_gender
            FROM sheffield_people
            WHERE birth_year BETWEEN ? AND ?
        `).all(birthYear - 2, birthYear + 2);

        let bestMatch = null;
        let bestScore = 0;

        for (const person of potentialMatches) {
            const { score, reasons } = calculateMatchScore(person, genRecord);

            if (score > bestScore) {
                bestScore = score;
                bestMatch = { person, score, reasons };
            }
        }

        // Update if confidence >= 70
        if (bestMatch && bestScore >= 70) {
            const address = genRecord.Address || null;
            const profession = genRecord.Profession || null;

            if (address || profession) {
                updatePerson.run(address, profession, bestMatch.person.id);

                fileMatches++;
                if (bestScore >= 90) {
                    fileHigh++;
                } else {
                    fileMedium++;
                }

                // Show first few matches
                if (fileMatches <= 2) {
                    console.log(`  ✓ ${name} (score: ${bestScore}) - ${bestMatch.reasons.join(', ')}`);
                    if (address) console.log(`    Address: ${address}`);
                    if (profession) console.log(`    Profession: ${profession}`);
                }
            }
        }
    }

    if (fileMatches > 0) {
        console.log(`  ${filename}: ${fileMatches} matches (${fileHigh} high, ${fileMedium} medium)\n`);
    }

    totalMatches += fileMatches;
    highConfidenceMatches += fileHigh;
    mediumConfidenceMatches += fileMedium;
}

console.log('='.repeat(60));
console.log(`Total matches: ${totalMatches}`);
console.log(`  High confidence (≥90): ${highConfidenceMatches}`);
console.log(`  Medium confidence (70-89): ${mediumConfidenceMatches}`);

const final = db.prepare(`
    SELECT
        COUNT(*) as total,
        COUNT(street_address) as with_address,
        COUNT(profession) as with_profession
    FROM sheffield_people
`).get();

console.log('\nFinal Statistics:');
console.log(`  Total people: ${final.total}`);
console.log(`  With address: ${final.with_address}`);
console.log(`  With profession: ${final.with_profession}`);

db.close();
console.log('\n✓ Genealogy matching complete!');
