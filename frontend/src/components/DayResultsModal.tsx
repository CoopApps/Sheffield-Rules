import React from 'react'
import { GameState, DayProcessingResult, EventResult, GameEvent } from '../types/GameState'
import '../styles/DayResultsModal.css'

interface DayResultsModalProps {
  results: DayProcessingResult
  onClose: () => void
  gameState: GameState
}

export function DayResultsModal({ results, onClose, gameState }: DayResultsModalProps) {
  const formatDate = (dateStr: string) => {
    try {
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

  const getTeamName = (teamId: string) => {
    const club = gameState.clubs.find(c => c.id === teamId)
    return club ? club.name : teamId
  }

  return (
    <div className="modal-overlay">
      <div className="modal-content day-results">
        <h2>Day Summary: {formatDate(gameState.currentDate)}</h2>

        {results.autoProcessed.length > 0 && (
          <section className="results-section">
            <h3>Day Events</h3>
            <div className="results-list">
              {results.autoProcessed.map((result) => (
                <div key={result.eventId} className="result-card">
                  <p className="result-summary">{result.summary}</p>
                  {result.data && result.data.matchId && (
                    <div className="match-details">
                      <span className="team home-team">{getTeamName(result.data.homeTeam)}</span>
                      <span className="score">
                        {result.data.homeScore} - {result.data.awayScore}
                      </span>
                      <span className="team away-team">{getTeamName(result.data.awayTeam)}</span>
                    </div>
                  )}
                </div>
              ))}
            </div>
          </section>
        )}

        {results.requireUserAction.length > 0 && (
          <section className="pending-section">
            <h3>Pending Actions</h3>
            <p className="pending-count">
              You have {results.requireUserAction.length} action(s) requiring attention
            </p>
            <div className="pending-list">
              {results.requireUserAction.map((event) => (
                <div key={event.id} className="pending-item">
                  <span className="event-type">
                    {event.eventType.type === 'Match' ? (
                      <>
                        <span className="badge">Match</span>
                        Your team has a match to play
                      </>
                    ) : event.eventType.type === 'MediaInquiry' ? (
                      <>
                        <span className="badge">Media</span>
                        A media inquiry requires your response
                      </>
                    ) : event.eventType.type === 'PlayerNegotiation' ? (
                      <>
                        <span className="badge">Negotiation</span>
                        A player negotiation requires your decision
                      </>
                    ) : event.eventType.type === 'CupAnnouncement' ? (
                      <>
                        <span className="badge cup">Cup News</span>
                        <div className="cup-announcement">
                          <h4>{event.eventType.data?.title || 'Cup Announcement'}</h4>
                          <p>{event.eventType.data?.description}</p>
                        </div>
                      </>
                    ) : event.eventType.type === 'CupDraw' ? (
                      <>
                        <span className="badge cup">Cup Draw</span>
                        <div className="cup-draw">
                          <h4>{event.eventType.data?.title || 'Cup Draw'}</h4>
                          <p>{event.eventType.data?.description}</p>
                        </div>
                      </>
                    ) : (
                      <>
                        <span className="badge">Event</span>
                        An event requires your attention
                      </>
                    )}
                  </span>
                </div>
              ))}
            </div>
          </section>
        )}

        <div className="modal-footer">
          <button
            className="close-btn"
            onClick={onClose}
            disabled={!results.allComplete}
            title={results.allComplete ? "Continue to next day" : "Complete pending actions first"}
          >
            {results.allComplete ? 'Continue' : 'Complete pending actions first'}
          </button>
        </div>
      </div>
    </div>
  )
}
