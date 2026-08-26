import React, { useState, useEffect } from 'react'
import { invoke } from '../utils/tauriInvoke'
import '../styles/sheffield-year-selector.css'
import '../styles/encyclopedia.css'
import { SHEFFIELD_RULES_YEARS, SheffieldYear } from '../constants/sheffieldYearsData'
import { EncyclopediaScreen } from '../screens/EncyclopediaScreen'

// SVG Icons
const CrownIcon = () => (
  <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
    <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
  </svg>
)

const RougeIcon = () => (
  <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
    <circle cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="2" fill="none"/>
    <circle cx="12" cy="12" r="3" fill="currentColor"/>
  </svg>
)

const CalendarIcon = () => (
  <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
    <rect x="3" y="4" width="18" height="18" rx="2" fill="none" stroke="currentColor" strokeWidth="2"/>
    <line x1="16" y1="2" x2="16" y2="6" stroke="currentColor" strokeWidth="2"/>
    <line x1="8" y1="2" x2="8" y2="6" stroke="currentColor" strokeWidth="2"/>
    <line x1="3" y1="10" x2="21" y2="10" stroke="currentColor" strokeWidth="2"/>
  </svg>
)

const SoccerBallIcon = () => (
  <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
    <circle cx="12" cy="12" r="10" fill="none" stroke="currentColor" strokeWidth="2"/>
    <path d="M12 2v20M2 12h20M6 6l12 12M6 18l12-12" stroke="currentColor" strokeWidth="1"/>
  </svg>
)

const TrophyIcon = () => (
  <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
    <path d="M3 6h2v9c0 2.2 1.8 4 4 4h6c2.2 0 4-1.8 4-4V6h2V4H3v2zm4 11c-1.1 0-2-.9-2-2v-8h10v8c0 1.1-.9 2-2 2H7z"/>
    <path d="M8 1h8v2H8z"/>
  </svg>
)

const LocationIcon = () => (
  <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
    <path d="M12 2C7.58 2 4 5.58 4 10c0 5.25 8 13 8 13s8-7.75 8-13c0-4.42-3.58-8-8-8zm0 11c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3z"/>
  </svg>
)

const SaveIcon = () => (
  <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
    <path d="M17 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V7l-4-4zm-5 16c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3zm3-10H5V5h10v4z"/>
  </svg>
)

const LoadIcon = () => (
  <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
    <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
  </svg>
)

const BookIcon = () => (
  <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
    <path d="M18 2H6c-1.1 0-2 .9-2 2v16c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zM9 4h6v12H9V4z"/>
  </svg>
)

const SettingsIcon = () => (
  <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
    <path d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l1.72-1.35c.15-.12.19-.34.1-.51l-1.63-2.83c-.12-.22-.37-.29-.59-.22l-2.03.81c-.42-.32-.92-.6-1.47-.78l-.31-2.15c-.05-.24-.24-.41-.5-.41h-3.26c-.26 0-.45.17-.49.41l-.31 2.15c-.56.18-1.05.46-1.47.78l-2.03-.81c-.22-.09-.47 0-.59.22l-1.63 2.83c-.1.17-.06.39.1.51l1.72 1.35c-.04.3-.07.62-.07.94s.02.64.07.94l-1.72 1.35c-.15.12-.19.34-.1.51l1.63 2.83c.12.22.37.29.59.22l2.03-.81c.42.32.92.6 1.47.78l.31 2.15c.05.24.24.41.5.41h3.26c.26 0 .45-.17.49-.41l.31-2.15c.56-.18 1.05-.46 1.47-.78l2.03.81c.22.09.47 0 .59-.22l1.63-2.83c.1-.17.06-.39-.1-.51l-1.72-1.35zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/>
  </svg>
)

const CloseIcon = () => (
  <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
    <path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12 19 6.41z"/>
  </svg>
)

const RulesIcon = () => (
  <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
    <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8l-6-6z"/>
    <polyline points="14 2 14 8 20 8"/>
    <line x1="12" y1="19" x2="12" y2="13"/>
    <line x1="9" y1="16" x2="15" y2="16"/>
  </svg>
)

