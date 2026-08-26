const Database = require('better-sqlite3');
const db = new Database('Sheffield1867.db');

console.log('Adding database indexes for matcher performance...\n');

// Genealogy indexes
db.exec(`CREATE INDEX IF NOT EXISTS idx_unmatched_genealogy_address
         ON unmatched_genealogy(address) WHERE address IS NOT NULL AND address != '';`);

db.exec(`CREATE INDEX IF NOT EXISTS idx_unmatched_genealogy_profession
         ON unmatched_genealogy(profession) WHERE profession IS NOT NULL;`);

db.exec(`CREATE INDEX IF NOT EXISTS idx_unmatched_genealogy_name_year
         ON unmatched_genealogy(name, year);`);

// Business indexes
db.exec(`CREATE INDEX IF NOT EXISTS idx_sheffield_businesses_surname
         ON sheffield_businesses(surname);`);

db.exec(`CREATE INDEX IF NOT EXISTS idx_sheffield_businesses_forename
         ON sheffield_businesses(forename);`);

db.exec(`CREATE INDEX IF NOT EXISTS idx_sheffield_businesses_address
         ON sheffield_businesses(address) WHERE address IS NOT NULL AND address != '';`);

db.exec(`CREATE INDEX IF NOT EXISTS idx_sheffield_businesses_occupation
         ON sheffield_businesses(occupation);`);

console.log('✓ All indexes created successfully\n');

// Analyze tables
db.exec('ANALYZE unmatched_genealogy;');
db.exec('ANALYZE sheffield_businesses;');

console.log('✓ Tables analyzed for query optimization\n');

const genCount = db.prepare('SELECT COUNT(*) as c FROM unmatched_genealogy').get().c;
const busCount = db.prepare('SELECT COUNT(*) as c FROM sheffield_businesses').get().c;

console.log(`Genealogy records: ${genCount.toLocaleString()}`);
console.log(`Business records: ${busCount.toLocaleString()}`);
console.log('\nDatabase ready for large-scale matching.');

db.close();
