import React, { useEffect, useState } from 'react'
import '../styles/player-generation.css'
import { invoke } from '../utils/tauriInvoke'

interface Club {
  id: string
  name: string
  founded_year: number
  ground_name: string
  origin: string
  city?: string
  region?: string
}

interface NewPlayer {
  id: string
  name: string
  position: string
  birth_year: number
  nationality: string
}

interface PlayerGenerationScreenProps {
  year: number
  onBack: () => void
}

export function PlayerGenerationScreen({ year, onBack }: PlayerGenerationScreenProps) {
  const [clubs, setClubs] = useState<Club[]>([])
  const [filteredClubs, setFilteredClubs] = useState<Club[]>([])
  const [selectedClubs, setSelectedClubs] = useState<Club[]>([])
  const [postcodeFilter, setPostcodeFilter] = useState<string>('')
  const [players, setPlayers] = useState<NewPlayer[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [saving, setSaving] = useState(false)

  useEffect(() => {
    loadClubs()
  }, [year])

  useEffect(() => {
    applyFilters()
  }, [postcodeFilter, clubs])

  const loadClubs = async () => {
    try {
      setLoading(true)
      setError(null)
      const availableClubs: Club[] = await invoke('get_sheffield_clubs', { filterYear: year })
      setClubs(availableClubs)
      setFilteredClubs(availableClubs)
    } catch (err) {
      setError(`Failed to load clubs: ${err}`)
      console.error(err)
    } finally {
      setLoading(false)
    }
  }

  const extractPostcode = (club: Club): string => {
    // Extract postcode from region field (e.g., "Unknown (S4)" -> "S4")
    const regionMatch = club.region?.match(/\(([A-Z]\d+)\)/)
    if (regionMatch) return regionMatch[1]

    // Try origin field
    const originMatch = club.origin?.match(/\(([A-Z]\d+)\)/)
    if (originMatch) return originMatch[1]

    return ''
  }

  const applyFilters = () => {
    let filtered = clubs

    if (postcodeFilter.trim()) {
      const filterUpper = postcodeFilter.trim().toUpperCase()
      filtered = filtered.filter(club => {
        const postcode = extractPostcode(club)
        return postcode.startsWith(filterUpper)
      })
    }

    setFilteredClubs(filtered)
  }

  const toggleClubSelection = (club: Club) => {
    setSelectedClubs(prev => {
      const isSelected = prev.find(c => c.id === club.id)
      if (isSelected) {
        return prev.filter(c => c.id !== club.id)
      } else {
        return [...prev, club]
      }
    })
  }

  const addPlayerRow = () => {
    const newPlayer: NewPlayer = {
      id: `temp_${Date.now()}`,
      name: '',
      position: 'FWD',
      birth_year: year - 20,
      nationality: 'English'
    }
    setPlayers([...players, newPlayer])
  }

  const updatePlayer = (id: string, field: keyof NewPlayer, value: string | number) => {
    setPlayers(prev => prev.map(player =>
      player.id === id ? { ...player, [field]: value } : player
    ))
  }

  const removePlayer = (id: string) => {
    setPlayers(prev => prev.filter(p => p.id !== id))
  }

  const savePlayersToDatabase = async () => {
    if (selectedClubs.length === 0) {
      alert('Please select at least one club')
      return
    }

    if (players.length === 0) {
      alert('Please add at least one player')
      return
    }

    const invalidPlayers = players.filter(p => !p.name.trim())
    if (invalidPlayers.length > 0) {
      alert('Please fill in all player names')
      return
    }

    try {
      setSaving(true)

      // Save players to each selected club
      for (const club of selectedClubs) {
        for (const player of players) {
          await invoke('create_player', {
            clubId: club.id,
            name: player.name.trim(),
            position: player.position,
            birthYear: player.birth_year,
            nationality: player.nationality
          })
        }
      }

      alert(`Successfully created ${players.length} player(s) for ${selectedClubs.length} club(s)`)
      setPlayers([])
    } catch (err) {
      console.error('Failed to save players:', err)
      alert(`Failed to save players: ${err}`)
    } finally {
      setSaving(false)
    }
  }

  const getAvailablePostcodes = (): string[] => {
    const postcodes = new Set<string>()
    clubs.forEach(club => {
      const postcode = extractPostcode(club)
      if (postcode) postcodes.add(postcode)
    })
    return Array.from(postcodes).sort()
  }

  if (loading) {
    return (
      <div className="player-generation-screen">
        <div className="loading">Loading clubs...</div>
      </div>
    )
  }

  if (error) {
    return (
      <div className="player-generation-screen">
        <div className="error">{error}</div>
        <button onClick={onBack} className="back-button">Back</button>
      </div>
    )
  }

  return (
    <div className="player-generation-screen">
      <div className="generation-header">
        <h2>Player Generation</h2>
        <button onClick={onBack} className="back-button">Back to Menu</button>
      </div>

      <div className="generation-content">
        {/* Left Panel: Club Selection */}
        <div className="club-selection-panel">
          <div className="panel-header">
            <h3>Select Clubs</h3>
            <div className="postcode-filter">
              <label>Filter by Postcode:</label>
              <input
                type="text"
                value={postcodeFilter}
                onChange={(e) => setPostcodeFilter(e.target.value)}
                placeholder="e.g., S4"
                className="postcode-input"
              />
              <div className="postcode-options">
                {getAvailablePostcodes().map(postcode => (
                  <button
                    key={postcode}
                    className={`postcode-btn ${postcodeFilter === postcode ? 'active' : ''}`}
                    onClick={() => setPostcodeFilter(postcode)}
                  >
                    {postcode}
                  </button>
                ))}
              </div>
            </div>
          </div>

          <div className="club-list-scroll">
            {filteredClubs.map(club => {
              const isSelected = selectedClubs.find(c => c.id === club.id) !== undefined
              const postcode = extractPostcode(club)

              return (
                <div
                  key={club.id}
                  className={`club-item ${isSelected ? 'selected' : ''}`}
                  onClick={() => toggleClubSelection(club)}
                >
                  <div className="club-item-header">
                    <input
                      type="checkbox"
                      checked={isSelected}
                      onChange={() => {}}
                      onClick={(e) => e.stopPropagation()}
                    />
                    <span className="club-name">{club.name}</span>
                  </div>
                  <div className="club-item-details">
                    <span className="club-ground">{club.ground_name}</span>
                    {postcode && <span className="club-postcode">({postcode})</span>}
                  </div>
                </div>
              )
            })}
          </div>

          <div className="selection-summary">
            {selectedClubs.length} club(s) selected
          </div>
        </div>

        {/* Right Panel: Player Entry */}
        <div className="player-entry-panel">
          <div className="panel-header">
            <h3>Add Players</h3>
            <button onClick={addPlayerRow} className="add-player-btn">
              + Add Player
            </button>
          </div>

          {players.length === 0 ? (
            <div className="empty-state">
              No players added yet. Click "Add Player" to begin.
            </div>
          ) : (
            <div className="player-table-container">
              <table className="player-table">
                <thead>
                  <tr>
                    <th>Name</th>
                    <th>Position</th>
                    <th>Birth Year</th>
                    <th>Nationality</th>
                    <th></th>
                  </tr>
                </thead>
                <tbody>
                  {players.map(player => (
                    <tr key={player.id}>
                      <td>
                        <input
                          type="text"
                          value={player.name}
                          onChange={(e) => updatePlayer(player.id, 'name', e.target.value)}
                          placeholder="Player Name"
                          className="player-input"
                        />
                      </td>
                      <td>
                        <select
                          value={player.position}
                          onChange={(e) => updatePlayer(player.id, 'position', e.target.value)}
                          className="player-select"
                        >
                          <option value="GK">GK</option>
                          <option value="CB">CB</option>
                          <option value="FB">FB</option>
                          <option value="MID">MID</option>
                          <option value="FWD">FWD</option>
                          <option value="WG">WG</option>
                        </select>
                      </td>
                      <td>
                        <input
                          type="number"
                          value={player.birth_year}
                          onChange={(e) => updatePlayer(player.id, 'birth_year', parseInt(e.target.value))}
                          className="player-input year-input"
                        />
                      </td>
                      <td>
                        <input
                          type="text"
                          value={player.nationality}
                          onChange={(e) => updatePlayer(player.id, 'nationality', e.target.value)}
                          className="player-input"
                        />
                      </td>
                      <td>
                        <button
                          onClick={() => removePlayer(player.id)}
                          className="remove-btn"
                          title="Remove player"
                        >
                          ×
                        </button>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}

          <div className="action-buttons">
            <button
              onClick={savePlayersToDatabase}
              disabled={saving || players.length === 0 || selectedClubs.length === 0}
              className="save-btn"
            >
              {saving ? 'Saving...' : `Save ${players.length} Player(s) to ${selectedClubs.length} Club(s)`}
            </button>
            {players.length > 0 && (
              <button
                onClick={() => setPlayers([])}
                className="clear-btn"
              >
                Clear All Players
              </button>
            )}
          </div>
        </div>
      </div>
    </div>
  )
}

export default PlayerGenerationScreen
