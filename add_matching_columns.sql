-- Add matching/linking columns to all data tables

-- Add to unmatched_ancestry
ALTER TABLE unmatched_ancestry ADD COLUMN matched_to_genealogy_id INTEGER;
ALTER TABLE unmatched_ancestry ADD COLUMN matched_to_business_id INTEGER;
ALTER TABLE unmatched_ancestry ADD COLUMN match_confidence REAL;
ALTER TABLE unmatched_ancestry ADD COLUMN match_status TEXT; -- 'pending', 'confirmed', 'rejected'

-- Add to unmatched_genealogy
ALTER TABLE unmatched_genealogy ADD COLUMN matched_to_ancestry_id INTEGER;
ALTER TABLE unmatched_genealogy ADD COLUMN matched_to_business_id INTEGER;
ALTER TABLE unmatched_genealogy ADD COLUMN match_confidence REAL;
ALTER TABLE unmatched_genealogy ADD COLUMN match_status TEXT;

-- Add to sheffield_businesses
ALTER TABLE sheffield_businesses ADD COLUMN matched_to_ancestry_id INTEGER;
ALTER TABLE sheffield_businesses ADD COLUMN matched_to_genealogy_id INTEGER;
ALTER TABLE sheffield_businesses ADD COLUMN match_confidence REAL;
ALTER TABLE sheffield_businesses ADD COLUMN match_status TEXT;

-- Add to institutional tables too
ALTER TABLE sheffield_asylum ADD COLUMN matched_to_ancestry_id INTEGER;
ALTER TABLE sheffield_asylum ADD COLUMN matched_to_genealogy_id INTEGER;
ALTER TABLE sheffield_asylum ADD COLUMN match_confidence REAL;
ALTER TABLE sheffield_asylum ADD COLUMN match_status TEXT;

ALTER TABLE sheffield_workhouse ADD COLUMN matched_to_ancestry_id INTEGER;
ALTER TABLE sheffield_workhouse ADD COLUMN matched_to_genealogy_id INTEGER;
ALTER TABLE sheffield_workhouse ADD COLUMN match_confidence REAL;
ALTER TABLE sheffield_workhouse ADD COLUMN match_status TEXT;

ALTER TABLE sheffield_prison ADD COLUMN matched_to_ancestry_id INTEGER;
ALTER TABLE sheffield_prison ADD COLUMN matched_to_genealogy_id INTEGER;
ALTER TABLE sheffield_prison ADD COLUMN match_confidence REAL;
ALTER TABLE sheffield_prison ADD COLUMN match_status TEXT;

ALTER TABLE sheffield_patron ADD COLUMN matched_to_ancestry_id INTEGER;
ALTER TABLE sheffield_patron ADD COLUMN matched_to_genealogy_id INTEGER;
ALTER TABLE sheffield_patron ADD COLUMN match_confidence REAL;
ALTER TABLE sheffield_patron ADD COLUMN match_status TEXT;
