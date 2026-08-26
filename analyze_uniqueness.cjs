const sqlite3 = require('better-sqlite3');
const fs = require('fs');
const csv = require('csv-parser');

console.log('=== ANALYZING DATA UNIQUENESS ===\n');

// Check what makes people unique in the sheffield census data
console.log('Reading sample from sheffield census/1838.csv...\n');

const people = [];
fs.createReadStream('./sheffield census/1838.csv')
  .pipe(csv())
  .on('data', (row) => {
    people.push({
      name: row.Name,
      age: row.AGE,
      birthYear: row['Birth Date'],
      address: row['HOUSEHOLD SCHEDULE NUMBER'],
      civilParish: row['CIVIL PARISH'],
      ecclesParish: row['ECCLESIASTICAL PARISH'],
      folio: row.FOLIO,
      page: row['PAGE NUMBER'],
      piece: row.PIECE,
      relation: row.RELATION,
      household: row['HOUSEHOLD MEMBERS']
    });
  })
  .on('end', () => {
    console.log(`Total rows in 1838.csv: ${people.length}\n`);

    // Count duplicates by name only
    const nameCount = {};
    people.forEach(p => {
      nameCount[p.name] = (nameCount[p.name] || 0) + 1;
    });

    const duplicateNames = Object.entries(nameCount)
      .filter(([name, count]) => count > 1)
      .sort((a, b) => b[1] - a[1])
      .slice(0, 10);

    console.log('Top 10 duplicate names:');
    duplicateNames.forEach(([name, count]) => {
      console.log(`  ${name}: ${count} occurrences`);
    });

    // Show one example of duplicates
    if (duplicateNames.length > 0) {
      const exampleName = duplicateNames[0][0];
      console.log(`\n=== Example: All records for "${exampleName}" ===`);
      const examples = people.filter(p => p.name === exampleName);
      examples.forEach((p, i) => {
        console.log(`\nRecord ${i + 1}:`);
        console.log(`  Age: ${p.age}, Birth Year: ${p.birthYear}`);
        console.log(`  Civil Parish: ${p.civilParish}`);
        console.log(`  Ecclesiastical Parish: ${p.ecclesParish}`);
        console.log(`  Relation: ${p.relation}`);
        console.log(`  Piece/Folio/Page: ${p.piece}/${p.folio}/${p.page}`);
        console.log(`  Household Schedule: ${p.address}`);
        console.log(`  Household Members: ${p.household?.substring(0, 80)}...`);
      });
    }

    // Propose unique key
    console.log('\n=== PROPOSED UNIQUE KEY ===');
    console.log('Combination of:');
    console.log('  1. Name');
    console.log('  2. Birth Year');
    console.log('  3. Census Piece + Folio + Page (exact census location)');
    console.log('  4. Household Schedule Number (within that page)');

    // Test uniqueness with proposed key
    const uniqueKeys = new Set();
    const duplicates = [];
    people.forEach(p => {
      const key = `${p.name}|${p.birthYear}|${p.piece}|${p.folio}|${p.page}|${p.address}`;
      if (uniqueKeys.has(key)) {
        duplicates.push(key);
      }
      uniqueKeys.add(key);
    });

    console.log(`\nWith this key:`);
    console.log(`  Total records: ${people.length}`);
    console.log(`  Unique keys: ${uniqueKeys.size}`);
    console.log(`  Duplicates: ${duplicates.length}`);

    if (duplicates.length > 0) {
      console.log('\nDuplicate keys found:');
      duplicates.slice(0, 5).forEach(key => console.log(`  ${key}`));
    }
  });
