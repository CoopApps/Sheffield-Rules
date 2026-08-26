const fs = require('fs');

const buffer = fs.readFileSync('./Sheffield1867.db');
const dbContent = buffer.toString('binary');

// Check if table exists
if (dbContent.includes('sheffield_clubs')) {
    console.log('✓ sheffield_clubs table exists in Sheffield1867.db');
} else {
    console.log('✗ sheffield_clubs table NOT found in Sheffield1867.db');
    console.log('\nNeed to create schema first!');
}

// Check file size
const stats = fs.statSync('./Sheffield1867.db');
console.log(`\nDatabase size: ${(stats.size / 1024 / 1024).toFixed(2)} MB`);
