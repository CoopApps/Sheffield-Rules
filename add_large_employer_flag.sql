-- Add column to identify large employers (10+ employees)

-- Add the column
ALTER TABLE sheffield_employers ADD COLUMN is_large_employer INTEGER DEFAULT 0;

-- Mark large employers (10+ employees)
UPDATE sheffield_employers
SET is_large_employer = 1
WHERE profession LIKE '%Employing 1_ %'
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
   OR profession LIKE '%Employing 9__ %';

-- Create index for faster filtering
CREATE INDEX IF NOT EXISTS idx_large_employer ON sheffield_employers(is_large_employer);

-- Report results
SELECT 'Large employers (is_large_employer = 1):' as label, COUNT(*) as count 
FROM sheffield_employers 
WHERE is_large_employer = 1;

SELECT 'Small employers (is_large_employer = 0):' as label, COUNT(*) as count 
FROM sheffield_employers 
WHERE is_large_employer = 0;

-- Show examples of large employers
SELECT 'Top 10 largest employers:' as label;
SELECT name, profession, street_address
FROM sheffield_employers
WHERE is_large_employer = 1
ORDER BY name
LIMIT 10;
