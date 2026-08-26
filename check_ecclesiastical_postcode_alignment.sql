-- Check how existing postcodes align with ecclesiastical districts

-- Show distribution of postcodes
SELECT 'Current postcode distribution:' as label;
SELECT postcode, COUNT(*) as count
FROM unmatched_genealogy
WHERE postcode IS NOT NULL AND postcode != ''
GROUP BY postcode
ORDER BY postcode;

-- Show ecclesiastical districts with existing postcodes
SELECT 'Ecclesiastical districts and their current postcodes:' as label;
SELECT
    ecclesiastical_parish,
    postcode,
    COUNT(*) as count
FROM unmatched_genealogy
WHERE ecclesiastical_parish IS NOT NULL
  AND ecclesiastical_parish != ''
  AND postcode IS NOT NULL
  AND postcode != ''
GROUP BY ecclesiastical_parish, postcode
ORDER BY ecclesiastical_parish, postcode;

-- Show ecclesiastical districts WITHOUT postcodes
SELECT 'Ecclesiastical districts without postcodes:' as label;
SELECT
    ecclesiastical_parish,
    COUNT(*) as count
FROM unmatched_genealogy
WHERE ecclesiastical_parish IS NOT NULL
  AND ecclesiastical_parish != ''
  AND (postcode IS NULL OR postcode = '')
GROUP BY ecclesiastical_parish
ORDER BY count DESC
LIMIT 20;

-- Show sample records with ecclesiastical parish but no postcode
SELECT 'Sample records with ecclesiastical parish but no postcode:' as label;
SELECT
    name,
    ecclesiastical_parish,
    street_address,
    civil_parish
FROM unmatched_genealogy
WHERE ecclesiastical_parish IS NOT NULL
  AND ecclesiastical_parish != ''
  AND (postcode IS NULL OR postcode = '')
LIMIT 10;
