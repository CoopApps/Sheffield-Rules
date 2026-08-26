-- Match same address + name (punctuation diff) where professions are related/variants
CREATE TEMP TABLE related_profession_matches AS
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
    AND b.profession IS NOT NULL AND b.profession <> '' AND b.profession <> '-'
    AND g.profession IS NOT NULL AND g.profession <> '' AND g.profession <> '-'
    AND (
        -- Same profession without spaces/punctuation
        REPLACE(REPLACE(REPLACE(LOWER(b.profession), ' ', ''), '.', ''), '&', '') =
        REPLACE(REPLACE(REPLACE(LOWER(g.profession), ' ', ''), '.', ''), '&', '')
        -- One is abbreviation of the other (within 5 chars and starts the same)
        OR (ABS(LENGTH(b.profession) - LENGTH(g.profession)) <= 5
            AND LOWER(SUBSTR(b.profession, 1, 4)) = LOWER(SUBSTR(g.profession, 1, 4)))
        -- Common profession synonyms
        OR (LOWER(b.profession) IN ('shoemaker', 'shoemr.', 'shoemkr.') AND LOWER(g.profession) IN ('cordwainer', 'boot maker', 'boot & shoe maker', 'cord wainer'))
        OR (LOWER(g.profession) IN ('shoemaker', 'shoemr.', 'shoemkr.') AND LOWER(b.profession) IN ('cordwainer', 'boot maker', 'boot & shoe maker', 'cord wainer'))
        OR (LOWER(b.profession) IN ('shopkeeper', 'shopkpr.', 'shopr.', 'shopkr') AND LOWER(g.profession) LIKE '%grocer%')
        OR (LOWER(g.profession) IN ('shopkeeper', 'shopkpr.', 'shopr.', 'shopkr') AND LOWER(b.profession) LIKE '%grocer%')
        OR (LOWER(b.profession) LIKE '%provision%' AND LOWER(g.profession) LIKE '%grocer%')
        OR (LOWER(g.profession) LIKE '%provision%' AND LOWER(b.profession) LIKE '%grocer%')
        OR (LOWER(b.profession) LIKE '%beer%house%' AND LOWER(g.profession) LIKE '%publican%')
        OR (LOWER(g.profession) LIKE '%beer%house%' AND LOWER(b.profession) LIKE '%publican%')
        OR (LOWER(b.profession) LIKE '%victualler%' AND LOWER(g.profession) LIKE '%publican%')
        OR (LOWER(g.profession) LIKE '%victualler%' AND LOWER(b.profession) LIKE '%publican%')
    );

-- Create index for fast lookup
CREATE INDEX idx_related_prof_matches ON related_profession_matches(genealogy_rowid);

-- Update genealogy with business rowid
UPDATE unmatched_genealogy
SET matched_to_business_id = (
    SELECT business_rowid
    FROM related_profession_matches
    WHERE related_profession_matches.genealogy_rowid = unmatched_genealogy.rowid
    LIMIT 1
)
WHERE rowid IN (SELECT genealogy_rowid FROM related_profession_matches);

-- Show results
SELECT COUNT(*) as total_matched_to_business
FROM unmatched_genealogy
WHERE matched_to_business_id IS NOT NULL;
