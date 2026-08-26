import React, { useEffect, useState, lazy, Suspense } from 'react'
import { invoke } from './utils/tauriInvoke'
import type { GameState, DayProcessingResult, Match } from './types/GameState'
import { initialGameState } from './types/GameState'
import { useHistory } from './hooks/useHistory'
import GameSelectionScreen from './screens/GameSelectionScreen'
import { SheffieldRulesSetupScreen } from './screens/SheffieldRulesSetupScreen'
import { GameplayScreen } from './screens/GameplayScreen'
import { GameplayScreenV2 } from './screens/GameplayScreenV2'
import { ClubSelectionScreen } from './screens/ClubSelectionScreen'
import { DayResultsModal } from './components/DayResultsModal'
import { MatchdayModeModal } from './components/MatchdayModeModal'
import { LoadingOverlay } from './components/LoadingOverlay'
import DatabaseEditorScreen from './screens/DatabaseEditorScreen'
import PlayerGenerationScreen from './screens/PlayerGenerationScreen'
import { LiveMatchScreen } from './screens/LiveMatchScreen'
import { SpriteEditorScreen } from './screens/SpriteEditorScreen'
import './index.css'

/**
 * FIXED-RESOLUTION UI ARCHITECTURE
 *
 * The entire UI is rendered at a fixed virtual resolution (1280x800)
 * and uniformly scaled to fit the window. The window acts like a camera
 * zoom — resizing NEVER causes layout reflow.
 *
 * This is the canonical scaling system per the UI specification.
 */
const BASE_WIDTH = 1280
const BASE_HEIGHT = 800

// Lazy load screens
const DashboardScreen = lazy(() => import('./screens/DashboardScreen').then(m => ({ default: m.DashboardScreen })))
const MatchdayScreen = lazy(() => import('./screens/MatchdayScreen').then(m => ({ default: m.MatchdayScreen })))
const SquadScreen = lazy(() => import('./screens/SquadScreen').then(m => ({ default: m.SquadScreen })))
const StandingsScreen = lazy(() => import('./screens/StandingsScreen').then(m => ({ default: m.StandingsScreen })))
const MatchViewerScreen = lazy(() => import('./screens/MatchViewerScreen').then(m => ({ default: m.MatchViewerScreen })))

type ScreenName = 'dashboard' | 'matchday' | 'squad' | 'standings' | 'matchviewer'
type GameType = 'sheffield-rules' | 'saturday-at-three' | 'database-editor' | 'quick-match' | 'sprite-editor' | null

function PanelLoadingFallback() {
  return <div className="panel-loading">Loading...</div>
}

