import sqlite3

db_path = r'D:/projects/Saturday at Three/Sheffield1867.db'
conn = sqlite3.connect(db_path)
cursor = conn.cursor()

# Check total footballers and those with stats
cursor.execute("""
  SELECT
    COUNT(*) as total_footballers,
    COUNT(pace) as players_with_pace,
    COUNT(position) as players_with_position,
    COUNT(current_ability) as players_with_ca
  FROM sheffield_footballers
""")

stats = cursor.fetchone()

print('========================================')
print('SHEFFIELD FOOTBALLERS STATS CHECK')
print('========================================')
print(f'Total footballers: {stats[0]}')
print(f'Players with pace stat: {stats[1]}')
print(f'Players with position: {stats[2]}')
print(f'Players with current_ability: {stats[3]}')
print('========================================')

# Show a sample player with stats
cursor.execute("""
  SELECT first_name, surname, position, pace, acceleration, current_ability, person_id
  FROM sheffield_footballers
  WHERE pace IS NOT NULL
  LIMIT 1
""")

player = cursor.fetchone()

if player:
    print('\nSample player with stats:')
    print(f'{player[0]} {player[1]} (person_id: {player[6]})')
    print(f'Position: {player[2]}')
    print(f'Pace: {player[3]}, Acceleration: {player[4]}')
    print(f'Current Ability: {player[5]}')
else:
    print('\nNo players found with stats.')

conn.close()
