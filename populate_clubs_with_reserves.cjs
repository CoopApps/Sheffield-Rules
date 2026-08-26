// Populate Sheffield clubs with reserves using era-appropriate names
const fs = require('fs');
const { exec } = require('child_process');

console.log('\n=== Extracting clubs and creating reserves ===\n');

const clubsFile = fs.readFileSync('./src-tauri/src/sheffield_rules/clubs.rs', 'utf8');

// Parse clubs
const clubs = [];
const clubBlocks = clubsFile.split('SheffieldClub {').slice(1);

for (const block of clubBlocks) {
    const idMatch = block.match(/id:\s*"([^"]+)"/);
    const nameMatch = block.match(/name:\s*"([^"]+)"/);
    const yearMatch = block.match(/founded_year:\s*(\d+)/);
    const groundMatch = block.match(/ground:\s*"([^"]+)"/);
    const originMatch = block.match(/origin:\s*"([^"]+)"/);
    const cityMatch = block.match(/city:\s*Some\("([^"]+)"\)/);
    const regionMatch = block.match(/region:\s*Some\("([^"]+)"\)/);

    if (idMatch && nameMatch && yearMatch && groundMatch && originMatch) {
        clubs.push({
            id: idMatch[1],
            name: nameMatch[1],
            founded_year: parseInt(yearMatch[1]),
            ground: groundMatch[1],
            origin: originMatch[1],
            city: cityMatch ? cityMatch[1] : null,
            region: regionMatch ? regionMatch[1] : null
        });
    }
}

console.log(`Found ${clubs.length} clubs\n`);

// Era-appropriate reserve team suffixes
const reserveSuffixes = [
    'Reserves',
    'Second XI',
    'B Team',
    'Juniors',
    'Second Team',
    'Reserve XI',
    'Junior XI',
    'Colts'
];

// Generate SQL for main clubs and reserves
const sqlStatements = [];

for (let i = 0; i < clubs.length; i++) {
    const club = clubs[i];

    // Escape single quotes
    const id = club.id.replace(/'/g, "''");
    const name = club.name.replace(/'/g, "''");
    const ground = club.ground.replace(/'/g, "''");
    const origin = club.origin.replace(/'/g, "''");
    const city = club.city ? `'${club.city.replace(/'/g, "''")}'` : 'NULL';
    const region = club.region ? `'${club.region.replace(/'/g, "''")}'` : 'NULL';

    // Main club
    sqlStatements.push(
        `INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) ` +
        `VALUES ('${id}', '${name}', ${club.founded_year}, '${ground}', '${origin}', ${city}, ${region});`
    );

    // Reserve team - use different suffix for variety
    const suffix = reserveSuffixes[i % reserveSuffixes.length];
    const reserveId = `${id}-reserves`;
    const reserveName = `${name} ${suffix}`;

    sqlStatements.push(
        `INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) ` +
        `VALUES ('${reserveId}', '${reserveName.replace(/'/g, "''")}', ${club.founded_year}, '${ground}', '${origin} (${suffix})', ${city}, ${region});`
    );
}

const sqlFile = `-- Sheffield Clubs Data with Reserves
-- Auto-generated from src-tauri/src/sheffield_rules/clubs.rs
-- ${clubs.length} main clubs + ${clubs.length} reserve teams = ${clubs.length * 2} total

${sqlStatements.join('\n')}

-- Verify
SELECT COUNT(*) as total_clubs FROM sheffield_clubs;
`;

fs.writeFileSync('./populate_clubs_with_reserves.sql', sqlFile);
console.log(`✓ Generated populate_clubs_with_reserves.sql`);
console.log(`  Main clubs: ${clubs.length}`);
console.log(`  Reserve teams: ${clubs.length}`);
console.log(`  Total: ${clubs.length * 2}\n`);
