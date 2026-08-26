const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('NON-SHEFFIELD BUSINESS LOCATIONS');
console.log('='.repeat(70) + '\n');

// Common non-Sheffield locations in addresses
const nonSheffieldLocations = [
    'Barnsley',
    'Rotherham',
    'Ecclesfield',
    'Doncaster',
    'Whiston',
    'Parkgate',
    'Rawmarsh',
    'Canklow',
    'Guilthwaite',
    'Tinsley',
    'Attercliffe',
    'Brightside',
    'Handsworth',
    'Wadsley',
    'Stannington',
    'Darnall',
    'Heeley',
    'Owlerton',
    'Walkley',
    'Crookes'
];

console.log('Searching for businesses in non-Sheffield locations...\n');

const locationCounts = {};

// Get all businesses
const allBusinesses = db.prepare('SELECT address FROM sheffield_businesses').all();

allBusinesses.forEach(business => {
    const address = business.address.toLowerCase();

    nonSheffieldLocations.forEach(location => {
        if (address.includes(location.toLowerCase())) {
            if (!locationCounts[location]) {
                locationCounts[location] = 0;
            }
            locationCounts[location]++;
        }
    });
});

// Sort by count
const sorted = Object.entries(locationCounts)
    .sort((a, b) => b[1] - a[1]);

console.log('='.repeat(70));
console.log('LOCATION COUNTS');
console.log('='.repeat(70) + '\n');

let total = 0;
sorted.forEach(([location, count]) => {
    console.log(`${location.padEnd(30)} ${count.toLocaleString().padStart(10)} businesses`);
    total += count;
});

console.log('\n' + '='.repeat(70));
console.log(`Total businesses in non-Sheffield locations: ${total.toLocaleString()}`);

// Show samples from top locations
console.log('\n' + '='.repeat(70));
console.log('SAMPLE BUSINESSES FROM TOP LOCATIONS');
console.log('='.repeat(70) + '\n');

sorted.slice(0, 5).forEach(([location, count]) => {
    console.log(`\n${location.toUpperCase()} (${count} total):`);
    console.log('-'.repeat(70));

    const samples = db.prepare(`
        SELECT full_name, occupation, address
        FROM sheffield_businesses
        WHERE LOWER(address) LIKE ?
        LIMIT 5
    `).all(`%${location.toLowerCase()}%`);

    samples.forEach((s, i) => {
        console.log(`${i + 1}. ${s.full_name}`);
        console.log(`   ${s.occupation || '(no occupation)'}`);
        console.log(`   ${s.address}`);
    });
});

db.close();
