#!/usr/bin/env python3
"""Quick test of genealogy matching logic"""

import csv
import sqlite3
from pathlib import Path

def normalize_parish(parish):
    """Normalize parish name for matching"""
    return (parish.lower()
            .replace("brightside bierlow", "brightside")
            .replace("ecclesall bierlow", "ecclesall")
            .replace("nether hallam", "hallam")
            .strip())

def parse_name(full_name):
    """Parse full name into first and surname"""
    parts = full_name.strip().split()
    if not parts:
        return "", ""
    if len(parts) == 1:
        return "", parts[0]
    return parts[0], parts[-1]

def calculate_match_confidence(player_name, player_birth_year, player_parish,
                               gen_name, gen_birth_year, gen_parish):
    """Calculate match confidence (0.0-1.0)"""
    score = 0.0
    reasons = []

    # Name match (0.5 weight)
    gen_first, gen_surname = parse_name(gen_name)
    if gen_surname.lower() in player_name.lower():
        score += 0.5
        reasons.append("surname match")

    # Birth year match (0.3 weight)
    year_diff = abs(player_birth_year - gen_birth_year)
    if year_diff == 0:
        score += 0.3
        reasons.append("exact birth year")
    elif year_diff == 1:
        score += 0.25
        reasons.append("birth year ±1")
    elif year_diff == 2:
        score += 0.15
        reasons.append("birth year ±2")

    # Parish match (0.2 weight)
    if player_parish:
        p_norm = normalize_parish(player_parish)
        g_norm = normalize_parish(gen_parish)
        if p_norm == g_norm or p_norm in g_norm or g_norm in p_norm:
            score += 0.2
            reasons.append("parish match")

    return score, ", ".join(reasons)

# Connect to database
db_path = "D:/projects/Saturday at Three/Sheffield1867.db"
conn = sqlite3.connect(db_path)
cursor = conn.cursor()

# Get sample of players
print("Loading players from database...")
cursor.execute("""
    SELECT id, name, birth_year, ecclesiastical_parish, civil_parish, surname
    FROM sheffield_players
    WHERE birth_year IS NOT NULL
    LIMIT 1000
""")
players = cursor.fetchall()
print(f"Loaded {len(players)} players")

# Read sample genealogy CSV
genealogy_dir = Path("D:/projects/Saturday at Three/genealogy")
test_file = genealogy_dir / "tg_1835_S.csv"

print(f"\nReading genealogy file: {test_file}")
gen_records = []
male_relations = ["Head", "Son", "Lodger", "Boarder", "Nephew", "Brother", "Father"]

with open(test_file, 'r', encoding='utf-8') as f:
    reader = csv.DictReader(f)
    for row in reader:
        if row['Relation'] in male_relations:
            try:
                age = int(row['Age']) if row['Age'] else 0
                if age > 0 and age <= 40:  # Only age <= 40
                    born_approx = int(row['Born Approx']) if row['Born Approx'] else 0
                    if born_approx > 0:
                        gen_records.append({
                            'name': row['Name'],
                            'address': row['Address'],
                            'parish': row['Parish'],
                            'birth_year': born_approx,
                            'profession': row['Profession']
                        })
            except ValueError:
                continue

print(f"Found {len(gen_records)} male records (age <= 40)")

# Try to match
matches = []
for gen in gen_records[:10]:  # Just test first 10
    best_match = None
    best_score = 0.0

    for player in players:
        player_id, player_name, birth_year, eccl_parish, civil_parish, surname = player
        parish = eccl_parish or civil_parish

        score, reason = calculate_match_confidence(
            player_name, birth_year, parish,
            gen['name'], gen['birth_year'], gen['parish']
        )

        if score >= 0.6 and score > best_score:  # Min 60% confidence
            best_match = {
                'player_id': player_id,
                'player_name': player_name,
                'player_birth_year': birth_year,
                'player_parish': parish,
                'gen_name': gen['name'],
                'gen_birth_year': gen['birth_year'],
                'address': gen['address'],
                'profession': gen['profession'],
                'confidence': score,
                'reason': reason
            }
            best_score = score

    if best_match:
        matches.append(best_match)

# Print results
print(f"\n{'='*80}")
print(f"MATCH RESULTS (Sample of first 10 genealogy records)")
print(f"{'='*80}\n")

for i, match in enumerate(matches, 1):
    print(f"Match #{i}:")
    print(f"  Player in DB: {match['player_name']} (born {match['player_birth_year']}, {match['player_parish'] or 'no parish'})")
    print(f"  Genealogy:    {match['gen_name']} (born {match['gen_birth_year']})")
    print(f"  Address:      {match['address']}")
    print(f"  Profession:   {match['profession'] or 'N/A'}")
    print(f"  Confidence:   {match['confidence']:.0%} ({match['reason']})")
    print()

print(f"\nTotal matches found: {len(matches)} out of 10 tested")
print(f"Match rate: {len(matches)/10:.0%}")

conn.close()
