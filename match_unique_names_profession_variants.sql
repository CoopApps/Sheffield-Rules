-- Match unique names where professions are variants (spacing, retired, additional details, etc.)
-- Only match if name appears exactly once in each database

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

-- Match unique names where professions are variants
CREATE TEMP TABLE profession_variant_matches AS
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
    AND g.matched_to_business_id IS NULL
    AND g.profession IS NOT NULL
    AND b.profession IS NOT NULL
    AND g.profession <> b.profession
    AND (
        -- Same profession without spaces
        REPLACE(LOWER(g.profession), ' ', '') = REPLACE(LOWER(b.profession), ' ', '')
        -- One profession contains the other (e.g., "farmer" in "Retired Farmer")
        OR LOWER(g.profession) LIKE '%' || REPLACE(LOWER(b.profession), ' ', '') || '%'
        OR LOWER(b.profession) LIKE '%' || REPLACE(LOWER(g.profession), ' ', '') || '%'
    );

-- Create index for fast lookup
CREATE INDEX idx_profession_variant_matches ON profession_variant_matches(genealogy_rowid);

-- Update genealogy with business rowid
UPDATE unmatched_genealogy
SET matched_to_business_id = (
    SELECT business_rowid
    FROM profession_variant_matches
    WHERE profession_variant_matches.genealogy_rowid = unmatched_genealogy.rowid
    LIMIT 1
)
WHERE rowid IN (SELECT genealogy_rowid FROM profession_variant_matches);

-- Show results
SELECT COUNT(*) as total_matched_to_business
FROM unmatched_genealogy
WHERE matched_to_business_id IS NOT NULL;
