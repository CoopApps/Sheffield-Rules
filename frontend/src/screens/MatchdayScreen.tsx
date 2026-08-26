import React, { useState } from 'react'
import { GameState, Match } from '../types/GameState'
import { invoke } from '../utils/tauriInvoke'
import { LiveMatchViewer } from '../components/LiveMatchViewer'
import '../styles/matchday-screen.css'

interface MatchdayScreenProps {
  gameState: GameState
  setGameState: (state: GameState | ((prev: GameState) => GameState)) => void
  theme: string
  advanceDay?: () => Promise<void>
  hasMatchesToday?: boolean
}

interface LiveMatch {
  matchId: string
  homeTeam: string
  awayTeam: string
  homeClubId: string
  awayClubId: string
}

export function MatchdayScreen({
  gameState,
  setGameState,
  theme,
  advanceDay: advanceDayProp,
  hasMatchesToday = false,
}: MatchdayScreenProps) {
  const [simulatingMatchId, setSimulatingMatchId] = useState<string | null>(null)
  const [advancingDay, setAdvancingDay] = useState(false)
  const [liveMatch, setLiveMatch] = useState<LiveMatch | null>(null)

  // Get matches for current gameweek
  const gameweekMatches = gameState.matches.filter(m => m.gameweek === gameState.currentGameweek)

  // Get user's team matches from pending events
  const userTeamMatches = gameState.pendingEvents.filter(e =>
    e.eventType.type === 'Match' &&
    (e.eventType.data as any).isUserTeam &&
    !e.processed
  )

  // Get user's club
  const userClub = gameState.clubs.find(c => c.id === gameState.userClubId)

  // Get standings for user's club
  const userStanding = gameState.standings.find(s => s.clubId === gameState.userClubId)

  // Get top 5 standings
  const topStandings = gameState.standings.slice(0, 5)

  // Get pending events (non-match events)
  const pendingEvents = gameState.pendingEvents.filter(e => e.eventType.type !== 'Match' && !e.processed)

  async function simulateMatch(matchId: string) {
    setSimulatingMatchId(matchId)
    try {
      // Find the match to get club IDs
      const match = gameState.matches.find(m => m.id === matchId)
      if (!match) {
        throw new Error('Match not found')
      }

      // Check if this is the user's team playing
      const isUserMatch = match.homeClubId === gameState.userClubId ||
                         match.awayClubId === gameState.userClubId

      if (isUserMatch) {
        // USER'S MATCH - Show live viewer with commentary
        const homeTeamName = getClubName(match.homeClubId)
        const awayTeamName = getClubName(match.awayClubId)

        // Set up live match viewer
        setLiveMatch({
          matchId: match.id,
          homeTeam: homeTeamName,
          awayTeam: awayTeamName,
          homeClubId: match.homeClubId,
          awayClubId: match.awayClubId
        })

        // Start live simulation
        const ruleYear = gameState.startYear || gameState.season
        await invoke('simulate_match_live', {
          matchId: match.id,
          homeClubId: match.homeClubId,
          awayClubId: match.awayClubId,
          ruleYear: ruleYear
        })
      } else {
        // AI vs AI MATCH - Instant simulation
        const ruleYear = gameState.startYear || gameState.season
        const result = await invoke('simulate_match_batch', {
          matchId: match.id,
          homeClubId: match.homeClubId,
          awayClubId: match.awayClubId,
          ruleYear: ruleYear
        })

        // Convert MatchResult to Match format for game state
        const updatedMatch: Match = {
          ...match,
          homeScore: (result as any).home_score,
          awayScore: (result as any).away_score,
          played: true
        }

        // Find the corresponding event and mark it as complete
        const event = gameState.pendingEvents.find(e =>
          e.eventType.type === 'Match' &&
          (e.eventType.data as any).matchId === matchId
        )

        if (event) {
          // Mark the event as complete
          const updatedGame = await invoke('complete_user_event', {
            game: gameState,
            eventId: event.id,
            userResponse: {
              matchId,
              result: {
                homeScore: updatedMatch.homeScore,
                awayScore: updatedMatch.awayScore,
              }
            }
          })
          setGameState(updatedGame as GameState)
        } else {
          // Update game state with match result only if no event tracking
          setGameState(prev => ({
            ...prev,
            matches: prev.matches.map(m => m.id === matchId ? updatedMatch : m),
          }))
        }
      }
    } catch (error) {
      console.error('Failed to simulate match:', error)
    } finally {
      setSimulatingMatchId(null)
    }
  }

  function handleMatchComplete(homeScore: number, awayScore: number, homeRouges?: number, awayRouges?: number) {
    if (!liveMatch) return

    // Update match with final result
    const match = gameState.matches.find(m => m.id === liveMatch.matchId)
    if (!match) return

    const updatedMatch: Match = {
      ...match,
      homeScore,
      awayScore,
      homeRouges,
      awayRouges,
      played: true
    }

    // Find the corresponding event and mark it as complete
    const event = gameState.pendingEvents.find(e =>
      e.eventType.type === 'Match' &&
      (e.eventType.data as any).matchId === liveMatch.matchId
    )

    // Update game state
    if (event) {
      invoke('complete_user_event', {
        game: gameState,
        eventId: event.id,
        userResponse: {
          matchId: liveMatch.matchId,
          result: {
            homeScore,
            awayScore,
            homeRouges,
            awayRouges
          }
        }
      }).then(updatedGame => {
        setGameState(updatedGame as GameState)
        setLiveMatch(null) // Close live viewer
      })
    } else {
      setGameState(prev => ({
        ...prev,
        matches: prev.matches.map(m => m.id === liveMatch.matchId ? updatedMatch : m),
      }))
      setLiveMatch(null) // Close live viewer
    }
  }

  async function advanceToNextDay() {
    setAdvancingDay(true)
    try {
      const result = await invoke('advance_day', { game: gameState })
      setGameState(result as GameState)
    } catch (error) {
      console.error('Failed to advance day:', error)
    } finally {
      setAdvancingDay(false)
    }
  }

  const getClubName = (clubId: string) => gameState.clubs.find(c => c.id === clubId)?.name || '?'

  const formatDate = (dateStr: string) => {
    const date = new Date(dateStr)
    return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' })
  }

  const getSeasonString = () => {
    const year = gameState.season
    return `${year}-${String(year + 1).slice(-2)}`
  }

  return (
    <div className="matchday-screen">
      {/* Live Match Viewer Modal */}
      {liveMatch && (
        <div className="live-match-modal-overlay">
          <LiveMatchViewer
            matchId={liveMatch.matchId}
            homeTeam={liveMatch.homeTeam}
            awayTeam={liveMatch.awayTeam}
            onMatchComplete={handleMatchComplete}
            onClose={() => setLiveMatch(null)}
            ruleYear={gameState.startYear || gameState.season}
          />
        </div>
      )}

      {/* TOP AREA: Matches Grid (70-75% width) */}
      <div className="matchday-top">
        <div className="matchday-header">
          <div className="header-content">
            <h1>Gameweek {gameState.currentGameweek}</h1>
            <p className="current-date">{formatDate(gameState.currentDate)}</p>
          </div>
          <div className="matchday-actions">
            <button
              className="action-btn next-gameweek"
              onClick={advanceDayProp || advanceToNextDay}
              disabled={advancingDay}
              title={hasMatchesToday ? "Continue to today's matches" : "Advance to the next day"}
            >
              {advancingDay ? 'Advancing...' : (hasMatchesToday ? 'Continue' : 'Next Day')}
            </button>
          </div>
        </div>

        <div className="matches-container">
          {gameweekMatches.length === 0 ? (
            <p className="no-matches">No matches scheduled for this gameweek</p>
          ) : (
            <div className="matches-grid">
              {gameweekMatches.map((match) => (
                <div key={match.id} className="match-card">
                  <div className="match-header">
                    <span className="match-date">
                      {formatDate(match.date)}
                    </span>
                  </div>

                  <div className="match-teams">
                    <div className="team home">
                      <span className="team-name">{getClubName(match.homeClubId)}</span>
                    </div>

                    <div className="match-score">
                      {match.played ? (
                        <>
                          <span className="score">{match.homeScore}</span>
                          <span className="vs">-</span>
                          <span className="score">{match.awayScore}</span>
                        </>
                      ) : (
                        <span className="vs">vs</span>
                      )}
                    </div>

                    <div className="team away">
                      <span className="team-name">{getClubName(match.awayClubId)}</span>
                    </div>
                  </div>

                  {!match.played && (
                    <button
                      className="simulate-btn"
                      onClick={() => simulateMatch(match.id)}
                      disabled={simulatingMatchId === match.id}
                    >
                      {simulatingMatchId === match.id ? 'Simulating...' : 'Play'}
                    </button>
                  )}
                </div>
              ))}
            </div>
          )}
        </div>
      </div>

      {/* SIDEBAR: Game State (25-30% width) */}
      <div className="matchday-sidebar">
        <div className="game-state-panel">
          <h3>Game State</h3>

          <div className="state-section">
            <div className="state-item">
              <span className="label">Season</span>
              <span className="value">{getSeasonString()}</span>
            </div>
            <div className="state-item">
              <span className="label">Year</span>
              <span className="value">{gameState.startYear || gameState.season}</span>
            </div>
            <div className="state-item">
              <span className="label">Mode</span>
              <span className="value">{gameState.gameMode || 'Standard'}</span>
            </div>
          </div>

          {userClub && (
            <>
              <div className="divider"></div>
              <div className="state-section">
                <h4>Your Club</h4>
                <div className="state-item">
                  <span className="label">Team</span>
                  <span className="value">{userClub.name}</span>
                </div>
                {userStanding && (
                  <>
                    <div className="state-item">
                      <span className="label">Position</span>
                      <span className="value">#{userStanding.position}</span>
                    </div>
                    <div className="state-item">
                      <span className="label">Points</span>
                      <span className="value">{userStanding.points}</span>
                    </div>
                    <div className="state-item">
                      <span className="label">Record</span>
                      <span className="value">{userStanding.won}W {userStanding.drawn}D {userStanding.lost}L</span>
                    </div>
                  </>
                )}
              </div>
            </>
          )}

          <div className="divider"></div>
          <div className="state-section">
            <h4>League Standings (Top 5)</h4>
            <div className="standings-mini">
              {topStandings.map((standing, idx) => (
                <div key={standing.clubId} className={`standing-row ${standing.clubId === gameState.userClubId ? 'user-club' : ''}`}>
                  <span className="position">#{standing.position}</span>
                  <span className="club-name">{standing.clubName}</span>
                  <span className="points">{standing.points}pts</span>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>

      {/* LOWER AREA: News Feed & Events (full width or 70%) */}
      <div className="matchday-lower">
        <div className="news-feed-panel">
          <h3>News & Events</h3>

          {pendingEvents.length === 0 ? (
            <p className="no-events">No pending events</p>
          ) : (
            <div className="events-list">
              {pendingEvents.map((event) => (
                <div key={event.id} className="event-item">
                  <div className="event-date">{formatDate(event.date)}</div>
                  <div className="event-content">
                    {event.eventType.type === 'PlayerNegotiation' && (
                      <div>
                        <strong>Player Negotiation</strong>
                        <p>{(event.eventType.data as any).offerType}</p>
                      </div>
                    )}
                    {event.eventType.type === 'MediaInquiry' && (
                      <div>
                        <strong>Media Inquiry</strong>
                        <p>{(event.eventType.data as any).question}</p>
                      </div>
                    )}
                    {event.eventType.type === 'HistoricalAnnouncement' && (
                      <div>
                        <strong>{(event.eventType.data as any).title}</strong>
                        <p>{(event.eventType.data as any).description}</p>
                      </div>
                    )}
                  </div>
                  {event.requiresUserAction && <span className="action-badge">Requires Action</span>}
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  )
}

export default MatchdayScreen
