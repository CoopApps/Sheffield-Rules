-- Match exact name and address (now that periods removed from business names)
CREATE TEMP TABLE exact_name_address_matches AS
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
    AND g.matched_to_business_id IS NULL;

-- Create index for fast lookup
CREATE INDEX idx_exact_name_addr_matches ON exact_name_address_matches(genealogy_rowid);

-- Update genealogy with business rowid
UPDATE unmatched_genealogy
SET matched_to_business_id = (
    SELECT business_rowid
    FROM exact_name_address_matches
    WHERE exact_name_address_matches.genealogy_rowid = unmatched_genealogy.rowid
    LIMIT 1
)
WHERE rowid IN (SELECT genealogy_rowid FROM exact_name_address_matches);

-- Show results
SELECT COUNT(*) as total_matched_to_business
FROM unmatched_genealogy
WHERE matched_to_business_id IS NOT NULL;
