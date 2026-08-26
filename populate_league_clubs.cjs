const Database = require('better-sqlite3');
const db = new Database('./Sheffield1867.db');

console.log('Populating Division 1 with clubs...\n');

const div1Clubs = [
  'Sheffield FC', 'Hallam FC', 'Norfolk FC', 'Cemetery Road Church FC',
  'York FC', 'Norton FC', 'Pitsmoor FC', 'Fir Vale FC',
  'Attercliffe FC', 'Exchange FC', 'Mechanics FC', 'Broomhall FC',
  'Brightside FC', 'Heeley FC'
];

const clubs = db.prepare('SELECT id, name FROM sheffield_clubs').all();
console.log(`Found ${clubs.length} total clubs in database\n`);

let position = 1;
let assigned = 0;

for (const targetName of div1Clubs) {
  const club = clubs.find(c =>
    c.name.toLowerCase().includes(targetName.toLowerCase().split(' ')[0])
  );

  if (club) {
    const id = `${club.id}-div-1`;
    db.prepare(`
      INSERT OR REPLACE INTO sheffield_league_clubs
      (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
      VALUES (?, ?, ?, ?, ?, ?)
    `).run(id, 'div-1', club.id, position, 0, null);

    console.log(`✓ ${position}. ${club.name} -> Division 1`);
    position++;
    assigned++;
  } else {
    console.log(`✗ Could not find: ${targetName}`);
  }
}

const count = db.prepare('SELECT COUNT(*) as count FROM sheffield_league_clubs WHERE division_id = ?').get('div-1');
console.log(`\n✓ Successfully assigned ${assigned} clubs to Division 1`);
console.log(`Database now contains ${count.count} clubs in Division 1`);

db.close();
