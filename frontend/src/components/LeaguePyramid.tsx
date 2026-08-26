import React from 'react'
import '../styles/league-pyramid.css'

interface Division {
  id: string
  name: string
  level: number
  region: string | null
}

interface LeaguePyramidProps {
  divisions: Division[]
  clubCounts: Map<string, number>
  onDivisionClick?: (divisionId: string) => void
}

export function LeaguePyramid({ divisions, clubCounts, onDivisionClick }: LeaguePyramidProps) {
  // Group divisions by level
  const divisionsByLevel = divisions.reduce((acc, division) => {
    if (!acc[division.level]) {
      acc[division.level] = []
    }
    acc[division.level].push(division)
    return acc
  }, {} as Record<number, Division[]>)

  // Sort levels in ascending order (1 at top, higher numbers below)
  const sortedLevels = Object.keys(divisionsByLevel)
    .map(Number)
    .sort((a, b) => a - b)

  // Get promotion/relegation spots (simplified - could be configurable later)
  const getPromotionSpots = (level: number) => {
    if (level === 1) return 0 // Top division has no promotion
    return 2 // Default: top 2 promote
  }

  const getRelegationSpots = (level: number) => {
    if (level === sortedLevels[sortedLevels.length - 1]) return 0 // Bottom division has no relegation
    return 2 // Default: bottom 2 relegate
  }

  return (
    <div className="league-pyramid">
      <div className="pyramid-header">
        <h2>Sheffield & Hallamshire League Pyramid</h2>
        <p>Visual representation of the {divisions.length} division structure</p>
      </div>

      <div className="pyramid-container">
        {sortedLevels.map((level, levelIndex) => {
          const levelDivisions = divisionsByLevel[level]
          const isTopLevel = levelIndex === 0
          const isBottomLevel = levelIndex === sortedLevels.length - 1

          return (
            <div key={level} className="pyramid-level">
              <div className="level-header">
                <span className="level-number">Level {level}</span>
                <span className="level-divisions">{levelDivisions.length} division{levelDivisions.length !== 1 ? 's' : ''}</span>
              </div>

              <div className="divisions-row" style={{
                gridTemplateColumns: `repeat(${levelDivisions.length}, 1fr)`,
                maxWidth: `${Math.min(levelDivisions.length * 250, 1200)}px`
              }}>
                {levelDivisions.map((division, divIndex) => {
                  const clubCount = clubCounts.get(division.id) || 0
                  const promotionSpots = getPromotionSpots(level)
                  const relegationSpots = getRelegationSpots(level)

                  return (
                    <div
                      key={division.id}
                      className="pyramid-division"
                      onClick={() => onDivisionClick?.(division.id)}
                    >
                      <div className="division-card">
                        <div className="division-card-header">
                          <h4>{division.name}</h4>
                          {division.region && (
                            <span className="division-region-tag">{division.region}</span>
                          )}
                        </div>

                        <div className="division-stats">
                          <div className="stat">
                            <span className="stat-label">Teams</span>
                            <span className="stat-value">{clubCount}</span>
                          </div>
                          <div className="stat">
                            <span className="stat-label">Level</span>
                            <span className="stat-value">{level}</span>
                          </div>
                        </div>

                        {!isTopLevel && promotionSpots > 0 && (
                          <div className="promotion-indicator">
                            <div className="flow-arrow up">
                              <svg width="24" height="24" viewBox="0 0 24 24">
                                <path d="M12 4l-8 8h6v8h4v-8h6z" fill="currentColor"/>
                              </svg>
                            </div>
                            <span className="flow-text">↑ {promotionSpots} up</span>
                          </div>
                        )}

                        {!isBottomLevel && relegationSpots > 0 && (
                          <div className="relegation-indicator">
                            <div className="flow-arrow down">
                              <svg width="24" height="24" viewBox="0 0 24 24">
                                <path d="M12 20l8-8h-6V4h-4v8H4z" fill="currentColor"/>
                              </svg>
                            </div>
                            <span className="flow-text">↓ {relegationSpots} down</span>
                          </div>
                        )}
                      </div>

                      {/* Connection line to next level */}
                      {!isBottomLevel && (
                        <div className="connection-line"></div>
                      )}
                    </div>
                  )
                })}
              </div>
            </div>
          )
        })}
      </div>

      <div className="pyramid-legend">
        <div className="legend-item">
          <div className="legend-icon promotion">↑</div>
          <span>Promotion places</span>
        </div>
        <div className="legend-item">
          <div className="legend-icon relegation">↓</div>
          <span>Relegation places</span>
        </div>
        <div className="legend-note">
          Click on any division to view its clubs in the management view
        </div>
      </div>
    </div>
  )
}
