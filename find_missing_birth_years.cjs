const sqlite3 = require('better-sqlite3');

const db = new sqlite3('Sheffield1867.db');

console.log('='.repeat(70));
console.log('FINDING MISSING BIRTH YEARS IN DATABASE');
console.log('='.repeat(70) + '\n');

const birthYears = db.prepare(`
    SELECT DISTINCT birth_year
    FROM sheffield_people
    WHERE birth_year IS NOT NULL
    ORDER BY birth_year
`).all();

console.log('Total distinct birth years in database: ' + birthYears.length + '\n');

const minYear = Math.min(...birthYears.map(r => r.birth_year));
const maxYear = Math.max(...birthYears.map(r => r.birth_year));

console.log('Year range: ' + minYear + ' to ' + maxYear);
console.log('Expected years in range: ' + (maxYear - minYear + 1) + '\n');

// Create set of all years that should exist
const allYears = [];
for (let y = minYear; y <= maxYear; y++) {
    allYears.push(y);
}

// Find missing years
const yearSet = new Set(birthYears.map(r => r.birth_year));
const missingYears = allYears.filter(y => !yearSet.has(y));

console.log('='.repeat(70));
console.log('MISSING BIRTH YEARS (' + missingYears.length + ' total):');
console.log('='.repeat(70) + '\n');

if (missingYears.length > 0) {
    missingYears.forEach((year, i) => {
        if (i % 10 === 0 && i > 0) console.log('');
        process.stdout.write(year + ', ');
        if ((i + 1) % 10 === 0) console.log('');
    });
    console.log('\n');
} else {
    console.log('No missing years - database has continuous coverage!\n');
}

// Check which census files exist for missing years
const fs = require('fs');
const censusFiles = fs.readdirSync('Sheffield Census');

console.log('='.repeat(70));
console.log('CENSUS FILES AVAILABLE FOR MISSING YEARS:');
console.log('='.repeat(70) + '\n');

const availableFiles = [];
missingYears.forEach(year => {
    const possibleFile = year + '.csv';
    if (censusFiles.includes(possibleFile)) {
        availableFiles.push(possibleFile);
        console.log('✓ ' + possibleFile);
    }
});

console.log('\n' + availableFiles.length + ' census files available for missing years');

db.close();

console.log('\n✓ Analysis complete!');
