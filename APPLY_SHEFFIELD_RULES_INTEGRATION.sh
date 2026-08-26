#!/bin/bash
# Script to integrate Sheffield Rules Year Selector into App.tsx

APP_FILE="/d/projects/Saturday at Three/frontend/src/App.tsx"

# Create a new App.tsx with Sheffield Rules integration
cat > "${APP_FILE}" << 'EOF'
import React, { useEffect, useState, lazy, Suspense } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { GameState, initialGameState, DayProcessingResult } from './types/GameState'
import { useHistory } from './hooks/useHistory'
import GameSelectionScreen from './screens/GameSelectionScreen'
import { SheffieldRulesSetupScreen } from './screens/SheffieldRulesSetupScreen'
import { DayResultsModal } from './components/DayResultsModal'
import './index.css'

// Lazy load screens
const DashboardScreen = lazy(() => import('./screens/DashboardScreen').then(m => ({ default: m.DashboardScreen })))
const MatchdayScreen = lazy(() => import('./screens/MatchdayScreen').then(m => ({ default: m.MatchdayScreen })))
const SquadScreen = lazy(() => import('./screens/SquadScreen').then(m => ({ default: m.SquadScreen })))
const StandingsScreen = lazy(() => import('./screens/StandingsScreen').then(m => ({ default: m.StandingsScreen })))

type ScreenName = 'dashboard' | 'matchday' | 'squad' | 'standings'
type GameType = 'sheffield-rules' | 'saturday-at-three' | null

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
  const handleSheffieldGameModeSelected = (gameMode: string, year?: number) => {
    // TODO: In future, create Sheffield Rules game with selected gameMode and year
    // For now, just mark setup as complete to move forward
    console.log(`Sheffield Rules selected: gameMode=${gameMode}, year=${year}`)
    setSheffieldSetupComplete(true)
    setGameState(prev => ({ ...prev, userClubId: 'sheffield-default', season: year || 1867 }))
  }

  // Advance game by one day
  const advanceDay = async () => {
    try {
      setIsProcessingDay(true)
      const result: DayProcessingResult = await invoke('advance_day', { game: gameState })

      // Reload game state to get updated matches/standings
      const updatedGame: GameState = await invoke('get_current_game')
      setGameState(updatedGame)

      // Always return to dashboard after advancing day
      setCurrentScreen('dashboard')

      // Show results modal if there are auto-processed events
      if (result.autoProcessed.length > 0 || result.requireUserAction.length > 0) {
        setDayProcessing(result)
      }

      // If user team has a match today, show indication in modal
      // (user can navigate to matchday from dashboard if needed)
      const userMatch = result.requireUserAction.find(e =>
        e.eventType.type === 'Match' && e.eventType.data.isUserTeam
      )
      // userMatch info is shown in the modal, no need for redirect

      setIsDirty(false)
    } catch (error) {
      console.error('Failed to advance day:', error)
    } finally {
      setIsProcessingDay(false)
    }
  }

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
  }

  // Show game selection if no game is selected yet
  if (!selectedGame) {
    return <GameSelectionScreen onSelectGame={handleGameSelect} />
  }

  // Show Sheffield Rules setup screen if selected but not complete
  if (selectedGame === 'sheffield-rules' && !sheffieldSetupComplete) {
    return (
      <SheffieldRulesSetupScreen
        onGameModeSelected={handleSheffieldGameModeSelected}
        onBack={() => handleGameSelect(null)}
      />
    )
  }

  return (
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
            Standings
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

        <button className="advance-day-btn" onClick={advanceDay} title="Advance one day">
          <span className="calendar-icon">📅</span>
          <span className="btn-text">Next Day</span>
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
        </Suspense>
      </div>

      {dayProcessing && (
        <DayResultsModal
          results={dayProcessing}
          onClose={completeDayProcessing}
          gameState={gameState}
        />
      )}
    </div>
  )
}
EOF

echo "App.tsx has been updated with Sheffield Rules integration!"
echo "Key changes:"
echo "1. ✅ Imported SheffieldRulesSetupScreen"
echo "2. ✅ Added sheffieldSetupComplete state"
echo "3. ✅ Added handleSheffieldGameModeSelected callback"
echo "4. ✅ Added routing to SheffieldRulesSetupScreen when Sheffield Rules selected"
echo "5. ✅ Only shows main game when setup is complete"
