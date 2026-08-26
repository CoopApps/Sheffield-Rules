-- Migration: Add geographic/census fields to sheffield_players table
-- Date: 2026-02-12
-- Purpose: Enable team assignment based on player geographic origin

-- Add geographic fields to players table
ALTER TABLE sheffield_players ADD COLUMN where_born TEXT;
ALTER TABLE sheffield_players ADD COLUMN birth_town TEXT;
ALTER TABLE sheffield_players ADD COLUMN birth_county TEXT;
ALTER TABLE sheffield_players ADD COLUMN birth_country TEXT;
ALTER TABLE sheffield_players ADD COLUMN civil_parish TEXT;
ALTER TABLE sheffield_players ADD COLUMN ecclesiastical_parish TEXT;
ALTER TABLE sheffield_players ADD COLUMN registration_district TEXT;
ALTER TABLE sheffield_players ADD COLUMN sub_registration_district TEXT;

-- Add has_stats tracking field
ALTER TABLE sheffield_players ADD COLUMN has_stats BOOLEAN DEFAULT 0;
