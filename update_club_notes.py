#!/usr/bin/env python3
"""
Script to generate updated club data with full notes from 1857-1875.txt
"""

# Read the text file with detailed notes
with open('1857-1875.txt', 'r', encoding='utf-8') as f:
    lines = f.readlines()

# Parse the clubs
clubs_notes = {}
for line in lines[1:]:  # Skip first empty line
    line = line.strip()
    if not line:
        continue

    parts = line.split('\t')
    if len(parts) >= 3:
        name = parts[0].strip()
        year = parts[1].strip()
        notes = parts[2].strip() if parts[2].strip() else ""
        clubs_notes[name] = notes

# Print the mapping for verification
print(f"Found {len(clubs_notes)} clubs with notes")
print("\nSample entries:")
for i, (name, notes) in enumerate(list(clubs_notes.items())[:5]):
    print(f"{name}: {notes}")

# Generate output file with the mapping
with open('club_notes_mapping.txt', 'w', encoding='utf-8') as f:
    for name, notes in clubs_notes.items():
        f.write(f"{name}|||{notes}\n")

print("\nGenerated club_notes_mapping.txt")
