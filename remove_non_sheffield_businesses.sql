-- Remove businesses from Rotherham, Doncaster, and Barnsley

-- First, show what we're removing
SELECT 'Businesses to be removed:' as label;
SELECT 'Rotherham' as town, COUNT(*) as count
FROM sheffield_businesses
WHERE street_address LIKE '%Rotherham%'
UNION ALL
SELECT 'Doncaster' as town, COUNT(*) as count
FROM sheffield_businesses
WHERE street_address LIKE '%Doncaster%'
UNION ALL
SELECT 'Barnsley' as town, COUNT(*) as count
FROM sheffield_businesses
WHERE street_address LIKE '%Barnsley%';

-- Delete businesses from Rotherham
DELETE FROM sheffield_businesses
WHERE street_address LIKE '%Rotherham%'
   OR civil_parish LIKE '%Rotherham%'
   OR where_born LIKE '%Rotherham%'
   OR birth_town LIKE '%Rotherham%';

-- Delete businesses from Doncaster
DELETE FROM sheffield_businesses
WHERE street_address LIKE '%Doncaster%'
   OR civil_parish LIKE '%Doncaster%'
   OR where_born LIKE '%Doncaster%'
   OR birth_town LIKE '%Doncaster%';

-- Delete businesses from Barnsley
DELETE FROM sheffield_businesses
WHERE street_address LIKE '%Barnsley%'
   OR civil_parish LIKE '%Barnsley%'
   OR where_born LIKE '%Barnsley%'
   OR birth_town LIKE '%Barnsley%';

-- Show results
SELECT 'Remaining businesses:' as label, COUNT(*) as count
FROM sheffield_businesses;

SELECT 'Breakdown after removal:' as label;
SELECT
    'Sheffield' as location,
    COUNT(*) as count
FROM sheffield_businesses
WHERE street_address LIKE '%Sheffield%'
UNION ALL
SELECT
    'Other/Unknown' as location,
    COUNT(*) as count
FROM sheffield_businesses
WHERE street_address IS NULL OR street_address NOT LIKE '%Sheffield%';
