const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('\n========================================');
console.log('MIGRATING GENEALOGY TO PEOPLE TABLES');
console.log('========================================\n');

// Step 1: Check sheffield_people structure
console.log('Step 1: Checking sheffield_people structure...');
const peopleCols = db.prepare('PRAGMA table_info(sheffield_people)').all();
console.log('Columns in sheffield_people:');
peopleCols.forEach(c => console.log('  ' + c.name));

// Step 2: Create institution tables if they don't exist
console.log('\nStep 2: Creating institution tables...');

// Asylum table
db.exec(`
    CREATE TABLE IF NOT EXISTS sheffield_asylum (
        id TEXT PRIMARY KEY,
        name TEXT,
        address TEXT,
        profession TEXT,
        year INTEGER,
        business_surname TEXT,
        business_occupation TEXT,
        postcode TEXT
    )
`);
console.log('✓ sheffield_asylum table ready');

// Workhouse table
db.exec(`
    CREATE TABLE IF NOT EXISTS sheffield_workhouse (
        id TEXT PRIMARY KEY,
        name TEXT,
        address TEXT,
        profession TEXT,
        year INTEGER,
        business_surname TEXT,
        business_occupation TEXT,
        postcode TEXT
    )
`);
console.log('✓ sheffield_workhouse table ready');

// Prison table
db.exec(`
    CREATE TABLE IF NOT EXISTS sheffield_prison (
        id TEXT PRIMARY KEY,
        name TEXT,
        address TEXT,
        profession TEXT,
        year INTEGER,
        business_surname TEXT,
        business_occupation TEXT,
        postcode TEXT
    )
`);
console.log('✓ sheffield_prison table ready\n');

// Step 3: Count records by type
console.log('Step 3: Analyzing genealogy records...');

const totalCount = db.prepare('SELECT COUNT(*) as c FROM unmatched_genealogy').get().c;

const asylumCount = db.prepare(`
    SELECT COUNT(*) as c FROM unmatched_genealogy
    WHERE address LIKE '%asylum%' OR address LIKE '%Asylum%'
`).get().c;

const workhouseCount = db.prepare(`
    SELECT COUNT(*) as c FROM unmatched_genealogy
    WHERE address LIKE '%workhouse%' OR address LIKE '%work house%'
       OR address LIKE '%Workhouse%' OR address LIKE '%Work House%'
`).get().c;

const prisonCount = db.prepare(`
    SELECT COUNT(*) as c FROM unmatched_genealogy
    WHERE address LIKE '%prison%' OR address LIKE '%Prison%'
       OR address LIKE '%gaol%' OR address LIKE '%Gaol%'
       OR address LIKE '%jail%' OR address LIKE '%Jail%'
`).get().c;

const regularCount = totalCount - asylumCount - workhouseCount - prisonCount;

console.log(`Total genealogy records: ${totalCount.toLocaleString()}`);
console.log(`  Asylum: ${asylumCount.toLocaleString()}`);
console.log(`  Workhouse: ${workhouseCount.toLocaleString()}`);
console.log(`  Prison: ${prisonCount.toLocaleString()}`);
console.log(`  Regular people: ${regularCount.toLocaleString()}\n`);

// Step 4: Migrate asylum records
console.log('Step 4: Migrating asylum records...');
db.exec(`
    INSERT INTO sheffield_asylum (id, name, address, profession, year, business_surname, business_occupation, postcode)
    SELECT id, name, address, profession, year, business_surname, business_occupation, postcode
    FROM unmatched_genealogy
    WHERE address LIKE '%asylum%' OR address LIKE '%Asylum%'
`);
console.log(`✓ Migrated ${asylumCount.toLocaleString()} asylum records\n`);

// Step 5: Migrate workhouse records
console.log('Step 5: Migrating workhouse records...');
db.exec(`
    INSERT INTO sheffield_workhouse (id, name, address, profession, year, business_surname, business_occupation, postcode)
    SELECT id, name, address, profession, year, business_surname, business_occupation, postcode
    FROM unmatched_genealogy
    WHERE address LIKE '%workhouse%' OR address LIKE '%work house%'
       OR address LIKE '%Workhouse%' OR address LIKE '%Work House%'
`);
console.log(`✓ Migrated ${workhouseCount.toLocaleString()} workhouse records\n`);

// Step 6: Migrate prison records
console.log('Step 6: Migrating prison records...');
db.exec(`
    INSERT INTO sheffield_prison (id, name, address, profession, year, business_surname, business_occupation, postcode)
    SELECT id, name, address, profession, year, business_surname, business_occupation, postcode
    FROM unmatched_genealogy
    WHERE address LIKE '%prison%' OR address LIKE '%Prison%'
       OR address LIKE '%gaol%' OR address LIKE '%Gaol%'
       OR address LIKE '%jail%' OR address LIKE '%Jail%'
`);
console.log(`✓ Migrated ${prisonCount.toLocaleString()} prison records\n`);

// Step 7: Migrate regular people
console.log('Step 7: Migrating regular people to sheffield_people...');
db.exec(`
    INSERT INTO sheffield_people (id, name, address, profession, year, business_surname, business_occupation, postcode)
    SELECT id, name, address, profession, year, business_surname, business_occupation, postcode
    FROM unmatched_genealogy
    WHERE NOT (
        address LIKE '%asylum%' OR address LIKE '%Asylum%'
        OR address LIKE '%workhouse%' OR address LIKE '%work house%'
        OR address LIKE '%Workhouse%' OR address LIKE '%Work House%'
        OR address LIKE '%prison%' OR address LIKE '%Prison%'
        OR address LIKE '%gaol%' OR address LIKE '%Gaol%'
        OR address LIKE '%jail%' OR address LIKE '%Jail%'
    )
`);
console.log(`✓ Migrated ${regularCount.toLocaleString()} regular people\n`);

// Step 8: Verify counts
console.log('Step 8: Verifying migration...');
const peopleCount = db.prepare('SELECT COUNT(*) as c FROM sheffield_people').get().c;
const asylumFinal = db.prepare('SELECT COUNT(*) as c FROM sheffield_asylum').get().c;
const workhouseFinal = db.prepare('SELECT COUNT(*) as c FROM sheffield_workhouse').get().c;
const prisonFinal = db.prepare('SELECT COUNT(*) as c FROM sheffield_prison').get().c;

console.log('\n========================================');
console.log('MIGRATION COMPLETE');
console.log('========================================');
console.log(`sheffield_people: ${peopleCount.toLocaleString()}`);
console.log(`sheffield_asylum: ${asylumFinal.toLocaleString()}`);
console.log(`sheffield_workhouse: ${workhouseFinal.toLocaleString()}`);
console.log(`sheffield_prison: ${prisonFinal.toLocaleString()}`);
console.log(`Total migrated: ${(peopleCount + asylumFinal + workhouseFinal + prisonFinal).toLocaleString()}`);
console.log('========================================\n');

db.close();
