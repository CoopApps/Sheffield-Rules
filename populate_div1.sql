-- Populate Division 1 with clubs
-- This matches clubs by name and assigns them to div-1

-- Sheffield FC
INSERT OR REPLACE INTO sheffield_league_clubs (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
SELECT 'div1-1', 'div-1', id, 1, 0, NULL FROM sheffield_clubs WHERE name LIKE '%Sheffield%' AND name NOT LIKE '%Hallam%' LIMIT 1;

-- Hallam FC
INSERT OR REPLACE INTO sheffield_league_clubs (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
SELECT 'div1-2', 'div-1', id, 2, 0, NULL FROM sheffield_clubs WHERE name LIKE '%Hallam%' LIMIT 1;

-- Norfolk FC
INSERT OR REPLACE INTO sheffield_league_clubs (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
SELECT 'div1-3', 'div-1', id, 3, 0, NULL FROM sheffield_clubs WHERE name LIKE '%Norfolk%' LIMIT 1;

-- Cemetery Road Church FC
INSERT OR REPLACE INTO sheffield_league_clubs (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
SELECT 'div1-4', 'div-1', id, 4, 0, NULL FROM sheffield_clubs WHERE name LIKE '%Cemetery%' LIMIT 1;

-- York FC
INSERT OR REPLACE INTO sheffield_league_clubs (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
SELECT 'div1-5', 'div-1', id, 5, 0, NULL FROM sheffield_clubs WHERE name LIKE '%York%' LIMIT 1;

-- Norton FC
INSERT OR REPLACE INTO sheffield_league_clubs (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
SELECT 'div1-6', 'div-1', id, 6, 0, NULL FROM sheffield_clubs WHERE name LIKE '%Norton%' LIMIT 1;

-- Pitsmoor FC
INSERT OR REPLACE INTO sheffield_league_clubs (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
SELECT 'div1-7', 'div-1', id, 7, 0, NULL FROM sheffield_clubs WHERE name LIKE '%Pitsmoor%' LIMIT 1;

-- Fir Vale FC
INSERT OR REPLACE INTO sheffield_league_clubs (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
SELECT 'div1-8', 'div-1', id, 8, 0, NULL FROM sheffield_clubs WHERE name LIKE '%Fir Vale%' LIMIT 1;

-- Newhall FC
INSERT OR REPLACE INTO sheffield_league_clubs (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
SELECT 'div1-9', 'div-1', id, 9, 0, NULL FROM sheffield_clubs WHERE name LIKE '%Newhall%' LIMIT 1;

-- Attercliffe FC
INSERT OR REPLACE INTO sheffield_league_clubs (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
SELECT 'div1-10', 'div-1', id, 10, 0, NULL FROM sheffield_clubs WHERE name LIKE '%Attercliffe%' LIMIT 1;

-- Sheaf House FC
INSERT OR REPLACE INTO sheffield_league_clubs (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
SELECT 'div1-11', 'div-1', id, 11, 0, NULL FROM sheffield_clubs WHERE name LIKE '%Sheaf House%' LIMIT 1;

-- Exchange FC
INSERT OR REPLACE INTO sheffield_league_clubs (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
SELECT 'div1-12', 'div-1', id, 12, 0, NULL FROM sheffield_clubs WHERE name LIKE '%Exchange%' LIMIT 1;

-- Mechanics FC
INSERT OR REPLACE INTO sheffield_league_clubs (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
SELECT 'div1-13', 'div-1', id, 13, 0, NULL FROM sheffield_clubs WHERE name LIKE '%Mechanics%' LIMIT 1;

-- Broomhall FC
INSERT OR REPLACE INTO sheffield_league_clubs (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
SELECT 'div1-14', 'div-1', id, 14, 0, NULL FROM sheffield_clubs WHERE name LIKE '%Broomhall%' LIMIT 1;

-- Brightside FC
INSERT OR REPLACE INTO sheffield_league_clubs (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
SELECT 'div1-15', 'div-1', id, 15, 0, NULL FROM sheffield_clubs WHERE name LIKE '%Brightside%' LIMIT 1;

-- Heeley FC
INSERT OR REPLACE INTO sheffield_league_clubs (id, division_id, club_id, position_in_division, is_reserve_team, reserve_of_club_id)
SELECT 'div1-16', 'div-1', id, 16, 0, NULL FROM sheffield_clubs WHERE name LIKE '%Heeley%' LIMIT 1;

-- Check results
SELECT COUNT(*) as 'Clubs assigned to Division 1' FROM sheffield_league_clubs WHERE division_id = 'div-1';
