import sqlite3

print('Linking unique businesses to people...\n')

conn = sqlite3.connect('Sheffield1867.db')
cursor = conn.cursor()

# Find unique matches with house numbers
cursor.execute("""
    SELECT
        b.id as business_id,
        p.id as person_id,
        b.name,
        b.street_address
    FROM sheffield_businesses b
    JOIN sheffield_people p ON
        b.name = p.name
        AND LOWER(REPLACE(b.street_address, ' ', '')) = LOWER(REPLACE(p.street_address, ' ', ''))
    WHERE b.street_address GLOB '*[0-9]*'
      AND p.street_address GLOB '*[0-9]*'
""")

all_matches = cursor.fetchall()

# Group by business_id to find unique matches
matches_by_business = {}
for match in all_matches:
    business_id = match[0]
    if business_id not in matches_by_business:
        matches_by_business[business_id] = []
    matches_by_business[business_id].append(match)

# Filter to only unique (1-to-1) matches
unique_matches = []
for business_id, matches in matches_by_business.items():
    if len(matches) == 1:
        unique_matches.append(matches[0])

print(f'Found {len(unique_matches)} unique 1-to-1 matches')

# Update businesses
for match in unique_matches:
    business_id, person_id, name, address = match
    cursor.execute("""
        UPDATE sheffield_businesses
        SET matched_to_genealogy_id = ?
        WHERE id = ?
    """, (person_id, business_id))

conn.commit()

# Verify
cursor.execute('SELECT COUNT(*) FROM sheffield_businesses WHERE matched_to_genealogy_id IS NOT NULL')
linked_count = cursor.fetchone()[0]
print(f'\nBusinesses now linked: {linked_count}')

# Show examples
print('\nSample linked records:')
cursor.execute("""
    SELECT
        b.id,
        b.name,
        b.street_address,
        b.profession as business_profession,
        p.profession as person_profession
    FROM sheffield_businesses b
    JOIN sheffield_people p ON b.matched_to_genealogy_id = p.id
    LIMIT 10
""")

for row in cursor.fetchall():
    print(f'  {row[1]} at {row[2]}')
    print(f'    Business: {row[3]}')
    print(f'    Person: {row[4]}\n')

conn.close()
print('✓ Complete!')
