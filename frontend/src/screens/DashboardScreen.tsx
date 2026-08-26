import React, { useState, useEffect } from 'react'
import { invoke } from '../utils/tauriInvoke'
import { GameState, Match } from '../types/GameState'
import { PlayerCard } from '../components/PlayerCard'
import { SaveLoadModal } from '../components/SaveLoadModal'

// SVG Icons
const SaveIcon = () => <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M17 3H5c-1.11 0-2 .9-2 2v14c0 1.1.89 2 2 2h14c1.1 0 2-.9 2-2V7l-4-4zm-5 16c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3zm3-10H5V5h10v4z"/></svg>
const LoadIcon = () => <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/></svg>
const SquadIcon = () => <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><circle cx="12" cy="8" r="4"/><path d="M12 14c-4.418 0-8 1.79-8 4v2h16v-2c0-2.21-3.582-4-8-4z"/><path d="M18 9c1.1 0 2-0.9 2-2s-0.9-2-2-2-2 0.9-2 2 0.9 2 2 2z"/><path d="M20 15.5c1.38 0 2.5 1.12 2.5 2.5v1.5h-3v-1.5c0-1.38 1.12-2.5 2.5-2.5z"/></svg>
const StandingsIcon = () => <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M3 13h2v8H3zm4-8h2v16H7zm4-2h2v18h-2zm4 4h2v14h-2zm4-2h2v16h-2z"/></svg>
const CalendarIcon = () => <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><rect x="3" y="4" width="18" height="18" rx="2"/><path d="M16 2v4M8 2v4M3 10h18" stroke="currentColor" fill="none" strokeWidth="2"/></svg>
const TrophyIcon = () => <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M6 9c0-1 1-2 2-2h8c1 0 2 1 2 2v3H6V9zm8-5c0-1-1-2-2-2s-2 1-2 2v1h4V4zm6 4c1 0 2 1 2 2v8c0 1-1 2-2 2h-1v2h-2v-2H9v2H7v-2H6c-1 0-2-1-2-2v-8c0-1 1-2 2-2h1V4c0-1 1-2 2-2s2 1 2 2v1h4V4c0-1 1-2 2-2s2 1 2 2v1h1z"/></svg>
const BallIcon = () => <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><circle cx="12" cy="12" r="10" stroke="currentColor" fill="none" strokeWidth="2"/><path d="M12 2v3M12 19v3M22 12h-3M5 12H2" stroke="currentColor" strokeWidth="2"/></svg>
const NewsIcon = () => <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><rect x="3" y="5" width="18" height="14" rx="1"/><path d="M3 10h18M3 14h10" stroke="currentColor" fill="none" strokeWidth="1.5"/></svg>

interface DashboardScreenProps {
  gameState: GameState
  setGameState: (state: GameState | ((prev: GameState) => GameState)) => void
  theme: string
  onNavigate?: (screen: string) => void
}

// Historical events removed - events now come from game state

function formatDate(dateStr: string) {
  const date = new Date(dateStr + 'T00:00:00')
  return date.toLocaleDateString('en-GB', { weekday: 'short', year: 'numeric', month: 'short', day: 'numeric' })
}

