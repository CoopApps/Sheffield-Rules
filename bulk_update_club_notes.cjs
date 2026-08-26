const fs = require('fs');

// Read and parse the 1857-1875.txt file
const textContent = fs.readFileSync('1857-1875.txt', 'utf-8');
const lines = textContent.split('\n').filter(l => l.trim());

// Build a map of club name -> notes
const notesMap = {};
lines.forEach(line => {
    const parts = line.split('\t');
    if (parts.length >= 3) {
        const clubName = parts[0].trim();
        const notes = parts[2].trim();
        notesMap[clubName] = notes;
    }
});

console.log(`Loaded notes for ${Object.keys(notesMap).length} clubs`);

// Read the Rust clubs.rs file
let rustContent = fs.readFileSync('src-tauri/src/sheffield_rules/clubs.rs', 'utf-8');

// Function to escape strings for use in regex
function escapeRegex(str) {
    return str.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

// Function to escape strings for Rust string literals
function escapeRustString(str) {
    return str.replace(/\\/g, '\\\\').replace(/"/g, '\\"');
}

// Update each club's origin field
let updatesCount = 0;
for (const [clubName, notes] of Object.entries(notesMap)) {
    if (!notes) continue; // Skip empty notes

    // Build regex to find this club's origin field
    // Pattern: name: "ClubName"...origin: "whatever"
    const nameEscaped = escapeRegex(clubName);
    const pattern = new RegExp(
        `(name:\\s*"${nameEscaped}"[\\s\\S]*?origin:\\s*)"[^"]*"`,
        'g'
    );

    const notesEscaped = escapeRustString(notes);
    const replacement = `$1"${notesEscaped}"`;

    const before = rustContent;
    rustContent = rustContent.replace(pattern, replacement);

    if (before !== rustContent) {
        updatesCount++;
        console.log(`✓ Updated: ${clubName}`);
    } else {
        console.log(`✗ Not found: ${clubName}`);
    }
}

// Write the updated content back
fs.writeFileSync('src-tauri/src/sheffield_rules/clubs.rs', rustContent);
console.log(`\nCompleted! Updated ${updatesCount} clubs.`);
