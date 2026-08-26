import sqlite3

# Connect to the database
conn = sqlite3.connect('D:/projects/Saturday at Three/Sheffield1867.db')
cursor = conn.cursor()

# Get all table names
cursor.execute("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
tables = cursor.fetchall()

print("Tables in Sheffield1867.db:")
print("=" * 50)
for table in tables:
    table_name = table[0]
    print(f"\n{table_name}")

    # Count rows in each table
    cursor.execute(f"SELECT COUNT(*) FROM {table_name}")
    count = cursor.fetchone()[0]
    print(f"  Rows: {count}")

    # Get column info
    cursor.execute(f"PRAGMA table_info({table_name})")
    columns = cursor.fetchall()
    print(f"  Columns:")
    for col in columns:
        print(f"    - {col[1]} ({col[2]})")

conn.close()