const TrophyAchievementIcon = () => (
  <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
    <path d="M6 9c0-1 1-2 2-2h8c1 0 2 1 2 2v3H6V9zm8-5c0-1-1-2-2-2s-2 1-2 2v1h4V4zm6 4c1 0 2 1 2 2v8c0 1-1 2-2 2h-1v2h-2v-2H9v2H7v-2H6c-1 0-2-1-2-2v-8c0-1 1-2 2-2h1V4c0-1 1-2 2-2s2 1 2 2v1h4V4c0-1 1-2 2-2s2 1 2 2v1h1z"/>
  </svg>
)

interface SheffieldYearSelectorProps {
  onSelectYear: (year: number) => void
  onSelectGameMode?: (gameMode: 'historical-timeline' | 'ahistorical' | 'ahistorical-1860' | 'ahistorical-1862' | 'ahistorical-1868' | 'ahistorical-1875' | 'historical-from-year' | 'sheffield-hallamshire-league', year?: number) => void
  onBack?: () => void
  showClubSelection?: boolean
  selectedGameMode?: string
  selectedYear?: number
  onClubSelected?: (clubId: string, clubName: string) => void
  onClubSelectionBack?: () => void
}
export function SheffieldYearSelector({ onSelectYear, onSelectGameMode, onBack, showClubSelection, selectedGameMode, selectedYear: propSelectedYear, onClubSelected, onClubSelectionBack }: SheffieldYearSelectorProps) {
  const [selectedYear, setSelectedYear] = useState<number | null>(propSelectedYear ?? 1867)
  const [hoveredYear, setHoveredYear] = useState<number | null>(null)
  const [clubs, setClubs] = useState<any[]>([])
  const [selectedClub, setSelectedClub] = useState<any | null>(null)
  const [loadingClubs, setLoadingClubs] = useState(false)
  const [clubError, setClubError] = useState<string | null>(null)
  const [showEncyclopedia, setShowEncyclopedia] = useState(false)
  const [clubFilter, setClubFilter] = useState<string>('A-E')

  const handleYearClick = (year: number) => {
    console.log('[handleYearClick] Year clicked:', year)
    setSelectedYear(year)
    onSelectYear(year)
  }
  const getYearPeriod = (year: number): string => {
    if (year < 1862) return 'early'
    if (year >= 1862 && year <= 1868) return 'rouge-era'
    if (year >= 1869 && year <= 1877) return 'late'
    return 'early'
  }

  // Load clubs when club selection is shown
  useEffect(() => {
    if (showClubSelection && clubs.length === 0) {
      loadClubs()
    }
  }, [showClubSelection])

  const loadClubs = async () => {
    try {
      setLoadingClubs(true)
      setClubError(null)
      const yearToFilter = propSelectedYear || selectedYear
      // Only Sheffield & Hallamshire League (Fantasy 1867) should show all clubs
      // All other modes should filter by the selected year
      const isFantasyMode = selectedGameMode === 'sheffield-hallamshire-league'
      console.log('[SheffieldYearSelector] Loading clubs:', {
        selectedYear,
        propSelectedYear,
        yearToFilter,
        selectedGameMode,
        isFantasyMode,
        filterYear: isFantasyMode ? undefined : yearToFilter
      })
      const clubData: any[] = await invoke('get_sheffield_clubs', {
        filterYear: isFantasyMode ? undefined : yearToFilter
      })
      console.log(`[SheffieldYearSelector] Loaded ${clubData.length} clubs for year ${yearToFilter}`)
      setClubs(clubData)
      if (clubData.length > 0) {
        setSelectedClub(clubData[0])
      }
    } catch (err) {
      setClubError(`Failed to load clubs: ${err}`)
      console.error(err)
    } finally {
      setLoadingClubs(false)
    }
  }

  const handleSelectClub = (club: any) => {
    setSelectedClub(club)
  }

  const handleConfirmClub = () => {
    if (selectedClub && onClubSelected) {
      onClubSelected(selectedClub.id, selectedClub.name)
    }
  }

  const getLetterRanges = () => {
    const letters = new Set<string>()
    clubs.forEach(club => {
      letters.add(club.name.charAt(0).toUpperCase())
    })
    const sortedLetters = Array.from(letters).sort()

    if (sortedLetters.length === 0) return ['A-E', 'F-H', 'I-L', 'M-P', 'Q-T', 'U-Z']

    const ranges: string[] = []
    let rangeStart = sortedLetters[0]
    let rangeEnd = sortedLetters[0]
    let itemsInRange = 1

    for (let i = 1; i < sortedLetters.length; i++) {
      itemsInRange++
      if (itemsInRange > 6 || i === sortedLetters.length - 1) {
        rangeEnd = sortedLetters[i - (itemsInRange > 6 ? 1 : 0)]
        ranges.push(`${rangeStart}-${rangeEnd}`)
        if (itemsInRange > 6) {
          rangeStart = sortedLetters[i]
          rangeEnd = sortedLetters[i]
          itemsInRange = 1
        }
      }
    }
    if (itemsInRange > 0 && !ranges[ranges.length - 1]?.endsWith(rangeEnd)) {
      rangeEnd = sortedLetters[sortedLetters.length - 1]
      ranges.push(`${rangeStart}-${rangeEnd}`)
    }

    return ranges.length > 0 ? ranges : ['A-Z']
  }

  const getFilteredClubs = () => {
    if (!clubFilter) return clubs.slice(0, 12)

    const [start, end] = clubFilter.split('-')
    const filtered = clubs.filter(club => {
      const firstLetter = club.name.charAt(0).toUpperCase()
      return firstLetter >= start && firstLetter <= end
    })
    // Limit to 12 clubs per screen
    return filtered.slice(0, 12)
  }

  const selectedYearData = SHEFFIELD_RULES_YEARS.find(y => y.year === selectedYear)
  return (
    <div className="sheffield-year-selector">
      {/* Top: Left side (4/5) + Right side (1/5) */}
      <div className="selector-top">
        {/* Left: Year Roundels (4/5 width) */}
        <div className="year-flowchart">
          <div className="years-grid">
            {SHEFFIELD_RULES_YEARS.map((yearData: SheffieldYear) => {
              const period = getYearPeriod(yearData.year)
              const isSelected = selectedYear === yearData.year
              const isHovered = hoveredYear === yearData.year
              return (
                <div
                  key={yearData.year}
                  className={`year-node period-${period} ${isSelected ? 'selected' : ''} ${isHovered ? 'hovered' : ''} ${yearData.isOfficial ? 'official' : ''}`}
                  onClick={() => handleYearClick(yearData.year)}
                  onMouseEnter={() => setHoveredYear(yearData.year)}
                  onMouseLeave={() => setHoveredYear(null)}
                >
                  {yearData.isOfficial && <div className="official-circle-ring"></div>}
                  <div className="year-node-inner">
                    <div className="year-number">{yearData.year}</div>
                  </div>
                  <div className={`period-label ${period}`}>
                    {period === 'early' && 'EARLY'}
                    {period === 'rouge-era' && 'ROUGE'}
                    {period === 'late' && 'LATE'}
                  </div>
                  <div className="year-tooltip">
                    <div className="tooltip-year">{yearData.year}</div>
                    <div className="tooltip-name">{yearData.name}</div>
                    {yearData.rougeActive && (
                      <div className="tooltip-rouge">
                        <RougeIcon />
                        <span>Rouge Scoring Active</span>
                      </div>
                    )}
                  </div>
                </div>
              )
            })}
          </div>
        </div>

        {/* Right: Details Panel (1/5 width) */}
        {selectedYearData && (
          <div className="year-details-panel">
            <div className="details-header">
              <h3>{selectedYearData.name}</h3>
              {selectedYearData.isOfficial && (
                <div className="official-label">
                  RECOMMENDED START YEAR
                </div>
              )}
            </div>
            <div className="significance">
              <strong>Historical Significance:</strong>
              <p>{selectedYearData.significance}</p>
            </div>
            <div className="changes">
              <strong>Rule Changes & Events:</strong>
              <ul>
                {selectedYearData.changes.map((change: string, idx: number) => (
                  <li key={idx}>{change}</li>
                ))}
              </ul>
            </div>
            {selectedYearData.rougeActive && (
              <div className="rouge-info">
                <div className="rouge-badge">
                  <span>ROUGE SCORING ACTIVE</span>
                </div>
                <p>
                  When a ball is kicked between the uprights below the crossbar, it counts
                  as a rouge and is worth 1 point, while a goal (above the crossbar) is worth 2 points.
                </p>
              </div>
            )}
            <div className="period-context">
              {selectedYearData.year < 1862 && (
                <div className="context-early">
                  <strong>Early Period (Pre-Rouge):</strong>
                  <p>The formative years of organized football rules, before the introduction of the rouge scoring system.</p>
                </div>
              )}
              {selectedYearData.year === 1868 && (
                <div className="context-rouge-end">
                  <strong>Post-Rouge Era:</strong>
                  <p>The rouge system is abolished, but Sheffield Rules continue to evolve toward modern football.</p>
                </div>
              )}
              {selectedYearData.year > 1868 && selectedYearData.year < 1877 && (
                <div className="context-late">
                  <strong>Late Period (Post-Rouge):</strong>
                  <p>Sheffield Rules progressively converge with FA Rules, setting the foundation for modern football.</p>
                </div>
              )}
              {selectedYearData.year === 1877 && (
                <div className="context-end">
                  <strong>Era Conclusion:</strong>
                  <p>Sheffield Rules formally merge with FA Rules, marking the end of this unique period in football history.</p>
                </div>
              )}
            </div>
            <div className="game-mode-selection">
              <strong>How would you like to play?</strong>
              {(() => { console.log('[SheffieldYearSelector] Rendering mode buttons, selectedYear:', selectedYear, 'is1860?', selectedYear === 1860); return null; })()}
              <div className="mode-buttons">
                {selectedYear === 1867 && (
                  <button
                    className="mode-button fantasy"
                    onClick={() => onSelectGameMode?.("sheffield-hallamshire-league")}
                  >
                    <span className="icon"><TrophyIcon /></span>
                    <div className="button-text">
                      <span className="label">Sheffield & Hallamshire League</span>
                      <span className="description">Fantasy 1867: All 186 historical clubs in unified pyramid</span>
                    </div>
                  </button>
                )}
                <button
                  className="mode-button ahistorical"
                  onClick={() => onSelectGameMode?.("ahistorical-1862")}
                >
                  <span className="icon"><RougeIcon /></span>
                  <div className="button-text">
                    <span className="label">Rouge Era (1862)</span>
                    <span className="description">Play indefinitely with rouge scoring active</span>
                  </div>
                </button>
                <button
                  className="mode-button ahistorical"
                  onClick={() => onSelectGameMode?.("ahistorical-1868")}
                >
                  <span className="icon"><SoccerBallIcon /></span>
                  <div className="button-text">
                    <span className="label">Post-Rouge (1868)</span>
                    <span className="description">Play indefinitely with goals-only scoring</span>
                  </div>
                </button>
                <button
                  className="mode-button ahistorical"
                  onClick={() => onSelectGameMode?.("ahistorical-1875")}
                >
                  <span className="icon"><TrophyIcon /></span>
                  <div className="button-text">
                    <span className="label">Stable Era (1875)</span>
                    <span className="description">Play indefinitely with most balanced rules</span>
                  </div>
                </button>
                {selectedYear !== 1858 && (
                  <button
                    className="mode-button historical-from"
                    onClick={() => onSelectGameMode?.("historical-from-year", selectedYear ?? undefined)}
                  >
                    <span className="icon"><LocationIcon /></span>
                    <div className="button-text">
                      <span className="label">Historical From {selectedYear}</span>
                      <span className="description">Start {selectedYear}, rules evolve to 1877</span>
                    </div>
                  </button>
                )}
                <button
                  className="mode-button ahistorical"
                  onClick={() => onSelectGameMode?.("ahistorical-1860")}
                >
                  <span className="icon"><LocationIcon /></span>
                  <div className="button-text">
                    <span className="label">Demo Mode (1860)</span>
                    <span className="description">Learn the match engine with Sheffield FC vs Hallam FC</span>
                  </div>
                </button>
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Club Selection Overlay */}
      {showClubSelection && (
        <div className="club-selection-overlay">
          <div className="club-selection-modal">
            <div className="club-selection-header">
              <h2>Select Your Club</h2>
              <p className="club-selection-subtitle">
                {selectedGameMode?.startsWith('ahistorical')
                  ? `All ${clubs.length} clubs available`
                  : `${clubs.length} clubs active in ${propSelectedYear || selectedYear}`}
              </p>
            </div>

            {loadingClubs ? (
              <div className="club-loading">Loading clubs...</div>
            ) : clubError ? (
              <div className="club-error">{clubError}</div>
            ) : (
              <div className="club-selection-content">
                {/* Filter Buttons */}
                <div className="club-filter-buttons">
                  {getLetterRanges().map((range) => {
                    const rangeClubs = clubs.filter(club => {
                      const [start, end] = range.split('-')
                      const firstLetter = club.name.charAt(0).toUpperCase()
                      return firstLetter >= start && firstLetter <= end
                    })
                    return (
                      <button
                        key={range}
                        className={`filter-btn ${clubFilter === range ? 'active' : ''} ${rangeClubs.length === 0 ? 'disabled' : ''}`}
                        onClick={() => setClubFilter(range)}
                        disabled={rangeClubs.length === 0}
                      >
                        {range} ({rangeClubs.length})
                      </button>
                    )
                  })}
                </div>

                {/* Club List */}
                <div className="club-list-container">
                  <div className="club-list">
                    {getFilteredClubs().map((club: any) => (
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

                {/* Club Details */}
                {selectedClub && (
                  <div className="club-details-container">
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
                    </div>
                    <div className="club-detail-description">
                      <p>
                        Manage {selectedClub.name} through the {propSelectedYear || selectedYear} season.
                      </p>
                    </div>
                  </div>
                )}
              </div>
            )}

            <div className="club-selection-buttons">
              <button className="club-confirm-button" onClick={handleConfirmClub} disabled={!selectedClub}>
                Select Club
              </button>
              <button className="club-back-button" onClick={onClubSelectionBack}>
                Back
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Bottom: Action Buttons (full width) */}
      <div className="year-action-buttons">
        <button className="action-button back-btn" onClick={onBack} title="Go back to game selection">
          <span className="action-button-icon">←</span>
          Back
        </button>
        <button className="action-button save-btn" title="Save the current game">
          <span className="action-button-icon"><SaveIcon /></span>
          Save Game
        </button>
        <button className="action-button load-btn" title="Load a saved game">
          <span className="action-button-icon"><LoadIcon /></span>
          Load Game
        </button>
        <button
          className="action-button encyclopedia-btn"
          title="View the encyclopedia"
          onClick={() => setShowEncyclopedia(true)}
        >
          <span className="action-button-icon"><BookIcon /></span>
          Encyclopedia
        </button>
        <button className="action-button rules-btn" title="View the Sheffield Rules">
          <span className="action-button-icon"><RulesIcon /></span>
          Rules
        </button>
        <button className="action-button achievements-btn" title="View achievements">
          <span className="action-button-icon"><TrophyAchievementIcon /></span>
          Achievements
        </button>
        <button className="action-button settings-btn" title="Open settings">
          <span className="action-button-icon"><SettingsIcon /></span>
          Settings
        </button>
      </div>

      {/* Game Info Box */}
      <div className="year-info-box">
        <div className="info-section">
          <div className="info-item">
            <span className="info-label">About</span>
            <span className="info-value">Sheffield Rules<br />Football Simulator (1858-1877)</span>
          </div>
          <div className="year-info-divider"></div>
          <div className="info-item">
            <span className="info-label">Description</span>
            <span className="info-value">Experience the birth of modern football through Sheffield Rules - the earliest standardized form of the beautiful game. From 1858 to 1877, witness the evolution of football as it was played in Sheffield, including the revolutionary rouge scoring system that defined an era. Manage your team's journey through this pivotal period in football history.</span>
          </div>
          <div className="year-info-divider"></div>
          <div className="info-item">
            <span className="info-label">Creator</span>
            <span className="info-value">Development Team</span>
          </div>
          <div className="year-info-divider"></div>
          <div className="info-item">
            <span className="info-label">Version</span>
            <span className="info-value">0.1.0</span>
          </div>
        </div>
      </div>

      {/* Encyclopedia Modal */}
      {showEncyclopedia && (
        <EncyclopediaScreen onClose={() => setShowEncyclopedia(false)} />
      )}
    </div>
  )
}
export default SheffieldYearSelector
