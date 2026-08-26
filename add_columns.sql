-- Run this SQL file directly on your database to add the missing columns
-- File: saturday_at_three.db or Sheffield1867.db

-- Add geographic columns (ignore errors if columns already exist)
ALTER TABLE sheffield_players ADD COLUMN where_born TEXT;
ALTER TABLE sheffield_players ADD COLUMN birth_town TEXT;
ALTER TABLE sheffield_players ADD COLUMN birth_county TEXT;
ALTER TABLE sheffield_players ADD COLUMN birth_country TEXT;
ALTER TABLE sheffield_players ADD COLUMN civil_parish TEXT;
ALTER TABLE sheffield_players ADD COLUMN ecclesiastical_parish TEXT;
ALTER TABLE sheffield_players ADD COLUMN registration_district TEXT;
ALTER TABLE sheffield_players ADD COLUMN sub_registration_district TEXT;

-- Add has_stats tracking column
ALTER TABLE sheffield_players ADD COLUMN has_stats BOOLEAN DEFAULT 0;

-- Verify columns were added
PRAGMA table_info(sheffield_players);
