-- Match unique names with same profession but different addresses
-- These are likely business vs home addresses for the same person

-- Find names with same profession but different addresses
CREATE TEMP TABLE name_prof_diff_addr AS
SELECT DISTINCT b.name, b.profession
FROM sheffield_businesses b
INNER JOIN unmatched_genealogy g
    ON b.name = g.name
    AND b.profession = g.profession
WHERE b.name IS NOT NULL
    AND b.name <> ''
    AND b.profession IS NOT NULL
    AND b.profession <> ''
    AND b.street_address <> g.street_address
    AND b.street_address IS NOT NULL
    AND g.street_address IS NOT NULL;

-- Count how many times each name appears in businesses
CREATE TEMP TABLE business_name_counts AS
SELECT name, COUNT(*) as count
FROM sheffield_businesses
GROUP BY name;

-- Count how many times each name appears in genealogy
CREATE TEMP TABLE genealogy_name_counts AS
SELECT name, COUNT(*) as count
FROM unmatched_genealogy
GROUP BY name;

-- Create matches for unique names only
CREATE TEMP TABLE unique_name_matches AS
SELECT DISTINCT
    g.rowid as genealogy_rowid,
    b.rowid as business_rowid
FROM unmatched_genealogy g
INNER JOIN sheffield_businesses b
    ON g.name = b.name
    AND g.profession = b.profession
INNER JOIN business_name_counts bc ON b.name = bc.name
INNER JOIN genealogy_name_counts gc ON g.name = gc.name
WHERE bc.count = 1
    AND gc.count = 1
    AND b.street_address <> g.street_address
    AND b.street_address IS NOT NULL
    AND g.street_address IS NOT NULL
    AND g.matched_to_business_id IS NULL;

-- Create index for fast lookup
CREATE INDEX idx_unique_matches ON unique_name_matches(genealogy_rowid);

-- Update genealogy with business rowid
UPDATE unmatched_genealogy
SET matched_to_business_id = (
    SELECT business_rowid
    FROM unique_name_matches
    WHERE unique_name_matches.genealogy_rowid = unmatched_genealogy.rowid
    LIMIT 1
)
WHERE rowid IN (SELECT genealogy_rowid FROM unique_name_matches);

-- Show results
SELECT COUNT(*) as total_matched_to_business
FROM unmatched_genealogy
WHERE matched_to_business_id IS NOT NULL;
