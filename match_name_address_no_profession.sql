-- Match exact name and address where business has no profession listed
CREATE TEMP TABLE no_profession_matches AS
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
    AND (b.profession IS NULL OR b.profession = '' OR b.profession = '-')
    AND g.matched_to_business_id IS NULL;

-- Create index for fast lookup
CREATE INDEX idx_no_prof_matches ON no_profession_matches(genealogy_rowid);

-- Update genealogy with business rowid
UPDATE unmatched_genealogy
SET matched_to_business_id = (
    SELECT business_rowid
    FROM no_profession_matches
    WHERE no_profession_matches.genealogy_rowid = unmatched_genealogy.rowid
    LIMIT 1
)
WHERE rowid IN (SELECT genealogy_rowid FROM no_profession_matches);

-- Show results
SELECT COUNT(*) as total_matched_to_business
FROM unmatched_genealogy
WHERE matched_to_business_id IS NOT NULL;
