-- Check how many businesses are in Doncaster and Rotherham

SELECT 'Businesses in Rotherham:' as label, COUNT(*) as count
FROM sheffield_businesses
WHERE street_address LIKE '%Rotherham%'
   OR civil_parish LIKE '%Rotherham%'
   OR where_born LIKE '%Rotherham%'
   OR birth_town LIKE '%Rotherham%';

SELECT 'Businesses in Doncaster:' as label, COUNT(*) as count
FROM sheffield_businesses
WHERE street_address LIKE '%Doncaster%'
   OR civil_parish LIKE '%Doncaster%'
   OR where_born LIKE '%Doncaster%'
   OR birth_town LIKE '%Doncaster%';

SELECT 'Businesses in Barnsley:' as label, COUNT(*) as count
FROM sheffield_businesses
WHERE street_address LIKE '%Barnsley%'
   OR civil_parish LIKE '%Barnsley%'
   OR where_born LIKE '%Barnsley%'
   OR birth_town LIKE '%Barnsley%';

-- Show breakdown
SELECT 'Location breakdown:' as label;
SELECT
    'Rotherham' as location,
    COUNT(*) as count
FROM sheffield_businesses
WHERE street_address LIKE '%Rotherham%'

UNION ALL

SELECT
    'Doncaster' as location,
    COUNT(*) as count
FROM sheffield_businesses
WHERE street_address LIKE '%Doncaster%'

UNION ALL

SELECT
    'Barnsley' as location,
    COUNT(*) as count
FROM sheffield_businesses
WHERE street_address LIKE '%Barnsley%'

UNION ALL

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
WHERE (street_address IS NULL OR
       (street_address NOT LIKE '%Sheffield%'
        AND street_address NOT LIKE '%Rotherham%'
        AND street_address NOT LIKE '%Doncaster%'
        AND street_address NOT LIKE '%Barnsley%'));
