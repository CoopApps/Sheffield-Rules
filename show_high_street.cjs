const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('WHITE\'S DIRECTORY - High Street, Attercliffe');
console.log('='.repeat(70) + '\n');

const businesses = db.prepare(`
    SELECT full_name, occupation, address
    FROM sheffield_businesses
    WHERE address LIKE '%High Street, Attercliffe%'
    ORDER BY full_name, occupation
`).all();

console.log('Total businesses on High Street, Attercliffe:', businesses.length);
console.log('');

businesses.forEach((b, i) => {
    console.log(`${i+1}. ${b.full_name} - ${b.occupation}`);
    console.log(`   Address: ${b.address}`);
});

console.log('\n' + '='.repeat(70));
console.log('GEORGE SMITH entries specifically:');
console.log('='.repeat(70) + '\n');

const georgeSmiths = db.prepare(`
    SELECT full_name, occupation, address
    FROM sheffield_businesses
    WHERE full_name = 'George Smith'
    AND address LIKE '%High Street, Attercliffe%'
`).all();

georgeSmiths.forEach((b, i) => {
    console.log(`${i+1}. ${b.occupation}`);
    console.log(`   Address: ${b.address}`);
});

db.close();
