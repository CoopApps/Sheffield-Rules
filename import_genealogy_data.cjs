const sqlite3 = require('better-sqlite3');
const fs = require('fs');
const path = require('path');

const db = new sqlite3('Sheffield1867.db');
const genealogyDir = 'Genealogy';

// Simple CSV parser function
function parseCSV(content) {
    const lines = content.split('\n');
    if (lines.length < 2) return [];

    const headers = lines[0].split(',').map(h => h.trim());
    const rows = [];

    for (let i = 1; i < lines.length; i++) {
        const line = lines[i].trim();
        if (!line) continue;

        // Parse CSV properly handling quoted fields with commas
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

// Normalize name for comparison
function normalizeName(name) {
    return name.toLowerCase()
        .replace(/[^a-z\s]/g, '')
        .replace(/\s+/g, ' ')
        .trim();
}

// Extract surnames from household members string
function extractHouseholdSurnames(householdMembers) {
    if (!householdMembers) return [];

    // Format: "Name Age | John Smith 34 | Mary Smith 30 | ..."
    const members = householdMembers.split('|').map(m => m.trim());
    const surnames = new Set();

    for (const member of members) {
        // Extract name (before the age number)
        const parts = member.split(/\s+/);
        if (parts.length >= 2) {
            // Last word before numbers is likely the surname
            for (let i = parts.length - 1; i >= 0; i--) {
                if (!/^\d+$/.test(parts[i]) && parts[i].length > 1) {
                    surnames.add(normalizeName(parts[i]));
                    break;
                }
            }
        }
    }

    return Array.from(surnames);
}

// Calculate match confidence score
function calculateMatchConfidence(censusPlayer, genealogyRecord) {
    let score = 0;
    let reasons = [];

    // Name match (required)
    const censusName = normalizeName(censusPlayer.name);
    const genName = normalizeName(genealogyRecord.Name);

    if (censusName === genName) {
        score += 40;
        reasons.push('exact name match');
    } else if (censusName.includes(genName) || genName.includes(censusName)) {
        score += 20;
        reasons.push('partial name match');
    } else {
        return { score: 0, reasons: ['name mismatch'] };
    }

    // Birth year match (required)
    const censusBirthYear = censusPlayer.birth_year;
    const genBirthYear = parseInt(genealogyRecord['Born Approx']);

    if (censusBirthYear === genBirthYear) {
        score += 40;
        reasons.push('exact birth year');
    } else if (Math.abs(censusBirthYear - genBirthYear) <= 1) {
        score += 30;
        reasons.push('birth year ±1');
    } else {
        return { score: 0, reasons: ['birth year mismatch'] };
    }

    // Household members match (strong confirmation)
    const householdSurnames = extractHouseholdSurnames(censusPlayer.census_household_members);
    const genSurname = normalizeName(genealogyRecord.Name.split(/\s+/).pop());

    if (householdSurnames.includes(genSurname)) {
        score += 20;
        reasons.push('surname in household');
    }

    // Parish match (additional confirmation)
    const censusParish = normalizeName(censusPlayer.ecclesiastical_parish || censusPlayer.civil_parish || '');
    const genParish = normalizeName(genealogyRecord.Parish || '');

    if (censusParish && genParish && censusParish === genParish) {
        score += 10;
        reasons.push('parish match');
    } else if (censusParish && genParish && (censusParish.includes(genParish) || genParish.includes(censusParish))) {
        score += 5;
        reasons.push('parish partial match');
    }

    return { score, reasons };
}

console.log('Adding missing genealogy columns to database...\n');

// Add columns for genealogy data
const genealogyColumns = [
    { name: 'street_address', type: 'TEXT' },
    { name: 'profession', type: 'TEXT' }
];

for (const column of genealogyColumns) {
    try {
        db.prepare(`ALTER TABLE sheffield_players ADD COLUMN ${column.name} ${column.type}`).run();
        console.log(`✓ Added column: ${column.name}`);
    } catch (error) {
        if (error.message.includes('duplicate column name')) {
            console.log(`○ Column already exists: ${column.name}`);
        } else {
            console.error(`✗ Error adding ${column.name}:`, error.message);
        }
    }
}

console.log('\n' + '='.repeat(60) + '\n');
console.log('Starting genealogy data import and matching...\n');

// Get all genealogy CSV files
const genFiles = fs.readdirSync(genealogyDir)
    .filter(f => f.toLowerCase().endsWith('.csv'))
    .sort();

console.log(`Found ${genFiles.length} genealogy CSV files\n`);

let totalFiles = genFiles.length;
let processedFiles = 0;
let totalMatches = 0;
let highConfidenceMatches = 0;
let errors = [];

// Process each file
for (const filename of genFiles) {
    try {
        console.log(`Processing ${filename}...`);

        const filePath = path.join(genealogyDir, filename);
        const content = fs.readFileSync(filePath, 'utf-8');
        const genRecords = parseCSV(content);

        let fileMatches = 0;
        let fileHighConfidence = 0;

        for (const genRecord of genRecords) {
            const name = genRecord.Name;
            const birthYear = parseInt(genRecord['Born Approx']);

            if (!name || !birthYear || birthYear < 1700 || birthYear > 1900) {
                continue;
            }

            // Find potential matches in census data
            const potentialMatches = db.prepare(`
                SELECT id, name, birth_year, census_household_members,
                       ecclesiastical_parish, civil_parish
                FROM sheffield_players
                WHERE birth_year = ?
                AND is_real_player = 1
            `).all(birthYear);

            let bestMatch = null;
            let bestScore = 0;

            for (const censusPlayer of potentialMatches) {
                const { score, reasons } = calculateMatchConfidence(censusPlayer, genRecord);

                if (score > bestScore) {
                    bestScore = score;
                    bestMatch = { player: censusPlayer, score, reasons };
                }
            }

            // Only update if we have a confident match (score >= 70)
            if (bestMatch && bestScore >= 70) {
                const address = genRecord.Address || null;
                const profession = genRecord.Profession || null;

                // Update the player with genealogy data
                db.prepare(`
                    UPDATE sheffield_players
                    SET street_address = COALESCE(?, street_address),
                        profession = COALESCE(?, profession)
                    WHERE id = ?
                `).run(address, profession, bestMatch.player.id);

                fileMatches++;
                if (bestScore >= 90) {
                    fileHighConfidence++;
                }

                if (fileMatches <= 3) {
                    console.log(`  ✓ Matched "${name}" (score: ${bestScore}) - ${bestMatch.reasons.join(', ')}`);
                    if (address) console.log(`    Address: ${address}`);
                    if (profession) console.log(`    Profession: ${profession}`);
                }
            }
        }

        totalMatches += fileMatches;
        highConfidenceMatches += fileHighConfidence;
        processedFiles++;

        console.log(`  → ${fileMatches} matches (${fileHighConfidence} high confidence)\n`);

    } catch (error) {
        console.error(`  ✗ Error processing ${filename}:`, error.message);
        errors.push(`${filename}: ${error.message}`);
    }
}

console.log('='.repeat(60) + '\n');
console.log('Import Complete!\n');
console.log(`Files processed: ${processedFiles}/${totalFiles}`);
console.log(`Total matches: ${totalMatches}`);
console.log(`High confidence matches (≥90): ${highConfidenceMatches}`);

if (errors.length > 0) {
    console.log(`\nErrors: ${errors.length}`);
    errors.forEach(err => console.log(`  - ${err}`));
}

// Show final statistics
const stats = db.prepare(`
    SELECT
        COUNT(*) as total,
        COUNT(street_address) as with_address,
        COUNT(profession) as with_profession
    FROM sheffield_players
    WHERE is_real_player = 1
`).get();

console.log('\nFinal Statistics:');
console.log(`  Total real players: ${stats.total}`);
console.log(`  With street address: ${stats.with_address}`);
console.log(`  With profession: ${stats.with_profession}`);

db.close();
console.log('\nDatabase connection closed.');
