-- Match unique names where professions are similar variations
-- This matches cases like "hairdresser" vs "Hair Dresser", "Jeweller" vs "Working Jeweller", etc.

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

-- Match unique names where one profession contains the other (similar professions)
CREATE TEMP TABLE similar_profession_matches AS
SELECT DISTINCT
    g.rowid as genealogy_rowid,
    b.rowid as business_rowid
FROM unmatched_genealogy g
INNER JOIN sheffield_businesses b
    ON g.name = b.name
INNER JOIN business_name_counts bc ON b.name = bc.name
INNER JOIN genealogy_name_counts gc ON g.name = gc.name
WHERE bc.count = 1
    AND gc.count = 1
    AND g.profession <> b.profession
    AND g.profession IS NOT NULL
    AND b.profession IS NOT NULL
    AND g.matched_to_business_id IS NULL
    AND (
        -- One profession contains the other (case insensitive)
        LOWER(g.profession) LIKE '%' || LOWER(b.profession) || '%'
        OR LOWER(b.profession) LIKE '%' || LOWER(g.profession) || '%'
        -- Or they're the same with different spacing/capitalization
        OR REPLACE(REPLACE(LOWER(g.profession), ' ', ''), '&', '') = REPLACE(REPLACE(LOWER(b.profession), ' ', ''), '&', '')
    );

-- Create index for fast lookup
CREATE INDEX idx_similar_prof_matches ON similar_profession_matches(genealogy_rowid);

-- Update genealogy with business rowid
UPDATE unmatched_genealogy
SET matched_to_business_id = (
    SELECT business_rowid
    FROM similar_profession_matches
    WHERE similar_profession_matches.genealogy_rowid = unmatched_genealogy.rowid
    LIMIT 1
)
WHERE rowid IN (SELECT genealogy_rowid FROM similar_profession_matches);

-- Show results
SELECT COUNT(*) as total_matched_to_business
FROM unmatched_genealogy
WHERE matched_to_business_id IS NOT NULL;
