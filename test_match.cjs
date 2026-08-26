const Database = require('better-sqlite3');
const fs = require('fs');
const { parse } = require('csv-parse/sync');

const db = new Database('D:/projects/Saturday at Three/Sheffield1867.db', { readonly: true });

// Get some players
const players = db.prepare(`
  SELECT name, birth_year, ecclesiastical_parish, surname
  FROM sheffield_players
  WHERE surname = 'Shaw' AND birth_year BETWEEN 1833 AND 1837
  LIMIT 10
`).all();

console.log('\n=== Players named Shaw born 1833-1837 ===');
players.forEach(p => {
  console.log(`${p.name} (born ${p.birth_year}, parish: ${p.ecclesiastical_parish || 'N/A'})`);
});

// Read genealogy CSV
const csvContent = fs.readFileSync('D:/projects/Saturday at Three/genealogy/tg_1835_S.csv', 'utf-8');
const records = parse(csvContent, { columns: true });

// Find Samuel Shaw
const samuelShaw = records.find(r =>
  r.Name.includes('Samuel') &&
  r.Name.includes('Shaw') &&
  r['Born Approx'] === '1835'
);

if (samuelShaw) {
  console.log('\n=== Genealogy Record ===');
  console.log(`Name: ${samuelShaw.Name}`);
  console.log(`Born: ${samuelShaw['Born Approx']}`);
  console.log(`Address: ${samuelShaw.Address}`);
  console.log(`Parish: ${samuelShaw.Parish}`);
  console.log(`Profession: ${samuelShaw.Profession || 'N/A'}`);
  console.log(`Relation: ${samuelShaw.Relation}`);
}

// Try another - John Robinson
console.log('\n\n=== Players named Robinson born 1833-1837 ===');
const robinsons = db.prepare(`
  SELECT name, birth_year, ecclesiastical_parish
  FROM sheffield_players
  WHERE surname = 'Robinson' AND birth_year BETWEEN 1833 AND 1837
  LIMIT 10
`).all();

robinsons.forEach(p => {
  console.log(`${p.name} (born ${p.birth_year}, parish: ${p.ecclesiastical_parish || 'N/A'})`);
});

const johnRobinson = records.find(r =>
  r.Name.includes('John') &&
  r.Name.includes('Robinson') &&
  r['Born Approx'] === '1835'
);

if (johnRobinson) {
  console.log('\n=== Genealogy Record ===');
  console.log(`Name: ${johnRobinson.Name}`);
  console.log(`Born: ${johnRobinson['Born Approx']}`);
  console.log(`Address: ${johnRobinson.Address}`);
  console.log(`Parish: ${johnRobinson.Parish}`);
  console.log(`Profession: ${johnRobinson.Profession || 'N/A'}`);
}

db.close();
