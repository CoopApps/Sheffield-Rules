import React, { useState } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { GameState } from '../types/GameState'
import '../styles/standings-screen.css'

interface StandingsScreenProps {
  gameState: GameState
  setGameState: (state: GameState | ((prev: GameState) => GameState)) => void
  theme: string
}

interface DivisionTable {
  division_id: string
  division_name: string
  clubs: Array<{
    club_id: string
    position: number
    name: string
    is_reserve: boolean
  }>
}

export function StandingsScreen({ gameState, setGameState, theme }: StandingsScreenProps) {
  // Determine if rouges era (1862-1868)
  const season = gameState.season || 1888
  const hasRouges = season >= 1862 && season <= 1868

  // State for division selection (for Sheffield & Hallamshire League)
  const [selectedDivision, setSelectedDivision] = useState<string | null>(null)
  const [divisionStandings, setDivisionStandings] = useState<any[]>([])
  const [isLoadingDivision, setIsLoadingDivision] = useState(false)
  const [currentDivision, setCurrentDivision] = useState<any>(null)
  const [isLoadingDivisionInfo, setIsLoadingDivisionInfo] = useState(false)

  // Check if this is Sheffield & Hallamshire League mode
  const isFantasyLeague = true  // HARDCODED FOR TESTING
  // const isFantasyLeague = gameState.gameMode === 'sheffield-hallamshire-league'

  // EMERGENCY DEBUG - Log on every render
  console.log('🚨 RENDER - isFantasyLeague:', isFantasyLeague, 'gameMode:', gameState.gameMode)

  // DEBUG LOGGING - FULL GAMESTATE INSPECTION
  console.log('═══════════════════════════════════════════════════════════')
  console.log('StandingsScreen DEBUG:')
  console.log('───────────────────────────────────────────────────────────')
  console.log('gameState =', JSON.stringify(gameState, null, 2))
  console.log('───────────────────────────────────────────────────────────')
  console.log('gameState.gameMode =', gameState.gameMode)
  console.log('typeof gameState.gameMode =', typeof gameState.gameMode)
  console.log('gameState.gameMode === "sheffield-hallamshire-league" =', gameState.gameMode === 'sheffield-hallamshire-league')
  console.log('───────────────────────────────────────────────────────────')
  console.log('isFantasyLeague =', isFantasyLeague)
  console.log('gameState.clubs.length =', gameState.clubs.length)
  console.log('gameState.standings.length =', gameState.standings.length)
  console.log('gameState.season =', gameState.season)
  console.log('gameState.userClubId =', gameState.userClubId)
  console.log('═══════════════════════════════════════════════════════════')

  // Load current club's division info and standings on mount
  React.useEffect(() => {
    if (isFantasyLeague && gameState.userClubId) {
      setIsLoadingDivisionInfo(true)
      invoke<any>('get_club_division_info', {
        clubId: gameState.userClubId
      })
        .then((divInfo) => {
          setCurrentDivision(divInfo)
          // Automatically load the user's division standings
          return loadDivisionStandings(divInfo.division_id)
        })
        .catch((error: any) => console.error('Failed to load division info:', error))
        .finally(() => setIsLoadingDivisionInfo(false))
    }
  }, [isFantasyLeague, gameState.userClubId])

  const loadDivisionStandings = async (divisionId: string) => {
    console.log('🔥🔥🔥 loadDivisionStandings CALLED with divisionId:', divisionId, 'season:', season)
    setIsLoadingDivision(true)
    try {
      console.log('🔥 About to call get_division_standings...')
      const standings = await invoke<Array<any>>('get_division_standings', {
        divisionId: divisionId,
        season: season
      })
      console.log('🔥 Division standings received:', standings)
      console.log('🔥 Standings length:', standings.length)
      if (standings.length > 0) {
        console.log('🔥 First standing object:', standings[0])
      }
      setDivisionStandings(standings)
      setSelectedDivision(divisionId)
      console.log('🔥 Division standings state updated')
    } catch (error) {
      console.error('🔥 ERROR: Failed to load division standings:', error)
      setDivisionStandings([])
    } finally {
      setIsLoadingDivision(false)
      console.log('🔥 loadDivisionStandings COMPLETE')
    }
  }

  const handleDivisionChange = async (divisionId: string) => {
    console.log('🎯🎯🎯 handleDivisionChange CALLED with divisionId:', divisionId)
    await loadDivisionStandings(divisionId)
    console.log('🎯 handleDivisionChange COMPLETE')
  }

  const goBackToMyDivision = () => {
    if (currentDivision) {
      loadDivisionStandings(currentDivision.division_id)
    }
  }

  return (
    <div className="standings-screen">
      <h1>League Standings</h1>

      <div className="standings-header">
        {/* Top row with club name and division */}
        {isFantasyLeague && (
          <div className="standings-info">
            <div className="info-box">
              <div className="info-label">Club</div>
              <div className="info-value">
                {gameState.clubs.find(c => c.id === gameState.userClubId)?.name || 'Unknown'}
              </div>
            </div>
            {currentDivision && (
              <div className="info-box">
                <div className="info-label">Your Division</div>
                <div className="info-value">{currentDivision.division_name}</div>
              </div>
            )}
          </div>
        )}

        {/* Season and date info */}
        <div className="standings-info">
          <div className="info-box">
            <div className="info-label">Season</div>
            <div className="info-value">{season}-{season % 100 + 1}</div>
          </div>
          <div className="info-box">
            <div className="info-label">Current Date</div>
            <div className="info-value">{gameState.currentDate ? gameState.currentDate.split('-')[1] + ' ' + new Date(gameState.currentDate).toLocaleString('en-US', { month: 'short' }) + ' ' + gameState.currentDate.split('-')[0] : 'Sep 1, ' + season}</div>
          </div>
        </div>

        {isFantasyLeague && (
          <div className="division-selector">
            <label htmlFor="division-select">View Other Divisions:</label>
            <select
              id="division-select"
              value={selectedDivision || ''}
              onChange={(e) => handleDivisionChange(e.target.value)}
            >
              <option value="">-- Select a Division --</option>
              <option value="div-1">First Division</option>
              <option value="div-2">Second Division</option>
              <option value="div-3">Third Division</option>
              <option value="div-4">Fourth Division</option>
              <option value="div-5a">Fifth Division A</option>
              <option value="div-5b">Fifth Division B</option>
              <option value="div-6a">Sixth Division West</option>
              <option value="div-6b">Sixth Division East</option>
              <option value="div-6c">Sixth Division North</option>
              <option value="div-6d">Sixth Division South</option>
              <option value="res-div-1">Reserve Division 1</option>
              <option value="res-div-2">Reserve Division 2</option>
              <option value="res-div-3">Reserve Division 3</option>
              <option value="res-div-4">Reserve Division 4</option>
              <option value="res-div-5a">Reserve Division 5A</option>
              <option value="res-div-5b">Reserve Division 5B</option>
              <option value="res-div-6a">Reserve Division 6A</option>
              <option value="res-div-6b">Reserve Division 6B</option>
              <option value="res-div-6c">Reserve Division 6C</option>
              <option value="res-div-6d">Reserve Division 6D</option>
            </select>
            {selectedDivision && currentDivision && selectedDivision !== currentDivision.division_id && (
              <button className="back-to-my-division-btn" onClick={goBackToMyDivision}>
                Back to My Division
              </button>
            )}
          </div>
        )}
      </div>

      {isFantasyLeague && (
        <div className="standings-legend">
          <button className="legend-btn" onClick={() => { alert('CLICK DETECTED: div-1'); handleDivisionChange('div-1') }}>First Division</button>
          <button className="legend-btn" onClick={() => { alert('CLICK DETECTED: div-2'); handleDivisionChange('div-2') }}>Second Division</button>
          <button className="legend-btn" onClick={() => { alert('CLICK DETECTED: div-3'); handleDivisionChange('div-3') }}>Third Division</button>
          <button className="legend-btn" onClick={() => handleDivisionChange('div-4')}>Fourth Division</button>
          <button className="legend-btn" onClick={() => handleDivisionChange('div-5a')}>Fifth Division A</button>
          <button className="legend-btn" onClick={() => handleDivisionChange('div-5b')}>Fifth Division B</button>
          <button className="legend-btn" onClick={() => handleDivisionChange('div-6a')}>Sixth Division West</button>
          <button className="legend-btn" onClick={() => handleDivisionChange('div-6b')}>Sixth Division East</button>
          <button className="legend-btn" onClick={() => handleDivisionChange('div-6c')}>Sixth Division North</button>
          <button className="legend-btn" onClick={() => handleDivisionChange('div-6d')}>Sixth Division South</button>
        </div>
      )}

      <div className="standings-table-container">
        {/* DEBUG INFO */}
        <div style={{ background: '#000', color: '#0f0', padding: '10px', fontFamily: 'monospace', fontSize: '12px', marginBottom: '10px' }}>
          <strong>DEBUG INFO:</strong><br/>
          isFantasyLeague: {isFantasyLeague ? 'TRUE' : 'FALSE'}<br/>
          gameState.gameMode: {String(gameState.gameMode)}<br/>
          divisionStandings.length: {divisionStandings.length}<br/>
          gameState.standings.length: {gameState.standings.length}<br/>
          selectedDivision: {selectedDivision || 'null'}<br/>
          isLoadingDivision: {isLoadingDivision ? 'TRUE' : 'FALSE'}
        </div>

        {isLoadingDivision ? (
          <p>Loading division standings...</p>
        ) : isFantasyLeague ? (
          // For fantasy league, show division standings
          divisionStandings.length === 0 ? (
            <p>No standings data yet</p>
          ) : (
            <table className="standings-table">
              <thead>
                <tr>
                  <th>Pos</th>
                  <th>Club</th>
                  <th>P</th>
                  <th>W</th>
                  <th>D</th>
                  <th>L</th>
                  <th>F</th>
                  <th>A</th>
                  {hasRouges && <th>RF</th>}
                  {hasRouges && <th>RA</th>}
                  <th>GD</th>
                  <th>Pts</th>
                </tr>
              </thead>
              <tbody>
                {divisionStandings.map((standing: any, index: number) => {
                  const position = standing.position || (index + 1)
                  let zoneClass = ''
                  const totalTeams = divisionStandings.length
                  if (position <= 2) {
                    zoneClass = 'promotion-zone'
                  } else if (position > totalTeams - 2) {
                    zoneClass = 'relegation-zone'
                  } else {
                    zoneClass = 'mid-table'
                  }
                  const isUserTeam = standing.club_id === gameState.userClubId ? 'user-team' : ''

                  return (
                    <tr key={standing.club_id} className={`${zoneClass} ${isUserTeam}`}>
                      <td className="position">{position}</td>
                      <td className="team-name">{standing.club_name}</td>
                      <td>{standing.played}</td>
                      <td>{standing.won}</td>
                      <td>{standing.drawn}</td>
                      <td>{standing.lost}</td>
                      <td>{standing.goals_for}</td>
                      <td>{standing.goals_against}</td>
                      {hasRouges && <td>{standing.rouges_for}</td>}
                      {hasRouges && <td>{standing.rouges_against}</td>}
                      <td className="goal-diff">{standing.goal_difference > 0 ? '+' : ''}{standing.goal_difference}</td>
                      <td className="points"><strong>{standing.points}</strong></td>
                    </tr>
                  )
                })}
              </tbody>
            </table>
          )
        ) : (
          // For other modes, show game state standings
          gameState.standings.length === 0 ? (
            <p>No standings data yet</p>
          ) : (
            <table className="standings-table">
              <thead>
                <tr>
                  <th>Pos</th>
                  <th>Club</th>
                  <th>P</th>
                  <th>W</th>
                  <th>D</th>
                  <th>L</th>
                  <th>F</th>
                  <th>A</th>
                  {hasRouges && <th>R</th>}
                  <th>GD</th>
                  <th>Pts</th>
                </tr>
              </thead>
              <tbody>
                {gameState.standings.map((standing, index) => {
                  const position = index + 1
                  let zoneClass = ''
                  const totalTeams = gameState.standings.length
                  if (position <= 4) {
                    zoneClass = 'promotion-zone'
                  } else if (position > totalTeams - 2) {
                    zoneClass = 'relegation-zone'
                  } else {
                    zoneClass = 'mid-table'
                  }
                  const isUserTeam = standing.clubId === gameState.userClubId ? 'user-team' : ''

                  return (
                    <tr key={standing.clubId} className={`${zoneClass} ${isUserTeam}`}>
                      <td className="position">{position}</td>
                      <td className="team-name">{standing.clubName}</td>
                      <td>{standing.played}</td>
                      <td>{standing.won}</td>
                      <td>{standing.drawn}</td>
                      <td>{standing.lost}</td>
                      <td>{standing.goalsFor}</td>
                      <td>{standing.goalsAgainst}</td>
                      {hasRouges && <td>{standing.rougesFor}</td>}
                      <td className="goal-diff">{standing.goalDifference > 0 ? '+' : ''}{standing.goalDifference}</td>
                      <td className="points"><strong>{standing.points}</strong></td>
                    </tr>
                  )
                })}
              </tbody>
            </table>
          )
        )}
      </div>
    </div>
  )
}

export default StandingsScreen
