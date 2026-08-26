const sqlite3 = require('better-sqlite3');
const XLSX = require('xlsx');

const db = new sqlite3('Sheffield1867.db', { timeout: 30000 });

console.log('='.repeat(60));
console.log('Mapping historical streets to modern postcodes');
console.log('='.repeat(60) + '\n');

// Read the Excel file
console.log('Reading postcodes.xlsx...');
const workbook = XLSX.readFile('postcodes.xlsx');
const sheetName = workbook.SheetNames[0];
const sheet = workbook.Sheets[sheetName];
const data = XLSX.utils.sheet_to_json(sheet);

console.log(`Loaded ${data.length} postcode records\n`);

// Show sample of what we have
console.log('Sample postcode data:');
data.slice(0, 5).forEach((row, i) => {
    console.log(`${i + 1}.`, Object.keys(row).map(k => `${k}: ${row[k]}`).join(', '));
});
console.log('');

// Get column names
const columns = data.length > 0 ? Object.keys(data[0]) : [];
console.log('Columns available:', columns.join(', '));
console.log('');

// Create mapping function (we'll need to see the data structure first to build proper logic)
console.log('Building street to postcode mapping...');

// Get unique streets from sheffield_people
const streets = db.prepare(`
    SELECT DISTINCT street_address
    FROM sheffield_people
    WHERE street_address IS NOT NULL
`).all();

console.log(`Found ${streets.length} unique street addresses in database`);
console.log('\nSample streets:');
streets.slice(0, 10).forEach(s => console.log('  - ' + s.street_address));

db.close();
console.log('\n✓ Analysis complete. Next: build matching logic based on data structure');
