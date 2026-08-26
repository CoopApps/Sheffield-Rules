SELECT 'Total matches after name+address matching:' as label,
       COUNT(*) as count
FROM unmatched_genealogy
WHERE matched_to_business_id IS NOT NULL
  AND matched_to_business_id != 0;
