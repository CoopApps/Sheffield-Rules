import sqlite3

db = sqlite3.connect('./Sheffield1867.db')
cursor = db.cursor()

print('\n=== Checking 105th Regiment Reserves ===')
cursor.execute("SELECT * FROM sheffield_clubs WHERE id LIKE ?", ('%105th-regiment%',))
clubs = cursor.fetchall()
print('Clubs table:', clubs)

cursor.execute("SELECT * FROM sheffield_league_clubs WHERE club_id LIKE ?", ('%105th-regiment%',))
league = cursor.fetchall()
print('League clubs table:', league)

print('\n=== Checking Albion FC Reserves ===')
cursor.execute("SELECT * FROM sheffield_clubs WHERE id LIKE ?", ('%albion-fc%',))
clubs = cursor.fetchall()
print('Clubs table:', clubs)

cursor.execute("SELECT * FROM sheffield_league_clubs WHERE club_id LIKE ?", ('%albion-fc%',))
league = cursor.fetchall()
print('League clubs table:', league)

print('\n=== Total clubs in sheffield_clubs ===')
cursor.execute("SELECT COUNT(*) FROM sheffield_clubs")
print(cursor.fetchone())

print('\n=== Total assignments in sheffield_league_clubs ===')
cursor.execute("SELECT COUNT(*) FROM sheffield_league_clubs")
print(cursor.fetchone())

print('\n=== Reserve teams in league (first 5) ===')
cursor.execute("SELECT club_id, division_id, is_reserve_team FROM sheffield_league_clubs WHERE is_reserve_team = 1 LIMIT 5")
reserves = cursor.fetchall()
for r in reserves:
    print(r)

db.close()
