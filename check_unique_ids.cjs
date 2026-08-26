const Database = require('better-sqlite3');
const db = new Database('sheffield1867.db');

// Get total rows and unique IDs
const result = db.prepare(`
  SELECT
    COUNT(*) as total_rows,
    COUNT(DISTINCT id) as unique_ids
  FROM sheffield_people
`).get();

console.log('Total rows:', result.total_rows);
console.log('Unique IDs:', result.unique_ids);

if (result.total_rows === result.unique_ids) {
  console.log('✓ All rows have unique IDs!');
} else {
  console.log('✗ There are duplicate IDs!');
  console.log('Duplicates:', result.total_rows - result.unique_ids);

  // Show duplicates
  const duplicates = db.prepare(`
    SELECT id, COUNT(*) as count
    FROM sheffield_people
    GROUP BY id
    HAVING COUNT(*) > 1
    ORDER BY count DESC
    LIMIT 10
  `).all();

  if (duplicates.length > 0) {
    console.log('\nDuplicate IDs (showing first 10):');
    duplicates.forEach(dup => {
      console.log(`  ID ${dup.id}: ${dup.count} occurrences`);
    });
  }
}

// Check for NULL IDs
const nullCount = db.prepare(`
  SELECT COUNT(*) as count
  FROM sheffield_people
  WHERE id IS NULL
`).get();

if (nullCount.count > 0) {
  console.log(`\n✗ Warning: ${nullCount.count} rows have NULL ids`);
} else {
  console.log('✓ No NULL IDs found');
}

db.close();
