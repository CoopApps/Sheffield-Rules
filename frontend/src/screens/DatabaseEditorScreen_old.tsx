import React, { useState, useEffect } from 'react';
import { invoke } from '../utils/tauriInvoke';
import '../styles/database-editor.css';

interface Club {
  id: string;
  name: string;
  founded_year: number;
  ground_name: string;
  city: string;
  region: string;
}

interface GeneratedPlayer {
  id: string;
  name: string;
  club_id: string;
  position: string;
  birth_year: number;
  nationality: string;
  height_cm: number;
  overall_rating: number;
  pace: number;
  strength: number;
  stamina: number;
  passing: number;
  dribbling: number;
  finishing: number;
  tackling: number;
  handling: number;
  reflexes: number;
}

const DatabaseEditorScreen: React.FC = () => {
  const [clubs, setClubs] = useState<Club[]>([]);
  const [selectedClubId, setSelectedClubId] = useState<string>('');
  const [year, setYear] = useState<number>(1867); // Default to league founding year
  const [hasGoalkeeper, setHasGoalkeeper] = useState<boolean>(true); // Default true for fantasy league
  const [playerNamesText, setPlayerNamesText] = useState<string>('');
  const [generatedPlayers, setGeneratedPlayers] = useState<GeneratedPlayer[]>([]);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string>('');
  const [success, setSuccess] = useState<string>('');
  const [editingLocation, setEditingLocation] = useState<boolean>(false);
  const [editCity, setEditCity] = useState<string>('');
  const [editRegion, setEditRegion] = useState<string>('');

  useEffect(() => {
    loadClubs();
  }, []);

  const loadClubs = async () => {
    try {
      const clubsList = await invoke<Club[]>('db_get_all_clubs');
      setClubs(clubsList);
      if (clubsList.length > 0) {
        setSelectedClubId(clubsList[0].id);
      }
    } catch (err) {
      setError(`Failed to load clubs: ${err}`);
    }
  };

  const handleEditLocation = () => {
    if (selectedClub) {
      setEditCity(selectedClub.city || '');
      setEditRegion(selectedClub.region || '');
      setEditingLocation(true);
    }
  };

  const handleSaveLocation = async () => {
    setError('');
    setSuccess('');

    try {
      await invoke('db_update_club_location', {
        clubId: selectedClubId,
        city: editCity || null,
        region: editRegion || null,
      });

      setSuccess('Location updated successfully!');
      setEditingLocation(false);
      await loadClubs(); // Reload clubs to show updated data
    } catch (err) {
      setError(`Failed to update location: ${err}`);
    }
  };

  const handleCancelEditLocation = () => {
    setEditingLocation(false);
    setEditCity('');
    setEditRegion('');
  };

  const handleGeneratePlayers = async () => {
    setError('');
    setSuccess('');
    setLoading(true);

    try {
      // Parse player names and ages from text area (one per line)
      // Supports formats: "Name, Age", "Name - Age", "Name (Age)", "Name Age", or just "Name"
      const playerData = playerNamesText
        .split('\n')
        .map(line => line.trim())
        .filter(line => line.length > 0)
        .map(line => {
          // Try different formats
          let name = line;
          let age: number | null = null;

          // Format: "Name, Age"
          const commaMatch = line.match(/^(.+?),\s*(\d+)$/);
          if (commaMatch) {
            name = commaMatch[1].trim();
            age = parseInt(commaMatch[2]);
          } else {
            // Format: "Name - Age"
            const dashMatch = line.match(/^(.+?)\s*-\s*(\d+)$/);
            if (dashMatch) {
              name = dashMatch[1].trim();
              age = parseInt(dashMatch[2]);
            } else {
              // Format: "Name (Age)"
              const parenMatch = line.match(/^(.+?)\s*\((\d+)\)$/);
              if (parenMatch) {
                name = parenMatch[1].trim();
                age = parseInt(parenMatch[2]);
              } else {
                // Format: "Name Age" (space before number at end)
                const spaceMatch = line.match(/^(.+?)\s+(\d+)$/);
                if (spaceMatch) {
                  name = spaceMatch[1].trim();
                  age = parseInt(spaceMatch[2]);
                }
              }
            }
          }

          return { name, age };
        });

      if (playerData.length === 0) {
        setError('Please enter at least one player name');
        setLoading(false);
        return;
      }

      if (!selectedClubId) {
        setError('Please select a club');
        setLoading(false);
        return;
      }

      // Convert ages to birth years based on selected year
      const playerInputs = playerData.map(({ name, age }) => ({
        name,
        birthYear: age !== null ? year - age : null
      }));

      const players = await invoke<GeneratedPlayer[]>('db_bulk_add_players_with_ages', {
        clubId: selectedClubId,
        playerInputs: playerInputs,
        year: year,
        hasGoalkeeper: hasGoalkeeper,
      });

      setGeneratedPlayers(players);
      setSuccess(`Successfully generated ${players.length} players!`);
      setPlayerNamesText(''); // Clear input
    } catch (err) {
      setError(`Failed to generate players: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const selectedClub = clubs.find(c => c.id === selectedClubId);

  return (
    <div className="database-editor-container">
      <div className="database-editor-header">
        <h1>Sheffield Rules Database Editor</h1>
        <p>Add players to clubs for the Sheffield & Hallamshire Fantasy League</p>
      </div>

      <div className="database-editor-content">
        {/* Configuration Panel */}
        <div className="config-panel">
          <h2>Player Generation Settings</h2>

          <div className="form-group">
            <label htmlFor="club-select">Select Club:</label>
            <select
              id="club-select"
              value={selectedClubId}
              onChange={(e) => setSelectedClubId(e.target.value)}
              className="club-selector"
            >
              {clubs.map(club => (
                <option key={club.id} value={club.id}>
                  {club.name} ({club.founded_year})
                </option>
              ))}
            </select>
            {selectedClub && !editingLocation && (
              <div className="club-info">
                <p><strong>Ground:</strong> {selectedClub.ground_name}</p>
                <p><strong>Area:</strong> {selectedClub.city || 'Unknown'}</p>
                <p><strong>Postcode:</strong> {selectedClub.region || 'Unknown'}</p>
                <button onClick={handleEditLocation} className="edit-location-button">
                  Edit Location
                </button>
              </div>
            )}

            {editingLocation && (
              <div className="club-location-edit">
                <h3>Edit Club Location</h3>
                <div className="form-group">
                  <label htmlFor="edit-city">Area/City:</label>
                  <input
                    id="edit-city"
                    type="text"
                    value={editCity}
                    onChange={(e) => setEditCity(e.target.value)}
                    placeholder="e.g., Loxley, Heeley, Dore"
                    className="location-input"
                  />
                </div>
                <div className="form-group">
                  <label htmlFor="edit-region">Postcode District:</label>
                  <input
                    id="edit-region"
                    type="text"
                    value={editRegion}
                    onChange={(e) => setEditRegion(e.target.value)}
                    placeholder="e.g., S6, S2, S17"
                    className="location-input"
                  />
                </div>
                <div className="location-edit-buttons">
                  <button onClick={handleSaveLocation} className="save-button">
                    Save
                  </button>
                  <button onClick={handleCancelEditLocation} className="cancel-button">
                    Cancel
                  </button>
                </div>
              </div>
            )}
          </div>

          <div className="form-group">
            <label htmlFor="year-input">Year:</label>
            <input
              id="year-input"
              type="number"
              min="1857"
              max="1877"
              value={year}
              onChange={(e) => setYear(parseInt(e.target.value))}
              className="year-input"
            />
            <p className="help-text">
              Players will be aged 18-30 in this year. Stats reflect {year}-era football.
            </p>
          </div>

          <div className="form-group">
            <label className="checkbox-label">
              <input
                type="checkbox"
                checked={hasGoalkeeper}
                onChange={(e) => setHasGoalkeeper(e.target.checked)}
              />
              Include Goalkeeper Position
            </label>
            <p className="help-text">
              For Fantasy League mode, goalkeepers are recommended.
              Disable only for pre-1862 historical accuracy.
            </p>
          </div>

          <div className="form-group">
            <label htmlFor="player-names">Player Names & Ages (one per line):</label>
            <textarea
              id="player-names"
              value={playerNamesText}
              onChange={(e) => setPlayerNamesText(e.target.value)}
              className="player-names-input"
              rows={15}
              placeholder="John Smith, 25&#10;William Jones, 23&#10;Robert Brown&#10;..."
            />
            <p className="help-text">
              Format: "Name, Age" or "Name - Age" or "Name (Age)" or just "Name". Ages are optional - if not provided, they'll be randomized (18-30).
            </p>
          </div>

          <button
            onClick={handleGeneratePlayers}
            disabled={loading || !selectedClubId || playerNamesText.trim().length === 0}
            className="generate-button"
          >
            {loading ? 'Generating Players...' : 'Generate & Add Players'}
          </button>

          {error && <div className="error-message">{error}</div>}
          {success && <div className="success-message">{success}</div>}
        </div>

        {/* Generated Players Display */}
        {generatedPlayers.length > 0 && (
          <div className="generated-players-panel">
            <h2>Generated Players ({generatedPlayers.length})</h2>
            <div className="players-table-container">
              <table className="players-table">
                <thead>
                  <tr>
                    <th>Name</th>
                    <th>Pos</th>
                    <th>Age</th>
                    <th>Ht</th>
                    <th>OVR</th>
                    <th>PAC</th>
                    <th>STR</th>
                    <th>STA</th>
                    <th>PAS</th>
                    <th>DRI</th>
                    <th>FIN</th>
                    <th>TAC</th>
                    <th>HAN</th>
                    <th>REF</th>
                  </tr>
                </thead>
                <tbody>
                  {generatedPlayers.map(player => {
                    const age = year - player.birth_year;
                    const heightFt = Math.floor(player.height_cm / 30.48);
                    const heightIn = Math.round((player.height_cm / 2.54) % 12);

                    return (
                      <tr key={player.id} className={`position-${player.position.toLowerCase()}`}>
                        <td className="player-name">{player.name}</td>
                        <td className="position-badge">{player.position}</td>
                        <td>{age}</td>
                        <td>{heightFt}'{heightIn}"</td>
                        <td className="overall-rating">{player.overall_rating}</td>
                        <td>{player.pace}</td>
                        <td>{player.strength}</td>
                        <td>{player.stamina}</td>
                        <td>{player.passing}</td>
                        <td>{player.dribbling}</td>
                        <td>{player.finishing}</td>
                        <td>{player.tackling}</td>
                        <td className={player.position === 'GK' ? 'highlight' : ''}>{player.handling}</td>
                        <td className={player.position === 'GK' ? 'highlight' : ''}>{player.reflexes}</td>
                      </tr>
                    );
                  })}
                </tbody>
              </table>
            </div>
            <div className="players-summary">
              <h3>Position Distribution:</h3>
              <ul>
                {Object.entries(
                  generatedPlayers.reduce((acc, p) => {
                    acc[p.position] = (acc[p.position] || 0) + 1;
                    return acc;
                  }, {} as Record<string, number>)
                ).map(([pos, count]) => (
                  <li key={pos}>
                    <span className="position-badge">{pos}</span>: {count} player{count !== 1 ? 's' : ''}
                  </li>
                ))}
              </ul>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default DatabaseEditorScreen;
