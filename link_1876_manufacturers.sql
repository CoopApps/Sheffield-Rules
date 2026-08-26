-- Link 1876 Manufacturers to sheffield_people records
-- Based on name and address matching from the Taylor thesis Appendix 6

-- First, let's see what we're working with
SELECT 'Total businesses:' as info, COUNT(*) as count FROM sheffield_businesses
UNION ALL
SELECT 'Total people:' as info, COUNT(*) as count FROM sheffield_people
UNION ALL
SELECT 'Businesses already linked:' as info, COUNT(*) as count FROM sheffield_businesses WHERE person_id IS NOT NULL
UNION ALL
SELECT 'Businesses not linked:' as info, COUNT(*) as count FROM sheffield_businesses WHERE person_id IS NULL;

-- Create a temporary table with our 1876 manufacturers
CREATE TEMP TABLE manufacturers_1876 (
    surname TEXT,
    forename TEXT,
    work_address TEXT,
    home_address TEXT
);

INSERT INTO manufacturers_1876 (surname, forename, work_address, home_address) VALUES
('Askam', 'J.', 'Broad Lane Works', 'Osborne Villa, Ranmoor Park'),
('Baker', 'J.', 'Wheeldon Works', '29, Redhill'),
('Barnes', 'V.', '103, Arundel St.', '144, Ecclesall Rd.'),
('Barston', 'J.', 'Harwood Works', 'Shorham St.'),
('Beardshaw', 'G.', '2, Marcus St.', '39, Brunswick Rd.'),
('Blyde', 'Wm.', '96, Carver St.', '118, Hanover St.'),
('Brooksbank', 'A.', 'Malinda St. Works', 'Moor Lodge, Clarkehouse Rd.'),
('Burnand', 'J.', 'Leicester St.', '40, Leafygreave Rd.'),
('Cantrell', 'E.', '68, Napier St.', 'Shelburn Pl.'),
('Copley', 'J.', 'Richmond Wks., Walkley', 'Carr Rd., Walkley'),
('Crossland', 'J.', 'Eclipse Wks., Edward St.', 'Norfolk Rd.'),
('Dawson', 'W.', 'Pool Wks., Burgess St.', '25, Evans St.'),
('Elliott', 'R.', '151, Arundel St.', '32, Chippinghouse Rd.'),
('Epworth', 'T.', 'Truss Wks., New George St.', '63, Highfield'),
('Greaves', 'F.', 'Radford Wks., Radford St.', '74, Upperthorpe'),
('Hardy', 'F.T.', 'Marsden''s Wheel, Love St.', 'Norton'),
('Haxton', 'R.', NULL, '164, Carr Rd., Walkley'),
('Holmes', 'T.', 'Scotland St.', 'Scotland St.'),
('Hunter', 'M.', 'Andrew St.', '135, Scotland St.'),
('Ibberson', 'G.', 'Central Wks., West St.', '135, Scotland St.'),
('Masterton', 'J.', 'Central Wks., West St.', '184, Witham Rd.'),
('Mosley', 'R.', 'Portland Wks., West St.', NULL),
('Nadin', 'A.', 'Court 8, Radford St.', NULL),
('Nowill', 'H.', 'Central Wks., West St.', 'Westbourne Rd.'),
('Nowill', 'J.', 'Central Wks., West St.', 'Westbourne Rd. East'),
('Paterson', 'A.', 'Forth Wks., Glossop Rd.', '195, Ecclesall Rd.'),
('Peace', 'W.K.', 'Mowbray St.', 'Dam House, 237, Glossop Rd.'),
('Pearce', 'H.K.', '20B, West St.', 'Wadsley Bridge'),
('Petty', 'Jos.', '58, Garden St.', 'Netherthorpe St.'),
('Pryor', 'M.', 'Scotland St.', '69, Havelock Sq.'),
('Renshaw', 'T.', '32, Birkendale', '76, Nether Edge Rd.'),
('Renton', 'G.', 'Carver St.', '41, Parkers Rd.'),
('Richardson', 'W.', 'Broomhall St.', 'Broomhall St.'),
('Roberts', 'L.', 'Rockingham St.', '116, Broad La.'),
('Rowland', 'L.', 'Solly St.', 'Solly St.'),
('Ryalls', 'J.', 'Solly St.', '73, William St.'),
('Scaife', 'F.', '73, Eyre St.', '73, Eyre St.'),
('Schofield', 'J.', '39, Broomspring La.', '101, Woodhead'),
('Shaw', 'J.', 'Orchard La.', 'Orchard La.'),
('Schemeld', 'J.', '47, Chester St.', '23, Broad La.'),
('Slinn', 'W.', '36, Thomas St.', 'Eagle House, Owlerton'),
('Taylor', 'H.H.', 'Times Wks., Paradise Sq.', 'Nicholson Rd., Heeley'),
('Taylor', 'W.', '188, Rockingham St.', '1, Blake St.'),
('Townsend', 'F.', 'Solly St.', 'Solly St.'),
('Twigg', 'F.', '25A, Owlerton Rd.', '25A, Owlerton Rd.'),
('Watson', 'G.', '25, Cornhill', '2, Shorham St.'),
('Webster', 'W.', 'Jessop St.', NULL),
('Whitham', 'J.', NULL, 'Cambridge St.'),
('Wragg', 'W.', NULL, '118, Cemetery Rd.');

-- Show which manufacturers are in sheffield_businesses
SELECT
    '=== MANUFACTURERS IN SHEFFIELD_BUSINESSES ===' as status,
    '' as business_id,
    '' as person_id,
    '' as surname,
    '' as forename,
    '' as occupation,
    '' as address
UNION ALL
SELECT
    'FOUND' as status,
    b.id,
    COALESCE(b.person_id, '(not linked)'),
    b.surname,
    b.forename,
    b.occupation,
    b.address
FROM manufacturers_1876 m
JOIN sheffield_businesses b ON b.surname = m.surname
WHERE b.forename IS NULL OR b.forename = '' OR b.forename LIKE m.forename || '%'
ORDER BY status DESC, surname, forename;

-- Now attempt to find matches in sheffield_people for businesses that aren't linked yet
SELECT
    '=== POTENTIAL PEOPLE MATCHES ===' as info,
    '' as business_surname,
    '' as business_forename,
    '' as business_address,
    '' as person_name,
    '' as person_address,
    '' as person_profession
UNION ALL
SELECT
    'MATCH' as info,
    b.surname,
    b.forename,
    b.address,
    p.name,
    p.street_address,
    p.profession
FROM manufacturers_1876 m
JOIN sheffield_businesses b ON b.surname = m.surname
LEFT JOIN sheffield_people p ON (
    p.surname = b.surname
    AND (b.forename IS NULL OR b.forename = '' OR p.first_name LIKE b.forename || '%')
)
WHERE b.person_id IS NULL
  AND p.id IS NOT NULL
ORDER BY info DESC, b.surname;

-- Clean up
DROP TABLE manufacturers_1876;
