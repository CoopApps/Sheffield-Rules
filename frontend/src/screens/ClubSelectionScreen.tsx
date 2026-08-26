import React, { useEffect, useState } from 'react'
import '../styles/club-selection.css'
import { invoke } from '../utils/tauriInvoke'

interface Club {
  id: string
  name: string
  founded_year: number
  ground_name: string
  origin: string
}

interface ClubSelectionScreenProps {
  year: number
  gameMode: string
  onClubSelected: (clubId: string, clubName: string) => void
  onBack: () => void
}

export function ClubSelectionScreen({ year, gameMode, onClubSelected, onBack }: ClubSelectionScreenProps) {
  const [clubs, setClubs] = useState<Club[]>([])
  const [selectedClub, setSelectedClub] = useState<Club | null>(null)
  const [selectedClubDivision, setSelectedClubDivision] = useState<string | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [activeFilter, setActiveFilter] = useState<string>('A-E')
  const isFantasyLeague = gameMode === 'sheffield-hallamshire-league'

  useEffect(() => {
    loadClubs()
  }, [year, gameMode])

  const loadClubs = async () => {
    try {
      setLoading(true)
      setError(null)

      // For fantasy league mode, read clubs directly from Sheffield1867.db
      // For other modes, use hardcoded clubs filtered by year
      let availableClubs: Club[]

      if (isFantasyLeague) {
        // Read all 372 clubs from Sheffield1867.db
        availableClubs = await invoke('get_sheffield_clubs_from_db')
      } else {
        // Determine if ahistorical
        const isAhistorical = gameMode.startsWith('ahistorical')
        // Use hardcoded clubs filtered by founding year
        availableClubs = await invoke('get_sheffield_clubs', { filterYear: isAhistorical ? undefined : year })
      }

      setClubs(availableClubs)
      if (availableClubs.length > 0) {
        setSelectedClub(availableClubs[0])
      }
    } catch (err) {
      setError(`Failed to load clubs: ${err}`)
      console.error(err)
    } finally {
      setLoading(false)
    }
  }

  const handleSelectClub = async (club: Club) => {
    setSelectedClub(club)

    // If fantasy league, fetch the club's division
    if (isFantasyLeague) {
      try {
        const divisionInfo = await invoke<any>('get_club_division_info', {
          clubId: club.id
        })
        setSelectedClubDivision(divisionInfo.division_name)
      } catch (err) {
        console.error('Failed to load division info:', err)
        setSelectedClubDivision(null)
      }
    }
  }

  const handleConfirm = () => {
    if (selectedClub) {
      onClubSelected(selectedClub.id, selectedClub.name)
    }
  }

  const getFilterRanges = () => {
    return ['A-E', 'F-H', 'I-L', 'M-P', 'Q-T', 'U-Z']
  }

  const filterClubsByLetter = (filterRange: string) => {
    const [start, end] = filterRange.split('-')
    const startCode = start.charCodeAt(0)
    const endCode = end.charCodeAt(0)

    return clubs.filter(club => {
      const firstLetter = club.name.charAt(0).toUpperCase().charCodeAt(0)
      return firstLetter >= startCode && firstLetter <= endCode
    })
  }

  const filteredClubs = filterClubsByLetter(activeFilter)
  const displayedClubs = filteredClubs.slice(0, 12)

  if (loading) {
    return (
      <div className="club-selection-screen">
        <div className="club-selection-container">
          <div className="loading">Loading clubs...</div>
        </div>
      </div>
    )
  }

  if (error) {
    return (
      <div className="club-selection-screen">
        <div className="club-selection-container">
          <div className="error">{error}</div>
          <button onClick={onBack} className="back-button">Back</button>
        </div>
      </div>
    )
  }

  return (
    <div className="club-selection-screen">
      <div className="club-selection-container">
        <div className="selection-header">
          <h2>Select Your Club</h2>
          <p className="selection-subtitle">
            {gameMode.startsWith('ahistorical')
              ? `All ${clubs.length} clubs available with ${gameMode.split('-')[1]} rules`
              : `${clubs.length} clubs active in ${year}`}
          </p>
        </div>

        <div className="club-selection-content">
          {/* Left: Club List */}
          <div className="club-list-panel">
            {/* Alphabetic Filter Buttons */}
            <div className="filter-buttons">
              {getFilterRanges().map((range) => {
                const rangeClubs = filterClubsByLetter(range)
                return (
                  <button
                    key={range}
                    className={`filter-btn ${activeFilter === range ? 'active' : ''} ${rangeClubs.length === 0 ? 'disabled' : ''}`}
                    onClick={() => setActiveFilter(range)}
                    disabled={rangeClubs.length === 0}
                  >
                    {range} ({rangeClubs.length})
                  </button>
                )
              })}
            </div>

            <div className="club-list">
              {displayedClubs.map((club) => (
                <div
                  key={club.id}
                  className={`club-item ${selectedClub?.id === club.id ? 'selected' : ''}`}
                  onClick={() => handleSelectClub(club)}
                >
                  <div className="club-item-name">{club.name}</div>
                  <div className="club-item-details">
                    Founded {club.founded_year} • {club.origin}
                  </div>
                </div>
              ))}
            </div>
          </div>

          {/* Right: Club Details */}
          {selectedClub && (
            <div className="club-details-panel">
              <div className="club-detail-header">
                <h3>{selectedClub.name}</h3>
              </div>

              <div className="club-detail-info">
                <div className="info-row">
                  <span className="info-label">Founded:</span>
                  <span className="info-value">{selectedClub.founded_year}</span>
                </div>
                <div className="info-row">
                  <span className="info-label">Origin:</span>
                  <span className="info-value">{selectedClub.origin}</span>
                </div>
                <div className="info-row">
                  <span className="info-label">Ground:</span>
                  <span className="info-value">{selectedClub.ground_name}</span>
                </div>
                {isFantasyLeague && selectedClubDivision && (
                  <div className="info-row">
                    <span className="info-label">Division:</span>
                    <span className="info-value">{selectedClubDivision}</span>
                  </div>
                )}
              </div>

              <div className="club-detail-description">
                <p>
                  Manage {selectedClub.name} through the {year} season{gameMode === 'historical-timeline' ? ' as rules evolve to 1877' : ''}.
                </p>
              </div>

              <div className="selection-buttons">
                <button
                  className="confirm-button"
                  onClick={handleConfirm}
                >
                  Select Club
                </button>
                <button
                  className="cancel-button"
                  onClick={onBack}
                >
                  Back
                </button>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  )
}

export default ClubSelectionScreen
