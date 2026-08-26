const fs = require('fs');

// Division structure
const divisions = [
  { id: 'div-1', size: 12 },
  { id: 'div-2', size: 16 },
  { id: 'div-3', size: 18 },
  { id: 'div-4', size: 20 },
  { id: 'div-5a', size: 14 },
  { id: 'div-5b', size: 14 },
  { id: 'div-6a', size: 13 },
  { id: 'div-6b', size: 13 },
  { id: 'div-6c', size: 13 },
  { id: 'div-6d', size: 13 },
  { id: 'div-7a', size: 10 },
  { id: 'div-7b', size: 10 },
  { id: 'div-7c', size: 10 },
  { id: 'div-7d', size: 10 }
];

// Read the clubs data from the generated SQL
const sqlContent = fs.readFileSync('./populate_sheffield_clubs_with_reserves.sql', 'utf8');

// Extract main clubs (not reserves) with their founding years
const clubPattern = /INSERT INTO sheffield_clubs \(id, name, short_name, founded_year[^)]+\) VALUES \('([^']+)', '([^']+)', '([^']+)', (\d+),[^)]+, 0\);/g;
const clubs = [];
let match;

while ((match = clubPattern.exec(sqlContent)) !== null) {
  clubs.push({
    id: match[1],
    name: match[2],
    year: parseInt(match[4])
  });
}

console.log(`Found ${clubs.length} main clubs`);

// Sort by founding year (oldest first), then by name
clubs.sort((a, b) => {
  if (a.year !== b.year) return a.year - b.year;
  return a.name.localeCompare(b.name);
});

// Assign clubs to divisions
let sql = `-- Assign clubs to pyramid based on founding year
-- Oldest clubs in top divisions, newest in bottom divisions

DELETE FROM sheffield_league_clubs;
DELETE FROM sheffield_league_divisions;

-- Create division tables
`;

// Create divisions
divisions.forEach(div => {
  const level = parseInt(div.id.match(/\d+/)[0]);
  const variant = div.id.match(/[a-d]$/)?.[0] || '';
  let name, region = 'NULL';

  if (div.id === 'div-1') name = 'First Division';
  else if (div.id === 'div-2') name = 'Second Division';
  else if (div.id === 'div-3') name = 'Third Division';
  else if (div.id === 'div-4') name = 'Fourth Division';
  else if (div.id === 'div-5a') name = 'Fifth Division A';
  else if (div.id === 'div-5b') name = 'Fifth Division B';
  else if (div.id === 'div-6a') { name = 'Sixth Division West'; region = "'West'"; }
  else if (div.id === 'div-6b') { name = 'Sixth Division East'; region = "'East'"; }
  else if (div.id === 'div-6c') { name = 'Sixth Division North'; region = "'North'"; }
  else if (div.id === 'div-6d') { name = 'Sixth Division South'; region = "'South'"; }
  else if (div.id === 'div-7a') { name = 'Seventh Division West'; region = "'West'"; }
  else if (div.id === 'div-7b') { name = 'Seventh Division East'; region = "'East'"; }
  else if (div.id === 'div-7c') { name = 'Seventh Division North'; region = "'North'"; }
  else if (div.id === 'div-7d') { name = 'Seventh Division South'; region = "'South'"; }

  sql += `INSERT INTO sheffield_league_divisions (id, name, level, region) VALUES ('${div.id}', '${name}', ${level}, ${region});\n`;
});

// Create reserve divisions
divisions.forEach(div => {
  const level = parseInt(div.id.match(/\d+/)[0]);
  const variant = div.id.match(/[a-d]$/)?.[0]?.toUpperCase() || '';
  let name, region = 'NULL';

  if (div.id === 'div-1') name = 'Reserve Division 1';
  else if (div.id === 'div-2') name = 'Reserve Division 2';
  else if (div.id === 'div-3') name = 'Reserve Division 3';
  else if (div.id === 'div-4') name = 'Reserve Division 4';
  else if (div.id === 'div-5a') name = 'Reserve Division 5A';
  else if (div.id === 'div-5b') name = 'Reserve Division 5B';
  else if (div.id === 'div-6a') { name = 'Reserve Division 6A'; region = "'West'"; }
  else if (div.id === 'div-6b') { name = 'Reserve Division 6B'; region = "'East'"; }
  else if (div.id === 'div-6c') { name = 'Reserve Division 6C'; region = "'North'"; }
  else if (div.id === 'div-6d') { name = 'Reserve Division 6D'; region = "'South'"; }
  else if (div.id === 'div-7a') { name = 'Reserve Division 7A'; region = "'West'"; }
  else if (div.id === 'div-7b') { name = 'Reserve Division 7B'; region = "'East'"; }
  else if (div.id === 'div-7c') { name = 'Reserve Division 7C'; region = "'North'"; }
  else if (div.id === 'div-7d') { name = 'Reserve Division 7D'; region = "'South'"; }

  sql += `INSERT INTO sheffield_league_divisions (id, name, level, region) VALUES ('res-${div.id}', '${name}', ${level}, ${region});\n`;
});

sql += '\n-- Assign clubs to divisions\n';

let clubIndex = 0;
divisions.forEach((div, divIdx) => {
  sql += `\n-- ${div.id}\n`;

  for (let pos = 1; pos <= div.size && clubIndex < clubs.length; pos++) {
    const club = clubs[clubIndex];
    const reserveId = `${club.id}-reserves`;

    // Main club
    sql += `INSERT INTO sheffield_league_clubs (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id) VALUES ('${club.id}-${div.id}', '${div.id}', '${club.id}', ${pos}, 0, NULL);\n`;

    // Reserve club
    sql += `INSERT INTO sheffield_league_clubs (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id) VALUES ('${reserveId}-res-${div.id}', 'res-${div.id}', '${reserveId}', ${pos}, 1, '${club.id}');\n`;

    clubIndex++;
  }
});

sql += '\nSELECT COUNT(*) as assigned_clubs FROM sheffield_league_clubs;\n';
sql += 'SELECT division_id, COUNT(*) as clubs FROM sheffield_league_clubs WHERE is_reserve_team = 0 GROUP BY division_id ORDER BY division_id;\n';

fs.writeFileSync('./assign_pyramid.sql', sql);
console.log(`✓ Generated assignment SQL for ${clubIndex} clubs across ${divisions.length} divisions`);
console.log(`✓ Total assignments: ${clubIndex * 2} (including reserves)`);
