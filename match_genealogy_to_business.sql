-- Create lookup table for name+profession matches
CREATE TEMP TABLE business_matches AS
SELECT DISTINCT
    g.rowid as genealogy_rowid,
    b.rowid as business_rowid
FROM unmatched_genealogy g
INNER JOIN sheffield_businesses b
    ON g.name = b.name
    AND g.profession = b.profession
WHERE g.name IS NOT NULL
    AND g.name <> ''
    AND g.profession IS NOT NULL
    AND g.profession <> '';

-- Create index for fast lookup
CREATE INDEX idx_business_matches ON business_matches(genealogy_rowid);

-- Update genealogy with business rowid
UPDATE unmatched_genealogy
SET matched_to_business_id = (
    SELECT business_rowid
    FROM business_matches
    WHERE business_matches.genealogy_rowid = unmatched_genealogy.rowid
    LIMIT 1
)
WHERE rowid IN (SELECT genealogy_rowid FROM business_matches);

-- Show results
SELECT COUNT(*) as records_matched_to_business
FROM unmatched_genealogy
WHERE matched_to_business_id IS NOT NULL;
