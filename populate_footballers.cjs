const Database = require("better-sqlite3");
const db = new Database("Sheffield1867.db");

console.log("Setting up footballers...\n");

// Step 1: Add is_footballer column
try {
  db.exec("ALTER TABLE sheffield_people ADD COLUMN is_footballer INTEGER DEFAULT 0");
  console.log("Added is_footballer column");
} catch(e) {
  if (e.message.includes("duplicate")) {
    console.log("is_footballer column already exists");
  } else throw e;
}

// Step 2: Calculate ages and mark footballers (men aged 14-40, NOT in institutions)
console.log("\nMarking footballers...");
const marked = db.prepare("UPDATE sheffield_people SET is_footballer = 1 WHERE id NOT IN (SELECT id FROM sheffield_asylum UNION SELECT id FROM sheffield_workhouse UNION SELECT id FROM sheffield_prison) AND estimated_birth_year IS NOT NULL AND (1867 - estimated_birth_year) BETWEEN 14 AND 40").run();

console.log("Marked", marked.changes, "people as footballers");

// Step 3: Create sheffield_players table if needed
db.exec("CREATE TABLE IF NOT EXISTS sheffield_players AS SELECT * FROM sheffield_people WHERE 1=0");

// Step 4: Copy footballers to sheffield_players
console.log("\nCopying to sheffield_players...");
db.exec("DELETE FROM sheffield_players");
const copied = db.exec("INSERT INTO sheffield_players SELECT * FROM sheffield_people WHERE is_footballer = 1");

const count = db.prepare("SELECT COUNT(*) as c FROM sheffield_players").get().c;
console.log("Total players:", count);

db.close();
