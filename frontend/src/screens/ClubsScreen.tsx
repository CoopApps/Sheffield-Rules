import React, { useState, useEffect } from 'react'
import { invoke } from '../utils/tauriInvoke'
import { GameState } from '../types/GameState'
import { SponsoredCupsPanel } from '../components/SponsoredCupsPanel'

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
  profession?: string
  parish?: string
  address?: string
  birthplace?: string
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

interface ClubsScreenProps {
  gameState: GameState
  setGameState: (state: GameState | ((prev: GameState) => GameState)) => void
  theme: string
  renderRightPanel?: () => React.ReactNode
  selectedAction?: string | null
  setSelectedAction?: (action: string | null) => void
}

export function ClubsScreen({ gameState, setGameState, theme, renderRightPanel, selectedAction: externalSelectedAction, setSelectedAction: externalSetSelectedAction }: ClubsScreenProps) {
  const [clubs, setClubs] = useState<ClubInfo[]>([])
  const [selectedClubId, setSelectedClubId] = useState<string | null>(null)
  const [squad, setSquad] = useState<PlayerDetail[]>([])
  const [clubInfo, setClubInfo] = useState<ClubInfo | null>(null)
  const [loading, setLoading] = useState(true)

  // Challenge letter state
  const [matchType, setMatchType] = useState('friendly')
  const [venue, setVenue] = useState('home')
  const [stakes, setStakes] = useState('honor')
  const [tone, setTone] = useState('cordial')
  const [proposedDate, setProposedDate] = useState<string>('')
  const [rulesType, setRulesType] = useState('sheffield')
  const [matchDuration, setMatchDuration] = useState('90')
  const [userClubInfo, setUserClubInfo] = useState<ClubInfo | null>(null)
  const [showSentLetterModal, setShowSentLetterModal] = useState(false)
  const [sentLetterRecipient, setSentLetterRecipient] = useState<string>('')

  // Use external selectedAction if provided, otherwise use local state
  const selectedAction = externalSelectedAction !== undefined ? externalSelectedAction : null
  const setSelectedAction = externalSetSelectedAction || (() => {})

  useEffect(() => {
    loadClubs()
    loadUserClub()
    // Initialize proposed date to 14 days from current game date
    const currentDate = new Date(gameState.currentDate + 'T00:00:00')
    const defaultDate = new Date(currentDate)
    defaultDate.setDate(defaultDate.getDate() + 14)
    setProposedDate(defaultDate.toISOString().split('T')[0])
  }, [])

  useEffect(() => {
    if (selectedClubId) {
      loadClubSquad()
    }
  }, [selectedClubId])

  async function loadClubs() {
    try {
      setLoading(true)
      const clubList = await invoke<ClubInfo[]>('get_all_clubs', {})
      setClubs(clubList)
      if (clubList.length > 0) {
        setSelectedClubId(clubList[0].id)
      }
    } catch (error) {
      console.error('Failed to load clubs:', error)
    } finally {
      setLoading(false)
    }
  }

  async function loadUserClub() {
    try {
      if (!gameState.userClubId) return
      const club = await invoke<ClubInfo>('get_club_info', { clubId: gameState.userClubId })
      setUserClubInfo(club)
    } catch (error) {
      console.error('Failed to load user club:', error)
    }
  }

  async function loadClubSquad() {
    try {
      if (!selectedClubId) return
      const [club, players] = await Promise.all([
        invoke<ClubInfo>('get_club_info', { clubId: selectedClubId }),
        invoke<PlayerDetail[]>('get_club_squad', { clubId: selectedClubId }),
      ])
      setClubInfo(club)
      setSquad(players)
    } catch (error) {
      console.error('Failed to load club squad:', error)
    }
  }

  // Sort players by position for consistent display
  const sortedSquad = [...squad].sort((a, b) => {
    const positionOrder: Record<string, number> = { GK: 1, CB: 2, FB: 3, MID: 4, FWD: 5, WG: 6 }
    const orderA = positionOrder[a.position] || 99
    const orderB = positionOrder[b.position] || 99
    if (orderA !== orderB) return orderA - orderB
    return (a.name || '').localeCompare(b.name || '')
  })

  // Challenge letter helper functions
  const getGreeting = () => {
    if (tone === 'cordial') return 'presents his warmest compliments'
    if (tone === 'formal') return 'presents his compliments'
    if (tone === 'respectful') return 'most respectfully presents his compliments'
    if (tone === 'confident') return 'presents his compliments with confidence'
    if (tone === 'humble') return 'humbly presents his compliments'
    if (tone === 'competitive') return 'presents his compliments and sporting challenge'
    return 'presents his respects'
  }

  const getMatchDescription = () => {
    if (matchType === 'friendly') return 'a friendly match'
    if (matchType === 'practice') return 'a practice match'
    return 'a challenge match'
  }

  const getVenueText = () => {
    if (venue === 'home') return `at ${userClubInfo?.ground_name || 'our ground'}`
    if (venue === 'away') return `at ${clubInfo?.ground_name || 'your esteemed ground'}`
    return 'at a neutral ground of mutual agreement'
  }

  const getStakesText = () => {
    if (stakes === 'honor') return 'for the honour and glory of our respective clubs'
    if (stakes === 'small') return 'with a modest wager of Five Pounds to add interest to the proceedings'
    if (stakes === 'medium') return 'with a wager of Ten Pounds to heighten the competitive spirit'
    if (stakes === 'trophy') return 'with a handsome trophy to be awarded to the victorious side'
    if (stakes === 'silver-cup') return 'with a fine Silver Cup to be held by the victors for the coming year'
    if (stakes === 'dinner') return 'with the losing side to provide a handsome dinner for the victors'
    if (stakes === 'charity') return 'with all gate receipts to be donated to charitable causes in Sheffield'
    return 'for the honour and glory of our respective clubs'
  }

  const getRulesText = () => {
    if (rulesType === 'sheffield') return 'Sheffield Rules'
    if (rulesType === 'fa') return 'Football Association Rules'
    if (rulesType === 'rugby') return 'Rugby Football Rules'
    return 'Sheffield Rules'
  }

  const getClosing = () => {
    if (tone === 'cordial') return 'We anticipate a most enjoyable and sporting contest, and look forward to your favourable reply.'
    if (tone === 'formal') return 'We trust this proposal meets with your approval and await your response at your earliest convenience.'
    if (tone === 'respectful') return 'We would be most honoured by your acceptance and await your gracious reply.'
    if (tone === 'confident') return 'We are certain this match shall prove a splendid contest, and confidently await your acceptance.'
    if (tone === 'humble') return 'We would be greatly honoured should you accept our humble proposal, and await your reply with anticipation.'
    if (tone === 'competitive') return 'We relish the prospect of testing our mettle against your esteemed club, and eagerly await your acceptance.'
    return 'We are confident our clubs shall provide a spirited exhibition of the football code, and eagerly await your acceptance.'
  }

  const getProposedDateText = () => {
    if (proposedDate) {
      return formatVictorianDate(new Date(proposedDate + 'T00:00:00'))
    }
    // Default to a fortnight from current date if no date selected
    const currentDate = new Date(gameState.currentDate)
    const date = new Date(currentDate)
    date.setDate(date.getDate() + 14)
    return formatVictorianDate(date)
  }

  const formatVictorianDate = (date: Date) => {
    const day = date.getDate()
    const month = date.toLocaleString('en-GB', { month: 'long' })
    const year = date.getFullYear()
    const suffix = day === 1 || day === 21 || day === 31 ? 'st' : day === 2 || day === 22 ? 'nd' : day === 3 || day === 23 ? 'rd' : 'th'
    return `${day}${suffix} ${month}, ${year}`
  }

  const getLetterDate = () => {
    return formatVictorianDate(new Date(gameState.currentDate))
  }

  if (loading) {
    return <div className="clubs-screen"><p>Loading clubs...</p></div>
  }

  const renderMainContent = () => {
    if (!selectedAction) {
      // Default view: Squad list
      return (
        <>
          {clubInfo && (
            <div className="squad-header">
              <div className="club-title-line">
                <h1>{clubInfo.name}</h1>
                <span className="founded-year">Founded {clubInfo.founded_year}</span>
              </div>
            </div>
          )}

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
                </tr>
              ))}
            </tbody>
          </table>

          <div className="squad-stats">
            <p><strong>Total Squad:</strong> {squad.length} players</p>
          </div>
        </>
      )
    }

    // Action-specific content
    switch (selectedAction) {
      case 'view-squad':
        return (
          <>
            {clubInfo && (
              <div className="squad-header">
                <div className="club-title-line">
                  <h1>{clubInfo.name}</h1>
                  <span className="founded-year">Founded {clubInfo.founded_year}</span>
                </div>
              </div>
            )}

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
                  </tr>
                ))}
              </tbody>
            </table>

            <div className="squad-stats">
              <p><strong>Total Squad:</strong> {squad.length} players</p>
            </div>
          </>
        )
      case 'recent-form':
        return (
          <div style={{ padding: '20px' }}>
            <h2>Recent Form</h2>
            <p>Recent match results for {clubInfo?.name} will appear here.</p>
          </div>
        )
      case 'league-position':
        return (
          <div style={{ padding: '20px' }}>
            <h2>League Position</h2>
            <p>Current standings and position for {clubInfo?.name} will appear here.</p>
          </div>
        )
      case 'head-to-head':
        return (
          <div style={{ padding: '20px' }}>
            <h2>Head-to-Head Record</h2>
            <p>Historical record against {clubInfo?.name} will appear here.</p>
          </div>
        )
      case 'send-challenge':
        return (
            <div style={{ padding: '20px' }}>
              {/* The Letter - Main Focus with Victorian styling */}
              <div style={{
                position: 'relative',
                backgroundColor: '#fdfbf7',
                color: '#2c3e50',
                padding: '32px',
                borderRadius: '4px',
                marginBottom: '24px',
                fontSize: '0.9rem',
                lineHeight: '1.9',
                fontFamily: 'Georgia, serif',
                border: '3px double #d4af37',
                boxShadow: '0 4px 16px rgba(0,0,0,0.15), inset 0 0 60px rgba(212,175,55,0.05)',
                backgroundImage: `
                  repeating-linear-gradient(0deg, transparent, transparent 2px, rgba(212,175,55,0.03) 2px, rgba(212,175,55,0.03) 4px),
                  repeating-linear-gradient(90deg, transparent, transparent 2px, rgba(212,175,55,0.03) 2px, rgba(212,175,55,0.03) 4px)
                `
              }}>
                {/* Wax seal decoration */}
                <div style={{
                  position: 'absolute',
                  top: '16px',
                  right: '16px',
                  width: '48px',
                  height: '48px',
                  borderRadius: '50%',
                  background: 'radial-gradient(circle, #8b0000 0%, #5c0000 100%)',
                  border: '2px solid #6b0000',
                  opacity: 0.7,
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                  fontSize: '0.7rem',
                  color: '#d4af37',
                  fontWeight: 'bold',
                  boxShadow: '0 2px 4px rgba(0,0,0,0.3)'
                }}>
                  FC
                </div>

                {/* Sender's address header */}
                <div style={{ marginBottom: '20px', fontSize: '0.85rem', color: '#5a5a5a', fontStyle: 'italic' }}>
                  <p style={{ margin: '0' }}>{userClubInfo?.ground_name || 'Our Ground'}</p>
                  <p style={{ margin: '0' }}>{userClubInfo?.city || 'Sheffield'}, {userClubInfo?.region || 'Yorkshire'}</p>
                </div>

                <p style={{ margin: '0 0 20px 0', textAlign: 'right', fontSize: '0.85rem', color: '#7f8c8d', fontStyle: 'italic' }}>
                  {getLetterDate()}
                </p>

                <p style={{ margin: '0 0 16px 0' }}>
                  Dear Sir,
                </p>
                <p style={{ margin: '0 0 16px 0' }}>
                  The Secretary of <strong>{userClubInfo?.name || 'our Club'}</strong> {getGreeting()} to the Secretary of <strong>{clubInfo?.name}</strong>,
                  and begs to request the honour of {getMatchDescription()} between our respective clubs.
                </p>
                <p style={{ margin: '0 0 16px 0' }}>
                  We propose to meet on <strong>{getProposedDateText()}</strong>, {getVenueText()}, to be played under <strong>{getRulesText()}</strong> ({matchDuration} minutes), {getStakesText()}.
                </p>
                <p style={{ margin: '0 0 16px 0' }}>
                  {getClosing()}
                </p>
                <p style={{ margin: '24px 0 0 0' }}>
                  Your obedient servant,<br/>
                  <strong style={{ marginTop: '8px', display: 'inline-block' }}>Secretary, {userClubInfo?.short_name || userClubInfo?.name || 'F.C.'}</strong>
                </p>
              </div>

              {/* Elegant Form Controls */}
              <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '16px', marginBottom: '20px' }}>
                <div>
                  <label style={{ display: 'block', marginBottom: '6px', fontSize: '0.8rem', color: '#7f8c8d', fontWeight: '600' }}>Match Type</label>
                  <select
                    value={matchType}
                    onChange={(e) => setMatchType(e.target.value)}
                    style={{ width: '100%', padding: '8px 10px', fontSize: '0.9rem', backgroundColor: '#34495e', color: '#ecf0f1', border: '1px solid #555', borderRadius: '4px', cursor: 'pointer' }}
                  >
                    <option value="friendly">Friendly Match</option>
                    <option value="practice">Practice Match</option>
                    <option value="challenge">Challenge Match</option>
                  </select>
                </div>

                <div>
                  <label style={{ display: 'block', marginBottom: '6px', fontSize: '0.8rem', color: '#7f8c8d', fontWeight: '600' }}>Venue</label>
                  <select
                    value={venue}
                    onChange={(e) => setVenue(e.target.value)}
                    style={{ width: '100%', padding: '8px 10px', fontSize: '0.9rem', backgroundColor: '#34495e', color: '#ecf0f1', border: '1px solid #555', borderRadius: '4px', cursor: 'pointer' }}
                  >
                    <option value="home">Home Ground</option>
                    <option value="away">Away Ground</option>
                    <option value="neutral">Neutral Ground</option>
                  </select>
                </div>

                <div>
                  <label style={{ display: 'block', marginBottom: '6px', fontSize: '0.8rem', color: '#7f8c8d', fontWeight: '600' }}>Stakes</label>
                  <select
                    value={stakes}
                    onChange={(e) => setStakes(e.target.value)}
                    style={{ width: '100%', padding: '8px 10px', fontSize: '0.9rem', backgroundColor: '#34495e', color: '#ecf0f1', border: '1px solid #555', borderRadius: '4px', cursor: 'pointer' }}
                  >
                    <option value="honor">Honor and Glory</option>
                    <option value="small">Five Pounds Wager</option>
                    <option value="medium">Ten Pounds Wager</option>
                    <option value="trophy">Handsome Trophy</option>
                    <option value="silver-cup">Silver Cup</option>
                    <option value="dinner">Dinner for Winners</option>
                    <option value="charity">Charitable Donation</option>
                  </select>
                </div>

                <div>
                  <label style={{ display: 'block', marginBottom: '6px', fontSize: '0.8rem', color: '#7f8c8d', fontWeight: '600' }}>Letter Tone</label>
                  <select
                    value={tone}
                    onChange={(e) => setTone(e.target.value)}
                    style={{ width: '100%', padding: '8px 10px', fontSize: '0.9rem', backgroundColor: '#34495e', color: '#ecf0f1', border: '1px solid #555', borderRadius: '4px', cursor: 'pointer' }}
                  >
                    <option value="cordial">Cordial</option>
                    <option value="formal">Formal</option>
                    <option value="respectful">Respectful</option>
                    <option value="confident">Confident</option>
                    <option value="humble">Humble</option>
                    <option value="competitive">Competitive</option>
                    <option value="bold">Bold</option>
                  </select>
                </div>

                <div>
                  <label style={{ display: 'block', marginBottom: '6px', fontSize: '0.8rem', color: '#7f8c8d', fontWeight: '600' }}>Proposed Date</label>
                  <input
                    type="date"
                    value={proposedDate}
                    onChange={(e) => setProposedDate(e.target.value)}
                    min={(() => {
                      const minDate = new Date(gameState.currentDate + 'T00:00:00')
                      minDate.setDate(minDate.getDate() + 5)
                      return minDate.toISOString().split('T')[0]
                    })()}
                    style={{
                      width: '100%',
                      padding: '8px 10px',
                      fontSize: '0.9rem',
                      backgroundColor: '#34495e',
                      color: '#ecf0f1',
                      border: '1px solid #555',
                      borderRadius: '4px',
                      cursor: 'pointer',
                      colorScheme: 'dark'
                    }}
                  />
                </div>

                <div>
                  <label style={{ display: 'block', marginBottom: '6px', fontSize: '0.8rem', color: '#7f8c8d', fontWeight: '600' }}>Rules</label>
                  <select
                    value={rulesType}
                    onChange={(e) => setRulesType(e.target.value)}
                    style={{ width: '100%', padding: '8px 10px', fontSize: '0.9rem', backgroundColor: '#34495e', color: '#ecf0f1', border: '1px solid #555', borderRadius: '4px', cursor: 'pointer' }}
                  >
                    <option value="sheffield">Sheffield Rules</option>
                    <option value="fa">FA Rules</option>
                    <option value="rugby">Rugby Rules</option>
                  </select>
                </div>

                <div style={{ gridColumn: 'span 2' }}>
                  <label style={{ display: 'block', marginBottom: '6px', fontSize: '0.8rem', color: '#7f8c8d', fontWeight: '600' }}>Match Duration</label>
                  <select
                    value={matchDuration}
                    onChange={(e) => setMatchDuration(e.target.value)}
                    style={{ width: '100%', padding: '8px 10px', fontSize: '0.9rem', backgroundColor: '#34495e', color: '#ecf0f1', border: '1px solid #555', borderRadius: '4px', cursor: 'pointer' }}
                  >
                    <option value="60">60 minutes (two halves of 30)</option>
                    <option value="80">80 minutes (two halves of 40)</option>
                    <option value="90">90 minutes (two halves of 45)</option>
                    <option value="120">120 minutes (two halves of 60)</option>
                  </select>
                </div>
              </div>

              {/* Acceptance Likelihood Indicator */}
              <div style={{
                padding: '12px 16px',
                backgroundColor: '#2c3e50',
                borderRadius: '6px',
                marginBottom: '16px',
                border: '1px solid #34495e'
              }}>
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '8px' }}>
                  <span style={{ fontSize: '0.8rem', color: '#bdc3c7', fontWeight: '600' }}>Acceptance Likelihood:</span>
                  <span style={{ fontSize: '0.85rem', color: '#2ecc71', fontWeight: 'bold' }}>Very Likely</span>
                </div>
                <div style={{
                  width: '100%',
                  height: '6px',
                  backgroundColor: '#34495e',
                  borderRadius: '3px',
                  overflow: 'hidden'
                }}>
                  <div style={{
                    width: '85%',
                    height: '100%',
                    backgroundColor: '#2ecc71',
                    transition: 'width 0.3s ease'
                  }}></div>
                </div>
                <p style={{ fontSize: '0.7rem', color: '#95a5a6', marginTop: '8px', marginBottom: '0' }}>
                  Based on club prestige, distance, and current form
                </p>
              </div>

              <div style={{ display: 'flex', gap: '12px', marginBottom: '16px' }}>
                <button
                  className="view-btn"
                  style={{ flex: 1, padding: '12px', fontSize: '0.95rem', backgroundColor: '#27ae60', border: 'none', fontWeight: '600', cursor: 'pointer' }}
                  onClick={async () => {
                    try {
                      if (!proposedDate) {
                        alert('Please select a proposed date for the match.')
                        return
                      }

                      await invoke('send_challenge_letter', {
                        game: gameState,
                        senderClubId: gameState.userClubId,
                        recipientClubId: selectedClubId,
                        currentDate: gameState.currentDate,
                        proposedMatchDate: proposedDate,
                        matchType,
                        venue,
                        stakes,
                        tone,
                        rulesType,
                        matchDuration: parseInt(matchDuration),
                      })

                      // Reload the game state to see the new event
                      const updatedGame = await invoke('get_current_game') as GameState
                      setGameState(updatedGame)

                      // Close the challenge form and show the sent letter modal
                      setSentLetterRecipient(clubInfo?.name || '')
                      setShowSentLetterModal(true)
                      setSelectedAction(null)
                    } catch (error) {
                      console.error('Failed to send challenge letter:', error)
                      alert('Failed to send challenge letter. Please try again.')
                    }
                  }}
                >
                  Send Letter by Post
                </button>
                <button
                  className="view-btn"
                  style={{ flex: 1, padding: '12px', fontSize: '0.95rem', backgroundColor: '#7f8c8d', border: 'none', fontWeight: '600', cursor: 'pointer' }}
                  onClick={() => setSelectedAction(null)}
                >
                  Cancel
                </button>
              </div>

              <div style={{ padding: '12px', backgroundColor: '#34495e', borderRadius: '6px', fontSize: '0.75rem', color: '#bdc3c7', lineHeight: '1.5' }}>
                <strong>Note:</strong> Your letter will be sent via post and a response typically arrives within 3-7 days.
                Acceptance depends on the recipient club's prestige, travel distance, current form, and the terms you've proposed.
              </div>
            </div>
          )
      case 'propose-friendly':
        return (
          <div style={{ padding: '20px' }}>
            <h2>Propose Friendly</h2>
            <p>Propose a friendly match with {clubInfo?.name}.</p>
          </div>
        )
      case 'arrange-series':
        return (
          <div style={{ padding: '20px' }}>
            <h2>Arrange Series</h2>
            <p>Arrange a series of matches with {clubInfo?.name}.</p>
          </div>
        )
      case 'club-history':
        return (
          <div style={{ padding: '20px' }}>
            <h2>Club History</h2>
            <p><strong>Founded:</strong> {clubInfo?.founded_year}</p>
            <p><strong>Ground:</strong> {clubInfo?.ground_name}</p>
            <p><strong>Location:</strong> {clubInfo?.city}, {clubInfo?.region}</p>
          </div>
        )
      case 'ground-info':
        return (
          <div style={{ padding: '20px' }}>
            <h2>Ground Information</h2>
            <p><strong>Ground Name:</strong> {clubInfo?.ground_name}</p>
            <p><strong>Capacity:</strong> {clubInfo?.ground_capacity}</p>
            <p><strong>Location:</strong> {clubInfo?.city}, {clubInfo?.region}</p>
          </div>
        )
      case 'view-map':
        return (
          <div style={{ padding: '20px' }}>
            <h2>View on Map</h2>
            <p>Map showing location of {clubInfo?.ground_name} will appear here.</p>
          </div>
        )
      case 'parish-info':
        return (
          <div style={{ padding: '20px' }}>
            <h2>Parish Information</h2>
            <p>Parish and district information for {clubInfo?.name} will appear here.</p>
          </div>
        )
      default:
        return null
    }
  }

  return (
    <div className="clubs-screen">
      <div className="clubs-container">
        {/* Left sidebar: Club list */}
        <div className="clubs-sidebar">
          <SponsoredCupsPanel gameState={gameState} />
          <h2>Teams</h2>
          <div className="clubs-list">
            {clubs.map(club => (
              <div
                key={club.id}
                className={`club-item ${selectedClubId === club.id ? 'selected' : ''}`}
                onClick={() => {
                  setSelectedClubId(club.id)
                  setSelectedAction(null) // Reset action when changing clubs
                }}
              >
                <div className="club-item-name">{club.name}</div>
              </div>
            ))}
          </div>
        </div>

        {/* Right side: Squad view */}
        <div className="squad-view-container">
          {renderMainContent()}
        </div>
      </div>

      {/* Sent Letter Modal */}
      {showSentLetterModal && (
        <div style={{
          position: 'fixed',
          top: 0,
          left: 0,
          right: 0,
          bottom: 0,
          backgroundColor: 'rgba(0, 0, 0, 0.85)',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          zIndex: 1000
        }}>
          <div style={{
            position: 'relative',
            width: '90%',
            maxWidth: '700px',
            maxHeight: '90vh',
            backgroundColor: '#2c3e50',
            borderRadius: '8px',
            overflow: 'auto',
            boxShadow: '0 8px 32px rgba(0,0,0,0.4)'
          }}>
            {/* Close button */}
            <button
              onClick={() => setShowSentLetterModal(false)}
              style={{
                position: 'absolute',
                top: '16px',
                right: '16px',
                width: '36px',
                height: '36px',
                borderRadius: '50%',
                border: 'none',
                backgroundColor: '#34495e',
                color: '#ecf0f1',
                fontSize: '24px',
                fontWeight: 'bold',
                cursor: 'pointer',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                zIndex: 10,
                transition: 'all 0.2s'
              }}
              onMouseEnter={(e) => {
                e.currentTarget.style.backgroundColor = '#e74c3c'
                e.currentTarget.style.transform = 'scale(1.1)'
              }}
              onMouseLeave={(e) => {
                e.currentTarget.style.backgroundColor = '#34495e'
                e.currentTarget.style.transform = 'scale(1)'
              }}
            >
              ×
            </button>

            {/* Header */}
            <div style={{
              padding: '24px 32px',
              borderBottom: '1px solid #34495e',
              backgroundColor: '#1a252f'
            }}>
              <h2 style={{ margin: 0, color: '#ecf0f1', fontSize: '1.4rem' }}>Letter Dispatched</h2>
              <p style={{ margin: '8px 0 0 0', color: '#95a5a6', fontSize: '0.9rem' }}>
                Your challenge has been sent via post to {sentLetterRecipient}
              </p>
            </div>

            {/* The Letter */}
            <div style={{ padding: '32px' }}>
              <div style={{
                position: 'relative',
                backgroundColor: '#fdfbf7',
                color: '#2c3e50',
                padding: '32px',
                borderRadius: '4px',
                fontSize: '0.9rem',
                lineHeight: '1.9',
                fontFamily: 'Georgia, serif',
                border: '3px double #d4af37',
                boxShadow: '0 4px 16px rgba(0,0,0,0.15), inset 0 0 60px rgba(212,175,55,0.05)',
                backgroundImage: `
                  repeating-linear-gradient(0deg, transparent, transparent 2px, rgba(212,175,55,0.03) 2px, rgba(212,175,55,0.03) 4px),
                  repeating-linear-gradient(90deg, transparent, transparent 2px, rgba(212,175,55,0.03) 2px, rgba(212,175,55,0.03) 4px)
                `
              }}>
                {/* Wax seal */}
                <div style={{
                  position: 'absolute',
                  top: '16px',
                  right: '16px',
                  width: '48px',
                  height: '48px',
                  borderRadius: '50%',
                  background: 'radial-gradient(circle, #8b0000 0%, #5c0000 100%)',
                  border: '2px solid #6b0000',
                  opacity: 0.7,
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                  fontSize: '0.7rem',
                  color: '#d4af37',
                  fontWeight: 'bold',
                  boxShadow: '0 2px 4px rgba(0,0,0,0.3)'
                }}>
                  FC
                </div>

                {/* Sender's address */}
                <div style={{ marginBottom: '20px', fontSize: '0.85rem', color: '#5a5a5a', fontStyle: 'italic' }}>
                  <p style={{ margin: '0' }}>{userClubInfo?.ground_name || 'Our Ground'}</p>
                  <p style={{ margin: '0' }}>{userClubInfo?.city || 'Sheffield'}, {userClubInfo?.region || 'Yorkshire'}</p>
                </div>

                <p style={{ margin: '0 0 20px 0', textAlign: 'right', fontSize: '0.85rem', color: '#7f8c8d', fontStyle: 'italic' }}>
                  {getLetterDate()}
                </p>

                <p style={{ margin: '0 0 16px 0' }}>
                  Dear Sir,
                </p>
                <p style={{ margin: '0 0 16px 0' }}>
                  The Secretary of <strong>{userClubInfo?.name || 'our Club'}</strong> {getGreeting()} to the Secretary of <strong>{sentLetterRecipient}</strong>,
                  and begs to request the honour of {getMatchDescription()} between our respective clubs.
                </p>
                <p style={{ margin: '0 0 16px 0' }}>
                  We propose to meet on <strong>{getProposedDateText()}</strong>, {getVenueText()}, to be played under <strong>{getRulesText()}</strong> ({matchDuration} minutes), {getStakesText()}.
                </p>
                <p style={{ margin: '0 0 16px 0' }}>
                  {getClosing()}
                </p>
                <p style={{ margin: '24px 0 0 0' }}>
                  Your obedient servant,<br/>
                  <strong style={{ marginTop: '8px', display: 'inline-block' }}>Secretary, {userClubInfo?.short_name || userClubInfo?.name || 'F.C.'}</strong>
                </p>
              </div>

              {/* Info note */}
              <div style={{
                marginTop: '24px',
                padding: '16px',
                backgroundColor: '#34495e',
                borderRadius: '6px',
                fontSize: '0.85rem',
                color: '#bdc3c7',
                lineHeight: '1.6'
              }}>
                <p style={{ margin: '0 0 8px 0', fontWeight: '600', color: '#ecf0f1' }}>Letter Successfully Dispatched</p>
                <p style={{ margin: '0' }}>
                  Your challenge letter has been sent via Royal Mail from {userClubInfo?.city || 'Sheffield'}.
                  You should receive a response within 3 days. The news feed will notify you when their reply arrives.
                </p>
              </div>
            </div>

            {/* Footer */}
            <div style={{
              padding: '16px 32px',
              borderTop: '1px solid #34495e',
              backgroundColor: '#1a252f',
              display: 'flex',
              justifyContent: 'flex-end'
            }}>
              <button
                onClick={() => setShowSentLetterModal(false)}
                style={{
                  padding: '10px 24px',
                  fontSize: '0.95rem',
                  backgroundColor: '#27ae60',
                  color: '#fff',
                  border: 'none',
                  borderRadius: '4px',
                  fontWeight: '600',
                  cursor: 'pointer',
                  transition: 'background-color 0.2s'
                }}
                onMouseEnter={(e) => {
                  e.currentTarget.style.backgroundColor = '#229954'
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.backgroundColor = '#27ae60'
                }}
              >
                Close Letter
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}

export default ClubsScreen
