import React, { useState, useEffect } from 'react'
import { GameState, Match } from '../types/GameState'
import { invoke } from '../utils/tauriInvoke'
import { listen } from '@tauri-apps/api/event'
import '../styles/MatchdayModeModal.css'

interface MatchdayModeModalProps {
  gameState: GameState
  matchesForDay: Match[]
  onComplete: (updatedGameState: GameState) => void
  onClose: () => void
}

interface MatchStatus {
  matchId: string
  status: 'pending' | 'simulating' | 'complete' | 'error'
  result?: Match
  error?: string
}

interface Sheffield1867Event {
  minute: number
  event_type: string
  description: string
  home_player: string | null
  away_player: string | null
  home_score: number
  away_score: number
  home_rouges: number
  away_rouges: number
}

interface Sheffield1867MatchUpdate {
  match_id: string
  minute: number
  event: Sheffield1867Event
}

interface LiveMatchData {
  matchId: string
  homeTeamName: string
  awayTeamName: string
  events: Sheffield1867Event[]
  currentMinute: number
  homeGoals: number
  awayGoals: number
  homeRouges: number
  awayRouges: number
  isComplete: boolean
}

export function MatchdayModeModal({
  gameState,
  matchesForDay,
  onComplete,
  onClose
}: MatchdayModeModalProps) {
  const [matchStatuses, setMatchStatuses] = useState<MatchStatus[]>([])
  const [allComplete, setAllComplete] = useState(false)
  const [currentGameState, setCurrentGameState] = useState<GameState>(gameState)
  const [liveMatchData, setLiveMatchData] = useState<LiveMatchData | null>(null)
  const [viewingMatch, setViewingMatch] = useState<string | null>(null)

  // Determine if we're using Sheffield 1867 rules
  const isSheffieldRules = gameState.startYear && gameState.startYear >= 1857 && gameState.startYear <= 1877

  // Separate user matches from AI matches
  const userMatches = matchesForDay.filter(m =>
    m.homeTeamId === gameState.userClubId || m.awayTeamId === gameState.userClubId
  )
  const aiMatches = matchesForDay.filter(m =>
    m.homeTeamId !== gameState.userClubId && m.awayTeamId !== gameState.userClubId
  )

  // Initialize match statuses
  useEffect(() => {
    const initialStatuses: MatchStatus[] = matchesForDay.map(m => ({
      matchId: m.id,
      status: 'pending'
    }))
    setMatchStatuses(initialStatuses)
  }, [matchesForDay])

  // Set up Sheffield 1867 event listener
  useEffect(() => {
    let unlisten: (() => void) | undefined

    if (isSheffieldRules) {
      listen('sheffield_1867_match_update', (event: any) => {
        const update = event.payload as Sheffield1867MatchUpdate

        setLiveMatchData(prev => {
          if (!prev || prev.matchId !== update.match_id) {
            // Initialize new match data
            const match = matchesForDay.find(m => m.id === update.match_id)
            if (!match) return prev

            return {
              matchId: update.match_id,
              homeTeamName: getClubName(match.homeTeamId),
              awayTeamName: getClubName(match.awayTeamId),
              events: [update.event],
              currentMinute: update.minute,
              homeGoals: update.event.home_score,
              awayGoals: update.event.away_score,
              homeRouges: update.event.home_rouges,
              awayRouges: update.event.away_rouges,
              isComplete: update.event.event_type === 'FullTime'
            }
          }

          // Update existing match data
          return {
            ...prev,
            events: [...prev.events, update.event],
            currentMinute: update.minute,
            homeGoals: update.event.home_score,
            awayGoals: update.event.away_score,
            homeRouges: update.event.home_rouges,
            awayRouges: update.event.away_rouges,
            isComplete: update.event.event_type === 'FullTime'
          }
        })

        // If match is complete, update match status
        if (update.event.event_type === 'FullTime') {
          const match = matchesForDay.find(m => m.id === update.match_id)
          if (match) {
            const updatedMatch: Match = {
              ...match,
              homeScore: update.event.home_score,
              awayScore: update.event.away_score,
              played: true
            }

            setCurrentGameState(prev => ({
              ...prev,
              matches: prev.matches.map(m => m.id === update.match_id ? updatedMatch : m)
            }))

            setMatchStatuses(prev =>
              prev.map(s =>
                s.matchId === update.match_id
                  ? { ...s, status: 'complete', result: updatedMatch }
                  : s
              )
            )
          }
        }
      }).then(fn => {
        unlisten = fn
      })
    }

    return () => {
      if (unlisten) unlisten()
    }
  }, [isSheffieldRules, matchesForDay])

  // Auto-simulate AI matches on mount
  useEffect(() => {
    simulateAIMatches()
  }, [])

  // Check if all matches are complete
  useEffect(() => {
    if (matchStatuses.length === 0) return

    const allDone = matchStatuses.every(s => s.status === 'complete')
    setAllComplete(allDone)
  }, [matchStatuses])

  async function simulateAIMatches() {
    for (const match of aiMatches) {
      await simulateMatch(match.id, false)
    }
  }

  async function simulateMatch(matchId: string, isUserMatch: boolean) {
    // Update status to simulating
    setMatchStatuses(prev =>
      prev.map(s => s.matchId === matchId ? { ...s, status: 'simulating' } : s)
    )

    // Set viewing match if it's a user match
    if (isUserMatch) {
      setViewingMatch(matchId)
      setLiveMatchData(null) // Reset live match data
    }

    try {
      // Find the match to get club IDs
      const match = matchesForDay.find(m => m.id === matchId)
      if (!match) {
        throw new Error('Match not found')
      }

      // Determine which simulation engine to use
      if (isSheffieldRules && isUserMatch) {
        // Use Sheffield 1867 live simulator for user matches
        await invoke('simulate_sheffield_1867_match', {
          matchId: match.id,
          homeClubId: match.homeTeamId,
          awayClubId: match.awayTeamId
        })
        // Events will be handled by the listener
      } else {
        // Use standard batch simulator for AI matches or non-Sheffield rules
        const ruleYear = gameState.startYear || gameState.season
        const result = await invoke('simulate_match_batch', {
          matchId: match.id,
          homeClubId: match.homeTeamId,
          awayClubId: match.awayTeamId,
          ruleYear: ruleYear
        })

        // Convert MatchResult to Match format for game state
        const updatedMatch: Match = {
          ...match,
          homeScore: (result as any).home_score,
          awayScore: (result as any).away_score,
          played: true
        }

        // Update local game state with the match result
        setCurrentGameState(prev => ({
          ...prev,
          matches: prev.matches.map(m => m.id === matchId ? updatedMatch : m)
        }))

        // Mark match as complete with result
        setMatchStatuses(prev =>
          prev.map(s =>
            s.matchId === matchId
              ? { ...s, status: 'complete', result: updatedMatch }
              : s
          )
        )
      }
    } catch (error) {
      console.error('Failed to simulate match:', matchId, error)
      setMatchStatuses(prev =>
        prev.map(s =>
          s.matchId === matchId
            ? { ...s, status: 'error', error: String(error) }
            : s
        )
      )
    }
  }

  function getMatchStatus(matchId: string): MatchStatus | undefined {
    return matchStatuses.find(s => s.matchId === matchId)
  }

  function getClubName(clubId: string): string {
    const club = currentGameState.clubs.find(c => c.id === clubId)
    return club ? club.name : clubId
  }

  function formatDate(dateStr: string): string {
    try {
      const parts = dateStr.split('-')
      if (parts.length === 3) {
        const date = new Date(parseInt(parts[0]), parseInt(parts[1]) - 1, parseInt(parts[2]))
        if (!isNaN(date.getTime())) {
          return date.toLocaleDateString('en-GB', {
            weekday: 'short',
            year: 'numeric',
            month: 'short',
            day: 'numeric'
          })
        }
      }
    } catch (e) {
      console.error('Error formatting date:', dateStr, e)
    }
    return dateStr
  }

  function handleContinue() {
    if (allComplete) {
      onComplete(currentGameState)
    }
  }

  function handleRetry(matchId: string) {
    const match = matchesForDay.find(m => m.id === matchId)
    if (match) {
      const isUserMatch = match.homeTeamId === gameState.userClubId ||
                          match.awayTeamId === gameState.userClubId
      simulateMatch(matchId, isUserMatch)
    }
  }

  function handleCloseLiveView() {
    setViewingMatch(null)
    setLiveMatchData(null)
  }

  function getSheffieldScoreDisplay(goals: number, rouges: number): string {
    if (rouges > 0) {
      return `${goals}G ${rouges}R`
    }
    return `${goals}`
  }

  function getSheffieldMatchResult(homeGoals: number, awayGoals: number, homeRouges: number, awayRouges: number): string {
    if (homeGoals > awayGoals) {
      return 'home'
    } else if (awayGoals > homeGoals) {
      return 'away'
    } else if (homeRouges > awayRouges) {
      return 'home-rouges'
    } else if (awayRouges > homeRouges) {
      return 'away-rouges'
    }
    return 'draw'
  }

  const completedCount = matchStatuses.filter(s => s.status === 'complete').length
  const progressPercentage = matchesForDay.length > 0
    ? (completedCount / matchesForDay.length) * 100
    : 0

  // Get top 5 standings
  const topStandings = currentGameState.standings.slice(0, 5)
  const userStanding = currentGameState.standings.find(s => s.clubId === gameState.userClubId)

  // If viewing a live Sheffield 1867 match
  if (viewingMatch && liveMatchData) {
    const result = getSheffieldMatchResult(
      liveMatchData.homeGoals,
      liveMatchData.awayGoals,
      liveMatchData.homeRouges,
      liveMatchData.awayRouges
    )

    return (
      <div className="matchday-mode-overlay">
        <div className="matchday-mode-modal sheffield-match-viewer">
          {/* Match Header */}
          <div className="sheffield-match-header">
            <h2>Sheffield Rules Match</h2>
            <div className="match-minute">Minute {liveMatchData.currentMinute}</div>
          </div>

          {/* Score Display */}
          <div className="sheffield-score-display">
            <div className="team-score home">
              <div className="team-name">{liveMatchData.homeTeamName}</div>
              <div className="score">{getSheffieldScoreDisplay(liveMatchData.homeGoals, liveMatchData.homeRouges)}</div>
            </div>
            <div className="score-separator">-</div>
            <div className="team-score away">
              <div className="team-name">{liveMatchData.awayTeamName}</div>
              <div className="score">{getSheffieldScoreDisplay(liveMatchData.awayGoals, liveMatchData.awayRouges)}</div>
            </div>
          </div>

          {/* Match Result Indicator */}
          {liveMatchData.isComplete && (
            <div className={`match-result-indicator ${result}`}>
              {result === 'home' && `${liveMatchData.homeTeamName} wins!`}
              {result === 'away' && `${liveMatchData.awayTeamName} wins!`}
              {result === 'home-rouges' && `${liveMatchData.homeTeamName} wins on rouges!`}
              {result === 'away-rouges' && `${liveMatchData.awayTeamName} wins on rouges!`}
              {result === 'draw' && 'Match drawn!'}
            </div>
          )}

          {/* Commentary Feed */}
          <div className="sheffield-commentary-feed">
            <h3>Match Commentary</h3>
            <div className="commentary-list">
              {liveMatchData.events.map((event, index) => (
                <div
                  key={index}
                  className={`commentary-event ${event.event_type.toLowerCase()}`}
                >
                  <span className="event-minute">{event.minute}'</span>
                  <span className="event-description">{event.description}</span>
                  {(event.event_type === 'Goal' || event.event_type === 'Rouge') && (
                    <span className="event-score">
                      {getSheffieldScoreDisplay(event.home_score, event.home_rouges)} - {getSheffieldScoreDisplay(event.away_score, event.away_rouges)}
                    </span>
                  )}
                </div>
              ))}
            </div>
          </div>

          {/* Footer */}
          <div className="sheffield-match-footer">
            {liveMatchData.isComplete && (
              <button className="close-match-btn" onClick={handleCloseLiveView}>
                Close Match Viewer
              </button>
            )}
          </div>
        </div>
      </div>
    )
  }

  // Standard matchday view
  return (
    <div className="matchday-mode-overlay">
      <div className="matchday-mode-modal">
        {/* Header */}
        <div className="matchday-modal-header">
          <h2>Matchday</h2>
          <div className="matchday-date">{formatDate(gameState.currentDate)}</div>
          <div className="matchday-gameweek">Gameweek {gameState.currentGameweek}</div>
          {isSheffieldRules && <div className="rules-indicator">Sheffield Rules 1867</div>}
        </div>

        {/* Main Content */}
        <div className="matchday-modal-content">
          {/* Left: Match List */}
          <div className="matchday-matches-panel">
            <h3>Today's Matches</h3>
            <div className="matches-list">
              {matchesForDay.map((match) => {
                const status = getMatchStatus(match.id)
                const isUserMatch = match.homeTeamId === gameState.userClubId ||
                                   match.awayTeamId === gameState.userClubId

                return (
                  <div
                    key={match.id}
                    className={`match-card ${isUserMatch ? 'user-team' : ''} ${status?.status || ''}`}
                  >
                    <div className="match-teams">
                      <span className={`team-name ${match.homeTeamId === gameState.userClubId ? 'highlight' : ''}`}>
                        {getClubName(match.homeTeamId)}
                      </span>
                      <span className="match-vs">vs</span>
                      <span className={`team-name ${match.awayTeamId === gameState.userClubId ? 'highlight' : ''}`}>
                        {getClubName(match.awayTeamId)}
                      </span>
                    </div>

                    {status?.status === 'pending' && isUserMatch && (
                      <button
                        className="play-match-btn"
                        onClick={() => simulateMatch(match.id, true)}
                      >
                        Play Match
                      </button>
                    )}

                    {status?.status === 'simulating' && (
                      <div className="match-status simulating">
                        {isSheffieldRules && isUserMatch ? 'Kick-off...' : 'Simulating...'}
                      </div>
                    )}

                    {status?.status === 'complete' && status.result && (
                      <div className="match-score">
                        <span className="score-value">{status.result.homeScore}</span>
                        <span className="score-separator">-</span>
                        <span className="score-value">{status.result.awayScore}</span>
                        <span className="match-complete-icon">✓</span>
                      </div>
                    )}

                    {status?.status === 'error' && (
                      <div className="match-error">
                        <span>Error: {status.error}</span>
                        <button
                          className="retry-btn"
                          onClick={() => handleRetry(match.id)}
                        >
                          Retry
                        </button>
                      </div>
                    )}
                  </div>
                )
              })}
            </div>
          </div>

          {/* Right: Context Panel */}
          <div className="matchday-context-panel">
            <div className="context-section">
              <h3>Your Team</h3>
              {userStanding && (
                <div className="user-team-status">
                  <div className="status-row">
                    <span className="label">Position:</span>
                    <span className="value">#{userStanding.position}</span>
                  </div>
                  <div className="status-row">
                    <span className="label">Points:</span>
                    <span className="value">{userStanding.points}</span>
                  </div>
                  <div className="status-row">
                    <span className="label">Record:</span>
                    <span className="value">
                      {userStanding.won}W-{userStanding.drawn}D-{userStanding.lost}L
                    </span>
                  </div>
                </div>
              )}
            </div>

            <div className="context-section">
              <h3>League Standings</h3>
              <div className="standings-mini">
                {topStandings.map((standing) => (
                  <div
                    key={standing.clubId}
                    className={`standing-row ${standing.clubId === gameState.userClubId ? 'user-club' : ''}`}
                  >
                    <span className="standing-pos">{standing.position}</span>
                    <span className="standing-club">{getClubName(standing.clubId)}</span>
                    <span className="standing-pts">{standing.points}</span>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>

        {/* Footer */}
        <div className="matchday-modal-footer">
          <div className="progress-container">
            <div className="progress-label">
              {completedCount} of {matchesForDay.length} matches complete
            </div>
            <div className="progress-bar">
              <div
                className="progress-fill"
                style={{ width: `${progressPercentage}%` }}
              />
            </div>
          </div>
          <button
            className="continue-btn"
            onClick={handleContinue}
            disabled={!allComplete}
          >
            {allComplete ? 'Continue' : 'Waiting for matches...'}
          </button>
        </div>
      </div>
    </div>
  )
}