export default function App() {
  const { state: gameState, setState: setGameState, undo, redo, canUndo, canRedo } = useHistory<GameState>(initialGameState)
  const [currentScreen, setCurrentScreen] = useState<ScreenName>('dashboard')
  const [isDirty, setIsDirty] = useState(false)
  const [selectedGame, setSelectedGame] = useState<GameType>(null)
  const [sheffieldSetupComplete, setSheffieldSetupComplete] = useState(false)
  const [dayProcessing, setDayProcessing] = useState<DayProcessingResult | null>(null)
  const [isProcessingDay, setIsProcessingDay] = useState(false)
  const [pixelPerfect, setPixelPerfect] = useState(false)
  const [scale, setScale] = useState(1)
  const [selectedYear, setSelectedYear] = useState<number | null>(null)
  const [selectedGameMode, setSelectedGameMode] = useState<string>('')
  const [selectedClub, setSelectedClub] = useState<string>('')
  const [isInGameplay, setIsInGameplay] = useState(false)
  const [showGameplayV2, setShowGameplayV2] = useState(false)
  const [showClubSelection, setShowClubSelection] = useState(false)
  const [isLoading, setIsLoading] = useState(false)
  const [loadingProgress, setLoadingProgress] = useState(0)
  const [loadingMessage, setLoadingMessage] = useState('')
  const [matchdayModal, setMatchdayModal] = useState<{
    matches: Match[]
    date: string
  } | null>(null)
  const [hasMatchesToday, setHasMatchesToday] = useState(false)

  // Initialize and handle fixed-resolution UI scaling
  useEffect(() => {
    const scaleUi = () => {
      const scaleX = window.innerWidth / BASE_WIDTH
      const scaleY = window.innerHeight / BASE_HEIGHT

      let nextScale = Math.min(scaleX, scaleY)

      // Pixel-perfect mode: integer scaling only
      if (pixelPerfect) {
        nextScale = Math.floor(nextScale)
        if (nextScale < 1) nextScale = 1
      }

      setScale(nextScale)

      const ui = document.getElementById('ui-root')
      if (ui) {
        ui.style.transform = `scale(${nextScale})`
      }
    }

    scaleUi()
    window.addEventListener('resize', scaleUi)
    return () => window.removeEventListener('resize', scaleUi)
  }, [pixelPerfect])

  // Handle game selection
  const handleGameSelect = (game: GameType) => {
    setSelectedGame(game)
    setSheffieldSetupComplete(false)  // Reset setup flag when changing games
    if (game === 'saturday-at-three') {
      // Set a default club for Saturday at Three
      setGameState(prev => ({ ...prev, userClubId: 'accrington' }))
    } else if (game === 'sheffield-rules') {
      // For Sheffield Rules, don't set a default club yet - let year selector handle setup
      // Just reset the game state
      setGameState(initialGameState)
    }
  }

  // Handle Sheffield Rules game mode selection
  const handleSheffieldGameModeSelected = (gameMode: string, year?: number, club?: string) => {
    console.log(`Sheffield Rules selected: gameMode=${gameMode}, year=${year}`)
    console.log(`Showing club selection screen`)
    setSelectedYear(year || 1867)
    setSelectedGameMode(gameMode)
    setSheffieldSetupComplete(true)
    setShowClubSelection(true)  // Go to club selection first
  }

  // Handle club selection
  const handleClubSelected = async (clubId: string, clubName: string) => {
    try {
      console.log(`Club selected: ${clubId} (${clubName})`)
      setSelectedClub(clubId)
      setShowClubSelection(false)

      // Show loading overlay and simulate progress
      setIsLoading(true)
      setLoadingProgress(0)
      setLoadingMessage('Preparing database...')

      // Simulate progress updates while initializing
      const progressInterval = setInterval(() => {
        setLoadingProgress((prev) => {
          if (prev < 75) {
            return prev + Math.random() * 12
          }
          return prev
        })
      }, 200)

      // Update message based on progress
      const messageInterval = setInterval(() => {
        setLoadingProgress((prev) => {
          if (prev < 15) setLoadingMessage('Creating database...')
          else if (prev < 25) setLoadingMessage('Initializing schema...')
          else if (prev < 35) setLoadingMessage('Loading clubs...')
          else if (prev < 45) setLoadingMessage('Setting up players...')
          else if (prev < 60) setLoadingMessage('Initializing standings...')
          else if (prev < 75) setLoadingMessage('Finalizing game state...')
          else setLoadingMessage('Launching game...')
          return prev
        })
      }, 400)

      // Initialize the game with the selected year, mode, and club
      console.log(`Calling initialize_sheffield_game with year=${selectedYear}, gameMode=${selectedGameMode}, clubId=${clubId}`)
      const gameState = await invoke<GameState>('initialize_sheffield_game', {
        year: selectedYear,
        gameMode: selectedGameMode,
        clubId: clubId
      })
      console.log('═══════════════════════════════════════════════════════════')
      console.log('App.tsx - GameState received from backend:')
      console.log('───────────────────────────────────────────────────────────')
      console.log(JSON.stringify(gameState, null, 2))
      console.log('───────────────────────────────────────────────────────────')
      console.log(`GameState.season = ${gameState.season}`)
      console.log(`GameState.gameMode = ${gameState.gameMode}`)
      console.log(`typeof GameState.gameMode = ${typeof gameState.gameMode}`)
      console.log('═══════════════════════════════════════════════════════════')

      // Complete the progress bar
      clearInterval(progressInterval)
      clearInterval(messageInterval)
      setLoadingProgress(100)
      setLoadingMessage('Launching game...')

      // Brief delay to show completion
      await new Promise((resolve) => setTimeout(resolve, 300))

      setGameState(gameState)
      setIsInGameplay(true)
      setIsLoading(false)
      console.log('Game initialized, entering gameplay')
    } catch (error) {
      console.error('Failed to initialize game:', error)
      setIsLoading(false)
      alert(`Failed to initialize game: ${error}`)
      setShowClubSelection(true)  // Go back to club selection on error
    }
  }

  // Handle returning from club selection
  const handleClubSelectionBack = () => {
    setShowClubSelection(false)
    setSheffieldSetupComplete(false)
    setSelectedYear(null)
    setSelectedGameMode('')
  }

  // Handle returning from gameplay to year selector
  const handleBackFromGameplay = () => {
    setIsInGameplay(false)
    setSelectedYear(null)
    setSelectedGameMode('')
    setSelectedClub('')
    setSheffieldSetupComplete(false)
  }

  // Advance game by one day OR continue to matchday modal
  const advanceDayOrContinue = async () => {
    // If button shows "Continue", open matchday modal
    if (hasMatchesToday) {
      const matchesForDay = gameState.matches.filter(m =>
        m.date === gameState.currentDate && !m.played
      )
      setMatchdayModal({ matches: matchesForDay, date: gameState.currentDate })
      return
    }

    // Otherwise, advance the day
    try {
      setIsProcessingDay(true)
      const result: DayProcessingResult = await invoke('advance_day', { game: gameState })

      // Reload game state to get updated matches/standings
      const updatedGame: GameState = await invoke('get_current_game')
      setGameState(updatedGame)

      // Always return to dashboard after advancing day
      setCurrentScreen('dashboard')

      // Check if new day has matches
      const matchesForDay = updatedGame.matches.filter(m =>
        m.date === updatedGame.currentDate && !m.played
      )

      if (matchesForDay.length > 0) {
        setHasMatchesToday(true)  // Change button to "Continue"
      } else {
        setHasMatchesToday(false)  // Keep as "Next Day"
        // Show DayResultsModal if there are other events
        if (result.autoProcessed.length > 0 || result.requireUserAction.length > 0) {
          setDayProcessing(result)
        }
      }

      setIsDirty(false)
    } catch (error) {
      console.error('Failed to advance day:', error)
    } finally {
      setIsProcessingDay(false)
    }
  }

  // Keep old function name as alias for backwards compatibility
  const advanceDay = advanceDayOrContinue

  const completeDayProcessing = () => {
    setDayProcessing(null)
  }

  // Format date for display
  const formatDate = (dateStr: string) => {
    try {
      // Parse ISO 8601 date format (YYYY-MM-DD)
      const parts = dateStr.split('-')
      if (parts.length === 3) {
        const date = new Date(parseInt(parts[0]), parseInt(parts[1]) - 1, parseInt(parts[2]))
        if (!isNaN(date.getTime())) {
          return date.toLocaleDateString('en-GB', { weekday: 'short', year: 'numeric', month: 'short', day: 'numeric' })
        }
      }
    } catch (e) {
      console.error('Error formatting date:', dateStr, e)
    }
    return 'Invalid Date'
  }

  // Load game on mount
  useEffect(() => {
    loadGame()
  }, [])

  // Auto-save on gameState change (debounced)
  useEffect(() => {
    if (!isDirty) return

    const timer = setTimeout(() => {
      saveGame()
    }, 1000) // Wait 1 second after last change

    return () => clearTimeout(timer)
  }, [isDirty, gameState])

  // Mark as dirty whenever gameState changes
  useEffect(() => {
    setIsDirty(true)
  }, [gameState])

  // Keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Ctrl+Z: Undo
      if ((e.ctrlKey || e.metaKey) && e.key === 'z' && !e.shiftKey) {
        e.preventDefault()
        undo()
      }
      // Ctrl+Y or Ctrl+Shift+Z: Redo
      if ((e.ctrlKey || e.metaKey) && (e.key === 'y' || (e.key === 'z' && e.shiftKey))) {
        e.preventDefault()
        redo()
      }
      // Ctrl+S: Save
      if ((e.ctrlKey || e.metaKey) && e.key === 's') {
        e.preventDefault()
        saveGame()
      }
      // Ctrl+N: New Game
      if ((e.ctrlKey || e.metaKey) && e.key === 'n') {
        e.preventDefault()
        newGame()
      }
    }

    window.addEventListener('keydown', handleKeyDown)
    return () => window.removeEventListener('keydown', handleKeyDown)
  }, [undo, redo])

  async function loadGame() {
    try {
      const game: GameState = await invoke('get_current_game')
      setGameState(game)
      setIsDirty(false)
    } catch (error) {
      console.error('Failed to load game:', error)
      // Create new game if load fails
      newGame()
    }
  }

  async function saveGame() {
    try {
      await invoke('save_game', { game: gameState })
      setIsDirty(false)
    } catch (error) {
      console.error('Failed to save game:', error)
    }
  }

  async function newGame() {
    try {
      const game: GameState = await invoke('new_game', { clubId: 'default' })
      setGameState(game)
      setIsDirty(false)
    } catch (error) {
      console.error('Failed to create new game:', error)
    }
  }

  const screenProps = {
    gameState,
    setGameState,
    theme: 'historical_1888',
    advanceDay: advanceDayOrContinue,
    hasMatchesToday,
  }

  // Show game selection if no game is selected yet
  if (!selectedGame) {
    return <GameSelectionScreen onSelectGame={handleGameSelect} />
  }

  // Show database editor if selected
  if (selectedGame === 'database-editor') {
    return <DatabaseEditorScreen />
  }

  // Show sprite editor if selected
  if (selectedGame === 'sprite-editor') {
    return <SpriteEditorScreen onBack={() => handleGameSelect(null)} />
  }

  // Quick Match — team select → live match engine
  if (selectedGame === 'quick-match') {
    return <LiveMatchScreen onBack={() => handleGameSelect(null)} />
  }

  // Show GameplayScreenV2 (V2 is now the default) if Sheffield Rules is selected
  if (selectedGame === 'sheffield-rules' && !isInGameplay) {
    return (
      <GameplayScreenV2
        selectedYear={selectedYear || 1867}
        gameMode={selectedGameMode}
        selectedClub={selectedClub}
        onBack={() => handleGameSelect(null)}
        onSelectGameMode={handleSheffieldGameModeSelected}
        showClubSelection={showClubSelection}
        onClubSelected={handleClubSelected}
        onClubSelectionBack={handleClubSelectionBack}
        isLoading={isLoading}
        loadingProgress={loadingProgress}
        loadingMessage={loadingMessage}
      />
    )
  }

  // Show gameplay screen if in gameplay mode
  if (selectedGame === 'sheffield-rules' && isInGameplay && selectedYear !== null) {
    return (
      <GameplayScreen
        gameState={gameState}
        setGameState={setGameState}
        onBack={handleBackFromGameplay}
      />
    )
  }

  return (
    <>
    <div className="app-container">
      <div className="sidebar">
        <h2>{selectedGame === 'saturday-at-three' ? 'Saturday at Three' : 'Sheffield Rules'}</h2>

        {selectedGame === 'saturday-at-three' && (
          <div className="club-selector">
            <label htmlFor="club-select">Select Club:</label>
            <select
              id="club-select"
              value={gameState.userClubId}
              onChange={(e) => setGameState(prev => ({ ...prev, userClubId: e.target.value }))}
            >
              <option value="accrington">Accrington FC</option>
              <option value="aston-villa">Aston Villa</option>
              <option value="blackburn">Blackburn Rovers</option>
              <option value="bolton">Bolton Wanderers</option>
              <option value="burnley">Burnley FC</option>
              <option value="derby">Derby County</option>
              <option value="everton">Everton</option>
              <option value="notts-county">Notts County</option>
              <option value="preston">Preston North End</option>
              <option value="stoke">Stoke City</option>
              <option value="sunderland">Sunderland AFC</option>
              <option value="wolves">Wolverhampton Wanderers</option>
            </select>
          </div>
        )}

        <nav className="main-nav">
          <button
            className={currentScreen === 'dashboard' ? 'active' : ''}
            onClick={() => setCurrentScreen('dashboard')}
          >
            Dashboard
          </button>
          <button
            className={currentScreen === 'matchday' ? 'active' : ''}
            onClick={() => setCurrentScreen('matchday')}
          >
            Matchday
          </button>
          <button
            className={currentScreen === 'squad' ? 'active' : ''}
            onClick={() => setCurrentScreen('squad')}
          >
            Squad
          </button>
          <button
            className={currentScreen === 'standings' ? 'active' : ''}
            onClick={() => setCurrentScreen('standings')}
          >
            🔥🔥🔥 TESTING 🔥🔥🔥
          </button>
          <button
            className={currentScreen === 'matchviewer' ? 'active' : ''}
            onClick={() => setCurrentScreen('matchviewer')}
          >
            Match Viewer
          </button>
          <button
            className="main-menu-btn"
            onClick={() => handleGameSelect(null)}
            title="Return to game selection"
          >
            Main Menu
          </button>
        </nav>

        <div className="game-info">
          <div className="info-row">
            <span>Season: {gameState.season}</span>
          </div>
          <div className="info-row">
            <span>Date: {formatDate(gameState.currentDate)}</span>
          </div>
          <div className="info-row">
            <span>Gameweek: {gameState.currentGameweek}</span>
          </div>
          <div className="info-row">
            {isDirty ? '• Unsaved' : '✓ Saved'}
          </div>
        </div>

        <button className="advance-day-btn" onClick={advanceDay} title={hasMatchesToday ? "Continue to match day" : "Advance one day"}>
          <span className="calendar-icon">📅</span>
          <span className="btn-text">{hasMatchesToday ? 'Continue' : 'Next Day'}</span>
        </button>

        <div className="toolbar">
          <button
            onClick={() => newGame()}
            title="Ctrl+N"
          >
            New Game
          </button>
          <button
            onClick={() => saveGame()}
            title="Ctrl+S"
          >
            Save
          </button>
          <button
            onClick={undo}
            disabled={!canUndo}
            title="Ctrl+Z"
          >
            Undo
          </button>
          <button
            onClick={redo}
            disabled={!canRedo}
            title="Ctrl+Y"
          >
            Redo
          </button>
        </div>
      </div>

      <div className="main-content">
        <Suspense fallback={<PanelLoadingFallback />}>
          {currentScreen === 'dashboard' && (
            <DashboardScreen {...screenProps} />
          )}
          {currentScreen === 'matchday' && (
            <MatchdayScreen {...screenProps} />
          )}
          {currentScreen === 'squad' && (
            <SquadScreen {...screenProps} />
          )}
          {currentScreen === 'standings' && (
            <StandingsScreen {...screenProps} />
          )}
          {currentScreen === 'matchviewer' && (
            <MatchViewerScreen {...screenProps} />
          )}
        </Suspense>
      </div>

      {dayProcessing && (
        <DayResultsModal
          results={dayProcessing}
          onClose={completeDayProcessing}
          gameState={gameState}
        />
      )}

      {matchdayModal && (
        <MatchdayModeModal
          gameState={gameState}
          matchesForDay={matchdayModal.matches}
          onComplete={(updated) => {
            setGameState(updated)
            setMatchdayModal(null)
            setHasMatchesToday(false)  // Reset button back to "Next Day"
          }}
          onClose={() => {
            setMatchdayModal(null)
            setHasMatchesToday(false)
          }}
        />
      )}
    </div>

    {/* DEBUG OVERLAY — Shows scaling information */}
    {/* Positioned outside app-container to not be affected by scaling */}
    <div className="debug-overlay">
      <div>Scale: {scale.toFixed(2)}×</div>
      <div>DPR: {window.devicePixelRatio.toFixed(2)}</div>
      <div>Pixel-perfect: {pixelPerfect ? 'ON' : 'OFF'}</div>
      <button
        onClick={() => setPixelPerfect(v => !v)}
        style={{
          marginTop: '4px',
          padding: '2px 4px',
          fontSize: '10px',
          background: '#444',
          color: '#fff',
          border: 'none',
          borderRadius: '2px',
          cursor: 'pointer'
        }}
      >
        Toggle Pixel-Perfect
      </button>
    </div>

    {/* LOADING OVERLAY — Shows during game initialization */}
    <LoadingOverlay
      isVisible={isLoading}
      progress={loadingProgress}
      message={loadingMessage}
    />
    </>
  )
}
