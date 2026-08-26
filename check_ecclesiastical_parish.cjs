const fs = require('fs');
const path = require('path');

console.log('\n========================================');
console.log('CHECKING ECCLESIASTICAL PARISH COLUMN');
console.log('========================================\n');

const censusDir = './sheffield census';
const files = fs.readdirSync(censusDir).filter(f => f.endsWith('.csv')).sort();

console.log(`Checking ${files.length} census files...\n`);

let hasColumn = 0;
let noColumn = 0;

files.forEach(file => {
    const filePath = path.join(censusDir, file);
    const content = fs.readFileSync(filePath, 'utf8');
    const header = content.split('\n')[0];
    const hasEccl = header.includes('ECCLESIASTICAL PARISH');
    const year = file.replace('.csv', '');

    console.log(`${year.padEnd(10)} : ${hasEccl ? 'YES ✓' : 'NO  ✗'}`);

    if (hasEccl) {
        hasColumn++;
    } else {
        noColumn++;
    }
});

console.log('\n========================================');
console.log(`Files WITH ecclesiastical parish: ${hasColumn}`);
console.log(`Files WITHOUT ecclesiastical parish: ${noColumn}`);
console.log(`Total files: ${files.length}`);
console.log('========================================\n');
