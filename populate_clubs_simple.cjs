// Simple script to populate Sheffield clubs from the Rust source
const fs = require('fs');
const { exec } = require('child_process');

console.log('\n=== Extracting clubs from Rust source ===\n');

// Read the Rust clubs file
const clubsFile = fs.readFileSync('./src-tauri/src/sheffield_rules/clubs.rs', 'utf8');

// Parse clubs using a simpler approach - split by SheffieldClub
const clubs = [];
const clubBlocks = clubsFile.split('SheffieldClub {').slice(1); // Skip first empty part

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

// Generate SQL INSERT statements
const sqlStatements = clubs.map(club => {
    const id = club.id.replace(/'/g, "''");
    const name = club.name.replace(/'/g, "''");
    const ground = club.ground.replace(/'/g, "''");
    const origin = club.origin.replace(/'/g, "''");
    const city = club.city ? `'${club.city.replace(/'/g, "''")}'` : 'NULL';
    const region = club.region ? `'${club.region.replace(/'/g, "''")}'` : 'NULL';

    return `INSERT OR REPLACE INTO sheffield_clubs (id, name, founded_year, ground_name, origin, city, region) VALUES ('${id}', '${name}', ${club.founded_year}, '${ground}', '${origin}', ${city}, ${region});`;
}).join('\n');

// Write SQL file
const sqlFile = `-- Sheffield Clubs Data
-- Auto-generated from src-tauri/src/sheffield_rules/clubs.rs

${sqlStatements}

-- Verify
SELECT COUNT(*) as total_clubs FROM sheffield_clubs;
`;

fs.writeFileSync('./populate_clubs.sql', sqlFile);
console.log('✓ Generated populate_clubs.sql\n');

// Execute SQL using sqlite3
const dbPath = 'D:/projects/Saturday at Three/Sheffield1867.db';

// Try to find sqlite3
exec('where sqlite3', (err, stdout) => {
    if (err || !stdout.trim()) {
        console.log('Note: sqlite3 not found in PATH');
        console.log('\nGenerated SQL file: populate_clubs.sql');
        console.log(`\nTo populate, run: sqlite3 "${dbPath}" < populate_clubs.sql\n`);
        console.log(`Or use: type populate_clubs.sql | sqlite3 "${dbPath}"\n`);
    } else {
        console.log('Found sqlite3, executing SQL...\n');
        exec(`type populate_clubs.sql | sqlite3 "${dbPath}"`, (err, stdout, stderr) => {
            if (err) {
                console.error('Error executing SQL:', err);
                console.log('\nTry manually: type populate_clubs.sql | sqlite3 "' + dbPath + '"\n');
            } else {
                console.log('✓ Clubs populated successfully!');
                if (stdout) console.log(stdout);
                if (stderr) console.error(stderr);
            }
        });
    }
});
