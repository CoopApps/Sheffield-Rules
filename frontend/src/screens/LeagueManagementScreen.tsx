import React, { useState, useEffect } from 'react';
import { invoke } from '../utils/tauriInvoke';
import { LeaguePyramid } from '../components/LeaguePyramid';
import { LeagueConfigPanel } from '../components/LeagueConfigPanel';
import { CreateLeagueModal } from '../components/CreateLeagueModal';
import '../styles/league-management.css';

interface Division {
  id: string;
  name: string;
  level: number;
  region: string | null;
}

interface Club {
  id: string;
  name: string;
  founded_year: number;
}

interface LeagueMetadata {
  id: string;
  name: string;
  description: string;
  season_year: number;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

interface ClubDivisionInfo {
  club_id: string;
  club_name: string;
  division_id: string;
  division_name: string;
  position: number;
  founded_year: number;
  postcode_area?: string | null;
}

const LeagueManagementScreen: React.FC = () => {
  const [leagues, setLeagues] = useState<LeagueMetadata[]>([]);
  const [activeLeague, setActiveLeague] = useState<LeagueMetadata | null>(null);
  const [showCreateModal, setShowCreateModal] = useState<boolean>(false);
  const [divisions, setDivisions] = useState<Division[]>([]);
  const [clubsByDivision, setClubsByDivision] = useState<Map<string, ClubDivisionInfo[]>>(new Map());
  const [unassignedClubs, setUnassignedClubs] = useState<Club[]>([]);
  const [draggedClub, setDraggedClub] = useState<ClubDivisionInfo | Club | null>(null);
  const [selectedClub, setSelectedClub] = useState<ClubDivisionInfo | Club | null>(null);
  const [mousePosition, setMousePosition] = useState<{ x: number; y: number } | null>(null);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string>('');
  const [success, setSuccess] = useState<string>('');
  const [hasChanges, setHasChanges] = useState<boolean>(false);
  const [importing, setImporting] = useState<boolean>(false);
  const [activeTab, setActiveTab] = useState<'management' | 'pyramid' | 'config'>('management');

  useEffect(() => {
    loadLeagues();
  }, []);

  useEffect(() => {
    if (activeLeague) {
      loadData();
    }
  }, [activeLeague]);

  const loadLeagues = async () => {
    try {
      const allLeagues = await invoke<LeagueMetadata[]>('db_get_all_leagues');
      setLeagues(allLeagues);

      const active = await invoke<LeagueMetadata | null>('db_get_active_league');
      if (active) {
        setActiveLeague(active);
      } else if (allLeagues.length > 0) {
        // If no active league but leagues exist, set the first one as active
        setActiveLeague(allLeagues[0]);
      }
    } catch (err) {
      console.error('Failed to load leagues:', err);
    }
  };

  const loadData = async () => {
    setLoading(true);
    setError('');

    try {
      // Load all divisions
      const divisionsData = await invoke<Division[]>('db_get_all_divisions');
      setDivisions(divisionsData);

      // Load all clubs
      const allClubs = await invoke<Club[]>('db_get_all_clubs');

      // Load club assignments
      const clubsMap = new Map<string, ClubDivisionInfo[]>();
      const assignedClubIds = new Set<string>();

      for (const division of divisionsData) {
        try {
          const clubs = await invoke<ClubDivisionInfo[]>('get_division_clubs', {
            divisionId: division.id
          });
          console.log(`Division ${division.id}: loaded ${clubs.length} clubs`);
          clubsMap.set(division.id, clubs);
          clubs.forEach(club => assignedClubIds.add(club.club_id));
        } catch (err) {
          console.error(`Error loading clubs for division ${division.id}:`, err);
          // Division might be empty
          clubsMap.set(division.id, []);
        }
      }

      console.log(`Total assigned clubs: ${assignedClubIds.size}`);
      console.log(`Total clubs in database: ${allClubs.length}`);

      setClubsByDivision(clubsMap);

      // Load unassigned clubs from backend
      const unassigned = await invoke<Club[]>('db_get_unassigned_clubs');
      setUnassignedClubs(unassigned);

      setHasChanges(false);
    } catch (err) {
      setError(`Failed to load data: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const handleClubClick = (club: ClubDivisionInfo | Club) => {
    if (selectedClub &&
        ('club_id' in selectedClub ? selectedClub.club_id : selectedClub.id) ===
        ('club_id' in club ? club.club_id : club.id)) {
      // Clicking the same club deselects it
      setSelectedClub(null);
    } else {
      setSelectedClub(club);
    }
  };

  const handleDivisionClick = (targetDivisionId: string) => {
    if (!selectedClub) return;

    const clubId = 'club_id' in selectedClub ? selectedClub.club_id : selectedClub.id;
    const clubName = 'club_name' in selectedClub ? selectedClub.club_name : selectedClub.name;
    const foundedYear = 'founded_year' in selectedClub ? selectedClub.founded_year : 0;

    // Don't move if already in this division
    if ('division_id' in selectedClub && selectedClub.division_id === targetDivisionId) {
      setSelectedClub(null);
      return;
    }

    // Create a new Map with updated values
    const newMap = new Map(clubsByDivision);

    // Remove from old location
    if ('division_id' in selectedClub) {
      // Remove from old division
      const oldDivisionClubs = newMap.get(selectedClub.division_id) || [];
      const updatedOldClubs = oldDivisionClubs.filter(c => c.club_id !== clubId);
      newMap.set(selectedClub.division_id, updatedOldClubs);
    } else {
      // Remove from unassigned
      setUnassignedClubs(prev => prev.filter(c => c.id !== clubId));
    }

    // Add to new division
    const targetClubs = newMap.get(targetDivisionId) || [];
    const newClub: ClubDivisionInfo = {
      club_id: clubId,
      club_name: clubName,
      division_id: targetDivisionId,
      division_name: divisions.find(d => d.id === targetDivisionId)?.name || '',
      position: targetClubs.length + 1,
      founded_year: foundedYear
    };

    newMap.set(targetDivisionId, [...targetClubs, newClub]);
    setClubsByDivision(newMap);
    setSelectedClub(null);
    setHasChanges(true);
  };

  const handleUnassignedClick = () => {
    if (!selectedClub || !('division_id' in selectedClub)) return;

    const clubId = selectedClub.club_id;
    const clubName = selectedClub.club_name;
    const foundedYear = selectedClub.founded_year;

    // Create a new Map with updated values
    const newMap = new Map(clubsByDivision);

    // Remove from division
    const oldDivisionClubs = newMap.get(selectedClub.division_id) || [];
    const updatedClubs = oldDivisionClubs.filter(c => c.club_id !== clubId);
    newMap.set(selectedClub.division_id, updatedClubs);
    setClubsByDivision(newMap);

    // Add to unassigned
    setUnassignedClubs(prev => [...prev, { id: clubId, name: clubName, founded_year: foundedYear }]);
    setSelectedClub(null);
    setHasChanges(true);
  };

  const handleSaveChanges = async () => {
    setLoading(true);
    setError('');
    setSuccess('');

    try {
      // Collect all updates
      const updates: { clubId: string; divisionId: string; position: number }[] = [];

      for (const [divisionId, clubs] of clubsByDivision.entries()) {
        clubs.forEach((club, index) => {
          updates.push({
            clubId: club.club_id,
            divisionId: divisionId,
            position: index + 1
          });
        });
      }

      // Save each update
      for (const update of updates) {
        await invoke('db_update_club_division', {
          clubId: update.clubId,
          divisionId: update.divisionId,
          position: update.position
        });
      }

      setSuccess(`Successfully updated ${updates.length} clubs!`);
      setHasChanges(false);

      // Reload to confirm changes
      setTimeout(() => {
        loadData();
      }, 1000);
    } catch (err) {
      setError(`Failed to save changes: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const handleResetChanges = () => {
    loadData();
  };

  const handlePopulateLeague = async () => {
    setImporting(true);
    setError('');
    setSuccess('');

    try {
      await invoke('db_import_league_structure');
      setSuccess('Successfully populated league structure! Clubs are now assigned to divisions.');
      setTimeout(() => {
        loadData();
      }, 1000);
    } catch (err) {
      setError(`Failed to populate league: ${err}`);
    } finally {
      setImporting(false);
    }
  };

  const handleLeagueCreated = async (league: LeagueMetadata) => {
    setSuccess(`Successfully created league: ${league.name}`);
    setShowCreateModal(false);
    await loadLeagues();
    setActiveLeague(league);
  };

  const handleLeagueChange = async (leagueId: string) => {
    const selected = leagues.find(l => l.id === leagueId);
    if (selected) {
      setActiveLeague(selected);
      try {
        await invoke('db_set_active_league', { leagueId });
      } catch (err) {
        console.error('Failed to set active league:', err);
      }
    }
  };

  if (loading && divisions.length === 0) {
    return <div className="league-management-container loading">Loading...</div>;
  }

  const handleMouseMove = (e: React.MouseEvent) => {
    if (selectedClub) {
      setMousePosition({ x: e.clientX, y: e.clientY });
    }
  };

  const handlePyramidDivisionClick = (divisionId: string) => {
    setActiveTab('management');
    // Scroll to the division in the management view
    setTimeout(() => {
      const element = document.querySelector(`[data-division-id="${divisionId}"]`);
      if (element) {
        element.scrollIntoView({ behavior: 'smooth', block: 'center' });
      }
    }, 100);
  };

  // Convert clubsByDivision Map to clubCounts Map (just the counts)
  const clubCountsMap = new Map<string, number>(
    Array.from(clubsByDivision.entries()).map(([id, clubs]) => [id, clubs.length])
  );

  return (
    <div className="league-management-container" onMouseMove={handleMouseMove}>
      <div className="league-management-header">
        <div className="header-top">
          <div>
            <h1>League Management</h1>
            <p>
              {activeTab === 'management'
                ? 'Drag and drop clubs between divisions to organize the league structure'
                : activeTab === 'pyramid'
                ? 'Visual representation of the league pyramid structure'
                : 'Configure season settings, points system, and match scheduling'}
            </p>
            <div className="league-selector-row">
              {leagues.length > 0 && (
                <div className="league-selector">
                  <label htmlFor="league-select">Active League:</label>
                  <select
                    id="league-select"
                    value={activeLeague?.id || ''}
                    onChange={(e) => handleLeagueChange(e.target.value)}
                    className="league-select"
                  >
                    {leagues.map(league => (
                      <option key={league.id} value={league.id}>
                        {league.name} ({league.season_year})
                      </option>
                    ))}
                  </select>
                </div>
              )}
              <button
                onClick={() => setShowCreateModal(true)}
                className="create-league-button"
              >
                + Create New League
              </button>
            </div>
          </div>
          <button
            onClick={handlePopulateLeague}
            className="import-button"
            disabled={importing || loading || !activeLeague}
          >
            {importing ? 'Populating...' : 'Initialize League Structure'}
          </button>
        </div>

        <div className="tab-navigation">
          <button
            className={`tab-button ${activeTab === 'management' ? 'active' : ''}`}
            onClick={() => setActiveTab('management')}
          >
            Division Management
          </button>
          <button
            className={`tab-button ${activeTab === 'pyramid' ? 'active' : ''}`}
            onClick={() => setActiveTab('pyramid')}
          >
            Pyramid View
          </button>
          <button
            className={`tab-button ${activeTab === 'config' ? 'active' : ''}`}
            onClick={() => setActiveTab('config')}
          >
            Configuration
          </button>
        </div>
        {hasChanges && (
          <div className="changes-banner">
            You have unsaved changes
            <div className="changes-buttons">
              <button onClick={handleSaveChanges} className="save-all-button" disabled={loading}>
                Save All Changes
              </button>
              <button onClick={handleResetChanges} className="reset-button" disabled={loading}>
                Reset
              </button>
            </div>
          </div>
        )}
      </div>

      {error && <div className="error-message">{error}</div>}
      {success && <div className="success-message">{success}</div>}

      {activeTab === 'pyramid' ? (
        <LeaguePyramid
          divisions={divisions}
          clubCounts={clubCountsMap}
          onDivisionClick={handlePyramidDivisionClick}
        />
      ) : activeTab === 'config' ? (
        <LeagueConfigPanel />
      ) : (
        <div className="divisions-grid">
        {divisions.map(division => {
          const clubs = clubsByDivision.get(division.id) || [];
          return (
            <div
              key={division.id}
              className="division-box"
              data-division-id={division.id}
            >
              <div className="division-header">
                <h3>{division.name}</h3>
                {division.region && <span className="division-region">({division.region})</span>}
                <span className="club-count">{clubs.length} clubs</span>
              </div>
              <div
                className="clubs-list"
                onClick={() => {
                  if (selectedClub) {
                    handleDivisionClick(division.id);
                  }
                }}
              >
                {clubs.length === 0 ? (
                  <div className="empty-division">
                    {selectedClub ? 'Click here to move club' : 'No clubs in division'}
                  </div>
                ) : (
                  clubs.map((club, index) => {
                    const clubId = 'club_id' in club ? (club as any).club_id : (club as any).id;
                    const isSelected = selectedClub && ('club_id' in selectedClub ? selectedClub.club_id : selectedClub.id) === clubId;
                    return (
                      <div
                        key={club.club_id}
                        className={`club-item ${isSelected ? 'selected' : ''}`}
                        onClick={(e) => {
                          e.stopPropagation();
                          handleClubClick(club);
                        }}
                      >
                        <span className="club-position">{index + 1}.</span>
                        <span className="club-name">{club.club_name}</span>
                        {club.postcode_area && <span className="club-postcode">{club.postcode_area}</span>}
                        <span className="club-year">({club.founded_year})</span>
                      </div>
                    );
                  })
                )}
              </div>
            </div>
          );
        })}

        {/* Unassigned clubs box */}
        <div
          className="division-box unassigned-box"
        >
          <div className="division-header">
            <h3>Unassigned Clubs</h3>
            <span className="club-count">{unassignedClubs.length} clubs</span>
          </div>
          <div
            className="clubs-list"
            onClick={() => {
              if (selectedClub && 'division_id' in selectedClub) {
                handleUnassignedClick();
              }
            }}
          >
            {unassignedClubs.length === 0 ? (
              <div className="empty-division">
                {selectedClub && 'division_id' in selectedClub ? 'Click here to unassign club' : 'All clubs assigned'}
              </div>
            ) : (
              unassignedClubs.map(club => {
                const isSelected = selectedClub && ('club_id' in selectedClub ? selectedClub.club_id : selectedClub.id) === club.id;
                return (
                  <div
                    key={club.id}
                    className={`club-item ${isSelected ? 'selected' : ''}`}
                    onClick={(e) => {
                      e.stopPropagation();
                      handleClubClick(club);
                    }}
                  >
                    <span className="club-name">{club.name}</span>
                    <span className="club-year">({club.founded_year})</span>
                  </div>
                );
              })
            )}
          </div>
        </div>
      </div>
      )}

      {/* Floating club preview when dragging */}
      {selectedClub && mousePosition && (
        <div
          className="floating-club-preview"
          style={{
            left: mousePosition.x + 10,
            top: mousePosition.y + 10,
          }}
        >
          {'club_name' in selectedClub ? selectedClub.club_name : selectedClub.name}
        </div>
      )}

      {/* Create League Modal */}
      {showCreateModal && (
        <CreateLeagueModal
          onClose={() => setShowCreateModal(false)}
          onLeagueCreated={handleLeagueCreated}
        />
      )}
    </div>
  );
};

export default LeagueManagementScreen;
