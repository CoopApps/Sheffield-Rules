-- Step 1: Move institutional residents to their respective tables
-- Move workhouse residents
INSERT INTO sheffield_workhouse
SELECT * FROM unmatched_genealogy
WHERE street_address LIKE '%workhouse%'
   OR street_address LIKE '%Workhouse%'
   OR civil_parish LIKE '%workhouse%'
   OR civil_parish LIKE '%Workhouse%';

-- Move prison residents
INSERT INTO sheffield_prison
SELECT * FROM unmatched_genealogy
WHERE street_address LIKE '%prison%'
   OR street_address LIKE '%Prison%'
   OR street_address LIKE '%gaol%'
   OR street_address LIKE '%Gaol%'
   OR street_address LIKE '%jail%'
   OR street_address LIKE '%Jail%';

-- Move asylum residents
INSERT INTO sheffield_asylum
SELECT * FROM unmatched_genealogy
WHERE street_address LIKE '%asylum%'
   OR street_address LIKE '%Asylum%'
   OR civil_parish LIKE '%asylum%'
   OR civil_parish LIKE '%Asylum%';

-- Delete from unmatched_genealogy after moving
DELETE FROM unmatched_genealogy
WHERE street_address LIKE '%workhouse%'
   OR street_address LIKE '%Workhouse%'
   OR civil_parish LIKE '%workhouse%'
   OR civil_parish LIKE '%Workhouse%'
   OR street_address LIKE '%prison%'
   OR street_address LIKE '%Prison%'
   OR street_address LIKE '%gaol%'
   OR street_address LIKE '%Gaol%'
   OR street_address LIKE '%jail%'
   OR street_address LIKE '%Jail%'
   OR street_address LIKE '%asylum%'
   OR street_address LIKE '%Asylum%'
   OR civil_parish LIKE '%asylum%'
   OR civil_parish LIKE '%Asylum%';

-- Step 2: Rename unmatched_genealogy to sheffield_people
ALTER TABLE unmatched_genealogy RENAME TO sheffield_people;

-- Report results
SELECT 'Workhouse residents moved:' as label, COUNT(*) as count FROM sheffield_workhouse;
SELECT 'Prison residents moved:' as label, COUNT(*) as count FROM sheffield_prison;
SELECT 'Asylum residents moved:' as label, COUNT(*) as count FROM sheffield_asylum;
SELECT 'Remaining in sheffield_people:' as label, COUNT(*) as count FROM sheffield_people;
