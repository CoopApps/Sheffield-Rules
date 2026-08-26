-- Migration to add missing columns to sheffield_clubs table
-- Current structure: id, name, founded_year, origin
-- Need to add: ground_name, city, region

-- Add ground_name column (use origin data as initial value)
ALTER TABLE sheffield_clubs ADD COLUMN ground_name TEXT;
UPDATE sheffield_clubs SET ground_name = origin;

-- Add city column (empty for now, can be populated later)
ALTER TABLE sheffield_clubs ADD COLUMN city TEXT DEFAULT '';

-- Add region column (empty for now, can be populated later)
ALTER TABLE sheffield_clubs ADD COLUMN region TEXT DEFAULT '';
