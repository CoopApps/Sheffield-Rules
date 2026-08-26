-- Find names with same profession but different addresses that are unique in both tables
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

-- Find names that appear exactly once in both tables
SELECT COUNT(DISTINCT np.name) as unique_names_in_both
FROM name_prof_diff_addr np
INNER JOIN business_name_counts bc ON np.name = bc.name
INNER JOIN genealogy_name_counts gc ON np.name = gc.name
WHERE bc.count = 1 AND gc.count = 1;
