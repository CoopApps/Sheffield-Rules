const XLSX = require('xlsx');

console.log('Analyzing postcodes.xlsx structure...\n');

const workbook = XLSX.readFile('postcodes.xlsx');

console.log('Sheet names:', workbook.SheetNames);
console.log('');

// Try each sheet
workbook.SheetNames.forEach(sheetName => {
    console.log(`\nSheet: ${sheetName}`);
    console.log('='.repeat(40));

    const sheet = workbook.Sheets[sheetName];

    // Get range
    const range = XLSX.utils.decode_range(sheet['!ref']);
    console.log(`Range: ${sheet['!ref']}`);
    console.log(`Rows: ${range.e.r - range.s.r + 1}, Cols: ${range.e.c - range.s.c + 1}`);

    // Show first few rows raw
    console.log('\nFirst 10 rows (raw):');
    for (let row = range.s.r; row <= Math.min(range.s.r + 9, range.e.r); row++) {
        const rowData = [];
        for (let col = range.s.c; col <= range.e.c; col++) {
            const cellAddress = XLSX.utils.encode_cell({ r: row, c: col });
            const cell = sheet[cellAddress];
            rowData.push(cell ? cell.v : '');
        }
        console.log(`Row ${row}:`, rowData.join(' | '));
    }

    // Try to parse as JSON
    console.log('\nAs JSON (first 5):');
    const data = XLSX.utils.sheet_to_json(sheet);
    data.slice(0, 5).forEach((row, i) => {
        console.log(`${i}:`, JSON.stringify(row).substring(0, 150));
    });
});
