import React, { useState } from 'react'
import { GameState, Match } from '../types/GameState'
import { invoke } from '../utils/tauriInvoke'
import { SaveLoadModal } from '../components/SaveLoadModal'
import { ComingSoonModal } from '../components/ComingSoonModal'
import { TrainingPanelModal } from '../components/TrainingPanelModal'
import { CommitteePanelModal } from '../components/CommitteePanelModal'
import { CoopPanelModal } from '../components/CoopPanelModal'
import { LetterDisplayModal } from '../components/LetterDisplayModal'
import { MatchdayModeModal } from '../components/MatchdayModeModal'
import { SquadScreen } from './SquadScreen'
import { ClubsScreen } from './ClubsScreen'
import '../styles/gameplay.css'

// SVG Icons
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

const TacticsIcon = () => (
  <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
    <circle cx="12" cy="4" r="2"/>
    <circle cx="6" cy="10" r="2"/>
    <circle cx="18" cy="10" r="2"/>
    <circle cx="6" cy="18" r="2"/>
    <circle cx="12" cy="18" r="2"/>
    <circle cx="18" cy="18" r="2"/>
    <line x1="12" y1="6" x2="6" y2="9" stroke="currentColor" strokeWidth="1"/>
    <line x1="12" y1="6" x2="18" y2="9" stroke="currentColor" strokeWidth="1"/>
    <line x1="6" y1="12" x2="6" y2="16" stroke="currentColor" strokeWidth="1"/>
    <line x1="12" y1="12" x2="12" y2="16" stroke="currentColor" strokeWidth="1"/>
    <line x1="18" y1="12" x2="18" y2="16" stroke="currentColor" strokeWidth="1"/>
  </svg>
)

const TrainingIcon = () => (
  <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
    <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm3.5-9c.83 0 1.5-.67 1.5-1.5S16.33 8 15.5 8 14 8.67 14 9.5s.67 1.5 1.5 1.5zm-7 0c.83 0 1.5-.67 1.5-1.5S9.33 8 8.5 8 7 8.67 7 9.5 7.67 11 8.5 11zm3.5 6.5c2.33 0 4.31-1.46 5.11-3.5H6.89c.8 2.04 2.78 3.5 5.11 3.5z"/>
  </svg>
)

const ClubsIcon = () => (
  <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
    <path d="M12 2L2 7v10c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V7l-10-5z"/>
    <path d="M10 17h4v-6h-4v6zm6-10H8v2h8v-2z"/>
  </svg>
)

const TrophyAchievementIcon = () => (
  <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
    <path d="M6 9c0-1 1-2 2-2h8c1 0 2 1 2 2v3H6V9zm8-5c0-1-1-2-2-2s-2 1-2 2v1h4V4zm6 4c1 0 2 1 2 2v8c0 1-1 2-2 2h-1v2h-2v-2H9v2H7v-2H6c-1 0-2-1-2-2v-8c0-1 1-2 2-2h1V4c0-1 1-2 2-2s2 1 2 2v1h4V4c0-1 1-2 2-2s2 1 2 2v1h1z"/>
  </svg>
)

const SquadIcon = () => (
  <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
    <circle cx="12" cy="8" r="4"/>
    <path d="M12 14c-4.418 0-8 1.79-8 4v2h16v-2c0-2.21-3.582-4-8-4z"/>
    <path d="M18 9c1.1 0 2-0.9 2-2s-0.9-2-2-2-2 0.9-2 2 0.9 2 2 2z"/>
    <path d="M20 15.5c1.38 0 2.5 1.12 2.5 2.5v1.5h-3v-1.5c0-1.38 1.12-2.5 2.5-2.5z"/>
  </svg>
)

const HomeIcon = () => (
  <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
    <path d="M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z"/>
  </svg>
)

const AwayIcon = () => (
  <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor">
    <path d="M7 18c-1.1 0-1.99.9-1.99 2S5.9 22 7 22s2-.9 2-2-0.9-2-2-2zm10-9l-1.44 2H6v9h12v-9zm3-7H4c-1.1 0-2 .9-2 2v11h2v9h3v-9h10v9h3v-9h2V4c0-1.1-.9-2-2-2zm-1 3.5h-8v2h8v-2z"/>
  </svg>
)

interface GameplayScreenProps {
  gameState: GameState
  setGameState: (state: GameState | ((prev: GameState) => GameState)) => void
  onBack: () => void
}

