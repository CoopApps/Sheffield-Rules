-- Check how many businesses are already matched
SELECT COUNT(*) as already_matched
FROM sheffield_businesses
WHERE matched_to_genealogy_id IS NOT NULL AND matched_to_genealogy_id != '';

-- Check how many are unmatched
SELECT COUNT(*) as unmatched
FROM sheffield_businesses
WHERE matched_to_genealogy_id IS NULL OR matched_to_genealogy_id = '';
