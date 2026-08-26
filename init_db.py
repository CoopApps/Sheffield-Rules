#!/usr/bin/env python3
"""Initialize Saturday at Three database with 1888-89 player and club data."""

import sqlite3
import os
from pathlib import Path

DB_PATH = "D:/projects/Saturday at Three/saturday_at_three.db"
FM_DB_PATH = "D:/projects/Football Man/players1888.db"

def create_tables(conn):
    """Create all necessary tables."""
    cursor = conn.cursor()

    # Clubs table
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS clubs (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            short_name TEXT,
            founded_year INTEGER,
            ground_name TEXT,
            ground_capacity INTEGER,
            city TEXT,
            region TEXT,
            primary_color TEXT,
            secondary_color TEXT,
            badge_url TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
    """)

    # Players table
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS players (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            club_id TEXT NOT NULL,
            position TEXT,
            birth_year INTEGER,
            age INTEGER,
            nationality TEXT,
            height REAL,
            weight REAL,
            pace INTEGER,
            strength INTEGER,
            stamina INTEGER,
            balance INTEGER,
            jumping INTEGER,
            agility INTEGER,
            passing INTEGER,
            dribbling INTEGER,
            heading INTEGER,
            crossing INTEGER,
            tackling INTEGER,
            handling INTEGER,
            reflexes INTEGER,
            courage INTEGER,
            concentration INTEGER,
            leadership INTEGER,
            aggression INTEGER,
            determination INTEGER,
            flair INTEGER,
            influence INTEGER,
            awareness INTEGER,
            marking INTEGER,
            positioning INTEGER,
            work_rate INTEGER,
            finishing INTEGER,
            penalties INTEGER,
            set_pieces INTEGER,
            is_injured BOOLEAN DEFAULT 0,
            injury_type TEXT,
            injury_duration INTEGER,
            suspension_games INTEGER DEFAULT 0,
            form_rating REAL DEFAULT 10.0,
            matches_played INTEGER DEFAULT 0,
            goals_scored INTEGER DEFAULT 0,
            assists INTEGER DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (club_id) REFERENCES clubs(id)
        )
    """)

    # Matches table
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS matches (
            id TEXT PRIMARY KEY,
            gameweek INTEGER NOT NULL,
            season INTEGER NOT NULL,
            home_club_id TEXT NOT NULL,
            away_club_id TEXT NOT NULL,
            home_score INTEGER,
            away_score INTEGER,
            played BOOLEAN DEFAULT 0,
            match_date DATETIME,
            attendance INTEGER,
            weather TEXT,
            pitch_condition TEXT,
            referee_id TEXT,
            home_possession REAL,
            away_possession REAL,
            home_shots INTEGER,
            away_shots INTEGER,
            home_shots_on_target INTEGER,
            away_shots_on_target INTEGER,
            home_passes INTEGER,
            away_passes INTEGER,
            home_fouls INTEGER,
            away_fouls INTEGER,
            home_cards_yellow INTEGER,
            away_cards_yellow INTEGER,
            home_cards_red INTEGER,
            away_cards_red INTEGER,
            match_report TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (home_club_id) REFERENCES clubs(id),
            FOREIGN KEY (away_club_id) REFERENCES clubs(id)
        )
    """)

    # Match incidents table
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS match_incidents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            match_id TEXT NOT NULL,
            minute INTEGER,
            incident_type TEXT,
            player_id TEXT,
            club_id TEXT,
            description TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (match_id) REFERENCES matches(id),
            FOREIGN KEY (player_id) REFERENCES players(id),
            FOREIGN KEY (club_id) REFERENCES clubs(id)
        )
    """)

    # League standings table
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS standings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            season INTEGER NOT NULL,
            position INTEGER NOT NULL,
            club_id TEXT NOT NULL,
            played INTEGER DEFAULT 0,
            won INTEGER DEFAULT 0,
            drawn INTEGER DEFAULT 0,
            lost INTEGER DEFAULT 0,
            goals_for INTEGER DEFAULT 0,
            goals_against INTEGER DEFAULT 0,
            goal_difference INTEGER DEFAULT 0,
            points INTEGER DEFAULT 0,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (club_id) REFERENCES clubs(id),
            UNIQUE(season, club_id)
        )
    """)

    conn.commit()
    print("✓ Tables created")

def import_clubs(conn):
    """Import 12 founding Football League clubs."""
    clubs = [
        ("accrington", "Accrington FC", "ACC", 1888, "Peel Park", 5000, "Accrington", "Lancashire", "#cc0000", "#ffffff"),
        ("aston-villa", "Aston Villa", "AV", 1874, "Perry Barr", 8000, "Birmingham", "Midlands", "#660099", "#000000"),
        ("blackburn", "Blackburn Rovers", "BRN", 1875, "Ewood Park", 8000, "Blackburn", "Lancashire", "#000099", "#ffffff"),
        ("bolton", "Bolton Wanderers", "BOL", 1877, "Pike Lane", 6000, "Bolton", "Lancashire", "#ffffff", "#000000"),
        ("burnley", "Burnley FC", "BFC", 1882, "Turf Moor", 5000, "Burnley", "Lancashire", "#6b3e99", "#ffffff"),
        ("derby", "Derby County", "DER", 1884, "The Racecourse", 5000, "Derby", "East Midlands", "#000000", "#ffffff"),
        ("everton", "Everton", "EVE", 1878, "Anfield", 8000, "Liverpool", "Merseyside", "#003DA5", "#ffffff"),
        ("notts-county", "Notts County", "NOT", 1862, "Trent Bridge", 5000, "Nottingham", "East Midlands", "#000000", "#ffffff"),
        ("preston", "Preston North End", "PRE", 1880, "Deepdale", 8000, "Preston", "Lancashire", "#ffffff", "#000000"),
        ("stoke", "Stoke City", "STO", 1863, "The Victoria Ground", 5000, "Stoke", "Staffordshire", "#e20e0e", "#ffffff"),
        ("sunderland", "Sunderland AFC", "SUN", 1879, "Roker Park", 8000, "Sunderland", "Northeast", "#FF0000", "#ffffff"),
        ("wolves", "Wolverhampton Wanderers", "WOL", 1877, "Molineux", 8000, "Wolverhampton", "Midlands", "#FFD700", "#000000"),
    ]

    cursor = conn.cursor()
    for club_id, name, short, year, ground, cap, city, region, primary, secondary in clubs:
        cursor.execute("""
            INSERT OR IGNORE INTO clubs
            (id, name, short_name, founded_year, ground_name, ground_capacity, city, region, primary_color, secondary_color)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        """, (club_id, name, short, year, ground, cap, city, region, primary, secondary))

    conn.commit()
    print(f"✓ {len(clubs)} founding clubs imported")

def import_players(conn):
    """Import players from Football Man database."""
    if not os.path.exists(FM_DB_PATH):
        print(f"⚠ Football Man database not found at {FM_DB_PATH}")
        print("✓ Database structure ready for player data")
        return

    try:
        fm_conn = sqlite3.connect(FM_DB_PATH)
        fm_cursor = fm_conn.cursor()

        # Get all players from Football Man
        fm_cursor.execute("""
            SELECT
                id, name, club_id, position, birth_year, age, nationality,
                pace, strength, stamina, balance, jumping, agility,
                passing, dribbling, heading, crossing, tackling, handling, reflexes,
                courage, concentration, leadership, aggression, determination, flair, influence,
                awareness, marking, positioning, work_rate, finishing, penalties, set_pieces
            FROM players
            LIMIT 2500
        """)

        players = fm_cursor.fetchall()
        cursor = conn.cursor()

        for player_data in players:
            cursor.execute("""
                INSERT OR IGNORE INTO players
                (id, name, club_id, position, birth_year, age, nationality,
                 pace, strength, stamina, balance, jumping, agility,
                 passing, dribbling, heading, crossing, tackling, handling, reflexes,
                 courage, concentration, leadership, aggression, determination, flair, influence,
                 awareness, marking, positioning, work_rate, finishing, penalties, set_pieces)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """, player_data)

        conn.commit()
        fm_conn.close()
        print(f"✓ {len(players)} players imported from 1888-89 season")

    except Exception as e:
        print(f"⚠ Error importing players: {e}")
        print("✓ Database structure ready, will populate player data later")

def main():
    """Initialize the database."""
    print("=" * 60)
    print("Saturday at Three - Database Initialization")
    print("=" * 60)
    print()

    # Remove existing database if present
    if os.path.exists(DB_PATH):
        os.remove(DB_PATH)
        print("⚠ Removed existing database")

    # Create new database
    conn = sqlite3.connect(DB_PATH)

    try:
        create_tables(conn)
        import_clubs(conn)
        import_players(conn)

        print()
        print("=" * 60)
        print("✓ Database initialized successfully!")
        print(f"✓ Database location: {DB_PATH}")
        print("✓ Ready for match simulation and gameplay")
        print("=" * 60)

    except Exception as e:
        print(f"✗ Error: {e}")
        return 1
    finally:
        conn.close()

    return 0

if __name__ == "__main__":
    exit(main())
