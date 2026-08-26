const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('='.repeat(70));
console.log('Removing Bad Business Links');
console.log('='.repeat(70) + '\n');

console.log('Finding businesses with no address or profession match...\n');

// Find all businesses linked to people where:
// - is_home_business = 0 (or NULL)
// - profession_match = 0 (or NULL)
const badLinks = db.prepare(`
    SELECT * FROM sheffield_businesses
    WHERE person_id IS NOT NULL
    AND (is_home_business IS NULL OR is_home_business = 0)
    AND (profession_match IS NULL OR profession_match = 0)
`).all();

console.log('Businesses with no address/profession match:', badLinks.length.toLocaleString());

if (badLinks.length === 0) {
    console.log('\n✓ No bad links to remove!');
    db.close();
    process.exit(0);
}

// Show sample bad links
console.log('\nSample bad links to be removed:');
badLinks.slice(0, 10).forEach((b, i) => {
    const person = db.prepare('SELECT name, profession, street_address FROM sheffield_people WHERE id = ?').get(b.person_id);
    console.log(`${i+1}. ${b.full_name} - ${b.occupation}`);
    console.log(`   Linked to: ${person.name} (${person.profession || 'No profession'})`);
    console.log(`   Business address: ${b.address}`);
    console.log(`   Person address: ${person.street_address}`);
    console.log('');
});

// Unlink these businesses
const unlinkBusiness = db.prepare(`
    UPDATE sheffield_businesses
    SET person_id = NULL, is_home_business = NULL, profession_match = NULL, employs_people = NULL
    WHERE id = ?
`);

console.log('Unlinking businesses...');
badLinks.forEach(b => {
    unlinkBusiness.run(b.id);
});

console.log(`✓ Unlinked ${badLinks.length.toLocaleString()} businesses\n`);

// Update people ownership flags
console.log('Updating people ownership flags...');

// Reset all owns_business flags
db.exec('UPDATE sheffield_people SET owns_business = 0, business_at_home = 0');

// Re-set owns_business for people who still have businesses
const updateOwnership = db.prepare(`
    UPDATE sheffield_people
    SET owns_business = 1,
        business_at_home = (
            SELECT COUNT(*) > 0
            FROM sheffield_businesses
            WHERE person_id = sheffield_people.id
            AND is_home_business = 1
        )
    WHERE id IN (
        SELECT DISTINCT person_id
        FROM sheffield_businesses
        WHERE person_id IS NOT NULL
    )
`);

updateOwnership.run();
console.log('✓ Updated ownership flags\n');

// Final statistics
console.log('='.repeat(70));
console.log('FINAL STATISTICS');
console.log('='.repeat(70));

const totalBusinesses = db.prepare('SELECT COUNT(*) as c FROM sheffield_businesses').get().c;
const linkedBusinesses = db.prepare('SELECT COUNT(*) as c FROM sheffield_businesses WHERE person_id IS NOT NULL').get().c;
const unlinkedBusinesses = db.prepare('SELECT COUNT(*) as c FROM sheffield_businesses WHERE person_id IS NULL').get().c;
const businessOwners = db.prepare('SELECT COUNT(*) as c FROM sheffield_people WHERE owns_business = 1').get().c;
const homeBusinessOwners = db.prepare('SELECT COUNT(*) as c FROM sheffield_people WHERE business_at_home = 1').get().c;

console.log(`\nTotal businesses: ${totalBusinesses.toLocaleString()}`);
console.log(`  - Linked to people: ${linkedBusinesses.toLocaleString()}`);
console.log(`  - Unlinked: ${unlinkedBusinesses.toLocaleString()}`);
console.log(`\nPeople who own businesses: ${businessOwners.toLocaleString()}`);
console.log(`  - With home-based business: ${homeBusinessOwners.toLocaleString()}`);

// Show breakdown of remaining matches
console.log('\n' + '='.repeat(70));
console.log('REMAINING BUSINESS MATCHES BREAKDOWN');
console.log('='.repeat(70));

const professionOnly = db.prepare('SELECT COUNT(*) as c FROM sheffield_businesses WHERE person_id IS NOT NULL AND profession_match = 1 AND (is_home_business IS NULL OR is_home_business = 0)').get().c;
const addressOnly = db.prepare('SELECT COUNT(*) as c FROM sheffield_businesses WHERE person_id IS NOT NULL AND is_home_business = 1 AND (profession_match IS NULL OR profession_match = 0)').get().c;
const both = db.prepare('SELECT COUNT(*) as c FROM sheffield_businesses WHERE person_id IS NOT NULL AND is_home_business = 1 AND profession_match = 1').get().c;

console.log(`\nProfession match only: ${professionOnly.toLocaleString()}`);
console.log(`Address match only: ${addressOnly.toLocaleString()}`);
console.log(`Both profession and address match: ${both.toLocaleString()}`);
console.log(`Total: ${(professionOnly + addressOnly + both).toLocaleString()}`);

db.close();
console.log('\n✓ Cleanup complete!');
