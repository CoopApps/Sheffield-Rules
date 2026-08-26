-- Find unique names with different professions

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

-- Find names that appear exactly once in both tables with different professions
SELECT COUNT(DISTINCT g.name) as unique_names_diff_prof
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
    AND g.matched_to_business_id IS NULL;
