-- Find employers with 10+ employees in sheffield_people
-- Looking for patterns like "Employing X Men", "Employing X Hand", etc.

SELECT 
    name,
    profession,
    street_address,
    postcode
FROM sheffield_people
WHERE profession LIKE '%Employing%'
  AND (
    -- Match patterns like "Employing 10 Men", "Employing 12 Mans", etc.
    profession LIKE '%Employing 1_ %'
    OR profession LIKE '%Employing 2_ %'
    OR profession LIKE '%Employing 3_ %'
    OR profession LIKE '%Employing 4_ %'
    OR profession LIKE '%Employing 5_ %'
    OR profession LIKE '%Employing 6_ %'
    OR profession LIKE '%Employing 7_ %'
    OR profession LIKE '%Employing 8_ %'
    OR profession LIKE '%Employing 9_ %'
    OR profession LIKE '%Employing 1__ %'
    OR profession LIKE '%Employing 2__ %'
    OR profession LIKE '%Employing 3__ %'
    OR profession LIKE '%Employing 4__ %'
    OR profession LIKE '%Employing 5__ %'
    OR profession LIKE '%Employing 6__ %'
    OR profession LIKE '%Employing 7__ %'
    OR profession LIKE '%Employing 8__ %'
    OR profession LIKE '%Employing 9__ %'
  )
ORDER BY name;
