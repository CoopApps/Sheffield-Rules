import React, { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import '../styles/CupManagement.css'

interface Competition {
  id: string
  name: string
  competitionType: string
  season: number
  minDivisionLevel: number | null
  maxDivisionLevel: number | null
  currentRound: number
  totalRounds: number
  isActive: boolean
  winnerClubId: string | null
  runnerUpClubId: string | null
  rulesType: string
  prestigeLevel: string
  startWeek: number
  announcementWeek: number
  drawWeek: number
}

interface CupTie {
  id: string
  competitionId: string
  roundNumber: number
  roundName: string
  tieNumber: number
  homeClubId: string | null
  awayClubId: string | null
  scheduledWeek: number
  played: boolean
  homeScore: number | null
  awayScore: number | null
  winnerClubId: string | null
  matchId: string | null
}

interface EligibleClub {
  id: string
  name: string
  divisionLevel: number
  divisionName: string
}

interface CupManagementProps {
  season: number
}

export function CupManagement({ season }: CupManagementProps) {
  const [competitions, setCompetitions] = useState<Competition[]>([])
  const [selectedCompetition, setSelectedCompetition] = useState<Competition | null>(null)
  const [bracket, setBracket] = useState<CupTie[]>([])
  const [eligibleClubs, setEligibleClubs] = useState<EligibleClub[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [showCreateForm, setShowCreateForm] = useState(false)
  const [editingCompetition, setEditingCompetition] = useState<Competition | null>(null)

  // Create cup form state
  const [cupName, setCupName] = useState('')
  const [minDivision, setMinDivision] = useState<number | null>(1)
  const [maxDivision, setMaxDivision] = useState<number | null>(10)
  const [startWeek, setStartWeek] = useState(12)
  const [announcementWeek, setAnnouncementWeek] = useState(8)
  const [drawWeek, setDrawWeek] = useState(10)
  const [prestigeLevel, setPrestigeLevel] = useState<'high' | 'standard' | 'low'>('standard')
  const [rulesType, setRulesType] = useState('sheffield_rules')

  useEffect(() => {
    loadCompetitions()
  }, [season])

  const loadCompetitions = async () => {
    try {
      setLoading(true)
      setError(null)
      const comps = await invoke<Competition[]>('db_get_competitions_for_season', { season })
      setCompetitions(comps)
      if (comps.length > 0 && !selectedCompetition) {
        setSelectedCompetition(comps[0])
        loadCompetitionDetails(comps[0].id)
      }
    } catch (err) {
      const errorMsg = String(err)
      if (errorMsg.includes('no such table: sheffield_competitions')) {
        // Tables don't exist, run migrations
        try {
          await invoke('db_run_schema_migrations')
          // Retry loading competitions
          const comps = await invoke<Competition[]>('db_get_competitions_for_season', { season })
          setCompetitions(comps)
          if (comps.length > 0 && !selectedCompetition) {
            setSelectedCompetition(comps[0])
            loadCompetitionDetails(comps[0].id)
          }
        } catch (migrationErr) {
          setError(`Failed to initialize database: ${migrationErr}`)
          console.error(migrationErr)
        }
      } else {
        setError(`Failed to load competitions: ${err}`)
        console.error(err)
      }
    } finally {
      setLoading(false)
    }
  }

  const loadCompetitionDetails = async (competitionId: string) => {
    try {
      const [bracketData, eligibleData] = await Promise.all([
        invoke<CupTie[]>('db_get_cup_bracket', { competitionId }),
        invoke<EligibleClub[]>('db_get_eligible_clubs', { competitionId })
      ])
      setBracket(bracketData)
      setEligibleClubs(eligibleData)
    } catch (err) {
      console.error('Failed to load competition details:', err)
    }
  }

  const handleCreateCups = async () => {
    try {
      setError(null)
      // Create cups starting in week 12 (roughly early March for Jan 7 start)
      const newComps = await invoke<Competition[]>('db_create_annual_cups', {
        season,
        startWeek: 12
      })
      setCompetitions(newComps)
      if (newComps.length > 0) {
        setSelectedCompetition(newComps[0])
        loadCompetitionDetails(newComps[0].id)
      }
    } catch (err) {
      setError(`Failed to create cups: ${err}`)
      console.error(err)
    }
  }

  const handleCreateCustomCup = async () => {
    if (!cupName.trim()) {
      setError('Please enter a cup name')
      return
    }

    if (announcementWeek >= drawWeek || drawWeek >= startWeek) {
      setError('Invalid schedule: announcement week < draw week < start week')
      return
    }

    try {
      setError(null)
      const newCup = await invoke<Competition>('db_create_custom_cup', {
        name: cupName,
        season,
        minDivisionLevel: minDivision,
        maxDivisionLevel: maxDivision,
        startWeek,
        announcementWeek,
        drawWeek,
        prestigeLevel,
        rulesType
      })

      await loadCompetitions()
      setSelectedCompetition(newCup)
      loadCompetitionDetails(newCup.id)
      setShowCreateForm(false)

      // Reset form
      setCupName('')
      setMinDivision(1)
      setMaxDivision(10)
      setStartWeek(12)
      setAnnouncementWeek(8)
      setDrawWeek(10)
      setPrestigeLevel('standard')
      setRulesType('sheffield_rules')
    } catch (err) {
      setError(`Failed to create cup: ${err}`)
      console.error(err)
    }
  }

  const handleCancelCreateCup = () => {
    setShowCreateForm(false)
    setCupName('')
    setMinDivision(1)
    setMaxDivision(10)
    setStartWeek(12)
    setAnnouncementWeek(8)
    setDrawWeek(10)
    setPrestigeLevel('standard')
    setRulesType('sheffield_rules')
    setError(null)
  }

  const handleEditCompetition = (comp: Competition) => {
    setEditingCompetition(comp)
    setCupName(comp.name)
    setMinDivision(comp.minDivisionLevel)
    setMaxDivision(comp.maxDivisionLevel)
    setStartWeek(comp.startWeek)
    setAnnouncementWeek(comp.announcementWeek)
    setDrawWeek(comp.drawWeek)
    setPrestigeLevel(comp.prestigeLevel as 'high' | 'standard' | 'low')
    setRulesType(comp.rulesType)
    setError(null)
  }

  const handleUpdateCompetition = async () => {
    if (!editingCompetition) return
    if (!cupName.trim()) {
      setError('Please enter a cup name')
      return
    }

    if (announcementWeek >= drawWeek || drawWeek >= startWeek) {
      setError('Invalid schedule: announcement week < draw week < start week')
      return
    }

    try {
      setError(null)
      const updated = await invoke<Competition>('db_update_cup_competition', {
        competitionId: editingCompetition.id,
        name: cupName,
        minDivisionLevel: minDivision,
        maxDivisionLevel: maxDivision,
        startWeek,
        announcementWeek,
        drawWeek,
        prestigeLevel,
        rulesType
      })

      await loadCompetitions()
      setSelectedCompetition(updated)
      loadCompetitionDetails(updated.id)
      setEditingCompetition(null)

      // Reset form
      setCupName('')
      setMinDivision(1)
      setMaxDivision(10)
      setStartWeek(12)
      setAnnouncementWeek(8)
      setDrawWeek(10)
      setPrestigeLevel('standard')
      setRulesType('sheffield_rules')
    } catch (err) {
      setError(`Failed to update cup: ${err}`)
      console.error(err)
    }
  }

  const handleCancelEdit = () => {
    setEditingCompetition(null)
    setCupName('')
    setMinDivision(1)
    setMaxDivision(10)
    setStartWeek(12)
    setAnnouncementWeek(8)
    setDrawWeek(10)
    setPrestigeLevel('standard')
    setRulesType('sheffield_rules')
    setError(null)
  }

  const handleGenerateDraw = async () => {
    if (!selectedCompetition) return

    try {
      setError(null)
      await invoke('db_generate_cup_draw', {
        competitionId: selectedCompetition.id,
        seeded: false
      })
      await loadCompetitionDetails(selectedCompetition.id)
      await loadCompetitions() // Reload to get updated round count
    } catch (err) {
      setError(`Failed to generate draw: ${err}`)
      console.error(err)
    }
  }

  const handleDeleteCups = async () => {
    if (!confirm('Are you sure you want to delete all cup competitions for this season?')) {
      return
    }

    try {
      setError(null)
      await invoke('db_delete_competitions_for_season', { season })
      setCompetitions([])
      setSelectedCompetition(null)
      setBracket([])
      setEligibleClubs([])
    } catch (err) {
      setError(`Failed to delete cups: ${err}`)
      console.error(err)
    }
  }

  const getDivisionRangeText = (comp: Competition) => {
    if (comp.minDivisionLevel === null || comp.maxDivisionLevel === null) {
      return 'All Divisions'
    }
    if (comp.minDivisionLevel === comp.maxDivisionLevel) {
      return `Division ${comp.minDivisionLevel}`
    }
    return `Divisions ${comp.minDivisionLevel}-${comp.maxDivisionLevel}`
  }

  const getWeekDateEstimate = (week: number) => {
    // Assuming game starts Jan 7, 1867
    const startDate = new Date(1867, 0, 7) // January 7, 1867
    const estimatedDate = new Date(startDate)
    estimatedDate.setDate(startDate.getDate() + (week * 7))
    return estimatedDate.toLocaleDateString('en-GB', {
      year: 'numeric',
      month: 'short',
      day: 'numeric'
    })
  }

  if (loading) {
    return <div className="cup-management loading">Loading cup competitions...</div>
  }

  return (
    <div className="cup-management">
      <div className="cup-header">
        <h2>Cup Competition Management - Season {season}</h2>
        <div className="cup-actions">
          <button onClick={handleCreateCups} className="btn-primary">
            Create Annual Cups
          </button>
          <button onClick={() => setShowCreateForm(true)} className="btn-primary">
            Create Custom Cup
          </button>
          {competitions.length > 0 && (
            <button onClick={handleDeleteCups} className="btn-danger">
              Delete All Cups
            </button>
          )}
        </div>
      </div>

      {error && <div className="error-message">{error}</div>}

      {(showCreateForm || editingCompetition) && (
        <div className="create-cup-form">
          <div className="form-header">
            <h3>{editingCompetition ? 'Edit Cup Competition' : 'Create Custom Cup Competition'}</h3>
            <button onClick={editingCompetition ? handleCancelEdit : handleCancelCreateCup} className="btn-close">×</button>
          </div>

          <div className="form-grid">
            <div className="form-group">
              <label htmlFor="cup-name">Cup Name *</label>
              <input
                id="cup-name"
                type="text"
                value={cupName}
                onChange={(e) => setCupName(e.target.value)}
                placeholder="e.g., Sheffield Challenge Cup 1867"
                className="form-input"
              />
            </div>

            <div className="form-group">
              <label htmlFor="prestige">Prestige Level</label>
              <select
                id="prestige"
                value={prestigeLevel}
                onChange={(e) => setPrestigeLevel(e.target.value as 'high' | 'standard' | 'low')}
                className="form-select"
              >
                <option value="high">High (Major Trophy)</option>
                <option value="standard">Standard</option>
                <option value="low">Low (Minor Cup)</option>
              </select>
              <p className="help-text">Affects club motivation and media attention</p>
            </div>

            <div className="form-section-header">
              <h4>Eligibility Criteria</h4>
            </div>

            <div className="form-group">
              <label htmlFor="min-division">Minimum Division Level</label>
              <input
                id="min-division"
                type="number"
                min="1"
                max="10"
                value={minDivision || ''}
                onChange={(e) => setMinDivision(e.target.value ? parseInt(e.target.value) : null)}
                className="form-input"
                placeholder="Leave empty for all divisions"
              />
              <p className="help-text">Lowest division level (1 = top division)</p>
            </div>

            <div className="form-group">
              <label htmlFor="max-division">Maximum Division Level</label>
              <input
                id="max-division"
                type="number"
                min="1"
                max="10"
                value={maxDivision || ''}
                onChange={(e) => setMaxDivision(e.target.value ? parseInt(e.target.value) : null)}
                className="form-input"
                placeholder="Leave empty for all divisions"
              />
              <p className="help-text">Highest division level (10 = bottom division)</p>
            </div>

            <div className="form-section-header">
              <h4>Competition Schedule</h4>
            </div>

            <div className="form-group">
              <label htmlFor="announcement-week">Announcement Week</label>
              <input
                id="announcement-week"
                type="number"
                min="1"
                max="52"
                value={announcementWeek}
                onChange={(e) => setAnnouncementWeek(parseInt(e.target.value) || 1)}
                className="form-input"
              />
              <p className="help-text">Week when cup is announced (~{getWeekDateEstimate(announcementWeek)})</p>
            </div>

            <div className="form-group">
              <label htmlFor="draw-week">Draw Week</label>
              <input
                id="draw-week"
                type="number"
                min="1"
                max="52"
                value={drawWeek}
                onChange={(e) => setDrawWeek(parseInt(e.target.value) || 1)}
                className="form-input"
              />
              <p className="help-text">Week when draw is made (~{getWeekDateEstimate(drawWeek)})</p>
            </div>

            <div className="form-group">
              <label htmlFor="start-week">First Match Week</label>
              <input
                id="start-week"
                type="number"
                min="1"
                max="52"
                value={startWeek}
                onChange={(e) => setStartWeek(parseInt(e.target.value) || 1)}
                className="form-input"
              />
              <p className="help-text">Week when first round is played (~{getWeekDateEstimate(startWeek)})</p>
            </div>

            <div className="form-group">
              <label htmlFor="rules-type">Rules Type</label>
              <select
                id="rules-type"
                value={rulesType}
                onChange={(e) => setRulesType(e.target.value)}
                className="form-select"
              >
                <option value="sheffield_rules">Sheffield Rules</option>
                <option value="modern_rules">Modern Rules</option>
              </select>
              <p className="help-text">Which ruleset to use for matches</p>
            </div>
          </div>

          <div className="form-actions">
            <button
              onClick={editingCompetition ? handleUpdateCompetition : handleCreateCustomCup}
              className="btn-primary btn-large"
            >
              {editingCompetition ? 'Update Cup Competition' : 'Create Cup Competition'}
            </button>
            <button
              onClick={editingCompetition ? handleCancelEdit : handleCancelCreateCup}
              className="btn-secondary btn-large"
            >
              Cancel
            </button>
          </div>
        </div>
      )}

      {competitions.length === 0 ? (
        <div className="empty-state">
          <p>No cup competitions found for this season.</p>
          <p>Click "Create Annual Cups" to set up the Youdan Cup and Cromwell Cup.</p>
        </div>
      ) : (
        <div className="cup-content">
          <div className="cup-list">
            <h3>Competitions</h3>
            {competitions.map(comp => (
              <div
                key={comp.id}
                className={`cup-item ${selectedCompetition?.id === comp.id ? 'selected' : ''} ${comp.prestigeLevel === 'high' ? 'prestigious' : ''}`}
                onClick={() => {
                  setSelectedCompetition(comp)
                  loadCompetitionDetails(comp.id)
                }}
              >
                <div className="cup-item-header">
                  <h4>{comp.name}</h4>
                  {comp.prestigeLevel === 'high' && <span className="badge-prestige">Prestigious</span>}
                </div>
                <div className="cup-item-details">
                  <span className="division-range">{getDivisionRangeText(comp)}</span>
                  <span className="status">{comp.isActive ? 'Active' : 'Completed'}</span>
                </div>
                <div className="cup-item-progress">
                  <span>Round {comp.currentRound} of {comp.totalRounds || '?'}</span>
                </div>
              </div>
            ))}
          </div>

          {selectedCompetition && (
            <div className="cup-details">
              <div className="cup-details-header">
                <h3>{selectedCompetition.name}</h3>
                <div className="cup-details-actions">
                  <button
                    onClick={() => handleEditCompetition(selectedCompetition)}
                    className="btn-secondary"
                  >
                    Edit Cup
                  </button>
                  <button
                    onClick={handleGenerateDraw}
                    className="btn-secondary"
                    disabled={bracket.length > 0}
                  >
                    {bracket.length > 0 ? 'Draw Generated' : 'Generate Draw'}
                  </button>
                </div>
              </div>

              <div className="cup-info-grid">
                <div className="info-item">
                  <label>Eligibility:</label>
                  <span>{getDivisionRangeText(selectedCompetition)}</span>
                </div>
                <div className="info-item">
                  <label>Participants:</label>
                  <span>{eligibleClubs.length} clubs</span>
                </div>
                <div className="info-item">
                  <label>Rules:</label>
                  <span>{selectedCompetition.rulesType.replace('_', ' ')}</span>
                </div>
                <div className="info-item">
                  <label>Announcement:</label>
                  <span>Week {selectedCompetition.announcementWeek} (~{getWeekDateEstimate(selectedCompetition.announcementWeek)})</span>
                </div>
                <div className="info-item">
                  <label>Draw Date:</label>
                  <span>Week {selectedCompetition.drawWeek} (~{getWeekDateEstimate(selectedCompetition.drawWeek)})</span>
                </div>
                <div className="info-item">
                  <label>Start Date:</label>
                  <span>Week {selectedCompetition.startWeek} (~{getWeekDateEstimate(selectedCompetition.startWeek)})</span>
                </div>
              </div>

              {eligibleClubs.length > 0 && (
                <div className="eligible-clubs">
                  <h4>Eligible Clubs ({eligibleClubs.length})</h4>
                  <div className="clubs-grid">
                    {eligibleClubs.map(club => (
                      <div key={club.id} className="club-card">
                        <span className="club-name">{club.name}</span>
                        <span className="club-division">{club.divisionName}</span>
                      </div>
                    ))}
                  </div>
                </div>
              )}

              {bracket.length > 0 && (
                <div className="cup-bracket">
                  <h4>Draw - Round 1</h4>
                  <div className="ties-list">
                    {bracket
                      .filter(tie => tie.roundNumber === 1)
                      .map(tie => (
                        <div key={tie.id} className="tie-card">
                          <div className="tie-number">Tie {tie.tieNumber}</div>
                          <div className="tie-matchup">
                            <span className="team home">{tie.homeClubId || 'TBD'}</span>
                            <span className="vs">vs</span>
                            <span className="team away">{tie.awayClubId || '(Bye)'}</span>
                          </div>
                          <div className="tie-info">
                            <span>Week {tie.scheduledWeek}</span>
                            {tie.played && <span className="played-badge">Played</span>}
                          </div>
                        </div>
                      ))}
                  </div>
                </div>
              )}
            </div>
          )}
        </div>
      )}
    </div>
  )
}
