# Fix for "no such column: c.ground_name" Error

## Problem
The `sheffield_clubs` table is missing columns that the code expects:
- `ground_name` (required)
- `city` (required)
- `region` (required)

Current table only has: `id`, `name`, `founded_year`, `origin`

## Solution

### Step 1: Stop All Dev Servers
You need to stop all running compilation/dev processes to allow file edits.

From Windows Task Manager or PowerShell:
```powershell
taskkill /F /IM node.exe
taskkill /F /IM cargo.exe
taskkill /F /IM rustc.exe
taskkill /F /IM saturday-at-three.exe
```

### Step 2: Add Migration Function

Add this function to `src-tauri/src/database/sheffield_db.rs` after the `migrate_sheffield_footballers` function (around line 102):

```rust
/// Migrate sheffield_clubs table to add any missing columns
pub async fn migrate_sheffield_clubs(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Get existing columns
    let rows = sqlx::query("PRAGMA table_info(sheffield_clubs)")
        .fetch_all(pool)
        .await?;

    use sqlx::Row;
    let existing_cols: std::collections::HashSet<String> = rows.iter()
        .map(|r| r.get::<String, _>("name"))
        .collect();

    // Columns that may be missing from older DB instances
    let required_columns: &[(&str, &str)] = &[
        ("ground_name", "TEXT"),
        ("city", "TEXT DEFAULT ''"),
        ("region", "TEXT DEFAULT ''"),
    ];

    for (col, def) in required_columns {
        if !existing_cols.contains(*col) {
            let sql = format!("ALTER TABLE sheffield_clubs ADD COLUMN {} {}", col, def);
            sqlx::query(&sql).execute(pool).await?;
        }
    }

    // If ground_name was just added and is NULL, copy origin to it
    if !existing_cols.contains("ground_name") {
        sqlx::query("UPDATE sheffield_clubs SET ground_name = origin WHERE ground_name IS NULL")
            .execute(pool)
            .await?;
    }

    Ok(())
}
```

### Step 3: Call Migration in initialize_schema

Update the `initialize_schema` function in the same file (around line 20) to call the new migration:

```rust
pub async fn initialize_schema(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let schema = include_str!("./sheffield_schema.sql");

    // Execute each statement separately since SQLite doesn't support multiple statements in one execute
    for statement in schema.split(';') {
        let trimmed = statement.trim();
        if !trimmed.is_empty() {
            sqlx::raw_sql(trimmed).execute(pool).await?;
        }
    }

    // Run migrations
    migrate_sheffield_footballers(pool).await?;
    migrate_sheffield_clubs(pool).await?;  // ADD THIS LINE

    Ok(())
}
```

### Step 4: Rebuild and Run

```bash
cd "D:/projects/Saturday at Three"
npm run tauri dev
```

The migration will run automatically on startup and add the missing columns to the `sheffield_clubs` table.

## Alternative: Manual SQL Approach

If you prefer to add the columns manually before rebuilding:

1. Open Sheffield1867.db with a SQLite tool
2. Run:
```sql
ALTER TABLE sheffield_clubs ADD COLUMN ground_name TEXT;
ALTER TABLE sheffield_clubs ADD COLUMN city TEXT DEFAULT '';
ALTER TABLE sheffield_clubs ADD COLUMN region TEXT DEFAULT '';
UPDATE sheffield_clubs SET ground_name = origin WHERE ground_name IS NULL;
```

Then the app should work without needing the migration code.

## Files Created
- `clubs_migration.rs` - Contains the migration function code
- `migrate_clubs_table.sql` - SQL migration script
- `add_club_columns.cjs` - Node.js helper script (requires better-sqlite3 rebuild)
- `FIX_CLUBS_ERROR.md` - This instruction file