export function GameplayScreen({ gameState, setGameState, onBack }: GameplayScreenProps) {
  const [simulatingMatchId, setSimulatingMatchId] = useState<string | null>(null)
  const [saveLoadModal, setSaveLoadModal] = useState<{ mode: 'save' | 'load' } | null>(null)
  const [showSquad, setShowSquad] = useState(false)
  const [showClubs, setShowClubs] = useState(false)
  const [showTrialPanel, setShowTrialPanel] = useState(false)
  const [trialPlayers, setTrialPlayers] = useState<any[]>([])
  const [showTrialDatePicker, setShowTrialDatePicker] = useState(false)
  const [selectedTrialDate, setSelectedTrialDate] = useState('')
  const [showLeagueTable, setShowLeagueTable] = useState(false)
  const [showSettings, setShowSettings] = useState(false)
  const [settingsSubmenu, setSettingsSubmenu] = useState<string | null>(null)
  const [gameSpeed, setGameSpeed] = useState(1)
  const [audioEnabled, setAudioEnabled] = useState(true)
  const [language, setLanguage] = useState('English')
  const [comingSoonModal, setComingSoonModal] = useState<{ title: string; description: string } | null>(null)
  const [showTraining, setShowTraining] = useState(false)
  const [showCommittee, setShowCommittee] = useState(false)
  const [showCoop, setShowCoop] = useState(false)
  const [selectedDivision, setSelectedDivision] = useState<string | null>(null)
  const [divisionStandings, setDivisionStandings] = useState<any>(null)
  const [selectedTableClubId, setSelectedTableClubId] = useState<string | null>(null)
  const [selectedClubStanding, setSelectedClubStanding] = useState<any>(null)
  const [selectedNewsEvent, setSelectedNewsEvent] = useState<any>(null)
  const [userDivisionInfo, setUserDivisionInfo] = useState<any>(null)
  const [clubActionSelected, setClubActionSelected] = useState<string | null>(null)
  const [newsItems, setNewsItems] = useState<any[]>([])
  const [letterModal, setLetterModal] = useState<{
    senderClubName: string
    senderGroundName?: string
    senderCity?: string
    senderRegion?: string
    responseText: string
    responseDate?: string
    accepted: boolean
    newsId?: string
    relatedInvitationId?: string
  } | null>(null)
  const [showMatchdayModal, setShowMatchdayModal] = useState(false)

  // Get matches for current gameweek AND friendly matches (gameweek = 0) for current date
  const gameweekMatches = gameState.matches.filter(m =>
    m.gameweek === gameState.currentGameweek ||
    (m.gameweek === 0 && m.date === gameState.currentDate)
  )

  // Check if there are unplayed matches today
  const hasMatchesToday = gameweekMatches.some(m => !m.played)

  // Get user's club
  const userClub = gameState.clubs.find(c => c.id === gameState.userClubId)

  // Get user's standing
  const userStanding = gameState.standings.find(s => s.clubId === gameState.userClubId)

  // Get top 5 standings
  const topStandings = gameState.standings.slice(0, 5)

  // Get pending events (non-match events)
  let pendingEvents = gameState.pendingEvents.filter(e => e.eventType.type !== 'Match' && !e.processed)

  // Convert news items to event format
  const newsEvents = newsItems.map(news => ({
    id: news.id,
    date: news.publish_date,
    eventType: {
      type: news.article_type,
      data: {
        headline: news.headline,
        title: news.headline,  // Add title field for display compatibility
        body: news.body_text,
        hasActionButton: news.has_action_button,
        actionButtonText: news.action_button_text,
        actionType: news.action_type,
        actionData: news.action_data,
        isImportant: news.is_important,
        relatedInvitationId: news.related_invitation_id,
      }
    },
    processed: news.is_read,
    requiresUserAction: news.requires_action,
    result: null,
  }))

  console.log(`[NEWS] Created ${newsEvents.length} news events from ${newsItems.length} news items`)

  // Merge news events with pending events
  pendingEvents = [...newsEvents, ...pendingEvents]
  console.log(`[NEWS] Total pending events after merge: ${pendingEvents.length}`)

  // Load ALL news items (not just current date - show historical events)
  // Reload whenever game state changes (e.g., after advancing days or sending challenge letters)
  React.useEffect(() => {
    async function loadNews() {
      try {
        console.log(`[NEWS] Loading all news items for date: ${gameState.currentDate}`)
        const items = await invoke<any[]>('get_all_news', { game: gameState })
        console.log(`[NEWS] Loaded ${items?.length || 0} news items:`, items)
        setNewsItems(items || [])
      } catch (error) {
        console.error('Failed to load news:', error)
        setNewsItems([])
      }
    }
    loadNews()
  }, [gameState])

  // Load standing for selected club
  React.useEffect(() => {
    if (selectedTableClubId) {
      const standing = gameState.standings.find(s => s.clubId === selectedTableClubId)
      setSelectedClubStanding(standing)
    } else {
      setSelectedClubStanding(null)
    }
  }, [selectedTableClubId, gameState.standings])

  // Load user's division info on mount
  React.useEffect(() => {
    if (gameState.gameMode === 'sheffield-hallamshire-league' && gameState.userClubId) {
      invoke<any>('get_club_division_info', {
        clubId: gameState.userClubId
      })
        .then((divInfo) => {
          setUserDivisionInfo(divInfo)
        })
        .catch((error: any) => console.error('Failed to load division info:', error))
    }
  }, [gameState.userClubId, gameState.gameMode])

  async function simulateMatch(matchId: string) {
    // Find the match to get club IDs
    const match = gameState.matches.find(m => m.id === matchId)
    if (!match) {
      throw new Error('Match not found')
    }

    // Check if this is a user team match
    const isUserMatch = match.homeTeamId === gameState.userClubId || match.awayTeamId === gameState.userClubId

    if (isUserMatch) {
      // Open matchday modal for user team matches
      setShowMatchdayModal(true)
      return
    }

    setSimulatingMatchId(matchId)
    try {
      // Use batch simulation with player attributes
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
    } catch (error) {
      console.error('Failed to simulate match:', error)
    } finally {
      setSimulatingMatchId(null)
    }
  }

  const getClubName = (clubId: string) => {
    const club = gameState.clubs.find(c => c.id === clubId)
    if (club?.name) return club.name

    // Handle undefined/null clubId
    if (!clubId) return '?'

    // Fallback: parse name from clubId (e.g., "sheffield_fc_1857" -> "Sheffield FC")
    const parsed = clubId
      .split('_')
      .slice(0, -1) // remove year
      .map(word => word.charAt(0).toUpperCase() + word.slice(1))
      .join(' ')

    return parsed || '?'
  }

  const formatDate = (dateStr: string) => {
    const date = new Date(dateStr)
    return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' })
  }

  const getSeasonString = () => {
    const year = gameState.season
    return `${year}-${String(year + 1).slice(-2)}`
  }

  const continueGame = async () => {
    try {
      // Check if there are unplayed matches today
      const matchesToday = gameState.matches.filter(m =>
        (m.gameweek === gameState.currentGameweek || (m.gameweek === 0 && m.date === gameState.currentDate)) &&
        !m.played
      )

      if (matchesToday.length > 0) {
        // Don't advance - there are matches to play
        alert('You have matches to play today! Please simulate them before continuing.')
        return
      }

      // Call advance_day command which processes events
      const result = await invoke<any>('advance_day', { game: gameState })

      // Reload game state to get updated matches and standings
      const updatedGame = await invoke<GameState>('get_current_game')
      setGameState(updatedGame)

      // Check if there are any events requiring user action
      if (result.require_user_action && result.require_user_action.length > 0) {
        // Check if any of the events is a TrialSession
        const trialEvent = result.require_user_action.find((e: any) => e.eventType?.type === 'TrialSession')

        if (trialEvent) {
          // Load trial players and show the trial window
          const year = new Date(updatedGame.currentDate).getFullYear()
          const players = await invoke<any[]>('generate_trial_players', {
            clubId: updatedGame.userClubId,
            currentYear: year
          })
          setTrialPlayers(players)
          setShowTrialPanel(true)

          // The event will be marked as processed when the user closes the trial window
          // For now, mark it as processed
          await invoke<GameState>('complete_user_event', {
            game: updatedGame,
            eventId: trialEvent.id,
            userResponse: { completed: true }
          })
        }
      }
    } catch (error) {
      console.error('Failed to advance day:', error)
    }
  }

  const handleDivisionSelect = async (divisionId: string) => {
    setSelectedDivision(divisionId)
    try {
      // Check if this is a grouped division (5, 6 or 7) that needs to fetch all regional tiers
      if (divisionId === 'div-5a' || divisionId === 'div-6a' || divisionId === 'div-7a') {
        let divisionsToFetch: string[] = []

        if (divisionId === 'div-5a') {
          divisionsToFetch = ['div-5a', 'div-5b']
        } else {
          const level = divisionId.startsWith('div-6') ? '6' : '7'
          divisionsToFetch = [
            `div-${level}a`,
            `div-${level}b`,
            `div-${level}c`,
            `div-${level}d`
          ]
        }

        const allClubs: any = {}
        for (const divId of divisionsToFetch) {
          const standings = await invoke<Array<any>>('get_division_standings', {
            divisionId: divId,
            season: gameState.season || 1888
          })
          allClubs[divId] = standings
        }
        setDivisionStandings(allClubs)
      } else {
        const standings = await invoke<Array<any>>('get_division_standings', {
          divisionId: divisionId,
          season: gameState.season || 1888
        })
        setDivisionStandings(standings)
      }
    } catch (error) {
      console.error('Failed to load division standings:', error)
      setDivisionStandings(null)
    }
  }

  return (
    <div className="sheffield-year-selector">
      {/* Top: Left side (4/5) + Right side (1/5) */}
      <div className="selector-top">
        {/* Left: Matches Grid or Squad View or League Table (4/5 width) */}
        <div className="year-flowchart">
          {showSquad ? (
            <div className="squad-view-container">
              <SquadScreen gameState={gameState} setGameState={setGameState} theme="dark" />
            </div>
          ) : showClubs ? (
            <div className="squad-view-container">
              <ClubsScreen
                gameState={gameState}
                setGameState={setGameState}
                theme="dark"
                selectedAction={clubActionSelected}
                setSelectedAction={setClubActionSelected}
              />
            </div>
          ) : showLeagueTable ? (
            <div className="league-view-wrapper">
              {selectedDivision && divisionStandings && typeof divisionStandings === 'object' && !Array.isArray(divisionStandings) ? (
                // Show grouped divisions (5, 6 or 7) with multiple tables
                <div className="league-table-container grouped-divisions-container">
                  {selectedDivision.startsWith('div-5') ? (
                    // Fifth Division: 2 tables (A and B)
                    ['a', 'b'].map((region) => {
                      const divKey = `div-5${region}`
                      const clubs = divisionStandings[divKey] || []
                      const regionNames = { a: 'A', b: 'B' }
                      return (
                        <div key={divKey} style={{ marginBottom: '0px' }}>
                          <table className="league-table">
                            <thead>
                              <tr>
                                <th>Position</th>
                                <th>Club</th>
                                <th>P</th>
                                <th>W</th>
                                <th>D</th>
                                <th>L</th>
                                <th>GF</th>
                                <th>GA</th>
                                {gameState.season >= 1862 && gameState.season <= 1868 && (
                                  <>
                                    <th>RF</th>
                                    <th>RA</th>
                                  </>
                                )}
                                <th>GD</th>
                                <th>Pts</th>
                              </tr>
                            </thead>
                            <tbody>
                              {clubs.sort((a: any, b: any) => {
                                return (b.points || 0) - (a.points || 0)
                              }).map((club: any, idx: number) => {
                                const hasPlayed = (club.played || 0) > 0
                                return (
                                <tr key={club.club_id} className={`${club.club_id === gameState.userClubId ? 'user-club' : ''} ${club.club_id === selectedTableClubId ? 'selected-club' : ''}`} onClick={() => setSelectedTableClubId(club.club_id)} style={{ cursor: 'pointer' }}>
                                  <td>{idx + 1}</td>
                                  <td>{club.club_name || club.name}</td>
                                  <td>{hasPlayed ? club.played : '-'}</td>
                                  <td>{hasPlayed ? club.won || 0 : '-'}</td>
                                  <td>{hasPlayed ? club.drawn || 0 : '-'}</td>
                                  <td>{hasPlayed ? club.lost || 0 : '-'}</td>
                                  <td>{hasPlayed ? club.goals_for || 0 : '-'}</td>
                                  <td>{hasPlayed ? club.goals_against || 0 : '-'}</td>
                                  {gameState.season >= 1862 && gameState.season <= 1868 && (
                                    <>
                                      <td>{hasPlayed ? club.rouges_for || 0 : '-'}</td>
                                      <td>{hasPlayed ? club.rouges_against || 0 : '-'}</td>
                                    </>
                                  )}
                                  <td>{hasPlayed ? (club.goals_for || 0) - (club.goals_against || 0) : '-'}</td>
                                  <td><strong>{hasPlayed ? club.points || 0 : '-'}</strong></td>
                                </tr>
                              )})}
                            </tbody>
                          </table>
                        </div>
                      )
                    })
                  ) : (
                    // Sixth/Seventh Division: 4 tables (a, b, c, d)
                    (['a', 'b', 'c', 'd'] as const).map((region) => {
                      const level = selectedDivision.startsWith('div-6') ? 'Sixth' : 'Seventh'
                      const divKey = `div-${selectedDivision.startsWith('div-6') ? '6' : '7'}${region}`
                      const clubs = divisionStandings[divKey] || []
                      const regionNames = { a: 'West', b: 'East', c: 'North', d: 'South' }
                      return (
                      <div key={divKey} style={{ marginBottom: '0px' }}>
                        <table className="league-table">
                          <thead>
                            <tr>
                              <th>Position</th>
                              <th>Club</th>
                              <th>P</th>
                              <th>W</th>
                              <th>D</th>
                              <th>L</th>
                              <th>GF</th>
                              <th>GA</th>
                              {gameState.season >= 1862 && gameState.season <= 1868 && (
                                <>
                                  <th>RF</th>
                                  <th>RA</th>
                                </>
                              )}
                              <th>GD</th>
                              <th>Pts</th>
                            </tr>
                          </thead>
                          <tbody>
                            {clubs.sort((a: any, b: any) => {
                              // Sort by points descending
                              return (b.points || 0) - (a.points || 0)
                            }).map((club: any, idx: number) => {
                              const hasPlayed = (club.played || 0) > 0
                              return (
                              <tr key={club.club_id} className={`${club.club_id === gameState.userClubId ? 'user-club' : ''} ${club.club_id === selectedTableClubId ? 'selected-club' : ''}`} onClick={() => setSelectedTableClubId(club.club_id)} style={{ cursor: 'pointer' }}>
                                <td>{idx + 1}</td>
                                <td>{club.club_name || club.name}</td>
                                <td>{hasPlayed ? club.played : '-'}</td>
                                <td>{hasPlayed ? club.won || 0 : '-'}</td>
                                <td>{hasPlayed ? club.drawn || 0 : '-'}</td>
                                <td>{hasPlayed ? club.lost || 0 : '-'}</td>
                                <td>{hasPlayed ? club.goals_for || 0 : '-'}</td>
                                <td>{hasPlayed ? club.goals_against || 0 : '-'}</td>
                                {gameState.season >= 1862 && gameState.season <= 1868 && (
                                  <>
                                    <td>{hasPlayed ? club.rouges_for || 0 : '-'}</td>
                                    <td>{hasPlayed ? club.rouges_against || 0 : '-'}</td>
                                  </>
                                )}
                                <td>{hasPlayed ? (club.goals_for || 0) - (club.goals_against || 0) : '-'}</td>
                                <td><strong>{hasPlayed ? club.points || 0 : '-'}</strong></td>
                              </tr>
                            )})}
                          </tbody>
                        </table>
                      </div>
                    )
                  })
                  )}
                </div>
              ) : (
                <div className="league-table-container">
                  <table className="league-table">
                    <thead>
                      <tr>
                        <th>Position</th>
                        <th>Club</th>
                        <th>P</th>
                        <th>W</th>
                        <th>D</th>
                        <th>L</th>
                        <th>GF</th>
                        <th>GA</th>
                        {gameState.season >= 1862 && gameState.season <= 1868 && (
                          <>
                            <th>RF</th>
                            <th>RA</th>
                          </>
                        )}
                        <th>GD</th>
                        <th>Pts</th>
                      </tr>
                    </thead>
                    <tbody>
                      {selectedDivision && divisionStandings && Array.isArray(divisionStandings) ? (
                        // Show selected single division standings
                        divisionStandings.sort((a: any, b: any) => {
                          // Sort by points descending
                          return (b.points || 0) - (a.points || 0)
                        }).map((club: any, idx: number) => {
                          const hasPlayed = (club.played || 0) > 0
                          return (
                          <tr key={club.club_id} className={`${club.club_id === gameState.userClubId ? 'user-club' : ''} ${club.club_id === selectedTableClubId ? 'selected-club' : ''}`} onClick={() => setSelectedTableClubId(club.club_id)} style={{ cursor: 'pointer' }}>
                            <td>{idx + 1}</td>
                            <td>{club.club_name || club.name}</td>
                            <td>{hasPlayed ? club.played : '-'}</td>
                            <td>{hasPlayed ? club.won || 0 : '-'}</td>
                            <td>{hasPlayed ? club.drawn || 0 : '-'}</td>
                            <td>{hasPlayed ? club.lost || 0 : '-'}</td>
                            <td>{hasPlayed ? club.goals_for || 0 : '-'}</td>
                            <td>{hasPlayed ? club.goals_against || 0 : '-'}</td>
                            {gameState.season >= 1862 && gameState.season <= 1868 && (
                              <>
                                <td>{hasPlayed ? club.rouges_for || 0 : '-'}</td>
                                <td>{hasPlayed ? club.rouges_against || 0 : '-'}</td>
                              </>
                            )}
                            <td>{hasPlayed ? (club.goals_for || 0) - (club.goals_against || 0) : '-'}</td>
                            <td><strong>{hasPlayed ? club.points || 0 : '-'}</strong></td>
                          </tr>
                        )})
                      ) : (
                        // Show full league standings
                        gameState.standings.sort((a, b) => {
                          // If no matches played, sort alphabetically by club name
                          const anyClubPlayed = gameState.standings.some(s => s.played > 0)
                          if (!anyClubPlayed) {
                            return getClubName(a.clubId).localeCompare(getClubName(b.clubId))
                          }
                          // Otherwise sort by position
                          return a.position - b.position
                        }).map((standing) => {
                          const isUserClub = standing.clubId === gameState.userClubId
                          return (
                            <tr key={standing.clubId} className={`${isUserClub ? 'user-club' : ''} ${standing.clubId === selectedTableClubId ? 'selected-club' : ''}`} onClick={() => setSelectedTableClubId(standing.clubId)} style={{ cursor: 'pointer' }}>
                              <td>{standing.position}</td>
                              <td>{getClubName(standing.clubId)}</td>
                              <td>{standing.played}</td>
                              <td>{standing.won}</td>
                              <td>{standing.drawn}</td>
                              <td>{standing.lost}</td>
                              <td>{standing.goalsFor}</td>
                              <td>{standing.goalsAgainst}</td>
                              {gameState.season >= 1862 && gameState.season <= 1868 && (
                                <>
                                  <td>{standing.rougesFor || 0}</td>
                                  <td>{standing.rougesAgainst || 0}</td>
                                </>
                              )}
                              <td>{standing.goalsFor - standing.goalsAgainst}</td>
                              <td><strong>{standing.points}</strong></td>
                            </tr>
                          )
                        })
                      )}
                    </tbody>
                  </table>
                </div>
              )}

              <div className="standings-legend">
                <button className="legend-btn" onClick={() => handleDivisionSelect('div-1')}>First Division</button>
                <button className="legend-btn" onClick={() => handleDivisionSelect('div-2')}>Second Division</button>
                <button className="legend-btn" onClick={() => handleDivisionSelect('div-3')}>Third Division</button>
                <button className="legend-btn" onClick={() => handleDivisionSelect('div-4')}>Fourth Division</button>
                <button className="legend-btn" onClick={() => handleDivisionSelect('div-5a')}>Fifth Division</button>
                <button className="legend-btn" onClick={() => handleDivisionSelect('div-6a')}>Sixth Division</button>
                <button className="legend-btn" onClick={() => handleDivisionSelect('div-7a')}>Seventh Division</button>
              </div>
            </div>
          ) : (
            <div className="matches-grid">
              {gameweekMatches.map((match) => {
                const matchType = match.gameweek === 0 ? 'Friendly' : 'League'
                const date = new Date(match.date)
                const day = date.getDate()
                const monthYear = date.toLocaleDateString('en-US', { month: 'short', year: 'numeric' })

                return (
                  <div key={match.id} className="match-card">
                    <div className="match-type-badge">
                      {matchType}
                    </div>

                    <div className="match-header">
                      <span className="match-date" data-day={day} data-month-year={monthYear}></span>
                    </div>

                    <div className="match-teams">
                      <div className="team home">
                        <span className="team-name">{getClubName(match.homeTeamId)}</span>
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
                        <span className="team-name">{getClubName(match.awayTeamId)}</span>
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
                )
              })}
            </div>
          )}
        </div>

        {/* Right: Game State Panel (1/5 width) */}
        <div className="year-details-wrapper">
          {/* Club & League Info */}
          {userDivisionInfo && (
            <div className="season-date-box" style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: '2px', margin: '0', padding: '8px 16px', border: 'none', borderBottom: 'none', borderRadius: '0' }}>
              <div style={{ fontSize: '14px', fontWeight: 'bold', color: 'white' }}>
                {userClub?.name || 'Unknown'}
              </div>
              <div style={{ fontSize: '12px', color: '#bdc3c7' }}>
                {userDivisionInfo.division_name}
              </div>
            </div>
          )}

          <div className="year-details-panel">
            {/* Quick Club Stats OR Club Menu when viewing clubs */}
          {showClubs ? (
            <div style={{ padding: '16px' }}>
              {/* Intelligence Section */}
              <div style={{ marginBottom: '16px' }}>
                <button onClick={() => setClubActionSelected('view-squad')} className="right-section-btn" style={{ width: '100%', marginBottom: '4px', fontSize: '0.75rem', padding: '8px' }}>View Squad List</button>
                <button onClick={() => setClubActionSelected('recent-form')} className="right-section-btn" style={{ width: '100%', marginBottom: '4px', fontSize: '0.75rem', padding: '8px' }}>Recent Form</button>
                <button onClick={() => setClubActionSelected('league-position')} className="right-section-btn" style={{ width: '100%', marginBottom: '4px', fontSize: '0.75rem', padding: '8px' }}>League Position</button>
                <button onClick={() => setClubActionSelected('head-to-head')} className="right-section-btn" style={{ width: '100%', fontSize: '0.75rem', padding: '8px' }}>Head-to-Head</button>
              </div>

              {/* Challenge Section */}
              <div style={{ marginBottom: '16px' }}>
                <button onClick={() => setClubActionSelected('send-challenge')} className="right-section-btn" style={{ width: '100%', fontSize: '0.75rem', padding: '8px' }}>Send Challenge Letter</button>
              </div>

              {/* Information Section */}
              <div>
                <button onClick={() => setClubActionSelected('club-history')} className="right-section-btn" style={{ width: '100%', marginBottom: '4px', fontSize: '0.75rem', padding: '8px' }}>Club History</button>
                <button onClick={() => setClubActionSelected('ground-info')} className="right-section-btn" style={{ width: '100%', marginBottom: '4px', fontSize: '0.75rem', padding: '8px' }}>Ground Info</button>
                <button onClick={() => setClubActionSelected('view-map')} className="right-section-btn" style={{ width: '100%', marginBottom: '4px', fontSize: '0.75rem', padding: '8px' }}>View on Map</button>
                <button onClick={() => setClubActionSelected('parish-info')} className="right-section-btn" style={{ width: '100%', fontSize: '0.75rem', padding: '8px' }}>Parish Info</button>
              </div>
            </div>
          ) : (() => {
            const displayClubId = selectedTableClubId || gameState.userClubId
            const displayClub = gameState.clubs.find(c => c.id === displayClubId)
            const displayStanding = selectedTableClubId ? selectedClubStanding : userStanding

            if (!displayClub || !displayStanding) {
              return null
            }

            return (
              <div className="changes">
                <strong>Club Stats {selectedTableClubId && selectedTableClubId !== gameState.userClubId ? '(Selected)' : ''}</strong>
                <div className="state-row">
                  <span className="label">{displayClub.name}</span>
                  <span className="value">#{displayStanding.position}</span>
                </div>
                <div className="state-row">
                  <span className="label">Record</span>
                  <span className="value">{displayStanding.won}W-{displayStanding.drawn}D-{displayStanding.lost}L</span>
                </div>
                <div className="state-row">
                  <span className="label">Goals</span>
                  <span className="value">{displayStanding.goalsFor}-{displayStanding.goalsAgainst}</span>
                </div>
                <div className="state-row">
                  <span className="label">Points</span>
                  <span className="value">{displayStanding.points}</span>
                </div>
              </div>
            )
          })()}

          {/* Upcoming Fixtures */}
          {(() => {
            const upcomingMatches = gameState.matches
              .filter(m => !m.played && gameState.clubs.some(c => c.id === gameState.userClubId && (c.id === m.homeTeamId || c.id === m.awayTeamId)))
              .sort((a, b) => new Date(a.date).getTime() - new Date(b.date).getTime())
              .slice(0, 3)
            return upcomingMatches.length > 0 ? (
              <div className="changes">
                <strong>Upcoming Fixtures</strong>
                {upcomingMatches.map((match) => (
                  <div key={match.id} className="fixture-row">
                    <div className="fixture-info">
                      <span className="fixture-date">{formatDate(match.date)}</span>
                      <span className="fixture-teams">
                        <span style={{ display: 'inline-flex', alignItems: 'center', gap: '4px' }}>
                          {match.homeTeamId === gameState.userClubId ? <HomeIcon /> : <AwayIcon />}
                          {match.homeTeamId === gameState.userClubId
                            ? getClubName(match.awayTeamId)
                            : getClubName(match.homeTeamId)
                          }
                        </span>
                      </span>
                    </div>
                  </div>
                ))}
              </div>
            ) : null
          })()}

          {/* Player Status - Hidden when viewing Clubs */}
          {!showClubs && (
            <div className="changes">
              <strong>Player Status</strong>
              <div className="state-row">
                <span className="label">Injuries</span>
                <span className="value">None</span>
              </div>
              <div className="state-row">
                <span className="label">Suspensions</span>
                <span className="value">None</span>
              </div>
            </div>
          )}

          {/* Historical Context - Hidden when viewing Clubs */}
          {!showClubs && (() => {
            const displayClubId = selectedTableClubId || gameState.userClubId
            const displayClub = gameState.clubs.find(c => c.id === displayClubId)
            const displayStanding = selectedTableClubId ? selectedClubStanding : userStanding

            return (
              <div className="changes">
                <strong>Season {getSeasonString()}</strong>
                <div className="state-row">
                  <span className="label">Matches</span>
                  <span className="value">{displayStanding?.played || 0}/{gameState.matches.length > 0 ? Math.max(...gameState.standings.map(s => s.played)) : 0}</span>
                </div>
                {/* Founded year not available in Club interface */}
              </div>
            )
          })()}
          </div>
        </div>
      </div>

      {/* Bottom: Action Buttons + News (full width) */}
      <div className="year-action-buttons">
        <button
          className="action-button continue-btn"
          onClick={continueGame}
          title={hasMatchesToday ? "Play today's matches" : "Advance to the next day"}
        >
          {hasMatchesToday ? "Matchday" : "Continue"}
        </button>
        <button
          className="action-button tables-btn"
          onClick={async () => {
            setShowLeagueTable(!showLeagueTable)
            if (!showLeagueTable) {
              setShowClubs(false)
              setShowSquad(false)

              // Auto-load the user's division
              console.log('🔥🔥🔥 Tables button clicked - loading user division for club:', gameState.userClubId)
              try {
                const divInfo = await invoke<any>('get_club_division_info', {
                  clubId: gameState.userClubId
                })
                console.log('🔥 Division info received:', divInfo)

                const standings = await invoke<Array<any>>('get_division_standings', {
                  divisionId: divInfo.division_id,
                  season: gameState.season || 1888
                })
                console.log('🔥 Standings received:', standings)

                setSelectedDivision(divInfo.division_id)
                setDivisionStandings(standings)
              } catch (error) {
                console.error('🔥 ERROR loading user division:', error)
              }
            } else {
              // Reset to managed club when hiding table
              setSelectedTableClubId(null)
              setSelectedDivision(null)
            }
          }}
          title={showLeagueTable ? "Hide league table" : "View league table"}
        >
          <span className="action-button-icon"><TrophyAchievementIcon /></span>
          {showLeagueTable ? "Hide Table" : "Tables"}
        </button>
        <button
          className="action-button squad-btn"
          onClick={() => {
            setShowSquad(!showSquad)
            if (!showSquad) {
              setShowLeagueTable(false)
              setShowClubs(false)
            }
          }}
          title={showSquad ? "Hide squad view" : "View your squad"}
        >
          <span className="action-button-icon"><SquadIcon /></span>
          {showSquad ? "Hide Squad" : "Squad"}
        </button>
        <button
          className="action-button tactics-btn"
          title="Manage team tactics"
          onClick={() => setComingSoonModal({
            title: 'Tactics',
            description: 'Set your team formation, defensive mentality, and strategic approach for upcoming matches.'
          })}
        >
          <span className="action-button-icon"><TacticsIcon /></span>
          Tactics
        </button>
        <button
          className="action-button training-btn"
          title="Arrange a training session"
          onClick={() => setShowTraining(true)}
        >
          <span className="action-button-icon"><TrainingIcon /></span>
          Training
        </button>
        <button
          className="action-button clubs-btn"
          title="View other clubs"
          onClick={() => {
            setShowClubs(!showClubs)
            if (!showClubs) {
              setShowLeagueTable(false)
              setShowSquad(false)
            }
          }}
        >
          <span className="action-button-icon"><ClubsIcon /></span>
          {showClubs ? "Hide Clubs" : "Clubs"}
        </button>
        <button
          className="action-button"
          title="Schedule trial session"
          onClick={() => {
            setShowTrialDatePicker(true)
          }}
        >
          <span className="action-button-icon"><SquadIcon /></span>
          Players
        </button>
        <button
          className="action-button"
          title="The committee's standing, the club's identity, your name"
          onClick={() => setShowCommittee(true)}
        >
          <span className="action-button-icon"><TacticsIcon /></span>
          Committee
        </button>
        <button
          className="action-button"
          title="The Co-operative Society"
          onClick={() => setShowCoop(true)}
        >
          <span className="action-button-icon"><TrophyAchievementIcon /></span>
          Co-op
        </button>
      </div>

      {showTraining && (
        <TrainingPanelModal gameState={gameState} onClose={() => setShowTraining(false)} />
      )}
      {showCommittee && (
        <CommitteePanelModal gameState={gameState} onClose={() => setShowCommittee(false)} />
      )}
      {showCoop && (
        <CoopPanelModal gameState={gameState} onClose={() => setShowCoop(false)} />
      )}

      {/* Settings Modal - Floating */}
      {showSettings && (
        <div className="settings-modal-overlay" onClick={() => setShowSettings(false)}>
          <div className="settings-modal" onClick={(e) => e.stopPropagation()}>
            {settingsSubmenu === null ? (
              <>
                <div className="settings-header">
                  <h3>Settings</h3>
                  <button className="settings-close" onClick={() => setShowSettings(false)}>✕</button>
                </div>
                <div className="settings-menu">
                  <button
                    className="settings-menu-item"
                    onClick={() => setSettingsSubmenu('speed')}
                  >
                    Game Speed
                  </button>
                  <button
                    className="settings-menu-item"
                    onClick={() => setSettingsSubmenu('audio')}
                  >
                    Audio
                  </button>
                  <button
                    className="settings-menu-item"
                    onClick={() => setSettingsSubmenu('language')}
                  >
                    Language
                  </button>
                  <button
                    className="settings-menu-item"
                    onClick={() => setSettingsSubmenu('about')}
                  >
                    About
                  </button>
                  <button
                    className="settings-menu-item"
                    onClick={() => setSettingsSubmenu('help')}
                  >
                    Help
                  </button>
                  <button
                    className="settings-menu-item"
                    onClick={() => {
                      setSaveLoadModal({ mode: 'save' });
                      setShowSettings(false);
                    }}
                  >
                    Save Game
                  </button>
                  <button
                    className="settings-menu-item"
                    onClick={async () => {
                      const { getCurrentWindow } = await import('@tauri-apps/api/window');
                      await getCurrentWindow().close();
                    }}
                  >
                    Quit
                  </button>
                </div>
              </>
            ) : settingsSubmenu === 'speed' ? (
              <>
                <div className="settings-header">
                  <button className="settings-back" onClick={() => setSettingsSubmenu(null)}>← Back</button>
                  <h3>Game Speed</h3>
                  <button className="settings-close" onClick={() => setShowSettings(false)}>✕</button>
                </div>
                <div className="speed-options-modal">
                  <button
                    className={`speed-option ${gameSpeed === 0.5 ? 'active' : ''}`}
                    onClick={() => setGameSpeed(0.5)}
                  >
                    0.5x
                  </button>
                  <button
                    className={`speed-option ${gameSpeed === 1 ? 'active' : ''}`}
                    onClick={() => setGameSpeed(1)}
                  >
                    1x
                  </button>
                  <button
                    className={`speed-option ${gameSpeed === 1.5 ? 'active' : ''}`}
                    onClick={() => setGameSpeed(1.5)}
                  >
                    1.5x
                  </button>
                  <button
                    className={`speed-option ${gameSpeed === 2 ? 'active' : ''}`}
                    onClick={() => setGameSpeed(2)}
                  >
                    2x
                  </button>
                </div>
              </>
            ) : settingsSubmenu === 'audio' ? (
              <>
                <div className="settings-header">
                  <button className="settings-back" onClick={() => setSettingsSubmenu(null)}>← Back</button>
                  <h3>Audio</h3>
                  <button className="settings-close" onClick={() => setShowSettings(false)}>✕</button>
                </div>
                <div className="settings-panel">
                  <label className="toggle-label">
                    <input
                      type="checkbox"
                      checked={audioEnabled}
                      onChange={(e) => setAudioEnabled(e.target.checked)}
                    />
                    <span>Background Music: {audioEnabled ? 'ON' : 'OFF'}</span>
                  </label>
                </div>
              </>
            ) : settingsSubmenu === 'language' ? (
              <>
                <div className="settings-header">
                  <button className="settings-back" onClick={() => setSettingsSubmenu(null)}>← Back</button>
                  <h3>Language</h3>
                  <button className="settings-close" onClick={() => setShowSettings(false)}>✕</button>
                </div>
                <div className="language-options">
                  {['English', 'Spanish', 'French', 'German'].map((lang) => (
                    <button
                      key={lang}
                      className={`language-option ${language === lang ? 'active' : ''}`}
                      onClick={() => setLanguage(lang)}
                    >
                      {lang}
                    </button>
                  ))}
                </div>
              </>
            ) : settingsSubmenu === 'about' ? (
              <>
                <div className="settings-header">
                  <button className="settings-back" onClick={() => setSettingsSubmenu(null)}>← Back</button>
                  <h3>About</h3>
                  <button className="settings-close" onClick={() => setShowSettings(false)}>✕</button>
                </div>
                <div className="about-panel">
                  <p><strong>Saturday at Three</strong></p>
                  <p>Version 0.1.0</p>
                  <p>A historical football management simulation game</p>
                  <p style={{ marginTop: '16px', fontSize: '12px', color: '#666' }}>
                    Experience football history from 1858 onwards. Manage legendary Sheffield clubs and shape their destinies.
                  </p>
                </div>
              </>
            ) : settingsSubmenu === 'help' ? (
              <>
                <div className="settings-header">
                  <button className="settings-back" onClick={() => setSettingsSubmenu(null)}>← Back</button>
                  <h3>Help</h3>
                  <button className="settings-close" onClick={() => setShowSettings(false)}>✕</button>
                </div>
                <div className="help-panel">
                  <p><strong>Getting Started:</strong></p>
                  <ul>
                    <li>Select a club and year to begin</li>
                    <li>Manage your squad and tactics</li>
                    <li>Progress through seasons and advance football history</li>
                  </ul>
                  <p><strong>Controls:</strong></p>
                  <ul>
                    <li>Click buttons to navigate menus</li>
                    <li>Use Game Speed to control simulation pace</li>
                    <li>Save regularly to avoid progress loss</li>
                  </ul>
                </div>
              </>
            ) : null}
          </div>
        </div>
      )}

      {/* Bottom Section: News (40%) + Empty Area (60%) */}
      <div className="gameplay-bottom-section">
        {/* News & Events List - Direct Child */}
        {pendingEvents.length === 0 ? (
          <div className="events-list">
            {/* Season & Date Info */}
            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '20px', padding: '12px 16px', background: 'linear-gradient(135deg, #34495e 0%, #34495e 100%)', marginBottom: '0' }}>
              <div className="date-item">
                <span className="date-label" style={{ fontSize: '0.65rem' }}>Season</span>
                <span className="date-value" style={{ fontSize: '0.75rem' }}>{getSeasonString()}</span>
              </div>
              <div className="date-item">
                <span className="date-label" style={{ fontSize: '0.65rem' }}>Current Date</span>
                <span className="date-value" style={{ fontSize: '0.75rem' }}>{formatDate(gameState.currentDate)}</span>
              </div>
            </div>
            <p className="no-events">No news or events</p>
          </div>
        ) : (
          <div className="events-list">
            {/* Season & Date Info */}
            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '20px', padding: '12px 16px', background: 'linear-gradient(135deg, #34495e 0%, #34495e 100%)', marginBottom: '0' }}>
              <div className="date-item">
                <span className="date-label" style={{ fontSize: '0.65rem' }}>Season</span>
                <span className="date-value" style={{ fontSize: '0.75rem' }}>{getSeasonString()}</span>
              </div>
              <div className="date-item">
                <span className="date-label" style={{ fontSize: '0.65rem' }}>Current Date</span>
                <span className="date-value" style={{ fontSize: '0.75rem' }}>{formatDate(gameState.currentDate)}</span>
              </div>
            </div>
            {pendingEvents.map((event) => (
              <div
                key={event.id}
                className={`event-item ${selectedNewsEvent?.id === event.id ? 'selected' : ''} ${event.processed ? 'news-read' : 'news-unread'}`}
                onClick={() => setSelectedNewsEvent(event)}
                style={{ cursor: 'pointer' }}
              >
                <div className="event-date-left">{formatDate(event.date)}</div>
                <div className="event-content">
                  {event.eventType.type === 'PlayerNegotiation' && (
                    <div className="event-text">
                      <strong>Player Negotiation:</strong> {(event.eventType.data as any).offerType}
                    </div>
                  )}
                  {event.eventType.type === 'MediaInquiry' && (
                    <div className="event-text">
                      <strong>Media Inquiry:</strong> {(event.eventType.data as any).question}
                    </div>
                  )}
                  {event.eventType.type === 'HistoricalAnnouncement' && (
                    <div className="event-text">
                      <strong>{(event.eventType.data as any).title}</strong>
                    </div>
                  )}
                  {event.eventType.type === 'CupAnnouncement' && (
                    <div className="event-text">
                      <strong>{(event.eventType.data as any).title}</strong>
                    </div>
                  )}
                  {(event.eventType.type === 'challenge_sent' ||
                    event.eventType.type === 'challenge_accepted' ||
                    event.eventType.type === 'challenge_declined' ||
                    event.eventType.type === 'challenge_received' ||
                    event.eventType.type === 'match_scheduled' ||
                    event.eventType.type === 'cup_announcement') && (
                    <div className="event-text">
                      <strong>{(event.eventType.data as any).headline}</strong>
                      {(event.eventType.data as any).isImportant && <span style={{ marginLeft: '8px', color: '#f39c12' }}>!</span>}
                    </div>
                  )}
                  {event.requiresUserAction && <span className="action-badge">Action Needed</span>}
                </div>
              </div>
            ))}
          </div>
        )}

        {/* Details area with blue border */}
        <div className="gameplay-empty-area">
          {selectedNewsEvent ? (
            <div className="news-details-panel">
              <div className="news-details-header">
                <h2>{
                  selectedNewsEvent.eventType.type === 'HistoricalAnnouncement'
                    ? (selectedNewsEvent.eventType.data as any).title
                    : (selectedNewsEvent.eventType.data as any).headline || 'Event Details'
                }</h2>
                <button className="close-details" onClick={() => setSelectedNewsEvent(null)}>×</button>
              </div>
              <div className="news-details-content">
                {selectedNewsEvent.eventType.type === 'PlayerNegotiation' && (
                  <>
                    <p><strong>Offer Type:</strong> {(selectedNewsEvent.eventType.data as any).offerType}</p>
                  </>
                )}
                {selectedNewsEvent.eventType.type === 'MediaInquiry' && (
                  <>
                    <p><strong>Question:</strong> {(selectedNewsEvent.eventType.data as any).question}</p>
                  </>
                )}
                {selectedNewsEvent.eventType.type === 'HistoricalAnnouncement' && (
                  <>
                    <p>{(selectedNewsEvent.eventType.data as any).description}</p>
                  </>
                )}
                {(selectedNewsEvent.eventType.type === 'challenge_sent' ||
                  selectedNewsEvent.eventType.type === 'challenge_accepted' ||
                  selectedNewsEvent.eventType.type === 'challenge_declined' ||
                  selectedNewsEvent.eventType.type === 'challenge_received' ||
                  selectedNewsEvent.eventType.type === 'match_scheduled') && (
                  <>
                    <p style={{ whiteSpace: 'pre-line' }}>{(selectedNewsEvent.eventType.data as any).body}</p>
                    {(selectedNewsEvent.eventType.data as any).hasActionButton && (
                      <button
                        className="view-btn"
                        style={{ marginTop: '16px', width: '100%' }}
                        onClick={async () => {
                          console.log('[LETTER] Button clicked, event data:', selectedNewsEvent.eventType.data)
                          try {
                            const actionData = JSON.parse((selectedNewsEvent.eventType.data as any).actionData || '{}')
                            console.log('[LETTER] Parsed action data:', actionData)

                            if (actionData.invitation_id) {
                              console.log('[LETTER] Fetching invitation:', actionData.invitation_id)
                              const invitation = await invoke<any>('get_invitation_by_id', {
                                game: gameState,
                                invitationId: actionData.invitation_id
                              })
                              console.log('[LETTER] Received invitation:', invitation)

                              // Get sender club info
                              const senderClub = gameState.clubs.find(c => c.id === invitation.recipient_club_id)
                              console.log('[LETTER] Sender club:', senderClub)

                              // Show the response letter in a proper Victorian-style modal
                              const modalData = {
                                senderClubName: senderClub?.name || 'Unknown Club',
                                senderGroundName: senderClub?.ground || undefined,
                                senderCity: undefined, // Can add if needed
                                senderRegion: undefined, // Can add if needed
                                responseText: invitation.response_text || 'No response text available',
                                responseDate: invitation.response_date || gameState.currentDate,
                                accepted: invitation.status === 'accepted',
                                newsId: selectedNewsEvent.id,
                                relatedInvitationId: actionData.invitation_id
                              }
                              console.log('[LETTER] Setting modal with data:', modalData)
                              setLetterModal(modalData)
                            } else {
                              console.error('[LETTER] No invitation_id in action data')
                            }
                          } catch (error) {
                            console.error('[LETTER] Failed to load letter:', error)
                            alert(`Error loading letter: ${error}`)
                          }
                        }}
                      >
                        {(selectedNewsEvent.eventType.data as any).actionButtonText || 'View Details'}
                      </button>
                    )}
                  </>
                )}
                {selectedNewsEvent.requiresUserAction && (
                  <div className="action-required">
                    <strong>⚠ This event requires your action</strong>
                  </div>
                )}
              </div>
            </div>
          ) : (
            <div className="news-details-placeholder">
              Select a news story to view details
            </div>
          )}
        </div>

        {/* Right section (20%) with blue border */}
        <div className="gameplay-right-section">
          <div className="right-section-content">
            <button className="right-section-btn">History</button>
            <button className="right-section-btn">Reputation</button>
            <button className="right-section-btn">Transfers</button>
            <button className="right-section-btn">Job Info</button>
            <button className="right-section-btn">Admin</button>
            <button
              className="right-section-btn"
              onClick={() => {
                setShowSettings(!showSettings)
                setSettingsSubmenu(null)
              }}
            >
              Settings
            </button>
          </div>
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


      {/* Coming Soon Modal */}
      {comingSoonModal && (
        <ComingSoonModal
          title={comingSoonModal.title}
          description={comingSoonModal.description}
          onClose={() => setComingSoonModal(null)}
        />
      )}

      {/* Trial Date Picker Modal */}
      {showTrialDatePicker && (
        <div className="modal-overlay" onClick={() => setShowTrialDatePicker(false)}>
          <div className="modal-content" onClick={(e) => e.stopPropagation()} style={{ maxWidth: '500px' }}>
            <div className="modal-header">
              <h2>Schedule Trial Session</h2>
              <button className="close-btn" onClick={() => setShowTrialDatePicker(false)}>✕</button>
            </div>
            <div style={{ padding: '20px' }}>
              <p style={{ marginBottom: '20px', fontSize: '0.95rem', color: '#666' }}>
                Select a future date for the trial session. On that day, 10-15 men aged 16-40 will arrive to try out for your club.
              </p>
              <div style={{ marginBottom: '20px' }}>
                <label style={{ display: 'block', marginBottom: '8px', fontWeight: 'bold' }}>
                  Trial Date:
                </label>
                <input
                  type="date"
                  value={selectedTrialDate}
                  onChange={(e) => setSelectedTrialDate(e.target.value)}
                  min={new Date(new Date(gameState.currentDate).getTime() + 86400000).toISOString().split('T')[0]}
                  style={{
                    width: '100%',
                    padding: '10px',
                    fontSize: '1rem',
                    border: '1px solid #ddd',
                    borderRadius: '4px'
                  }}
                />
              </div>
              <div style={{ display: 'flex', gap: '10px', justifyContent: 'flex-end' }}>
                <button
                  className="view-btn"
                  onClick={() => setShowTrialDatePicker(false)}
                  style={{ padding: '10px 20px', backgroundColor: '#999' }}
                >
                  Cancel
                </button>
                <button
                  className="view-btn"
                  onClick={async () => {
                    if (!selectedTrialDate) {
                      alert('Please select a date')
                      return
                    }
                    try {
                      const updatedGame = await invoke<GameState>('schedule_trial_session', {
                        game: gameState,
                        trialDate: selectedTrialDate
                      })
                      setGameState(updatedGame)
                      setShowTrialDatePicker(false)
                      setSelectedTrialDate('')
                      alert(`Trial session scheduled for ${selectedTrialDate}`)
                    } catch (error) {
                      console.error('Failed to schedule trial:', error)
                      alert(`Failed to schedule trial: ${error}`)
                    }
                  }}
                  style={{ padding: '10px 20px' }}
                  disabled={!selectedTrialDate}
                >
                  Schedule Trial
                </button>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Letter Display Modal */}
      {letterModal && (
        <LetterDisplayModal
          letterData={letterModal}
          onClose={async () => {
            try {
              // Mark news as read
              if (letterModal.newsId) {
                await invoke('mark_news_as_read', { game: gameState, newsId: letterModal.newsId })
                console.log('[NEWS] Marked news as read:', letterModal.newsId)
              }

              // If this was a challenge acceptance letter, create the "MATCH CONFIRMED" news
              if (letterModal.accepted && letterModal.relatedInvitationId) {
                console.log('[NEWS] Creating match confirmed news for accepted challenge')
                await invoke('create_match_confirmed_news', {
                  game: gameState,
                  invitationId: letterModal.relatedInvitationId
                })
              }

              // Reload news items to reflect the changes
              const items = await invoke<any[]>('get_all_news', { game: gameState })
              setNewsItems(items || [])
            } catch (error) {
              console.error('[NEWS] Error handling letter close:', error)
            }

            setLetterModal(null)
          }}
        />
      )}

      {/* Trial Session Panel */}
      {showTrialPanel && (
        <div className="modal-overlay" onClick={() => setShowTrialPanel(false)}>
          <div className="modal-content" onClick={(e) => e.stopPropagation()} style={{ maxWidth: '900px', maxHeight: '80vh', overflow: 'auto' }}>
            <div className="modal-header">
              <h2>Trial Session</h2>
              <button className="close-btn" onClick={() => setShowTrialPanel(false)}>✕</button>
            </div>
            <div style={{ padding: '20px' }}>
              <p style={{ marginBottom: '20px', fontSize: '0.95rem', color: '#666' }}>
                {trialPlayers.length} men have arrived for the trial session. Review their abilities and select those you wish to sign to your club.
              </p>

              {trialPlayers.length === 0 ? (
                <div style={{ textAlign: 'center', padding: '40px', color: '#999' }}>
                  Loading trial players...
                </div>
              ) : (
                <div style={{ display: 'grid', gap: '12px' }}>
                  {trialPlayers.map((player: any) => (
                    <div key={player.id} style={{
                      padding: '12px',
                      border: '1px solid #ddd',
                      borderRadius: '4px',
                      backgroundColor: '#f9f9f9',
                      display: 'flex',
                      justifyContent: 'space-between',
                      alignItems: 'center'
                    }}>
                      <div style={{ flex: 1 }}>
                        <div style={{ fontWeight: 'bold', fontSize: '1.05rem', marginBottom: '4px' }}>
                          {player.name}
                        </div>
                        <div style={{ fontSize: '0.85rem', color: '#666', display: 'flex', gap: '12px', flexWrap: 'wrap' }}>
                          <span><strong>Age:</strong> {player.age}</span>
                          {player.profession && <span><strong>Occupation:</strong> {player.profession}</span>}
                          {player.parish && <span><strong>Parish:</strong> {player.parish}</span>}
                          {player.position && <span><strong>Position:</strong> {player.position}</span>}
                        </div>
                        <div style={{ fontSize: '0.85rem', color: '#444', marginTop: '6px', display: 'flex', gap: '10px' }}>
                          {player.pace && <span>Pace: {player.pace}</span>}
                          {player.strength && <span>Strength: {player.strength}</span>}
                          {player.stamina && <span>Stamina: {player.stamina}</span>}
                          {player.passing && <span>Passing: {player.passing}</span>}
                          {player.dribbling && <span>Dribbling: {player.dribbling}</span>}
                        </div>
                      </div>
                      <div>
                        <button
                          className="view-btn"
                          style={{ padding: '8px 16px', fontSize: '0.9rem' }}
                          onClick={async () => {
                            try {
                              await invoke('transfer_player', {
                                playerId: player.id,
                                newClubId: gameState.userClubId
                              })
                              setTrialPlayers(prev => prev.filter(p => p.id !== player.id))
                            } catch (error) {
                              console.error('Failed to sign player:', error)
                              alert('Failed to sign player')
                            }
                          }}
                        >
                          Sign Player
                        </button>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>
        </div>
      )}

      {/* Matchday Modal */}
      {showMatchdayModal && (
        <MatchdayModeModal
          gameState={gameState}
          matchesForDay={gameweekMatches.filter(m => !m.played)}
          onComplete={(updatedGameState) => {
            setGameState(updatedGameState)
            setShowMatchdayModal(false)
          }}
          onClose={() => setShowMatchdayModal(false)}
        />
      )}
    </div>
  )
}
