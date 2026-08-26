-- Match same address where names differ only by punctuation/spacing
-- AND professions are similar or one is missing (safer matching)
CREATE TEMP TABLE punctuation_matches AS
SELECT DISTINCT
    g.rowid as genealogy_rowid,
    b.rowid as business_rowid
FROM unmatched_genealogy g
INNER JOIN sheffield_businesses b
    ON g.street_address = b.street_address
WHERE g.street_address IS NOT NULL
    AND g.street_address <> ''
    AND g.matched_to_business_id IS NULL
    AND b.name <> g.name
    AND REPLACE(REPLACE(LOWER(b.name), '.', ''), ' ', '') = REPLACE(REPLACE(LOWER(g.name), '.', ''), ' ', '')
    AND (
        -- No profession in one or both
        b.profession IS NULL OR b.profession = '' OR b.profession = '-'
        OR g.profession IS NULL OR g.profession = '' OR g.profession = '-'
        -- Professions match or are similar
        OR b.profession = g.profession
        OR LOWER(g.profession) LIKE '%' || LOWER(b.profession) || '%'
        OR LOWER(b.profession) LIKE '%' || LOWER(g.profession) || '%'
    );

-- Create index for fast lookup
CREATE INDEX idx_punct_matches ON punctuation_matches(genealogy_rowid);

-- Update genealogy with business rowid
UPDATE unmatched_genealogy
SET matched_to_business_id = (
    SELECT business_rowid
    FROM punctuation_matches
    WHERE punctuation_matches.genealogy_rowid = unmatched_genealogy.rowid
    LIMIT 1
)
WHERE rowid IN (SELECT genealogy_rowid FROM punctuation_matches);

-- Show results
SELECT COUNT(*) as total_matched_to_business
FROM unmatched_genealogy
WHERE matched_to_business_id IS NOT NULL;