export function DashboardScreen({
  gameState,
  setGameState,
  theme,
  onNavigate,
}: DashboardScreenProps) {
  const [upcomingFixtures, setUpcomingFixtures] = useState<Match[]>([])
  const [saveLoadModal, setSaveLoadModal] = useState<{ mode: 'save' | 'load' } | null>(null)

  useEffect(() => {
    async function loadFixtures() {
      try {
        const fixtures = await invoke('get_upcoming_fixtures', { game: gameState, days: 14 })
        setUpcomingFixtures(fixtures as Match[])
      } catch (error) {
        console.error('Failed to load fixtures:', error)
        // Fall back to filtering matches if command not available
        const upcoming = gameState.matches.filter(m => !m.played).slice(0, 10)
        setUpcomingFixtures(upcoming)
      }
    }
    loadFixtures()
  }, [gameState.currentDate])

  // Get top 3 players by rating from user's club
  const topPlayers = gameState.players
    .filter(p => p.clubId === gameState.userClubId)
    .sort((a, b) => b.overallRating - a.overallRating)
    .slice(0, 3)

  // Get upcoming matches for user's club
  const upcomingMatches = gameState.matches
    .filter(m => !m.played && (m.homeClubId === gameState.userClubId || m.awayClubId === gameState.userClubId))
    .slice(0, 3)

  const userClub = gameState.clubs.find(c => c.id === gameState.userClubId)
  const userStanding = gameState.standings.find(s => s.clubId === gameState.userClubId)

  // Generate news feed from recent events and match results
  const generateNewsFeed = () => {
    const newsItems: Array<{
      id: string
      date: string
      title: string
      description: string
      type: 'match' | 'event'
      priority: number
    }> = []

    // Add pending events (highest priority - these should appear first)
    gameState.pendingEvents
      .filter(e => e.eventType.type !== 'Match')
      .forEach(event => {
        let title = event.eventType.type
        let description = `An important ${event.eventType.type.toLowerCase()} requires your attention`

        // Extract title and description from HistoricalAnnouncement events
        if (event.eventType.type === 'HistoricalAnnouncement' && event.eventType.data) {
          title = event.eventType.data.title || title
          description = event.eventType.data.description || description
        }

        newsItems.push({
          id: event.id,
          date: event.date,
          title,
          description,
          type: 'event' as const,
          priority: 1, // Highest priority
        })
      })

    // Add recent match results (lower priority)
    const recentMatches = gameState.matches
      .filter(m => m.played)
      .sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime())
      .slice(0, 3)

    recentMatches.forEach(match => {
      const home = gameState.clubs.find(c => c.id === match.homeClubId)
      const away = gameState.clubs.find(c => c.id === match.awayClubId)
      newsItems.push({
        id: match.id,
        date: match.date,
        title: `Match Result`,
        description: `${home?.name || 'Unknown'} ${match.homeScore}-${match.awayScore} ${away?.name || 'Unknown'}`,
        type: 'match' as const,
        priority: 2, // Lower priority
      })
    })

    // Sort by priority first (ascending), then by date (descending)
    return newsItems
      .sort((a, b) => {
        // First sort by priority (lower number = higher priority)
        if (a.priority !== b.priority) {
          return a.priority - b.priority
        }
        // Then sort by date (newer first)
        return new Date(b.date).getTime() - new Date(a.date).getTime()
      })
      .slice(0, 5)
  }

  const newsFeed = generateNewsFeed()

  // Calculate season stats
  const totalMatches = gameState.matches.filter(m => m.played).length
  const totalGoalsFor = gameState.standings.find(s => s.clubId === gameState.userClubId)?.goalsFor || 0
  const totalGoalsAgainst = gameState.standings.find(s => s.clubId === gameState.userClubId)?.goalsAgainst || 0
  const goalDifference = totalGoalsFor - totalGoalsAgainst

  return (
    <div className="dashboard-screen">
      <div className="dashboard-header">
        <div className="header-top">
          <h1>Dashboard</h1>
          <div className="quick-actions">
            <button className="quick-btn" onClick={() => setSaveLoadModal({ mode: 'save' })} title="Save your game">
              💾 Save
            </button>
            <button className="quick-btn" onClick={() => setSaveLoadModal({ mode: 'load' })} title="Load a saved game">
              📂 Load
            </button>
            {onNavigate && (
              <>
                <button className="quick-btn" onClick={() => onNavigate('squad')} title="View your squad">
                  👥 Squad
                </button>
                <button className="quick-btn" onClick={() => onNavigate('standings')} title="View league standings">
                  📊 Standings
                </button>
              </>
            )}
          </div>
        </div>
        <div className="game-info-bar">
          <span className="info-item">📅 {formatDate(gameState.currentDate)}</span>
          <span className="info-item">🗂️ Gameweek {gameState.currentGameweek}/38</span>
          <span className="info-item">🏆 Season {gameState.season}</span>
        </div>
      </div>

      {historicalEvent && (
        <div className="historical-event">
          <div className="event-icon">📜</div>
          <div className="event-content">
            <h2>{historicalEvent.title}</h2>
            <p>{historicalEvent.description}</p>
          </div>
        </div>
      )}

      <div className="dashboard-layout">
        {/* Left Column */}
        <div className="dashboard-left">
          {/* Club Status */}
          {userClub && userStanding && (
            <div className="dashboard-card club-card">
              <h2>{userClub.name}</h2>
              <div className="club-status-grid">
                <div className="status-box">
                  <div className="status-label">Position</div>
                  <div className="status-value position">#{userStanding.position}</div>
                </div>
                <div className="status-box">
                  <div className="status-label">Points</div>
                  <div className="status-value points">{userStanding.points}</div>
                </div>
                <div className="status-box">
                  <div className="status-label">Record</div>
                  <div className="status-value record">
                    <span className="wins">{userStanding.won}W</span>
                    <span className="draws">{userStanding.drawn}D</span>
                    <span className="losses">{userStanding.lost}L</span>
                  </div>
                </div>
                <div className="status-box">
                  <div className="status-label">Goal Diff</div>
                  <div className={`status-value goal-diff ${goalDifference > 0 ? 'positive' : goalDifference < 0 ? 'negative' : 'neutral'}`}>
                    {goalDifference > 0 ? '+' : ''}{goalDifference}
                  </div>
                </div>
              </div>
            </div>
          )}

          {/* Season Stats */}
          <div className="dashboard-card stats-card">
            <h3>Season Stats</h3>
            <div className="stats-table">
              <div className="stat-row">
                <span className="stat-label">Matches Played</span>
                <span className="stat-value">{totalMatches}</span>
              </div>
              <div className="stat-row">
                <span className="stat-label">Goals For</span>
                <span className="stat-value positive">{totalGoalsFor}</span>
              </div>
              <div className="stat-row">
                <span className="stat-label">Goals Against</span>
                <span className="stat-value negative">{totalGoalsAgainst}</span>
              </div>
              <div className="stat-row">
                <span className="stat-label">Goal Average</span>
                <span className="stat-value">{(totalGoalsFor / Math.max(totalMatches, 1)).toFixed(2)}</span>
              </div>
            </div>
          </div>

          {/* News Feed */}
          {newsFeed.length > 0 && (
            <div className="dashboard-card news-card">
              <h3>📰 Recent News</h3>
              <div className="news-items">
                {newsFeed.map((item) => (
                  <div key={item.id} className={`news-item news-${item.type}`}>
                    <div className="news-header">
                      <span className="news-title">{item.title}</span>
                      <span className="news-date">{formatDate(item.date)}</span>
                    </div>
                    <p className="news-description">{item.description}</p>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>

        {/* Right Column */}
        <div className="dashboard-right">
          {/* Top Players */}
          {topPlayers.length > 0 && (
            <div className="dashboard-card">
              <h3>⭐ Top Players</h3>
              <div className="players-list">
                {topPlayers.map((player) => (
                  <div key={player.id} className="player-row">
                    <div className="player-info">
                      <div className="player-name">{player.name}</div>
                      <div className="player-pos">{player.position}</div>
                    </div>
                    <div className="player-rating">{player.overallRating.toFixed(1)}</div>
                  </div>
                ))}
              </div>
            </div>
          )}

          {/* Upcoming Matches */}
          {upcomingMatches.length > 0 && (
            <div className="dashboard-card">
              <h3>🎯 Your Upcoming Matches</h3>
              <div className="matches-list">
                {upcomingMatches.map((match) => {
                  const homeClub = gameState.clubs.find(c => c.id === match.homeClubId)
                  const awayClub = gameState.clubs.find(c => c.id === match.awayClubId)
                  const isHome = match.homeClubId === gameState.userClubId
                  return (
                    <div key={match.id} className="match-preview">
                      <div className="match-date">{formatDate(match.date)}</div>
                      <div className="match-teams">
                        <span className={`team ${isHome ? 'home' : ''}`}>{homeClub?.name}</span>
                        <span className="vs">vs</span>
                        <span className={`team ${!isHome ? 'away' : ''}`}>{awayClub?.name}</span>
                      </div>
                    </div>
                  )
                })}
              </div>
            </div>
          )}

          {/* Upcoming Fixtures Preview */}
          {upcomingFixtures.length > 0 && (
            <div className="dashboard-card">
              <h3>📅 Next Gameweek Fixtures</h3>
              <div className="fixtures-preview">
                {upcomingFixtures.slice(0, 5).map((match) => {
                  const homeClub = gameState.clubs.find(c => c.id === match.homeClubId)
                  const awayClub = gameState.clubs.find(c => c.id === match.awayClubId)
                  return (
                    <div key={match.id} className="fixture-row">
                      <span className="fixture-teams">
                        <span className="team-short">{homeClub?.name.substring(0, 3).toUpperCase()}</span>
                        <span className="vs">-</span>
                        <span className="team-short">{awayClub?.name.substring(0, 3).toUpperCase()}</span>
                      </span>
                      <span className="fixture-gw">GW {match.gameweek}</span>
                    </div>
                  )
                })}
              </div>
            </div>
          )}
        </div>
      </div>

      {/* Save/Load Modal */}
      {saveLoadModal && (
        <SaveLoadModal
          gameState={gameState}
          mode={saveLoadModal.mode}
          onClose={() => setSaveLoadModal(null)}
          onSave={setGameState}
          onLoad={setGameState}
        />
      )}
    </div>
  )
}

export default DashboardScreen
