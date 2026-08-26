-- Check how many businesses in sheffield_businesses aren't in Sheffield
-- Using street_address, civil_parish, and other location fields

SELECT 'Total businesses:' as label, COUNT(*) as count
FROM sheffield_businesses;

-- Businesses with Sheffield in any location field
SELECT 'Businesses with Sheffield in location:' as label, COUNT(*) as count
FROM sheffield_businesses
WHERE street_address LIKE '%Sheffield%'
   OR civil_parish LIKE '%Sheffield%'
   OR where_born LIKE '%Sheffield%'
   OR birth_town LIKE '%Sheffield%';

-- Businesses WITHOUT Sheffield in any location field
SELECT 'Businesses WITHOUT Sheffield in location:' as label, COUNT(*) as count
FROM sheffield_businesses
WHERE (street_address IS NULL OR street_address NOT LIKE '%Sheffield%')
  AND (civil_parish IS NULL OR civil_parish NOT LIKE '%Sheffield%')
  AND (where_born IS NULL OR where_born NOT LIKE '%Sheffield%')
  AND (birth_town IS NULL OR birth_town NOT LIKE '%Sheffield%');

-- Show examples of non-Sheffield businesses
SELECT 'Examples of non-Sheffield businesses:' as label;
SELECT name, street_address, civil_parish, birth_town, where_born
FROM sheffield_businesses
WHERE (street_address IS NULL OR street_address NOT LIKE '%Sheffield%')
  AND (civil_parish IS NULL OR civil_parish NOT LIKE '%Sheffield%')
  AND (where_born IS NULL OR where_born NOT LIKE '%Sheffield%')
  AND (birth_town IS NULL OR birth_town NOT LIKE '%Sheffield%')
LIMIT 20;
