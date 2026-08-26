-- Check if businesses have any matches at all
SELECT 'Matched to ancestry_id:' as label, COUNT(*) as count
FROM sheffield_businesses
WHERE matched_to_ancestry_id IS NOT NULL AND matched_to_ancestry_id != 0;

SELECT 'Matched to genealogy_id:' as label, COUNT(*) as count
FROM sheffield_businesses
WHERE matched_to_genealogy_id IS NOT NULL AND matched_to_genealogy_id != 0;

-- Show some examples if any exist
SELECT 'Examples with ancestry matches:' as label;
SELECT name, business_type, matched_to_ancestry_id
FROM sheffield_businesses
WHERE matched_to_ancestry_id IS NOT NULL AND matched_to_ancestry_id != 0
LIMIT 5;

SELECT 'Examples with genealogy matches:' as label;
SELECT name, business_type, matched_to_genealogy_id
FROM sheffield_businesses
WHERE matched_to_genealogy_id IS NOT NULL AND matched_to_genealogy_id != 0
LIMIT 5;
