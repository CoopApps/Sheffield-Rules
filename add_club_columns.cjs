// This script adds the missing columns to sheffield_clubs table
// Run this to fix the "no such column: c.ground_name" error

const sql_migration = `
-- Check if columns exist, if not add them
-- SQLite doesn't support IF NOT EXISTS for ALTER TABLE, so we'll handle errors

-- Add ground_name column
ALTER TABLE sheffield_clubs ADD COLUMN ground_name TEXT;

-- Add city column
ALTER TABLE sheffield_clubs ADD COLUMN city TEXT DEFAULT '';

-- Add region column
ALTER TABLE sheffield_clubs ADD COLUMN region TEXT DEFAULT '';

-- Copy origin data to ground_name for existing rows
UPDATE sheffield_clubs SET ground_name = origin WHERE ground_name IS NULL OR ground_name = '';
`;

console.log('SQL Migration Script:');
console.log('====================');
console.log(sql_migration);
console.log('\nTo apply this migration:');
console.log('1. Call the db_run_schema_migrations command from the frontend');
console.log('2. Or manually connect to Sheffield1867.db and run the SQL above');
console.log('3. The migration function needs to be added to sheffield_db.rs first');
