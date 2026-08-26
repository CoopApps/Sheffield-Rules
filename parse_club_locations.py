#!/usr/bin/env python3
"""
Parse club locations from 1857-1875.txt and match them to postcodes from postcode.txt
Outputs Rust code for updating the SheffieldClub struct
"""

import re
from typing import Dict, Optional, Tuple

# Parse postcode.txt to build location -> postcode mapping
def parse_postcodes(postcode_file: str) -> Dict[str, str]:
    """Returns a dict mapping location names (lowercase) to postcodes"""
    location_to_postcode = {}

    with open(postcode_file, 'r', encoding='utf-8') as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith('S1-S36') or line.startswith('Postcode'):
                continue

            parts = line.split('\t')
            if len(parts) < 3:
                continue

            postcode = parts[0].strip()
            coverage = parts[2].strip()

            # Split coverage by commas and add each location
            locations = [loc.strip() for loc in coverage.split(',')]
            for location in locations:
                if location and location != 'City Centre':
                    location_to_postcode[location.lower()] = postcode

    return location_to_postcode

# Parse 1857-1875.txt to extract club data
def parse_clubs(clubs_file: str) -> list:
    """Returns list of (name, year, location_text) tuples"""
    clubs = []

    with open(clubs_file, 'r', encoding='utf-8') as f:
        for line in f:
            line = line.strip()
            if not line:
                continue

            # Parse format: "Club Name\tYear\tLocation info"
            parts = line.split('\t')
            if len(parts) >= 3:
                name = parts[0].strip()
                year = parts[1].strip()
                location = parts[2].strip()
                clubs.append((name, year, location))

    return clubs

# Extract area name from location text
def extract_area(location_text: str, location_to_postcode: Dict[str, str]) -> Tuple[Optional[str], Optional[str]]:
    """
    Extract the area name from location text and find matching postcode.
    Returns (area_name, postcode) tuple
    """
    # Common patterns to extract area names
    patterns = [
        r'(?:from|at|in|near)\s+([A-Z][a-z]+(?:\s+[A-Z][a-z]+)*)',
        r'Met at .+?,\s*([A-Z][a-z]+(?:\s+[A-Z][a-z]+)*)',
        r'played at .+?,\s*([A-Z][a-z]+(?:\s+[A-Z][a-z]+)*)',
        r',\s*([A-Z][a-z]+(?:\s+[A-Z][a-z]+)*)$',
    ]

    # Try to find known locations in the text
    text_lower = location_text.lower()

    # Direct match - check if any known location appears in the text
    best_match = None
    best_match_postcode = None
    longest_match_len = 0

    for location, postcode in location_to_postcode.items():
        if location in text_lower:
            # Prefer longer matches (e.g., "Wadsley Bridge" over "Wadsley")
            if len(location) > longest_match_len:
                best_match = location.title()
                best_match_postcode = postcode
                longest_match_len = len(location)

    if best_match:
        return (best_match, best_match_postcode)

    # If no direct match, try pattern extraction
    for pattern in patterns:
        match = re.search(pattern, location_text)
        if match:
            area = match.group(1)
            area_lower = area.lower()
            if area_lower in location_to_postcode:
                return (area, location_to_postcode[area_lower])

    # Special cases - extract from well-known patterns
    special_cases = {
        'east bank': 'Sheffield',
        'sandygate': 'Sheffield',
        'hunters bar': 'Ecclesall',
        'bramall lane': 'Sheffield',
        'hillsborough': 'Hillsborough',
    }

    for pattern, area in special_cases.items():
        if pattern in text_lower:
            area_lower = area.lower()
            if area_lower in location_to_postcode:
                return (area, location_to_postcode[area_lower])

    return (None, None)

def main():
    postcode_file = 'D:/projects/Saturday at Three/postcode.txt'
    clubs_file = 'D:/projects/Saturday at Three/1857-1875.txt'

    print("Parsing postcodes...")
    location_to_postcode = parse_postcodes(postcode_file)
    print(f"Found {len(location_to_postcode)} locations with postcodes")

    print("\nParsing clubs...")
    clubs = parse_clubs(clubs_file)
    print(f"Found {len(clubs)} clubs")

    print("\nMatching clubs to areas and postcodes...")
    matched = 0
    unmatched = []

    results = []
    for name, year, location_text in clubs:
        area, postcode = extract_area(location_text, location_to_postcode)
        if area and postcode:
            matched += 1
            results.append({
                'name': name,
                'year': year,
                'location_text': location_text,
                'area': area,
                'postcode': postcode
            })
        else:
            unmatched.append((name, location_text))

    print(f"\nMatched: {matched}/{len(clubs)}")
    print(f"Unmatched: {len(unmatched)}")

    # Output results
    print("\n" + "="*80)
    print("MATCHED CLUBS:")
    print("="*80)
    for r in results:
        print(f"{r['name']:<40} {r['year']:<6} {r['area']:<20} {r['postcode']:<5}")

    if unmatched:
        print("\n" + "="*80)
        print("UNMATCHED CLUBS (need manual review):")
        print("="*80)
        for name, location in unmatched[:20]:  # Show first 20
            print(f"{name:<40} {location}")

    # Export to CSV for easy review
    with open('D:/projects/Saturday at Three/club_locations.csv', 'w', encoding='utf-8') as f:
        f.write("Club Name,Year,Area,Postcode,Original Location\n")
        for r in results:
            f.write(f'"{r["name"]}",{r["year"]},"{r["area"]}",{r["postcode"]},"{r["location_text"]}"\n')

    print(f"\nResults exported to club_locations.csv")

if __name__ == '__main__':
    main()
