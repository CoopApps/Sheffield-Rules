const sqlite3 = require('better-sqlite3');
const db = new sqlite3('Sheffield1867.db');

console.log('Verifying census data import...\n');
console.log('='.repeat(60) + '\n');

// Get sample players with census data
const sample = db.prepare(`
    SELECT name, birth_year, census_birth_date, census_birth_place,
           census_county, census_household_members
    FROM sheffield_players
    WHERE census_household_members IS NOT NULL
    LIMIT 3
`).all();

console.log('Sample players with census data:\n');
sample.forEach((p, i) => {
    console.log(`${i+1}. ${p.name} (born ${p.birth_year})`);
    console.log(`   Birth Date: ${p.census_birth_date || 'N/A'}`);
    console.log(`   Birth Place: ${p.census_birth_place || 'N/A'}`);
    console.log(`   County: ${p.census_county || 'N/A'}`);
    const household = p.census_household_members || '';
    console.log(`   Household Members: ${household.substring(0, 100)}${household.length > 100 ? '...' : ''}`);
    console.log('');
});

console.log('='.repeat(60) + '\n');

// Get statistics
const stats = db.prepare(`
    SELECT
        COUNT(*) as total,
        COUNT(census_household_members) as with_household,
        COUNT(census_birth_place) as with_birth_place,
        COUNT(census_county) as with_county,
        COUNT(census_birth_date) as with_birth_date
    FROM sheffield_players
    WHERE is_real_player = 1
`).get();

console.log('Statistics:');
console.log(`  Total real players: ${stats.total}`);
console.log(`  With household members: ${stats.with_household}`);
console.log(`  With birth place: ${stats.with_birth_place}`);
console.log(`  With county: ${stats.with_county}`);
console.log(`  With birth date: ${stats.with_birth_date}`);

console.log('\n✓ Census data import verification complete!');

db.close();
