import React, { useState, useEffect } from 'react'
import { invoke } from '../utils/tauriInvoke'
import { GameState } from '../types/GameState'

// Rating bar component
const RatingBar = ({ value, max = 20 }: { value: number; max?: number }) => {
  const percentage = (value / max) * 100
  let color = '#4CAF50' // Green
  if (percentage < 33) {
    color = '#f44336' // Red
  } else if (percentage < 66) {
    color = '#f39c12' // Orange
  }

  return (
    <div style={{ display: 'flex', alignItems: 'center', gap: '6px', fontSize: '0.85em' }}>
      <div style={{
        width: '60px',
        height: '20px',
        backgroundColor: '#333',
        borderRadius: '3px',
        overflow: 'hidden',
        border: '1px solid #666'
      }}>
        <div style={{
          width: `${percentage}%`,
          height: '100%',
          backgroundColor: color,
          transition: 'width 0.3s ease'
        }} />
      </div>
      <span style={{ minWidth: '25px', textAlign: 'right' }}>{value}</span>
    </div>
  )
}

interface PlayerDetail {
  id: string
  name: string
  club_id: string
  position: string
  birth_year: number
  age: number
  nationality: string
  pace: number
  strength: number
  stamina: number
  balance: number
  jumping: number
  agility: number
  passing: number
  dribbling: number
  heading: number
  crossing: number
  tackling: number
  handling: number
  reflexes: number
  courage: number
  concentration: number
  leadership: number
  aggression: number
  determination: number
  flair: number
  influence: number
  awareness: number
  marking: number
  positioning: number
  work_rate: number
  finishing: number
  penalties: number
  set_pieces: number
}

interface ClubInfo {
  id: string
  name: string
  short_name: string
  founded_year: number
  ground_name: string
  ground_capacity: number
  city: string
  region: string
  primary_color: string
  secondary_color: string
}

interface SquadScreenProps {
  gameState: GameState
  setGameState: (state: GameState | ((prev: GameState) => GameState)) => void
  theme: string
}

