const sqlite3 = require('better-sqlite3');
const db = new sqlite3('Sheffield1867.db');

console.log('='.repeat(60));
console.log('Verifying sheffield_people data');
console.log('='.repeat(60) + '\n');

// Sample people with addresses
const sample = db.prepare(`
    SELECT name, birth_year, census_relation, census_gender,
           street_address, profession
    FROM sheffield_people
    WHERE street_address IS NOT NULL
    LIMIT 10
`).all();

console.log('Sample people with addresses:\n');
sample.forEach((p, i) => {
    console.log(`${i+1}. ${p.name} (${p.birth_year}) - ${p.census_relation}, ${p.census_gender}`);
    console.log(`   Address: ${p.street_address}`);
    console.log(`   Profession: ${p.profession || 'N/A'}`);
    console.log('');
});

// Statistics
const stats = db.prepare(`
    SELECT
        COUNT(*) as total,
        COUNT(CASE WHEN is_player = 1 THEN 1 END) as players,
        COUNT(CASE WHEN is_player = 0 THEN 1 END) as non_players,
        COUNT(street_address) as with_address,
        COUNT(profession) as with_profession,
        COUNT(census_household_members) as with_household
    FROM sheffield_people
`).get();

console.log('='.repeat(60));
console.log('Sheffield People Statistics:');
console.log(`  Total people: ${stats.total}`);
console.log(`  Players: ${stats.players}`);
console.log(`  Non-players: ${stats.non_players}`);
console.log(`  With addresses: ${stats.with_address} (${(stats.with_address / stats.total * 100).toFixed(1)}%)`);
console.log(`  With professions: ${stats.with_profession} (${(stats.with_profession / stats.total * 100).toFixed(1)}%)`);
console.log(`  With household members: ${stats.with_household} (${(stats.with_household / stats.total * 100).toFixed(1)}%)`);

// Gender breakdown
const genders = db.prepare(`
    SELECT census_gender, COUNT(*) as count
    FROM sheffield_people
    WHERE census_gender IS NOT NULL
    GROUP BY census_gender
    ORDER BY count DESC
`).all();

console.log('\nGender breakdown:');
genders.forEach(g => console.log(`  ${g.census_gender}: ${g.count}`));

// Relation breakdown
const relations = db.prepare(`
    SELECT census_relation, COUNT(*) as count
    FROM sheffield_people
    WHERE census_relation IS NOT NULL
    GROUP BY census_relation
    ORDER BY count DESC
    LIMIT 10
`).all();

console.log('\nTop 10 household relations:');
relations.forEach(r => console.log(`  ${r.census_relation}: ${r.count}`));

db.close();
console.log('\n✓ Verification complete!');
