const Database = require('better-sqlite3');
const fs = require('fs');
const { parse } = require('csv-parse/sync');

const db = new Database('Sheffield1867.db');

console.log('Creating tables for unmatched records...\n');

// Create table for unmatched Sheffield Census records
db.exec(`
  CREATE TABLE IF NOT EXISTS unmatched_sheffield (
    id TEXT PRIMARY KEY,
    year INTEGER NOT NULL,
    name TEXT NOT NULL,
    age INTEGER,
    gender TEXT,
    birth_year INTEGER,
    birth_place TEXT,
    relation TEXT,
    household_members TEXT,
    civil_parish TEXT,
    ecclesiastical_parish TEXT,
    registration_district TEXT,
    county TEXT,
    piece TEXT,
    folio TEXT,
    page TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
  )
`);

// Create table for unmatched Genealogy records
db.exec(`
  CREATE TABLE IF NOT EXISTS unmatched_genealogy (
    id TEXT PRIMARY KEY,
    year INTEGER NOT NULL,
    name TEXT NOT NULL,
    age INTEGER,
    birth_year INTEGER,
    birth_place TEXT,
    address TEXT,
    profession TEXT,
    spouse TEXT,
    relation TEXT,
    parish TEXT,
    area TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
  )
`);

console.log('✓ Tables created');
console.log('\nNow populating with unmatched records from all years...\n');

const crypto = require('crypto');

function normalizeName(name) {
  if (!name) return '';
  return name.toLowerCase()
    .replace(/[.,\-]/g, ' ')
    .replace(/\s+/g, ' ')
    .trim();
}

// Get all matched names
const matchedRecords = db.prepare("SELECT name, source FROM sheffield_people").all();
const matchedNames = new Set(matchedRecords.map(r => normalizeName(r.name)));
const matchedByYear = new Map();
matchedRecords.forEach(r => {
  const year = r.source.match(/^(\d{4})/)?.[1];
  if (year) {
    if (!matchedByYear.has(year)) matchedByYear.set(year, new Set());
    matchedByYear.get(year).add(normalizeName(r.name));
  }
});

const insertSheffield = db.prepare(`
  INSERT OR IGNORE INTO unmatched_sheffield (
    id, year, name, age, gender, birth_year, birth_place, relation,
    household_members, civil_parish, ecclesiastical_parish,
    registration_district, county, piece, folio, page
  ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
`);

const insertGenealogy = db.prepare(`
  INSERT OR IGNORE INTO unmatched_genealogy (
    id, year, name, age, birth_year, birth_place, address,
    profession, spouse, relation, parish, area
  ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
`);

// Years to process
const years = [];
for (let y = 1772; y <= 1852; y++) {
  years.push(y);
}

let totalSheffieldInserted = 0;
let totalGenealogyInserted = 0;

for (const year of years) {
  const yearMatched = matchedByYear.get(String(year)) || new Set();

  // Process Sheffield Census
  const sheffieldPath = `Sheffield Census/${year}.csv`;
  const sheffieldPathDone = `Sheffield Census/${year} - done.csv`;

  let sheffieldFile = null;
  if (fs.existsSync(sheffieldPath)) {
    sheffieldFile = sheffieldPath;
  } else if (fs.existsSync(sheffieldPathDone)) {
    sheffieldFile = sheffieldPathDone;
  }

  if (sheffieldFile) {
    const content = fs.readFileSync(sheffieldFile, 'utf-8');
    const records = parse(content, { columns: true, skip_empty_lines: true });

    let inserted = 0;
    for (const record of records) {
      const normalized = normalizeName(record.NAME);
      if (!normalized || yearMatched.has(normalized)) continue;

      insertSheffield.run(
        crypto.randomUUID(),
        year,
        record.NAME,
        record.AGE ? parseInt(record.AGE) : null,
        record.GENDER,
        record['ESTIMATED BIRTH YEAR'] ? parseInt(record['ESTIMATED BIRTH YEAR']) : null,
        record['Birth Place'],
        record.RELATION,
        record['HOUSEHOLD MEMBERS'],
        record['CIVIL PARISH'],
        record['ECCLESIASTICAL PARISH'],
        record['REGISTRATION DISTRICT'],
        record['COUNTY/ISLAND'],
        record.PIECE,
        record.FOLIO,
        record['PAGE NUMBER']
      );
      inserted++;
    }

    if (inserted > 0) {
      console.log(`${year} Sheffield: ${inserted} unmatched`);
      totalSheffieldInserted += inserted;
    }
  }

  // Process Genealogy
  const genealogyDir = 'Genealogy';
  const genealogyFiles = fs.existsSync(genealogyDir)
    ? fs.readdirSync(genealogyDir).filter(f => f.includes(`tg_${year}_`) && f.endsWith('.csv'))
    : [];

  if (genealogyFiles.length > 0) {
    let inserted = 0;
    for (const file of genealogyFiles) {
      const content = fs.readFileSync(`${genealogyDir}/${file}`, 'utf-8');
      const records = parse(content, { columns: true, skip_empty_lines: true });

      for (const record of records) {
        const normalized = normalizeName(record.Name);
        if (!normalized || yearMatched.has(normalized)) continue;

        insertGenealogy.run(
          crypto.randomUUID(),
          year,
          record.Name,
          record.Age ? parseInt(record.Age) : null,
          record['Born Approx'] ? parseInt(record['Born Approx']) : null,
          record['Birth Place'],
          record.Address,
          record.Profession,
          record.Spouse,
          record.Relation,
          record.Parish,
          record.Area
        );
        inserted++;
      }
    }

    if (inserted > 0) {
      console.log(`${year} Genealogy: ${inserted} unmatched`);
      totalGenealogyInserted += inserted;
    }
  }
}

console.log('\n=== SUMMARY ===');
console.log(`Total unmatched Sheffield Census: ${totalSheffieldInserted.toLocaleString()}`);
console.log(`Total unmatched Genealogy: ${totalGenealogyInserted.toLocaleString()}`);

// Create indexes for faster searching
db.exec('CREATE INDEX IF NOT EXISTS idx_unmatched_sheffield_name ON unmatched_sheffield(name)');
db.exec('CREATE INDEX IF NOT EXISTS idx_unmatched_sheffield_year ON unmatched_sheffield(year)');
db.exec('CREATE INDEX IF NOT EXISTS idx_unmatched_genealogy_name ON unmatched_genealogy(name)');
db.exec('CREATE INDEX IF NOT EXISTS idx_unmatched_genealogy_year ON unmatched_genealogy(year)');

console.log('\n✓ Indexes created');

db.close();
console.log('\nDone! Unmatched records are now in the database.');
console.log('Ready to build the GUI matching tool.');
