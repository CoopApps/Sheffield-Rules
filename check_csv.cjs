const fs = require('fs');
const { parse } = require('csv-parse/sync');

const files = ['Sheffield Census/1792.csv', 'Sheffield Census/1792 - done.csv'];

for (const file of files) {
    if (fs.existsSync(file)) {
        console.log(`\n=== Checking ${file} ===\n`);
        const content = fs.readFileSync(file, 'utf-8');
        const records = parse(content, { columns: true, skip_empty_lines: true });
        
        const benjamins = records.filter(r => r.NAME && r.NAME.includes('Benjamin Harrop'));
        console.log(`Found ${benjamins.length} Benjamin Harrop records\n`);
        
        benjamins.forEach((r, i) => {
            console.log(`Record ${i+1}:`);
            console.log('  Name:', r.NAME);
            console.log('  Age:', r.AGE);
            console.log('  Birth Year:', r['ESTIMATED BIRTH YEAR']);
            console.log('  Piece/Folio/Page:', r.PIECE, r.FOLIO, r['PAGE NUMBER']);
            console.log('');
        });
    }
}