export function SquadScreen({ gameState, setGameState, theme }: SquadScreenProps) {
  const [squad, setSquad] = useState<PlayerDetail[]>([])
  const [clubInfo, setClubInfo] = useState<ClubInfo | null>(null)
  const [selectedPlayer, setSelectedPlayer] = useState<PlayerDetail | null>(null)
  const [loading, setLoading] = useState(true)
  const [showReserves, setShowReserves] = useState(false)
  const [reserveTeamId, setReserveTeamId] = useState<string | null>(null)
  const [currentClubId, setCurrentClubId] = useState<string>(gameState.userClubId)

  useEffect(() => {
    loadSquad()
  }, [gameState.userClubId])

  async function loadSquad() {
    try {
      setLoading(true)
      const [club, players, reserveId] = await Promise.all([
        invoke<ClubInfo>('get_club_info', { clubId: gameState.userClubId }),
        invoke<PlayerDetail[]>('get_club_squad', { clubId: gameState.userClubId }),
        invoke<string | null>('get_reserve_team_id', { parentClubId: gameState.userClubId }),
      ])
      setClubInfo(club)
      setSquad(players)
      setReserveTeamId(reserveId)
      setCurrentClubId(gameState.userClubId)
      setShowReserves(false)
    } catch (error) {
      console.error('Failed to load squad:', error)
    } finally {
      setLoading(false)
    }
  }

  async function toggleTeam() {
    if (!reserveTeamId) return

    try {
      setLoading(true)
      const newClubId = showReserves ? gameState.userClubId : reserveTeamId
      const [club, players] = await Promise.all([
        invoke<ClubInfo>('get_club_info', { clubId: newClubId }),
        invoke<PlayerDetail[]>('get_club_squad', { clubId: newClubId })
      ])
      setClubInfo(club)
      setSquad(players)
      setCurrentClubId(newClubId)
      setShowReserves(!showReserves)
    } catch (error) {
      console.error('Failed to switch team:', error)
    } finally {
      setLoading(false)
    }
  }

  async function handlePromoteRelegate(player: PlayerDetail) {
    if (!reserveTeamId) return

    try {
      setLoading(true)
      // If viewing reserves (B team), promote to first team (A team)
      // If viewing first team (A team), relegate to reserves (B team)
      const targetClubId = showReserves ? gameState.userClubId : reserveTeamId

      console.log(`Moving player ${player.name} to club ${targetClubId}`)
      await invoke('transfer_player', {
        playerId: player.id,
        newClubId: targetClubId
      })

      // Reload the current squad
      const players = await invoke<PlayerDetail[]>('get_club_squad', { clubId: currentClubId })
      setSquad(players)
    } catch (error) {
      console.error('Failed to transfer player:', error)
      alert(`Failed to transfer player: ${error}`)
    } finally {
      setLoading(false)
    }
  }

  // Sort players by position for consistent display
  const sortedSquad = [...squad].sort((a, b) => {
    const positionOrder: Record<string, number> = { GK: 1, CB: 2, FB: 3, MID: 4, FWD: 5, WG: 6 }
    const orderA = positionOrder[a.position] || 99
    const orderB = positionOrder[b.position] || 99
    if (orderA !== orderB) return orderA - orderB
    return a.name.localeCompare(b.name)
  })

  if (loading) {
    return <div className="squad-screen"><p>Loading squad...</p></div>
  }

  return (
    <div className="squad-screen" style={{ overflow: selectedPlayer ? 'hidden' : 'auto' }}>
      {clubInfo && (
        <div className="squad-header">
          <div className="club-title-line">
            <h1>{clubInfo.name}</h1>
            <span className="founded-year">Founded {clubInfo.founded_year}</span>
            {reserveTeamId && (
              <button
                onClick={selectedPlayer ? () => setSelectedPlayer(null) : toggleTeam}
                className="view-btn"
                style={{ marginLeft: 'auto' }}
                disabled={loading}
              >
                {selectedPlayer ? 'Back to Squad' : (showReserves ? 'First Team' : 'Reserve Team')}
              </button>
            )}
          </div>
        </div>
      )}

      {selectedPlayer ? (
        <div className="player-detail-panel">
          <div className="player-info" style={{ fontSize: '0.85rem', color: '#000000', width: 'calc(100% + 60px)', marginLeft: '-30px', marginRight: '-30px', paddingLeft: '30px', paddingRight: '30px', paddingBottom: '0.5rem' }}>
            <h2 style={{ fontSize: '1.4rem', marginBottom: '0.5rem' }}>{selectedPlayer.name}</h2>
            <div style={{ display: 'flex', gap: '0.8rem', width: '100%', flexWrap: 'nowrap' }}>
              <div style={{ minWidth: '60px' }}>
                <strong>Position</strong><br />
                {selectedPlayer.position}
              </div>
              <div style={{ minWidth: '40px' }}>
                <strong>Age</strong><br />
                {selectedPlayer.age}
              </div>
              {selectedPlayer.profession && (
                <div style={{ flex: '1 1 auto', minWidth: '100px', whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>
                  <strong>Occupation</strong><br />
                  <span title={selectedPlayer.profession}>{selectedPlayer.profession}</span>
                </div>
              )}
              {selectedPlayer.parish && (
                <div style={{ minWidth: '80px' }}>
                  <strong>Parish</strong><br />
                  {selectedPlayer.parish}
                </div>
              )}
              {selectedPlayer.address && (
                <div style={{ flex: '1 1 auto', minWidth: '100px' }}>
                  <strong>Address</strong><br />
                  {selectedPlayer.address}
                </div>
              )}
              {selectedPlayer.birthplace && (
                <div style={{ minWidth: '80px' }}>
                  <strong>Birthplace</strong><br />
                  {selectedPlayer.birthplace.split(',')[0].trim()}
                </div>
              )}
            </div>
          </div>

          <div className="player-attributes">
            <div className="attribute-group">
              <div className="attr">Aggression: <RatingBar value={selectedPlayer.aggression} /></div>
              <div className="attr">Agility: <RatingBar value={selectedPlayer.agility} /></div>
              <div className="attr">Awareness: <RatingBar value={selectedPlayer.awareness} /></div>
              <div className="attr">Balance: <RatingBar value={selectedPlayer.balance} /></div>
              <div className="attr">Concentration: <RatingBar value={selectedPlayer.concentration} /></div>
              <div className="attr">Courage: <RatingBar value={selectedPlayer.courage} /></div>
              <div className="attr">Crossing: <RatingBar value={selectedPlayer.crossing} /></div>
              <div className="attr">Determination: <RatingBar value={selectedPlayer.determination} /></div>
              <div className="attr">Dribbling: <RatingBar value={selectedPlayer.dribbling} /></div>
            </div>

            <div className="attribute-group">
              <div className="attr">Finishing: <RatingBar value={selectedPlayer.finishing} /></div>
              <div className="attr">Flair: <RatingBar value={selectedPlayer.flair} /></div>
              <div className="attr">Handling: <RatingBar value={selectedPlayer.handling} /></div>
              <div className="attr">Heading: <RatingBar value={selectedPlayer.heading} /></div>
              <div className="attr">Influence: <RatingBar value={selectedPlayer.influence} /></div>
              <div className="attr">Jumping: <RatingBar value={selectedPlayer.jumping} /></div>
              <div className="attr">Leadership: <RatingBar value={selectedPlayer.leadership} /></div>
              <div className="attr">Marking: <RatingBar value={selectedPlayer.marking} /></div>
              <div className="attr">Pace: <RatingBar value={selectedPlayer.pace} /></div>
            </div>

            <div className="attribute-group">
              <div className="attr">Passing: <RatingBar value={selectedPlayer.passing} /></div>
              <div className="attr">Penalties: <RatingBar value={selectedPlayer.penalties} /></div>
              <div className="attr">Positioning: <RatingBar value={selectedPlayer.positioning} /></div>
              <div className="attr">Reflexes: <RatingBar value={selectedPlayer.reflexes} /></div>
              <div className="attr">Set Pieces: <RatingBar value={selectedPlayer.set_pieces} /></div>
              <div className="attr">Stamina: <RatingBar value={selectedPlayer.stamina} /></div>
              <div className="attr">Strength: <RatingBar value={selectedPlayer.strength} /></div>
              <div className="attr">Tackling: <RatingBar value={selectedPlayer.tackling} /></div>
              <div className="attr">Work Rate: <RatingBar value={selectedPlayer.work_rate} /></div>
            </div>
          </div>
        </div>
      ) : (
        <table className="squad-table">
          <thead>
            <tr>
              <th>Name</th>
              <th>Pos</th>
              <th>Pace</th>
              <th>Strength</th>
              <th>Passing</th>
              <th>Dribbling</th>
              <th>Finishing</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {sortedSquad.map(player => (
              <tr key={player.id}>
                <td><strong>{player.name}</strong></td>
                <td><span className="position-badge">{player.position}</span></td>
                <td><RatingBar value={player.pace} /></td>
                <td><RatingBar value={player.strength} /></td>
                <td><RatingBar value={player.passing} /></td>
                <td><RatingBar value={player.dribbling} /></td>
                <td><RatingBar value={player.finishing} /></td>
                <td style={{ paddingLeft: '8px' }}>
                  <div style={{ display: 'flex', gap: '4px' }}>
                    <button
                      onClick={() => setSelectedPlayer(player)}
                      className="view-btn"
                      style={{ fontSize: '0.75em', padding: '6px 8px' }}
                    >
                      View Stats
                    </button>
                    {reserveTeamId && (
                      <button
                        onClick={() => handlePromoteRelegate(player)}
                        className="view-btn"
                        style={{ fontSize: '0.75em', padding: '6px 8px' }}
                        disabled={loading}
                      >
                        {showReserves ? 'Promote' : 'Relegate'}
                      </button>
                    )}
                  </div>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      )}

      {!selectedPlayer && (
        <div className="squad-stats">
          <p><strong>Total Squad:</strong> {squad.length} players</p>
        </div>
      )}
    </div>
  )
}

export default SquadScreen
