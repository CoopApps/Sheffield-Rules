const Database = require('better-sqlite3');
const db = new Database('./Sheffield1867.db');

console.log('Fixing reserve team names to proper title case...\n');

// Function to convert string to title case, preserving abbreviations
function toTitleCase(str) {
  const abbreviations = ['FC', 'XI', 'B'];
  return str
    .split(' ')
    .map(word => {
      if (word.length === 0) return word;
      // Keep abbreviations in uppercase
      if (abbreviations.includes(word.toUpperCase())) {
        return word.toUpperCase();
      }
      return word.charAt(0).toUpperCase() + word.slice(1).toLowerCase();
    })
    .join(' ');
}

// Get all clubs with names that are all uppercase (likely reserve teams)
const allCapsClubs = db.prepare(`
  SELECT id, name
  FROM sheffield_clubs
  WHERE name = UPPER(name)
  AND name != LOWER(name)
  AND (name LIKE '%RESERVE%' OR name LIKE '%JUNIOR%' OR name LIKE '%SECOND%' OR name LIKE '% B%')
`).all();

console.log(`Found ${allCapsClubs.length} clubs with all-caps names\n`);

let updated = 0;
for (const club of allCapsClubs) {
  // Remove " (Reserve)" suffix if it exists
  let cleanName = club.name.replace(' (RESERVE)', '').replace(' (Reserve)', '');

  // Convert to title case
  const newName = toTitleCase(cleanName);

  // Update the database
  db.prepare('UPDATE sheffield_clubs SET name = ? WHERE id = ?').run(newName, club.id);

  console.log(`✓ ${club.name} → ${newName}`);
  updated++;
}

console.log(`\n✓ Successfully updated ${updated} reserve team names to title case`);

db.close();
