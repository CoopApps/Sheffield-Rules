import sqlite3

# Connect to the database
conn = sqlite3.connect('sheffield_save.db')
cursor = conn.cursor()

# Get all clubs from the database
cursor.execute("SELECT id, name FROM sheffield_clubs")
clubs = {name.lower(): club_id for club_id, name in cursor.fetchall()}

print(f"Found {len(clubs)} clubs in database")

# Hardcoded league structure (from sheffield_league.rs)
assignments = [
    # DIVISION 1 (16 clubs)
    ("sheffield-fc", "div-1", 1),
    ("hallam-fc", "div-1", 2),
    ("norfolk-fc", "div-1", 3),
    ("cemetery-road-church-fc", "div-1", 4),
    ("york-fc", "div-1", 5),
    ("norton-fc", "div-1", 6),
    ("pitsmoor-fc", "div-1", 7),
    ("fir-vale-fc", "div-1", 8),
    ("newhall-fc", "div-1", 9),
    ("attercliffe-fc", "div-1", 10),
    ("sheaf-house-fc", "div-1", 11),
    ("exchange-fc", "div-1", 12),
    ("mechanics-fc", "div-1", 13),
    ("broomhall-fc", "div-1", 14),
    ("brightside-fc", "div-1", 15),
    ("heeley-fc", "div-1", 16),
]

# Function to find club ID by name
def find_club_id(hardcoded_id):
    # Extract name from ID
    name_parts = hardcoded_id.replace("-fc", "").replace("-", " ").title()

    # Try exact match first
    for db_name, db_id in clubs.items():
        if name_parts.lower() in db_name or db_name in name_parts.lower():
            return db_id

    return None

# Insert assignments
success_count = 0
not_found = []

for hardcoded_id, division_id, position in assignments:
    club_id = find_club_id(hardcoded_id)

    if club_id:
        try:
            cursor.execute("""
                INSERT OR REPLACE INTO sheffield_league_clubs
                (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
                VALUES (?, ?, ?, ?, ?, ?)
            """, (f"{club_id}-{division_id}", division_id, club_id, position, False, None))
            success_count += 1
            print(f"✓ Assigned {hardcoded_id} to {division_id} at position {position}")
        except Exception as e:
            print(f"✗ Error assigning {hardcoded_id}: {e}")
    else:
        not_found.append(hardcoded_id)
        print(f"✗ Could not find club: {hardcoded_id}")

conn.commit()
conn.close()

print(f"\n✓ Successfully assigned {success_count} clubs")
if not_found:
    print(f"✗ Could not find {len(not_found)} clubs: {', '.join(not_found)}")
