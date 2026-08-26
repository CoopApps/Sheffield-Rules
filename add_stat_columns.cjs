const sqlite3 = require('sqlite3').verbose();
const db = new sqlite3.Database('./Sheffield1867.db');

console.log('Adding stat columns to sheffield_footballers table...\n');

const columns = [
  'position TEXT',
  'nationality TEXT',
  // Physical
  'pace INTEGER',
  'acceleration INTEGER',
  'strength INTEGER',
  'stamina INTEGER',
  'balance INTEGER',
  'jumping INTEGER',
  'agility INTEGER',
  'natural_fitness INTEGER',
  // Technical
  'passing INTEGER',
  'dribbling INTEGER',
  'first_touch INTEGER',
  'technique INTEGER',
  'heading INTEGER',
  'long_passing INTEGER',
  'crossing INTEGER',
  'long_shots INTEGER',
  'tackling INTEGER',
  'handling INTEGER',
  'reflexes INTEGER',
  'corners INTEGER',
  'free_kicks INTEGER',
  'throw_ins INTEGER',
  'vision INTEGER',
  'left_foot INTEGER',
  'right_foot INTEGER',
  'one_on_ones INTEGER',
  // Mental
  'courage INTEGER',
  'bravery INTEGER',
  'concentration INTEGER',
  'decision_making INTEGER',
  'leadership INTEGER',
  'aggression INTEGER',
  'anticipation INTEGER',
  'determination INTEGER',
  'flair INTEGER',
  'influence INTEGER',
  'adaptability INTEGER',
  'ambition INTEGER',
  'loyalty INTEGER',
  'pressure INTEGER',
  'professionalism INTEGER',
  'sportsmanship INTEGER',
  'temperament INTEGER',
  // Positioning
  'awareness INTEGER',
  'marking INTEGER',
  'positioning INTEGER',
  'work_rate INTEGER',
  'off_the_ball INTEGER',
  'movement INTEGER',
  'teamwork INTEGER',
  // Specialization
  'finishing INTEGER',
  'penalties INTEGER',
  'set_pieces INTEGER',
  // Hidden
  'consistency INTEGER',
  'dirtiness INTEGER',
  'versatility INTEGER',
  'injury_proneness INTEGER',
  'important_matches INTEGER',
  // Ability & Reputation
  'current_ability INTEGER',
  'potential_ability INTEGER',
  'current_reputation INTEGER'
];

let completed = 0;
let errors = 0;

function addNextColumn(index) {
  if (index >= columns.length) {
    console.log(`\n✓ Completed! Added ${completed} columns`);
    if (errors > 0) {
      console.log(`⚠ ${errors} columns already existed`);
    }
    db.close();
    return;
  }

  const column = columns[index];
  const columnName = column.split(' ')[0];

  db.run(`ALTER TABLE sheffield_footballers ADD COLUMN ${column}`, (err) => {
    if (err) {
      if (err.message.includes('duplicate column name')) {
        console.log(`⊘ ${columnName} - already exists`);
        errors++;
      } else {
        console.error(`✗ ${columnName} - Error:`, err.message);
        errors++;
      }
    } else {
      console.log(`✓ ${columnName}`);
      completed++;
    }
    addNextColumn(index + 1);
  });
}

addNextColumn(0);
