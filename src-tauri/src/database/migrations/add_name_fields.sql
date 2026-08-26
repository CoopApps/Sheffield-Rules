-- Migration: Add separate name fields to sheffield_players table
-- Date: 2026-02-12
-- Purpose: Split name into first_name, middle_name, surname for better organization

-- Add name component fields
ALTER TABLE sheffield_players ADD COLUMN first_name TEXT;
ALTER TABLE sheffield_players ADD COLUMN middle_name TEXT;
ALTER TABLE sheffield_players ADD COLUMN surname TEXT;

-- Populate the new fields from existing name data
-- This is a simple split - first word becomes first_name, last word becomes surname
-- Everything in between becomes middle_name
UPDATE sheffield_players
SET
  first_name = CASE
    WHEN instr(name, ' ') > 0 THEN substr(name, 1, instr(name, ' ') - 1)
    ELSE name
  END,
  surname = CASE
    WHEN instr(name, ' ') > 0 THEN substr(name, instr(name, ' ', length(name) - instr(name || ' ', ' ', -1) + 2) + 1)
    ELSE ''
  END,
  middle_name = CASE
    WHEN length(name) - length(replace(name, ' ', '')) > 1 THEN
      trim(substr(name, instr(name, ' ') + 1, length(name) - instr(name, ' ') - length(substr(name, instr(name, ' ', length(name) - instr(name || ' ', ' ', -1) + 2) + 1))))
    ELSE ''
  END
WHERE first_name IS NULL;
