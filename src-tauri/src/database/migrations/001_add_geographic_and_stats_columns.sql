-- Migration: Add geographic columns and has_stats flag to sheffield_players table

-- Add geographic/census information columns if they don't exist
ALTER TABLE sheffield_players ADD COLUMN where_born TEXT;
ALTER TABLE sheffield_players ADD COLUMN birth_town TEXT;
ALTER TABLE sheffield_players ADD COLUMN birth_county TEXT;
ALTER TABLE sheffield_players ADD COLUMN birth_country TEXT;
ALTER TABLE sheffield_players ADD COLUMN civil_parish TEXT;
ALTER TABLE sheffield_players ADD COLUMN ecclesiastical_parish TEXT;
ALTER TABLE sheffield_players ADD COLUMN registration_district TEXT;
ALTER TABLE sheffield_players ADD COLUMN sub_registration_district TEXT;

-- Add has_stats column to track stat assignment
ALTER TABLE sheffield_players ADD COLUMN has_stats BOOLEAN DEFAULT 0;
