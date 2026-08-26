-- Create lookup table for name+address matches (where profession differs)
CREATE TEMP TABLE business_address_matches AS
SELECT DISTINCT
    g.rowid as genealogy_rowid,
    b.rowid as business_rowid
FROM unmatched_genealogy g
INNER JOIN sheffield_businesses b
    ON g.name = b.name
    AND g.street_address = b.street_address
WHERE g.name IS NOT NULL
    AND g.name <> ''
    AND g.street_address IS NOT NULL
    AND g.street_address <> ''
    AND g.profession <> b.profession
    AND g.profession IS NOT NULL
    AND b.profession IS NOT NULL
    AND g.matched_to_business_id IS NULL;  -- Don't overwrite existing exact matches

-- Create index for fast lookup
CREATE INDEX idx_business_address_matches ON business_address_matches(genealogy_rowid);

-- Update genealogy with business rowid
UPDATE unmatched_genealogy
SET matched_to_business_id = (
    SELECT business_rowid
    FROM business_address_matches
    WHERE business_address_matches.genealogy_rowid = unmatched_genealogy.rowid
    LIMIT 1
)
WHERE rowid IN (SELECT genealogy_rowid FROM business_address_matches);

-- Show results
SELECT COUNT(*) as total_matched_to_business
FROM unmatched_genealogy
WHERE matched_to_business_id IS NOT NULL;
