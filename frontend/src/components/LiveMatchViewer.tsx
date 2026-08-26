import React, { useState, useEffect, useRef } from 'react'
import { listen } from '@tauri-apps/api/event'
import '../styles/live-match.css'

interface MatchEvent {
  minute: number
  event_type: string
  description?: string
  player_name?: string
  team?: string
  [key: string]: any
}

interface VisualState {
  home_score: number
  away_score: number
  home_rouges?: number
  away_rouges?: number
  possession: {
    home: number
    away: number
  }
  [key: string]: any
}

interface MatchStatistics {
  shots: { home: number; away: number }
  shots_on_target: { home: number; away: number }
  corners: { home: number; away: number }
  fouls: { home: number; away: number }
  possession: { home: number; away: number }
  rouges?: { home: number; away: number }
  [key: string]: any
}

interface MatchUpdate {
  match_id: string
  minute: number
  event: MatchEvent
  visual_state: VisualState
  statistics: MatchStatistics
}

interface LiveMatchViewerProps {
  matchId: string
  homeTeam: string
  awayTeam: string
  onMatchComplete: (homeScore: number, awayScore: number, homeRouges?: number, awayRouges?: number) => void
  onClose?: () => void
  ruleYear?: number
}

export function LiveMatchViewer({
  matchId,
  homeTeam,
  awayTeam,
  onMatchComplete,
  onClose,
  ruleYear = 1867
}: LiveMatchViewerProps) {
  const [commentary, setCommentary] = useState<MatchEvent[]>([])
  const [currentMinute, setCurrentMinute] = useState(0)
  const [visualState, setVisualState] = useState<VisualState>({
    home_score: 0,
    away_score: 0,
    home_rouges: 0,
    away_rouges: 0,
    possession: { home: 50, away: 50 }
  })
  const [statistics, setStatistics] = useState<MatchStatistics>({
    shots: { home: 0, away: 0 },
    shots_on_target: { home: 0, away: 0 },
    corners: { home: 0, away: 0 },
    fouls: { home: 0, away: 0 },
    possession: { home: 50, away: 50 },
    rouges: { home: 0, away: 0 }
  })
  const [matchComplete, setMatchComplete] = useState(false)
  const [selectedTab, setSelectedTab] = useState<'commentary' | 'stats'>('commentary')
  const commentaryRef = useRef<HTMLDivElement>(null)

  // Show rouges only for 1862-1868 (rouge era)
  const showRouges = ruleYear >= 1862 && ruleYear <= 1868

  useEffect(() => {
    let unlisten: (() => void) | undefined

    // Subscribe to match updates
    const setupListener = async () => {
      unlisten = await listen<MatchUpdate>('match_update', (event) => {
        const update = event.payload

        if (update.match_id !== matchId) return

        // Update commentary
        setCommentary(prev => [...prev, update.event])
        setCurrentMinute(update.minute)
        setVisualState(update.visual_state)
        setStatistics(update.statistics)

        // Check for match completion (90 minutes)
        if (update.minute >= 90 && !matchComplete) {
          setMatchComplete(true)
          setTimeout(() => {
            onMatchComplete(
              update.visual_state.home_score,
              update.visual_state.away_score,
              update.visual_state.home_rouges,
              update.visual_state.away_rouges
            )
          }, 3000) // Give 3 seconds to view final stats
        }
      })
    }

    setupListener()

    return () => {
      if (unlisten) {
        unlisten()
      }
    }
  }, [matchId, onMatchComplete, matchComplete])

  // Auto-scroll commentary to bottom
  useEffect(() => {
    if (commentaryRef.current) {
      commentaryRef.current.scrollTop = commentaryRef.current.scrollHeight
    }
  }, [commentary])

  const formatEventDescription = (event: MatchEvent): string => {
    if (event.description) return event.description

    // Format based on event type
    switch (event.event_type) {
      case 'goal':
        return `⚽ GOAL! ${event.player_name} scores for ${event.team}!`
      case 'rouge':
        return `○ Rouge! ${event.player_name} for ${event.team}`
      case 'shot':
        return `🎯 Shot by ${event.player_name} (${event.team})`
      case 'save':
        return `🧤 Save by ${event.player_name} (${event.team})`
      case 'corner':
        return `⚐ Corner for ${event.team}`
      case 'foul':
        return `🚫 Foul by ${event.player_name} (${event.team})`
      case 'kickoff':
        return `⚽ Kick-off! ${event.team} to start`
      case 'halftime':
        return `⏸️  HALF TIME`
      case 'fulltime':
        return `🏁 FULL TIME`
      default:
        return event.event_type
    }
  }

  const getEventClass = (eventType: string): string => {
    switch (eventType) {
      case 'goal': return 'event-goal'
      case 'rouge': return 'event-rouge'
      case 'shot': return 'event-shot'
      case 'save': return 'event-save'
      case 'halftime':
      case 'fulltime': return 'event-major'
      default: return 'event-normal'
    }
  }

  return (
    <div className="live-match-viewer">
      {/* Match Header */}
      <div className="match-header">
        <div className="match-title">
          <h2>{homeTeam} vs {awayTeam}</h2>
          <div className="match-minute">
            {matchComplete ? 'FT' : `${currentMinute}'`}
          </div>
        </div>

        {/* Score Display */}
        <div className="score-display">
          <div className="team-score">
            <div className="team-name">{homeTeam}</div>
            <div className="score">{visualState.home_score}</div>
            {showRouges && visualState.home_rouges !== undefined && (
              <div className="rouges">({visualState.home_rouges})</div>
            )}
          </div>
          <div className="score-separator">-</div>
          <div className="team-score">
            <div className="score">{visualState.away_score}</div>
            <div className="team-name">{awayTeam}</div>
            {showRouges && visualState.away_rouges !== undefined && (
              <div className="rouges">({visualState.away_rouges})</div>
            )}
          </div>
        </div>

        {onClose && (
          <button className="close-btn" onClick={onClose}>✕</button>
        )}
      </div>

      {/* Tab Navigation */}
      <div className="match-tabs">
        <button
          className={`tab ${selectedTab === 'commentary' ? 'active' : ''}`}
          onClick={() => setSelectedTab('commentary')}
        >
          Commentary
        </button>
        <button
          className={`tab ${selectedTab === 'stats' ? 'active' : ''}`}
          onClick={() => setSelectedTab('stats')}
        >
          Statistics
        </button>
      </div>

      {/* Commentary Tab */}
      {selectedTab === 'commentary' && (
        <div className="commentary-container" ref={commentaryRef}>
          {commentary.length === 0 ? (
            <div className="no-commentary">Waiting for kick-off...</div>
          ) : (
            <div className="commentary-feed">
              {commentary.map((event, index) => (
                <div key={index} className={`commentary-event ${getEventClass(event.event_type)}`}>
                  <span className="event-minute">{event.minute}'</span>
                  <span className="event-text">{formatEventDescription(event)}</span>
                </div>
              ))}
            </div>
          )}
        </div>
      )}

      {/* Statistics Tab */}
      {selectedTab === 'stats' && (
        <div className="statistics-container">
          <div className="stats-section">
            <div className="stat-row">
              <div className="stat-value">{statistics.shots.home}</div>
              <div className="stat-label">Shots</div>
              <div className="stat-value">{statistics.shots.away}</div>
            </div>

            <div className="stat-row">
              <div className="stat-value">{statistics.shots_on_target.home}</div>
              <div className="stat-label">On Target</div>
              <div className="stat-value">{statistics.shots_on_target.away}</div>
            </div>

            <div className="stat-row">
              <div className="stat-value">{statistics.corners.home}</div>
              <div className="stat-label">Corners</div>
              <div className="stat-value">{statistics.corners.away}</div>
            </div>

            <div className="stat-row">
              <div className="stat-value">{statistics.fouls.home}</div>
              <div className="stat-label">Fouls</div>
              <div className="stat-value">{statistics.fouls.away}</div>
            </div>

            {showRouges && statistics.rouges && (
              <div className="stat-row highlight">
                <div className="stat-value">{statistics.rouges.home}</div>
                <div className="stat-label">Rouges</div>
                <div className="stat-value">{statistics.rouges.away}</div>
              </div>
            )}

            {/* Possession Bar */}
            <div className="possession-section">
              <div className="possession-label">Possession</div>
              <div className="possession-bar">
                <div
                  className="possession-home"
                  style={{ width: `${statistics.possession.home}%` }}
                >
                  <span>{statistics.possession.home}%</span>
                </div>
                <div
                  className="possession-away"
                  style={{ width: `${statistics.possession.away}%` }}
                >
                  <span>{statistics.possession.away}%</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Match Status */}
      {matchComplete && (
        <div className="match-complete-overlay">
          <div className="match-complete-message">
            <h2>FULL TIME</h2>
            <div className="final-score">
              <span>{homeTeam} {visualState.home_score}</span>
              <span>-</span>
              <span>{visualState.away_score} {awayTeam}</span>
            </div>
            {showRouges && (
              <div className="final-rouges">
                (Rouges: {visualState.home_rouges} - {visualState.away_rouges})
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  )
}

export default LiveMatchViewer
