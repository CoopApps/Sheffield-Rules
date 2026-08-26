const Database = require('better-sqlite3');
const path = require('path');

const dbPath = path.join(__dirname, 'Sheffield1867.db');
const db = new Database(dbPath);

// Function to list all tables
function listTables() {
  console.log('\n=== DATABASE TABLES ===');
  const tables = db.prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name").all();
  tables.forEach(table => console.log(`  - ${table.name}`));
  return tables.map(t => t.name);
}

// Function to show table schema
function showSchema(tableName) {
  console.log(`\n=== SCHEMA: ${tableName} ===`);
  const schema = db.prepare(`PRAGMA table_info(${tableName})`).all();
  schema.forEach(col => {
    console.log(`  ${col.name} (${col.type})${col.pk ? ' PRIMARY KEY' : ''}${col.notnull ? ' NOT NULL' : ''}`);
  });
}

// Function to show table contents
function showTable(tableName, limit = 10) {
  console.log(`\n=== DATA: ${tableName} (first ${limit} rows) ===`);
  try {
    const rows = db.prepare(`SELECT * FROM ${tableName} LIMIT ${limit}`).all();
    if (rows.length === 0) {
      console.log('  (empty table)');
    } else {
      console.table(rows);
    }
    const count = db.prepare(`SELECT COUNT(*) as count FROM ${tableName}`).get();
    console.log(`Total rows: ${count.count}`);
  } catch (err) {
    console.error(`Error reading table: ${err.message}`);
  }
}

// Function to run custom SQL
function runQuery(sql) {
  console.log(`\n=== RUNNING QUERY ===\n${sql}\n`);
  try {
    if (sql.trim().toUpperCase().startsWith('SELECT')) {
      const rows = db.prepare(sql).all();
      console.table(rows);
      console.log(`Rows returned: ${rows.length}`);
    } else {
      const result = db.prepare(sql).run();
      console.log(`Changes: ${result.changes}`);
    }
  } catch (err) {
    console.error(`Error: ${err.message}`);
  }
}

// Main menu
const args = process.argv.slice(2);

if (args.length === 0) {
  // Interactive mode - show all tables
  const tables = listTables();
  console.log('\n=== ALL TABLE SCHEMAS ===');
  tables.forEach(table => showSchema(table));
  console.log('\n=== TABLE PREVIEWS ===');
  tables.forEach(table => showTable(table, 5));

  console.log('\n\nUSAGE:');
  console.log('  node view_db.cjs                    - Show all tables and data');
  console.log('  node view_db.cjs tables             - List all tables');
  console.log('  node view_db.cjs schema [table]     - Show table schema');
  console.log('  node view_db.cjs show [table] [n]   - Show first n rows (default 10)');
  console.log('  node view_db.cjs query "SELECT..."  - Run custom SQL query');

} else {
  const command = args[0].toLowerCase();

  switch (command) {
    case 'tables':
      listTables();
      break;

    case 'schema':
      if (args[1]) {
        showSchema(args[1]);
      } else {
        console.log('Usage: node view_db.cjs schema [table_name]');
      }
      break;

    case 'show':
      if (args[1]) {
        const limit = args[2] ? parseInt(args[2]) : 10;
        showTable(args[1], limit);
      } else {
        console.log('Usage: node view_db.cjs show [table_name] [limit]');
      }
      break;

    case 'query':
      if (args[1]) {
        runQuery(args[1]);
      } else {
        console.log('Usage: node view_db.cjs query "SELECT * FROM table"');
      }
      break;

    default:
      console.log('Unknown command. Use: tables, schema, show, or query');
  }
}

db.close();
