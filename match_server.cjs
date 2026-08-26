const http = require('http');
const Database = require('better-sqlite3');
const crypto = require('crypto');

const PORT = 3000;

const db = new Database('Sheffield1867.db');

// Helper function to parse full name
function parseFullName(fullName) {
  if (!fullName) return { firstName: '', surname: '' };
  const parts = fullName.trim().split(/\s+/);
  if (parts.length === 0) return { firstName: '', surname: '' };
  if (parts.length === 1) return { firstName: parts[0], surname: '' };

  const firstName = parts[0];
  const surname = parts[parts.length - 1];
  const middleName = parts.slice(1, -1).join(' ');

  return { firstName, middleName, surname };
}

// Prepare insert statement
const insertStmt = db.prepare(`
  INSERT INTO sheffield_people (
    id, name, first_name, middle_name, surname,
    birth_year, gender,
    census_age, census_birth_date, census_birth_place, census_county,
    census_relation, census_gender, census_ed, census_household_schedule,
    census_household_members, census_piece, census_folio, census_page,
    where_born, birth_town, birth_county, birth_country,
    civil_parish, ecclesiastical_parish, registration_district, sub_registration_district,
    street_address, profession, spouse_name, relation, parish_area, source
  ) VALUES (
    ?, ?, ?, ?, ?,
    ?, ?,
    ?, ?, ?, ?,
    ?, ?, ?, ?,
    ?, ?, ?, ?,
    ?, ?, ?, ?,
    ?, ?, ?, ?,
    ?, ?, ?, ?, ?, ?
  )
`);

const server = http.createServer((req, res) => {
  // Enable CORS
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Access-Control-Allow-Methods', 'POST, OPTIONS');
  res.setHeader('Access-Control-Allow-Headers', 'Content-Type');

  if (req.method === 'OPTIONS') {
    res.writeHead(200);
    res.end();
    return;
  }

  if (req.method === 'POST' && req.url === '/save-match') {
    let body = '';

    req.on('data', chunk => {
      body += chunk.toString();
    });

    req.on('end', () => {
      try {
        const { sheffield, genealogy } = JSON.parse(body);

        // Use genealogy name as primary
        const fullName = genealogy.Name || sheffield.NAME;
        const { firstName, middleName, surname } = parseFullName(fullName);

        const id = crypto.randomUUID();
        const birthYear = genealogy['Born Approx'] || sheffield['ESTIMATED BIRTH YEAR'] || null;

        insertStmt.run(
          id,
          fullName,
          firstName,
          middleName || null,
          surname,
          birthYear,
          sheffield.GENDER || null,

          // Census data from Sheffield
          sheffield.AGE || null,
          sheffield['Birth Date'] || null,
          sheffield['Birth Place'] || null,
          sheffield['COUNTY/ISLAND'] || null,
          sheffield.RELATION || null,
          sheffield.GENDER || null,
          sheffield['ED, INSTITUTION, OR VESSEL'] || null,
          sheffield['HOUSEHOLD SCHEDULE NUMBER'] || null,
          sheffield['HOUSEHOLD MEMBERS'] || null,
          sheffield.PIECE || null,
          sheffield.FOLIO || null,
          sheffield['PAGE NUMBER'] || null,

          // Birth location
          sheffield['WHERE BORN'] || genealogy['Birth Place'] || null,
          sheffield.TOWN || null,
          sheffield['COUNTY/ISLAND'] || null,
          sheffield.COUNTRY || null,

          // Parish data from Sheffield
          sheffield['CIVIL PARISH'] || null,
          sheffield['ECCLESIASTICAL PARISH'] || null,
          sheffield['REGISTRATION DISTRICT'] || null,
          sheffield['SUB-REGISTRATION DISTRICT'] || null,

          // Address and profession from Genealogy
          genealogy.Address || null,
          genealogy.Profession || null,
          genealogy.Spouse || null,
          genealogy.Relation || sheffield.RELATION || null,
          genealogy.Area || genealogy.Parish || null,
          '1827_matched_manual'
        );

        console.log(`✓ Matched: ${fullName}`);
        console.log(`  Sheffield: "${sheffield.NAME}" | Parish: ${sheffield['ECCLESIASTICAL PARISH']}`);
        console.log(`  Genealogy: ${genealogy.Address} | ${genealogy.Profession || 'no profession'}`);
        console.log();

        res.writeHead(200, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ success: true }));

      } catch (error) {
        console.error('Error saving match:', error);
        res.writeHead(500, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ success: false, error: error.message }));
      }
    });
  } else {
    res.writeHead(404);
    res.end();
  }
});

server.listen(PORT, () => {
  console.log(`\n🚀 Match server running on http://localhost:${PORT}`);
  console.log('\nWaiting for matches from the web interface...');
  console.log('Press Ctrl+C to stop\n');
});

process.on('SIGINT', () => {
  console.log('\n\nClosing database connection...');
  db.close();
  process.exit(0);
});
