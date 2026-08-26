const fs = require('fs');

function parseCSV(content) {
    const lines = content.split('\n');
    if (lines.length < 2) return [];

    const headers = lines[0].split(',').map(h => h.trim());
    const rows = [];

    for (let i = 1; i < lines.length; i++) {
        const line = lines[i].trim();
        if (!line) continue;

        const values = [];
        let current = '';
        let inQuotes = false;

        for (let j = 0; j < line.length; j++) {
            const char = line[j];
            if (char === '"') {
                inQuotes = !inQuotes;
            } else if (char === ',' && !inQuotes) {
                values.push(current.trim());
                current = '';
            } else {
                current += char;
            }
        }
        values.push(current.trim());

        const row = {};
        headers.forEach((h, i) => row[h] = values[i] || '');
        rows.push(row);
    }

    return rows;
}

const content = fs.readFileSync('Genealogy/tg_1834_S.csv', 'utf-8');
const records = parseCSV(content);

const addressMap = {};
records.forEach(r => {
    const addr = r.Address?.trim();
    if (addr) {
        if (!addressMap[addr]) addressMap[addr] = [];
        addressMap[addr].push({ name: r.Name, relation: r.Relation });
    }
});

const multi = Object.entries(addressMap).filter(([a, p]) => p.length > 1);

console.log('Sample addresses with multiple people:\n');
multi.slice(0, 10).forEach(([addr, people]) => {
    console.log(addr + ':');
    people.forEach(p => console.log('  - ' + p.name + ' (' + p.relation + ')'));
    console.log('');
});

console.log(`\nTotal addresses: ${Object.keys(addressMap).length}`);
console.log(`Addresses with multiple people: ${multi.length}`);
