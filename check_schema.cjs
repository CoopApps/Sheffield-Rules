const sqlite3 = require('better-sqlite3');
const db = new sqlite3('./sheffield1867.db');
const schema = db.prepare("SELECT sql FROM sqlite_master WHERE name='sheffield_people'").get();
if (schema) console.log(schema.sql);
else console.log('Table does not exist');
db.close();
